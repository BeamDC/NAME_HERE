/*
 * read in the buffer contents. split them into tokens
 */

enum TokenType {
    Operator,
    Variable,
    Function,
    Type,
    ControlFlow,
    Comment,
    Lparen,
    Rparen,
    Lsquare,
    Rsquare,
    Lcurly,
    Rcurly,
    Langled,
    Rangled,
}

struct Token {
    value: String,
    species: TokenType, // 'type' didn't work, so I found a synonym
}

fn split_contents(src: &String) -> Vec<String> {
    src.split_whitespace().collect()
}

pub fn tokenize(contents: &String) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let src = split_contents(&contents);



    tokens
}