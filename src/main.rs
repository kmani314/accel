mod entry;
mod tag;
mod parse;

fn main() {
    let input: String = "<terms !anki Terms to know></terms ------------->".to_string();
    let tokens = parse::lex_string(&input);
    let tags = parse::parse(&tokens);
    println!("{:?}", tags);
}
