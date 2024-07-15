use logos::Logos;

#[derive(Logos, Debug)]
#[logos(skip r"[ \t\n\r]")]
#[allow(dead_code)]
enum Token<'a> {
    #[regex(r#"\/\*[^\/\*]*\*\/"#, |lex| lex.slice())]
    MultilineComment(&'a str),

    #[regex(r#"//[^\n]*"#, |lex| lex.slice())]
    Comment(&'a str),

    #[regex(r#""[^"]+""#, |lex| lex.slice().trim_start_matches('"').trim_end_matches('"'))]
    #[regex("[a-zA-Z0-9]+", |lex| lex.slice())]
    Keyword(&'a str),

    // #[regex(r#""[^"]+"[ \t\n\r]+\{"#, |lex| lex.slice().trim_start_matches('"').trim_end_matches('"'))]
    // #[regex(r#"[a-zA-Z0-9]+[ \t\n\r]+\{"#, |lex| lex.slice())]
    // DictStart(&'a str),
    #[token("{")]
    DictStart,

    #[token("}")]
    DictEnd,

    #[token("(")]
    ListStart,

    #[token(")")]
    ListEnd,

    #[token(";")]
    End,
}

fn print(source: &str) {
    let lex = Token::lexer(source);
    let content = lex.collect::<Vec<_>>();
    println!("{source}:\n{content:?}");
    println!();
}

fn main() {
    print("variable \"value is weird(but cool)[not so much]\" ;");
    print("dict { dict { var value; } }");
    print("var1 2;\nvar2 2;");
    print(r#"var 1 2 3;\nvar2 (1 2 3);"#);
    print("variables (\"phi\" \"meanT\");\nruns\n( 1 2 3 );");

    let source = std::fs::read("src/example.foam").unwrap();
    print(&std::str::from_utf8(&source).unwrap());

    print("/* multiline\ncomment*/var value;");
    print("## 123;");
}
