mod codegen;
mod ir;
mod ir_gen;
mod lexer;
mod parser;
// mod vm;

fn main() {
    let matches = Command::new("RV32I")
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

    let max_memory_kib: u64 = *matches.get_one::<u64>("max_memory").unwrap();
    let max_memory_words: u64 = max_memory_kib * 256 & 0xFFFFFFFF;
    let max_memory: usize = max_memory_words
        .try_into()
        .expect("Maximum memory too large.");

    let min_memory_kib: u64 = *matches.get_one::<u64>("min_memory").unwrap();
    let min_memory_words: u64 = min_memory_kib * 256 & 0xFFFFFFFF;
    let min_memory: usize = min_memory_words
        .try_into()
        .expect("Minimum memory too large.");

    let tokens = vec![
        "fn".to_string(),
        "main".to_string(),
        "(".to_string(),
        ")".to_string(),
        "{".to_string(),
        "u8".to_string(),
        "my_u8".to_string(),
        "=".to_string(),
        "123".to_string(),
        "i32".to_string(),
        "my_i32".to_string(),
        "=".to_string(),
        "456".to_string(),
        "f32".to_string(),
        "my_f32".to_string(),
        "=".to_string(),
        "3.14".to_string(),
        "return".to_string(),
        "(".to_string(),
        "my_i32".to_string(),
        ")".to_string(),
        "}".to_string(),
    ];

    let mut parser = parser::parser::Parser::new(tokens);
    let program = parser.parse();

    println!("{:?}", program);
}
