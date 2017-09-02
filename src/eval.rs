use std::collections::HashMap;
use node::Node;
use node::Node::{Int, Bool, Symbol, List, Vector, Lambda, Nil, Error};

pub type Env = HashMap<String, Node>;

pub fn eval(node: &mut Node, env: &mut Env) -> Node {
    match node {
        &mut List(ref mut args) => {
            let operator = args.remove(0);

            match operator {
                Symbol(s) => {
                    let s = s.as_ref();
                    match s {
                        "+"     => add(args, env),
                        "-"     => subtract(args, env),
                        "*"     => multiply(args, env),
                        "/"     => divide(args, env),
                        ">"     => gt(args, env),
                        "<"     => lt(args, env),
                        ">="    => gt_or_eq(args, env),
                        "<="    => lt_or_eq(args, env),
                        "="     => equal(args,env),
                        "def"   => def(args, env),
                        "defn"  => defn(args,env),
                        "if"    => _if(args, env),
                        "or"    => or(args, env),
                        "and"   => and(args, env),
                        "do"    => _do(args, env),
                        "fn"    => func(args),
                        "nth"   => nth(args, env),
                        "println" => println(args, env),
                        "let"   => _let(args, env),
                        "conj"  => conj(args, env),
                        "for"   => _for(args, env),
                        "reduce" => _reduce(args, env),
                        "range" => range(args, env),
                        "count" => count(args, env),
                        "mod"   => _mod(args, env),
                        _       => {
                            match env.get(s) {
                                Some(n) => {
                                    let evaled = eval(&mut n.clone(), &mut env.clone());
                                    match evaled {
                                        lambda @ Lambda { .. } => {
                                            args.insert(0, lambda);
                                            eval(&mut List(args.clone()), &mut env.clone())
                                        },
                                        _ => return Error("Couldn't invoke"),
                                    }
                                },
                                None => return Error("Couldn't resolve symbol"),
                            }
                        },
                    }
                },
                Lambda { ref params, ref body } => {
                    let mut new_env = env.clone();
                    for p in params {
                        new_env.insert(p.to_owned(), args.remove(0));
                    }
                    eval(&mut body.clone(), &mut new_env)
                },
                mut list @ List(_) => {
                    let evaled = eval(&mut list, env);
                    args.insert(0, evaled);
                    eval(&mut List(args.clone()), env)
                },
                _ => Error("Couldn't invoke"),
            }
        },
        &mut Vector(ref mut v) => {
            let v = v.iter_mut()
                .map(|n| eval(n, env))
                .collect::<Vec<Node>>();
            Vector(v)
        },
        &mut Symbol(ref s) => {
            let mut n: Node =
            match env.get(s) {
                Some(node) => node.clone(),
                _ => return Error("Couldn't resolve symbol"),
            };
            eval(&mut n, env)
        },
        _ => node.to_owned(),
    }
}

fn add(args: &mut Vec<Node>, env: &mut Env) -> Node {
    let start = eval(&mut args.remove(0), env);
    args.iter_mut().fold(start, |acc, n| acc + eval(n, env))
}

fn subtract(args: &mut Vec<Node>, env: &mut Env) -> Node {
    let start = eval(&mut args.remove(0), env);
    args.iter_mut().fold(start, |acc, n| acc - eval(n, env))
}

fn multiply(args: &mut Vec<Node>, env: &mut Env) -> Node {
    let start = eval(&mut args.remove(0), env);
    args.iter_mut().fold(start, |acc, n| acc * eval(n, env))
}

fn divide(args: &mut Vec<Node>, env: &mut Env) -> Node {
    let start = eval(&mut args.remove(0), env);
    args.iter_mut().fold(start, |acc, n| acc / eval(n, env))
}

fn def(args: &mut Vec<Node>, env: &mut Env) -> Node {
    let symbol = match args.remove(0) {
        Symbol(s) => s,
        _ => return Error("Expected symbol as first argument to def"),
    };
    let mut node = args.remove(0);
    let mut env2 = env.clone();
    env.insert(symbol, eval(&mut node, &mut env2));
    Nil
}

fn defn(mut args: &mut Vec<Node>, env: &mut Env) -> Node {
    let mut node = vec![args.remove(1), args.remove(1)];
    let func = func(&mut node);
    args.push(func);
    def(&mut args, env)
}

fn equal(args: &mut Vec<Node>, env: &mut Env) -> Node {
    Bool(eval(&mut args[0], env) == eval(&mut args[1], env))
}

fn gt(args: &mut Vec<Node>, env: &mut Env) -> Node {
    let first  = eval(&mut args.remove(0), env);
    let second = eval(&mut args.remove(0), env);

    match (first, second) {
        (Int(n1), Int(n2)) => Bool(n1 > n2),
        _ => Error("Expected numbers as arguments to >"),
    }
}

fn gt_or_eq(args: &mut Vec<Node>, env: &mut Env) -> Node {
    let first  = eval(&mut args.remove(0), env);
    let second = eval(&mut args.remove(0), env);

    match (first, second) {
        (Int(n1), Int(n2)) => Bool(n1 > n2 || n1 == n2),
        _ => Error("Expected numbers as arguments to >="),
    }
}


fn lt(args: &mut Vec<Node>, env: &mut Env) -> Node {
    match gt(args, env) {
        Bool(b) => Bool(!b),
        _       => Error("Expected numbers as arguments to <"),
    }
}

fn lt_or_eq(args: &mut Vec<Node>, env: &mut Env) -> Node {
    let first  = eval(&mut args.remove(0), env);
    let second = eval(&mut args.remove(0), env);

    match (first, second) {
        (Int(n1), Int(n2)) => Bool(n1 < n2 || n1 == n2),
        _ => Error("Expected numbers as arguments to <="),
    }
}


fn _if(args: &mut Vec<Node>, env: &mut Env) -> Node {
    let cond = eval(&mut args.remove(0), env);
    let mut expr1 = args.remove(0);
    let mut expr2 = if args.len() > 0 { args.remove(0) } else { Nil };

    match cond {
        Bool(b) => if b { eval(&mut expr1, env) } else { eval(&mut expr2, env) },
        Nil     => eval(&mut expr2, env),
        _       => eval(&mut expr1, env),
    }
}

fn or(args: &mut Vec<Node>, env: &mut Env) -> Node {
    if args.len() == 0 {
        return Nil
    }
    let mut last = args.pop().unwrap();
    for n in args.iter_mut() {
        let evaled = eval(n, env);
        match evaled {
            Bool(b) => if b { return evaled },
            Nil => (),
            _ => return evaled,
        }
    }
    eval(&mut last, env)
}

fn and(args: &mut Vec<Node>, env: &mut Env) -> Node {
    if args.len() == 0 {
        return Bool(true)
    }
    let mut last = args.pop().unwrap();
    for n in args.iter_mut() {
        let evaled = eval(n, env);
        match evaled {
            Bool(b) => if !b { return evaled },
            Nil => return Nil,
            _ => (),
        }
    }
    eval(&mut last, env)
}

fn _do(args: &mut Vec<Node>, env: &mut Env) -> Node {
    let mut evaled = args.iter_mut()
        .map(|n| eval(n, env))
        .collect::<Vec<Node>>();

    evaled.pop().unwrap()
}

fn func(args: &mut Vec<Node>) -> Node {
    let fn_params =
        match args.remove(0) {
            Vector(v) => {
                v.iter()
                    .map(|n| {
                        match n {
                            &Symbol(ref s) => s.to_owned(),
                            _ => panic!("Expected symbols as function parameters"),
                        }
                    })
                    .collect()
            },
            _ => return Error("Expected vector as second argument to fn")
        };

    let fn_body = Box::new(args.remove(0));

    Lambda { params: fn_params, body: fn_body }
}

fn nth(args: &mut Vec<Node>, env: &mut Env) -> Node {
    let v = eval(&mut args.remove(0), env);
    let i = eval(&mut args.remove(0), env);

    match (v, i) {
        (Vector(v), Int(i)) => {
            v.get(i as usize).unwrap().clone()
        },
        (_, Int(_)) => {
            Error("Expected vector as first argument to nth")
        },
        (Vector(_), _) => {
            Error("Expected number as second argument to nth")
        },
        _ => Error("Expected vector and number as arguments to nth")
    }
}

fn println(args: &mut Vec<Node>, env: &mut Env) -> Node {
    for mut arg in args {
        print!("{} ", eval(&mut arg, env));
    }
    print!("\n");
    Nil
}

fn _let(args: &mut Vec<Node>, env: &mut Env) -> Node {
    let bindings = match args.remove(0) {
        Vector(v) => v,
        _         => return Error("Expected vector as first argument to let"),
    };
    let mut body = args;
    let mut new_env = env.clone();

    if bindings.len() % 2 != 0 {
        return Error("Expected binding vector to contain an even number of forms")
    }

    let idxs = (0..bindings.len()).filter(|x| (x + 1) % 2 != 0);

    for i in idxs {
        let symbol_str = match bindings.get(i).unwrap() {
            &Symbol(ref s) => s.to_owned(),
            _              => return Error("Expected odd items in binding vector to be symbols"),
        };
        let val_node = bindings.get(i + 1).unwrap();
        let mut _env = new_env.clone();

        new_env.insert(symbol_str, eval(&mut val_node.clone(), &mut _env));
    }

    _do(&mut body, &mut new_env)
}

fn conj(args: &mut Vec<Node>, env: &mut Env) -> Node {
    let v = eval(&mut args.remove(0), env);
    let n = eval(&mut args.remove(0), env);

    match v {
        Vector(mut v) => {
            v.push(n);
            Vector(v)
        },
        _ => Error("Expected vector as first argument to conj"),
    }
}

fn _for(args: &mut Vec<Node>, env: &mut Env) -> Node {
    let binding = match args.remove(0) {
        Vector(v) => v,
        _         => return Error("Expected binding vector as first argument to for"),
    };

    let symbol_str = match binding.get(0).unwrap() {
        &Symbol(ref s) => s.to_owned(),
        _              => return Error("Expected first item in binding vector to be a symbol"),
    };

    let binding_vec = match eval(&mut binding.get(1).unwrap().clone(), env) {
        Vector(ref v) => {
            match eval(&mut Vector(v.clone()), env) {
                Vector(v) => v,
                _         => return Error("Expected vector as second item in binding vector"),
            }
        },
        _         => return Error("Expected vector as second item in binding vector"),
    };

    let body = args;
    let mut results = Vec::new();
    let mut new_env = env.clone();

    for n in binding_vec {
        new_env.insert(symbol_str.clone(), n);
        let iter_result = _do(&mut body.clone(), &mut new_env);
        match iter_result {
            Nil => (),
            _   => results.push(iter_result),
        }
    }
    Vector(results)
}

fn range(args: &mut Vec<Node>, env: &mut Env) -> Node {
    let from = eval(&mut args.remove(0), env);
    let to = eval(&mut args.remove(0), env);

    match (from, to) {
        (Int(n1), Int(n2)) => {
            let nodes = (n1..n2).map(|n| Int(n)).collect();
            Vector(nodes)
        }
        _ => return Error("Expected numbers as arguments to range"),
    }
}

fn count(args: &mut Vec<Node>, env: &mut Env) -> Node {
    let n = eval(&mut args.remove(0), env);
    match n {
        Vector(v) => Int(v.len() as i64),
        _ => Error("Expected vector as argument to count"),
    }
}

fn _mod(args: &mut Vec<Node>, env: &mut Env) -> Node {
    let dividend = eval(&mut args.remove(0), env);
    let divisor = eval(&mut args.remove(0), env);

    match (dividend, divisor) {
        (Int(n1), Int(n2)) => Int(n1 % n2),
        _ => Error("Expected numbers as arguments to modulo")
    }
}

fn _reduce(args: &mut Vec<Node>, env: &mut Env) -> Node {
    let reducer = eval(&mut args.remove(0), env);
    let mut acc = eval(&mut args.remove(0), env);
    let v = match eval(&mut args.remove(0), env) {
        Vector(v) => v,
        _ => return Error("Expected vector as third argument to reduce"),
    };

    for n in v {
        acc = eval(&mut List(vec![reducer.clone(), acc, n]), env);
    }

    acc
}
