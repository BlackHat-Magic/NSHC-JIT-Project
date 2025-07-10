mod lexer;
mod parser;
mod ir;
mod ir_gen;
mod codegen;
// mod vm;

fn main() {
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
