use clap::{Arg, Command, value_parser};

mod codegen;
mod ir;
mod ir_gen;
mod lexer;
mod parser;
// mod vm;

fn main() {
    let matches = Command::new("NSHC-VMJIT")
        .arg(
            Arg::new("max_memory")
                .short('x')
                .long("max_memory")
                .value_name("MAXIMUM MEMORY")
                .help("Sets the maximum amount of memory the VM will use in KiB.")
                .default_value("1024")
                .value_parser(value_parser!(u64).range(1..)),
        )
        .arg(
            Arg::new("min_memory")
                .short('n')
                .long("min_memory")
                .value_name("MINIMUM MEMORY")
                .help("Sets the minimum amount of memory the VM will allocate in KiB.")
                .default_value("1024")
                .value_parser(value_parser!(u64).range(1..)),
        )
        .get_matches();

    let _max_memory_kib: u64 = *matches.get_one::<u64>("max_memory").unwrap();
    let _max_memory_words: u64 = _max_memory_kib * 256 & 0xFFFFFFFF;
    let _max_memory: usize = _max_memory_words
        .try_into()
        .expect("Maximum memory too large.");

    let _min_memory_kib: u64 = *matches.get_one::<u64>("min_memory").unwrap();
    let _min_memory_words: u64 = _min_memory_kib * 256 & 0xFFFFFFFF;
    let _min_memory: usize = _min_memory_words
        .try_into()
        .expect("Minimum memory too large.");

    let code = "fn main() { u8 my_u8 = 123; i32 my_i32 = 456; f32 my_f32 = 3.14; return(my_i32); }";
    let mut tokenizer = lexer::lexer::Tokenizer::new(code);
    let mut tokens = Vec::new();
    loop {
        let token = tokenizer.next_token();
        if token == lexer::token::Token::EOF {
            break;
        }
        tokens.push(token);
    }

    let mut parser = parser::parser::Parser::new(tokens);
    let program = parser.parse();

    println!("{:?}", program);
}