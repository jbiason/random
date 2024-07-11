use logos::Logos;

#[derive(Logos, Debug)]
#[logos(skip r"[ \t\n\r]")]
enum Token<'a> {
    #[regex(r#""[^"]+""#, |lex| lex.slice().trim_start_matches('"').trim_end_matches('"'))]
    #[regex("[a-zA-Z0-9]+", |lex| lex.slice())]
    Keyword(&'a str),

    #[regex(r#"//[^\n]*"#, |lex| lex.slice())]
    Comment(&'a str),

    #[token(";")]
    End,

    #[token("{")]
    DictStart,

    #[token("}")]
    DictEnd,

    #[token("(")]
    ListStart,

    #[token(")")]
    ListEnd,

}

fn main() {
    let lex = Token::lexer("variable \"value is weird(but cool)[not so much]\" ;");
    let content = lex.collect::<Vec<_>>();
    println!("{content:#?}");

    let lex = Token::lexer("var1 2;\nvar2 2;");
    let content = lex.collect::<Vec<_>>();
    println!("{content:#?}");

    let lex = Token::lexer(r#"var 1 2 3;\nvar2 (1 2 3);"#);
    let content = lex.collect::<Vec<_>>();
    println!("{content:#?}");

    let lex = Token::lexer("variables (\"phi\" \"meanT\");\nruns\n( 1 2 3 );");
    let content = lex.collect::<Vec<_>>();
    println!("{content:#?}");

    let source = std::fs::read("src/example.foam").unwrap();
    let lex = Token::lexer(&std::str::from_utf8(&source).unwrap());
    let content = lex.collect::<Vec<_>>();
    println!("{content:#?}");
}
