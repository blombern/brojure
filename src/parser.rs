use node::Node;
use node::Node::{Int, Float, Str, Bool, Symbol, List, Vector, Nil, Error};

pub fn tokenize(s: &str) -> Vec<String> {
    s
        .replace("(", " ( ")
        .replace(")", " ) ")
        .replace("[", " [ ")
        .replace("]", " ] ")
        .split_whitespace()
        .map(|s| s.to_owned())
        .collect()
}

pub fn parse(tokens: &mut Vec<String>) -> Node {
    if tokens.len() == 0 {
        Error("Unexpected EOF")
    } else {
        let mut stack = Vec::new();

        while tokens.len() > 0 {
            let t = tokens.remove(0);

            match t.as_ref() {
                "(" => {
                    stack.push(Vec::new());
                },
                ")" => {
                    match stack.pop() {
                        Some(top) => {
                            if stack.len() > 0 {
                                match stack.last_mut() {
                                    Some(prev) => prev.push(List(top)),
                                    None       => return Error("Couldn't parse"),
                                }
                            } else {
                                stack.push(top);
                            }
                        },
                        None => return Error("Couldn't parse"),
                    }
                },
                "[" => {
                    stack.push(vec![Bool(true)]);
                },
                "]" => {
                    match stack.pop() {
                        Some(mut top) => {
                            if stack.len() > 0 {
                                match stack.last_mut() {
                                    Some(prev) => {
                                        top.remove(0);
                                        prev.push(Vector(top))
                                    },
                                    None       => return Error("Couldn't parse"),
                                }
                            } else {
                                stack.push(top);
                            }
                        },
                        None => return Error("Couldn't parse"),
                    }
                },
                _ => {
                    if stack.len() > 0 {
                        match stack.last_mut() {
                            Some(prev) => prev.push(atom(&t)),
                            None       => return Error("Couldn't parse"),
                        }
                    } else {
                        return atom(&t);
                    }
                },

            }
        }
        if stack.len() == 1 {
            let mut top = stack.pop().unwrap();
            match top[0] {
                Bool(_)   => {
                    top.remove(0);
                    Vector(top)
                },
                _         => List(top),
            }
        } else {
            return Error("Couldn't parse")
        }
    }
}

fn atom(token: &str) -> Node {
    match token.parse::<i64>() {
        Ok(num) => Int(num),
        Err(_) => {
            match token.parse::<f64>() {
                Ok(num) => Float(num),
                Err(_) => {
                    match token {
                        "true"  => Bool(true),
                        "false" => Bool(false),
                        "nil"   => Nil,
                        _       => {
                            let txt = token.clone();
                            let first = txt.chars().nth(0).unwrap();
                            let last = txt.chars().last().unwrap();
                            if first == '"' && last == '"' {
                                let txt: String = txt.chars()
                                    .filter(|c| *c != '"')
                                    .collect();

                                Str(String::from(txt))
                            } else {
                                Symbol(token.to_owned())
                            }
                        },
                    }
                }
            }
        },
    }
}
