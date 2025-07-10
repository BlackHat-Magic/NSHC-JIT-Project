#include <stdint.h>
#include <stdio.h>
#include <signal.h>

// Linux-specific
#include <stdlib.h>
#include <unistd.h>
#include <fcntl.h>
#include <sys/time.h>
#include <sys/types.h>
#include <sys/termios.h>
#include <sys/mman.h>

#define MEMORY_MAX (1 << 16)

// ===== input stuff =====

struct termios original_tio;

void disable_input_buffering()
{
    tcgetattr(STDIN_FILENO, &original_tio);
    struct termios new_tio = original_tio;
    new_tio.c_lflag &= ~ICANON & ~ECHO;
    tcsetattr(STDIN_FILENO, TCSANOW, &new_tio);
}

void restore_input_buffering()
{
    tcsetattr(STDIN_FILENO, TCSANOW, &original_tio);
}

uint16_t check_key()
{
    fd_set readfds;
    FD_ZERO(&readfds);
    FD_SET(STDIN_FILENO, &readfds);

    struct timeval timeout;
    timeout.tv_sec = 0;
    timeout.tv_usec = 0;
    return select(1, &readfds, NULL, NULL, &timeout) != 0;
}

// allows us to poll the state of keyboard
// program stays responsive while waiting for input
// instead of halting execution while waiting for input

enum {
    MR_KBSR = 0xFE00,   // keyboard status
    MR_KBDR = 0xFE02    // keyboard data
};

void handle_interrupt (int signal) {
    restore_input_buffering ();
    printf ("\n");
    exit (-2);
}

// ===== end input stuff =====

int running = 1;

// 2^16 = 65,536 memory registers
uint16_t memory[MEMORY_MAX];

// 16 registers
enum {
    R_R0 = 0,
    R_R1,
    R_R2,
    R_R3,
    R_R4,
    R_R5,
    R_R6,
    R_R7,
    R_PC,       // program counter
    R_COND,
    R_COUNT
};
uint16_t reg[R_COUNT];

void mem_write (uint16_t address, uint16_t val) {
    memory[address] = val;
}

uint16_t mem_read (uint16_t address) {
    if (address == MR_KBSR) {
        if (check_key ()) {
            memory[MR_KBSR] = (1 << 15);
            memory[MR_KBDR] = getchar ();
        } else {
            memory [MR_KBSR] = 0;
        }
    }
    return (memory[address]);
}

// opcodes
enum {
    OP_BR = 0,  // branch
    OP_ADD,     // add
    OP_LD,      // load
    OP_ST,      // store
    OP_JSR,     // jump register
    OP_AND,     // bitwise and
    OP_LDR,     // load register
    OP_STR,     // store register
    OP_RTI,     // unused
    OP_NOT,     // bitwise not
    OP_LDI,     // load indirect
    OP_STI,     // store indirect
    OP_JMP,     // jump
    OP_RES,     // reserved (unused)
    OP_LEA,     // load effective address
    OP_TRAP,    // execute trap
};

enum {
    TRAP_GETC = 0x20,   // get character from keyboard; not echoed to terminal
    TRAP_OUT,           // output a character
    TRAP_PUTS,          // output a word string
    TRAP_IN,            // get character from keyboard, echoed to terminal
    TRAP_PUTSP,         // output byte string
    TRAP_HALT           // halt program
};

// Condition flags
// indicates sign of previous calculation
enum {
    FL_POS = 1 << 0,    // Positive
    FL_ZRO = 1 << 1,    // Zero
    FL_NEG = 1 << 2,    // Negative
};

/* ===== opcode implementations ===== */

uint16_t sign_extend (uint16_t x, int bit_count) {
    if ((x >> (bit_count - 1)) & 1) {
        x |= (0xffff << bit_count);
    }
    return (x);
}

void update_flags (uint16_t r) {
    if (reg[r] == 0) {
        reg[R_COND] = FL_ZRO;
    } else if (reg[r] >> 15) {
        reg[R_COND] = FL_NEG;
    } else {
        reg[R_COND] = FL_POS;
    }
}

// opcode 0
void op_branch (uint16_t instruction) {
    // Encoding:
    //      opcode  cond    pc offset
    //  0b  0000    000     000000000
    uint16_t pc_offset = sign_extend (instruction & 0x1FF, 9);
    uint16_t cond_flag = (instruction >> 9) & 0x7;
    if (cond_flag & reg[R_COND]) {
        reg[R_PC] += pc_offset;
    }
}

// opcode 1
void op_add (uint16_t instruction) {
    // Encoding:
    //      opcode  dest.       addend1     imm?...
    // 0b   0001    000         000         0   00  000 register mode
    // 0b   0001    000         000         1   00000   immediate mode

    uint16_t r0 = (instruction >> 9) & 0x7;     // destination register
    uint16_t r1 = (instruction >> 6) & 0x7;     // first operand
    uint16_t imm = (instruction >> 5) & 0x1;    // immediate?

    if (imm) {
        uint16_t imm5 = sign_extend (instruction & 0x1F, 5);
        reg[r0] = reg[r1] + imm5;
    } else {
        uint16_t r2 = instruction & 0x7;
        reg[r0] = reg[r1] + reg[r2];
    }

    update_flags (r0);
}

// opcode 2
void op_load (uint16_t instruction) {
    //  Encoding:
    //      opcode  dest.   pc offset
    //  0b  0010    000     000000000
    uint16_t r0 = (instruction >> 9) & 0x7;
    uint16_t pc_offset = sign_extend (instruction & 0x1FF, 9);
    reg[r0] = mem_read (reg[R_PC] + pc_offset);
}

// opcode 3
void op_store (uint16_t instruction) {
    //  Encoding:
    //      opcode  dest.   pc offset
    //  0b  0011    000     000000000
    uint16_t r0 = (instruction >> 9) & 0x7;
    uint16_t pc_offset = sign_extend (instruction & 0x1FF, 9);
    mem_write (reg[R_PC] + pc_offset, reg[r0]);
    update_flags (r0);
}

// opcode 4
void op_jump_register (uint16_t instruction) {
    //  Encoding:
    //      opcode  long    address...?
    //  0b  0100    0       00000000000
    uint16_t long_flag = (instruction >> 11) & 1;
    reg[R_R7] = reg[R_PC];
    if (long_flag) {
        uint16_t long_pc_offset = sign_extend (instruction & 0x7FF, 11);
        reg[R_PC] += long_pc_offset;
    } else {
        uint16_t r1 = (instruction >> 6) & 0x7;
        reg[R_PC] = reg[r1];
    }
}

// opcode 5
void op_and (uint16_t instruction) {
    // Encoding:
    //      opcode  dest.       operand1    imm     operand2
    //  0b  0101    000         000         0       00  000 register mode
    //  0b  0101    000         000         1       00000   immediate mode
    uint16_t r0 = (instruction >> 9) & 0x7;     // destination register
    uint16_t r1 = (instruction >> 6) & 0x7;     // first operand
    uint16_t imm = (instruction >> 5) & 0x1;    // immediate?

    if (imm) {
        uint16_t imm5 = sign_extend (instruction & 0x1F, 5);
        reg[r0] = reg[r1] & imm5;
    } else {
        uint16_t r2 = instruction & 0x7;
        reg[r0] = reg[r1] & reg[r2];
    }
    update_flags (r0);
}

// opcode 6
void op_load_register (uint16_t instruction) {
    //  Encoding:
    //      opcode  dest.   source  offset...?
    //  0b  0110    000     000     000000
    uint16_t r0 = (instruction >> 9) & 0x7;
    uint16_t r1 = (instruction >> 6) & 0x7;
    uint16_t offset = sign_extend (instruction & 0x3F, 6);
    reg[r0] = mem_read (reg[r1] + offset);
    update_flags (r0);
}

// opcode 7
void op_store_register (uint16_t instruction) {
    //  Encoding:
    //      opcode  dest.   source  offset
    //  0b  0111    000     000     000000
    uint16_t r0 = (instruction >> 9) & 0x7;
    uint16_t r1 = (instruction >> 6) & 0x7;
    uint16_t offset = sign_extend (instruction & 0x3F, 6);
    mem_write(reg[r1] + offset, reg[r0]);
    update_flags (r0);
}

// opcode 9
void op_not (uint16_t instruction) {
    // Encoding:
    //      opcode  dest.   operand     unused
    //  0b  1001    000     000         000000
    uint16_t r0 = (instruction >> 9) & 0x7;     // destination register
    uint16_t r1 = (instruction >> 6) & 0x7;     // source register

    reg[r0] = ~reg[r1];
    update_flags (r0);
}

// opcode 10
void op_load_indirect (uint16_t instruction) {
    // Encoding:
    //      opcode  dest.   pc offset
    // 0b   1010    000     000000000

    uint16_t r0 = (instruction >> 9) & 0x7;
    uint16_t offset = sign_extend (instruction & 0x1FF, 9);
    reg[r0] = mem_read (mem_read (reg[R_PC] + offset));
    update_flags (r0);
}

// opcode 11
void op_store_indirect (uint16_t instruction) {
    //  Encoding:
    //      opcode  dest.   pc_offset
    //  0b  1011    000     000000000
    uint16_t r0 = (instruction >> 9) & 0x7;
    uint16_t pc_offset = sign_extend (instruction & 0x1FF, 9);
    mem_write (mem_read (reg[R_PC] + pc_offset), reg[r0]);
}

// opode 12
void op_jump (uint16_t instruction) {
    //  Encoding:
    //      opcode  unused...?  address
    //  0b  1100    000000000   000
    uint16_t r1 = (instruction >> 6) & 0x7;
    reg[R_PC] = reg[r1];
}

// opcode 14
void op_load_effective_address (uint16_t instruction) {
    //  Encoding:
    //      opcode  dest    pc offset
    //  0b  1110    000     000000000
    uint16_t r0 = (instruction >> 9) & 0x7;
    uint16_t pc_offset = sign_extend (instruction & 0x1FF, 9);
    reg[r0] = reg[R_PC] + pc_offset;
    update_flags (r0);
}

// ===== trap implementations =====

void trap_getc () {
    reg[R_R0] = (uint16_t) getchar ();
    update_flags (R_R0);
}

void trap_out () {
    putc ((char) reg[R_R0], stdout);
    fflush (stdout);
}

void trap_puts () {
    // index into memory starting at R_R0
    uint16_t* c = memory + reg[R_R0];
    while (*c) {
        putc ((char) *c, stdout);
        c++;
    }
    fflush (stdout);
}

void trap_in () {
    char c = getchar ();
    putc (c, stdout);
    fflush (stdout);
    reg[R_R0] = (uint16_t) c;
    update_flags (R_R0);
}

void trap_putsp () {
    uint16_t* c = memory + reg[R_R0];
    while (*c) {
        char char1 = (*c) & 0xFF;
        putc (char1, stdout);
        char char2 = (*c) >> 8;
        if (char2) putc (char2, stdout);
        c++;
    }
    fflush (stdout);
}

void trap_halt () {
    puts ("HALT");
    fflush (stdout);
    running = 0;
}

// opcode 15
void op_trap (uint16_t instruction) {
    // Encoding
    //      opcode  trapcode    payload
    //  0b  1111    0000        00000000
    reg[R_R7] = reg[R_PC];
    switch (instruction & 0xFF) {
        case TRAP_GETC:
            trap_getc ();
            break;
        case TRAP_OUT:
            trap_out ();
            break;
        case TRAP_PUTS:
            trap_puts ();
            break;
        case TRAP_IN:
            trap_in ();
            break;
        case TRAP_PUTSP:
            trap_putsp ();
            break;
        case TRAP_HALT:
            trap_halt ();
            break;
    }
}

// ====== end trap implementations =====

// ====== end opcode implementations =====

// ===== read images =====

uint16_t swap16 (uint16_t x) {
    return (x << 8) | (x >> 8);
}

void read_image_file (FILE* file) {
    // where in memory to put image
    uint16_t origin;
    fread (&origin, sizeof (origin), 1, file);
    origin = swap16 (origin);

    // maximum file size; only one fread
    uint16_t max_read = MEMORY_MAX - origin;
    uint16_t* p = memory + origin;
    size_t read = fread (p, sizeof (uint16_t), max_read, file);

    // swap to little endian
    while (read-- > 0) {
        *p = swap16 (*p);
        p++;
    }
}

int read_image (const char* image_path) {
    FILE* file = fopen (image_path, "rb");
    if (!file) {
        return (0);
    }
    read_image_file (file);
    fclose (file);
    return (1);
}

// ===== end read images =====

int main (int argc, char** argv) {
    // load arguments
    if (argc < 2) {
        printf ("Usage: lc3 [image-file1] ...\n");
        exit (2);
    }
    for (int i = 1; i < argc; i++) {
        if (!read_image (argv[i])) {
            printf ("Failed to load image: %s\n", argv[i]);
            exit (1);
        }
    }

    // setup
    signal (SIGINT, handle_interrupt);
    disable_input_buffering ();
    
    reg[R_COND] = FL_ZRO;

    enum { PC_START = 0x3000 };
    reg[R_PC] = PC_START;

    // int running = 1;
    while (running) {
        // read the current instruction and increment PC
        uint16_t instruction = mem_read (reg[R_PC]++);
        uint16_t op = instruction >> 12;

        switch (op) {
            case OP_ADD:
                op_add (instruction);
                break;
            case OP_AND:
                op_and (instruction);
                break;
            case OP_NOT:
                op_not (instruction);
                break;
            case OP_BR:
                op_branch (instruction);
                break;
            case OP_JMP:
                op_jump (instruction);
                break;
            case OP_JSR:
                op_jump_register (instruction);
                break;
            case OP_LD:
                op_load (instruction);
                break;
            case OP_LDI:
                op_load_indirect (instruction);
                break;
            case OP_LDR:
                op_load_register (instruction);
                break;
            case OP_LEA:
                op_load_effective_address (instruction);
                break;
            case OP_ST:
                op_store (instruction);
                break;
            case OP_STI:
                op_store_indirect (instruction);
                break;
            case OP_STR:
                op_store_register (instruction);
                break;
            case OP_TRAP:
                op_trap (instruction);
                break;
            case OP_RES:
            case OP_RTI:
            default:
                abort ();
                break;
        }
    }
    // shut down

    restore_input_buffering ();
}