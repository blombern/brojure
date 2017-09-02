use std::ops;
use std::fmt;
use self::Node::*;

#[derive(Debug, Clone)]
pub enum Node {
    Int(i64),
    Float(f64),
    Str(String),
    Symbol(String),
    List(Vec<Node>),
    Vector(Vec<Node>),
    Bool(bool),
    Lambda { params: Vec<String>, body: Box<Node> },
    Error(&'static str),
    Nil,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Int(n)          => write!(f, "{}", n),
            &Float(n)        => write!(f, "{}", n),
            &Str(ref s)      => write!(f, "\"{}\"", s),
            &Symbol(ref s)   => write!(f, "{}", s),
            &Bool(b)         => write!(f, "{}", b),
            &List(ref v)     => write!(f, "List {:?}", v),
            &Vector(ref v)   => {
                let _ = write!(f, "[");
                for (i, n) in v.iter().enumerate() {
                    if i == v.len() - 1 {
                        let _ = write!(f, "{}", n);
                    } else {
                        let _ = write!(f, "{}, ", n);
                    }
                }
                write!(f, "]")
            },
            &Nil             => write!(f, "{}", "nil"),
            &Lambda{ref params, ref body}
                             => write!(f, "Lambda params:{:?} body:{}", params, body),
            &Error(ref s)    => write!(f, "Error: {}", s),
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        match (self, other) {
            (&Int(n1), &Int(n2))         => n1 == n2,
            (&Int(n1), &Float(n2))       => n1 as f64 == n2,
            (&Float(n1), &Float(n2))     => n1 == n2,
            (&Float(n1), &Int(n2))       => n1 == n2 as f64,
            (&Str(ref s1), &Str(ref s2)) => s1 == s2,
            (&Bool(b1), &Bool(b2))       => b1 == b2,
            (&Nil, &Nil)                 => true,
            _                            => false,
        }
    }
}

impl ops::Add for Node {
    type Output = Node;
    fn add(self, other: Node) -> Node {
        match (self, other) {
            (Int(n1), Int(n2)) => Int(n1 + n2),
            (Int(n1), Float(n2)) => Float(n1 as f64 + n2),
            (Float(n1), Float(n2)) => Float(n1 + n2),
            (Float(n1), Int(n2)) => Float(n1 + n2 as f64),
            _ => Error("Can only add numbers"),
        }
    }
}

impl ops::Sub for Node {
    type Output = Node;
    fn sub(self, other: Node) -> Node {
        match (self, other) {
            (Int(n1), Int(n2)) => Int(n1 - n2),
            (Int(n1), Float(n2)) => Float(n1 as f64 - n2),
            (Float(n1), Float(n2)) => Float(n1 - n2),
            (Float(n1), Int(n2)) => Float(n1 - n2 as f64),
            _ => Error("Can only subtract numbers"),
        }
    }
}

impl ops::Mul for Node {
    type Output = Node;
    fn mul(self, other: Node) -> Node {
        match (self, other) {
            (Int(n1), Int(n2)) => Int(n1 * n2),
            (Int(n1), Float(n2)) => Float(n1 as f64 * n2),
            (Float(n1), Float(n2)) => Float(n1 * n2),
            (Float(n1), Int(n2)) => Float(n1 * n2 as f64),
            _ => Error("Can only multiply numbers"),
        }
    }
}

impl ops::Div for Node {
    type Output = Node;
    fn div(self, other: Node) -> Node {
        match (self, other) {
            (Int(n1), Int(n2)) => {
                let n1 = n1 as f64;
                let n2 = n2 as f64;
                if n1 % n2 == 0.0 {
                    Int(n1 as i64 / n2 as i64)
                } else {
                    Float(n1 / n2)
                }
            },
            (Int(n1), Float(n2)) => Float(n1 as f64 / n2),
            (Float(n1), Float(n2)) => Float(n1 / n2),
            (Float(n1), Int(n2)) => Float(n1 / n2 as f64),
            _ => Error("Can only divide numbers"),
        }
    }
}
