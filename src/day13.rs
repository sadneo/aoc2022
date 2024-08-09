use core::cmp::Ordering;

#[derive(Debug, PartialEq)]
enum Token {
    LeftBracket,
    RightBracket,
    Integer(u8),
}

#[derive(Debug, PartialEq)]
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

            if compare(&lhs, &rhs).expect("Should reach a conclusion on order") {
                Some(index + 1)
            } else {
                None
            }
        })
        .sum();

    println!("{}", result1);

    let mut values: Vec<Value> = input
        .split('\n')
        .filter_map(|line| {
            if line.is_empty() {
                return None;
            }
            Some(parse(line))
        })
        .collect();

    values.push(parse("[[2]]"));
    values.push(parse("[[6]]"));

    values.sort_by(|lhs, rhs| {
        let value = compare(lhs, rhs).expect("Should reach a conclusion on order");
        match value {
            true => Ordering::Less,
            false => Ordering::Greater,
        }
    });

    let v1 = values.iter().position(|x| *x == parse("[[2]]")).unwrap() + 1;
    let v2 = values.iter().position(|x| *x == parse("[[6]]")).unwrap() + 1;

    println!("{}", v1 * v2);
}

fn parse(line: &str) -> Value {
    let tokens = tokenize(line);
    let (value, _) = parse_brackets(&tokens, 0);
    value
}

fn parse_brackets(tokens: &Vec<Token>, mut index: usize) -> (Value, usize) {
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
                let (value, i) = parse_brackets(tokens, index);
                vector.push(value);
                index = i + 1;
            }
            Token::RightBracket => {
                return (Value::List(vector), index);
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

fn compare(lhs: &Value, rhs: &Value) -> Option<bool> {
    match (lhs, rhs) {
        (Value::Integer(i), Value::Integer(j)) => match i.cmp(j) {
            Ordering::Less => Some(true),
            Ordering::Greater => Some(false),
            Ordering::Equal => None,
        },
        (Value::List(v1), Value::List(v2)) => compare_lists(v1, v2),
        (Value::Integer(i), Value::List(v)) => {
            let new_v = vec![Value::Integer(*i)];
            compare_lists(&new_v, v)
        }
        (Value::List(v), Value::Integer(i)) => {
            let new_v = vec![Value::Integer(*i)];
            compare_lists(v, &new_v)
        }
    }
}

fn compare_lists(lhs: &[Value], rhs: &[Value]) -> Option<bool> {
    let len = lhs.len().max(rhs.len());
    for i in 0..len {
        match (lhs.get(i), rhs.get(i)) {
            (Some(lhv), Some(rhv)) => match compare(lhv, rhv) {
                Some(result) => return Some(result),
                None => continue,
            },
            (None, Some(_)) => return Some(true),
            (Some(_), None) => return Some(false),
            (None, None) => unreachable!(),
        };
    }
    None
}
