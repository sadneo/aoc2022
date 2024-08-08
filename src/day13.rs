#[derive(Debug, PartialEq)]
enum Token {
    LeftBracket,
    RightBracket,
    Integer(u8),
}

#[derive(Debug)]
enum Value {
    Integer(u8),
    List(Vec<Value>),
}

pub fn run(input: &str) {
    let result1: usize = input
        .split("\n\n")
        .enumerate()
        .filter_map(|(index, pair)| {
            let pair: Vec<&str> = pair.split('\n').collect();

            let lhs = parse(pair[0]);
            let rhs = parse(pair[1]);
            println!("{:?}\n{:?}", rhs, lhs);
            if compare(lhs, rhs) {
                Some(index)
            } else {
                None
            }
        })
        .sum();

    println!("{}", result1);
}

fn parse(line: &str) -> Value {
    let tokens = tokenize(line);
    parse_brackets(&tokens, 0)
}

/*
As you go through the tokens,
Left bracket creates new vector, and anything that passes in until a right bracket is put into this vector.
After a left bracket,
- A comma is a syntax error.
- An integer is added to the vector.
- A right bracket closes the vector and returns
- A left bracket creates a new function
*/

fn parse_brackets(tokens: &Vec<Token>, mut index: usize) -> Value {
    assert!(tokens[index] == Token::LeftBracket);
    index += 1;
    let mut vector: Vec<Value> = vec![];
    while let Some(token) = tokens.get(index) {
        match token {
            Token::Integer(n) => {
                vector.push(Value::Integer(*n));
                index += 1;
            }
            Token::LeftBracket => {
                parse_brackets(tokens, index);
                index += 1;
            }
            Token::RightBracket => {
                return Value::List(vector);
            }
        }
    }

    unreachable!()
}

fn tokenize(line: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let mut bytes = line.bytes().peekable();
    while let Some(byte) = bytes.next() {
        match byte {
            b'0'..=b'9' => {
                let mut utf8_vec = vec![byte];
                while bytes.peek().unwrap().is_ascii_digit() {
                    utf8_vec.push(bytes.next().unwrap());
                }
                let integer = String::from_utf8(utf8_vec).unwrap().parse().unwrap();
                tokens.push(Token::Integer(integer))
            }
            b'[' => tokens.push(Token::LeftBracket),
            b']' => tokens.push(Token::RightBracket),
            b',' => continue,
            _ => unreachable!(),
        }
    }
    tokens
}

fn compare(lhs: Value, rhs: Value) -> bool {
    true
}
