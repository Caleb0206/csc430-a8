// SHEQ4
// WIP implementation of SHEQ4.

// Data definitions

// Value - Numbers, Booleans, String, CloV, PrimV
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Real(f64),
    Boolean(bool),
    String(String),
    CloV(CloV),
    PrimV(PrimV)
}

// CloV - Closures contain list of symbol params, body of ExprC, Env
#[derive(Debug, Clone, PartialEq)]
pub struct CloV {
    pub params: Vec<String>,
    pub body: Box<ExprC>,
    pub env: Env
}

// PrimV - Represents a primitive operator by its symbol
#[derive(Debug, Clone, PartialEq)]
pub struct PrimV {
    pub op: String
}

// Binding : pair of a Symbol and a Value
#[derive(Debug, Clone, PartialEq)]
pub struct Binding {
    pub name : String,
    pub val : Box<Value>
}

// Env : a list of Bindings
pub type Env = Vec<Binding>;

// ExprC type : NumC, IfC, IdC, AppC, LamC, StringC
#[derive(Debug, Clone, PartialEq)]
pub enum ExprC {
    NumC(NumC),
    StringC(StringC),
    IdC(IdC),
    IfC(IfC),
    AppC(AppC),
    LamC(LamC),
}

// NumC : a Real
#[derive(Debug, Clone, PartialEq)]
pub struct NumC {
    pub n : f64
}

// StringC : a String
#[derive(Debug, Clone, PartialEq)]
pub struct StringC {
    pub s : String
}

// IdC : a symbol representing an ID
#[derive(Debug, Clone, PartialEq)]
pub struct IdC {
    pub name : String
}

// IfC : an if statement of ExprC, and ExprC's to act on if true or false
#[derive(Debug, Clone, PartialEq)]
pub struct IfC {
    pub v : Box<ExprC>,
    pub iftrue : Box<ExprC>,
    pub iffalse : Box<ExprC>
}

// AppC : Represents a function application.function ExprC with a list of arg ExprC's
#[derive(Debug, Clone, PartialEq)]
pub struct AppC {
    pub expr : Box<ExprC>,
    pub args : Vec<Box<ExprC>>
}

// LamC - Lambdas contain a list of symbol args, and a body of ExprC
#[derive(Debug, Clone, PartialEq)]
pub struct LamC {
    pub args : Vec<String>,
    pub body : Box<ExprC>
}

// reserved-keywords - a list of key-words
const RESERVED_KEYWORDS: [&str; 7] = ["if", "lambda", "let", "=", "in", "end", "else"];

// top_env
pub fn top_env() -> Env {
    vec![
        Binding { name: "true".into(), val: Box::new(Value::Boolean(true)) },
        Binding { name: "false".into(), val: Box::new(Value::Boolean(false)) },
        Binding { name: "+".into(), val: Box::new(Value::PrimV(PrimV { op: "+".into() })) },
        Binding { name: "-".into(), val: Box::new(Value::PrimV(PrimV { op: "-".into() })) },
        Binding { name: "*".into(), val: Box::new(Value::PrimV(PrimV { op: "*".into() })) },
        Binding { name: "/".into(), val: Box::new(Value::PrimV(PrimV { op: "/".into() })) },
        Binding { name: "<=".into(), val: Box::new(Value::PrimV(PrimV { op: "<=".into() })) },
        Binding { name: "equal?".into(), val: Box::new(Value::PrimV(PrimV { op: "equal?".into() })) },
        Binding { name: "substring".into(), val: Box::new(Value::PrimV(PrimV { op: "substring".into() })) },
        Binding { name: "strlen".into(), val: Box::new(Value::PrimV(PrimV { op: "strlen".into() })) },
        Binding { name: "error".into(), val: Box::new(Value::PrimV(PrimV { op: "error".into() })) },
    ]
}

// interp - takes the complete AST (ExprC) with an Env, returning a Value
fn interp(e: &ExprC, env: &Env) -> Value {
    match e {
        ExprC::NumC(NumC {n}) => Value::Real(*n),
        ExprC::StringC(StringC {s}) => Value::String(s.clone()),
        ExprC::IdC(IdC {name}) => {
            if is_reserved(name) {
                panic!("SHEQ: id name is a reserved word, got {}", name);
            } else {
                get_binding_val(name, env)
            }
        },
        ExprC::IfC(IfC {v, iftrue, iffalse}) => {
            let test_val = interp(v, env);
            match test_val {
                Value::Boolean(b) => {
                    if b {
                        interp(iftrue, env)
                    } else {
                        interp(iffalse, env)
                    }
                }
                other => {
                    panic!("SHEQ: if expected boolean test, got {:?}", other);
                }
            }
        },
        ExprC::LamC(LamC {args, body}) => {
            Value::CloV(CloV {
                params: args.clone(),
                body: body.clone(),
                env: env.clone(),
            })
        },
        ExprC::AppC(AppC {expr, args}) => {
            let f_val = interp(expr, env);
            let arg_vals: Vec<Value> = args.iter().map(|a| interp(a, env)).collect();

            match f_val {
                Value::CloV(clo) => {
                    if arg_vals.len() != clo.params.len() {
                        panic!("SHEQ: Incorrect number of arguments, got {}, expected {}", arg_vals.len(), clo.params.len());
                    }
                    // extend the env
                    let new_env = create_env(&clo.params, &arg_vals, &clo.env);
                    interp(&clo.body, &new_env)
                }
                Value::PrimV(prim) => {
                    interp_prim(&prim, arg_vals)
                }
                other => panic!("SHEQ: attempted to apply non function value of {:?}", other)
            }
        },
        
    }
}

// interp_prim - interprets primops, takesa PrimV and a list of Values, returns a Value
fn interp_prim(prim: &PrimV, args: Vec<Value>) -> Value {
    match prim.op.as_str() {
        "+" => {
            match args.as_slice() {
                // correct arity and types
                [Value::Real(a), Value::Real(b)] => Value::Real(a + b),
                // correct arity but wrong types
                [_, _] => {
                    panic!("SHEQ: Primv + expected 2 numbers, got {:?}", args);
                }
                // wrong arity
                _ => {
                    panic!("SHEQ: Incorrect number of arguments, got {:?}", args.len());
                }
            }
        }
        "-" => {
            match args.as_slice() {
                // correct arity and types
                [Value::Real(a), Value::Real(b)] => Value::Real(a - b),
                // correct arity but wrong types
                [_, _] => {
                    panic!("SHEQ: Primv - expected 2 numbers, got {:?}", args);
                }
                // wrong arity
                _ => {
                    panic!("SHEQ: Incorrect number of arguments, got {:?}", args.len());
                }
            }
        }
        "*" => {
            match args.as_slice() {
                // correct arity and types
                [Value::Real(a), Value::Real(b)] => Value::Real(a * b),
                // correct arity but wrong types
                [_, _] => {
                    panic!("SHEQ: Primv * expected 2 numbers, got {:?}", args);
                }
                // wrong arity
                _ => {
                    panic!("SHEQ: Incorrect number of arguments, got {:?}", args.len());
                }
            }
        }
        "/" => {
            match args.as_slice() {
                // correct arity and types
                [Value::Real(a), Value::Real(b)] => if *b != 0.0 {
                                                        Value::Real(a / b)
                                                    }
                                                    else {
                                                        panic!("SHEQ: Divide by zero error")
                                                    },
                // correct arity but wrong types
                [_, _] => {
                    panic!("SHEQ: Primv / expected 2 numbers, got {:?}", args);
                }
                // wrong arity
                _ => {
                    panic!("SHEQ: Incorrect number of arguments, got {:?}", args.len());
                }
            }
        }
        "<=" => {
            match args.as_slice() {
                // correct arity and types
                [Value::Real(a), Value::Real(b)] => if a <= b {
                                                        Value::Boolean(true)
                                                    }
                                                    else {
                                                        Value::Boolean(false)
                                                    },
                // correct arity but wrong types
                [_, _] => {
                    panic!("SHEQ: Primv <= expected 2 numbers, got {:?}", args);
                }
                // wrong arity
                _ => {
                    panic!("SHEQ: Incorrect number of arguments, got {:?}", args.len());
                }
            }
        }
        "equal?" => {
            match args.as_slice() {
                // correct arity 
                [a, b] => if a == b 
                        {
                            Value::Boolean(true)
                        }
                        else 
                        {
                            Value::Boolean(false)
                        }
                // wrong arity
                _ => {
                    panic!("SHEQ: Incorrect number of arguments, got {:?}", args.len());
                }
            }
        }
        "substring" => {
            match args.as_slice() {
                // correct arity and types
                [Value::String(string), Value::Real(start), Value::Real(stop)] =>   {
                
                if start.fract() != 0.0 || stop.fract() != 0.0 {
                    panic!("SHEQ: substring expected integer indices, got {} and {}", start, stop);
                }

                let start_i = *start as usize;
                let stop_i  = *stop as usize;

                if start_i <= stop_i && stop_i <= string.len()
                {
                    Value::String(string[start_i..stop_i].to_string())
                }
                else
                {
                    panic!("SHEQ: string index out of range")
                }}
                // correct arity but wrong types
                [_, _, _] => {
                    panic!("SHEQ: Primv substring expected 1 string and 2 numbers, got {:?}", args);
                }
                // wrong arity
                _ => {
                    panic!("SHEQ: Incorrect number of arguments, got {:?}", args.len());
                }
            }
        }
        "strlen" => {
            match args.as_slice() {
                // correct arity and types
                [Value::String(s)] => Value::Real(s.len() as f64),
                // correct arity but wrong types
                [_] => {
                    panic!("SHEQ: Primv strlen expected string, got {:?}", args);
                }
                // wrong arity
                _ => {
                    panic!("SHEQ: Incorrect number of arguments, got {:?}", args.len());
                }
            }
        }
        "error" => {
            match args.as_slice() {
                // correct arity and types
                [Value::String(e)] => panic!("SHEQ: {}", e),
                // correct arity but wrong types
                [_] => {
                    panic!("SHEQ: Primv error expected string, got {:?}", args);
                }
                // wrong arity
                _ => {
                    panic!("SHEQ: Incorrect number of arguments, got {:?}", args.len());
                }
            }
        }
        op => {
            panic!("SHEQ: Invalid PrimV op, got {}", op);
        }
    }
}


// is_reserved - helper method to check if word is reserved
fn is_reserved(name: &str) -> bool {
    RESERVED_KEYWORDS.iter().any(|kw| kw == &name)
}

// get_binding_val takes a symbol and enviornment, performs a lookup and returns a Value if found
fn get_binding_val(name: &str, env: &Env) -> Value {
    for binding in env.iter().rev() {
        if binding.name == name {
            return (*binding.val).clone();
        }
    }
    panic!("SHEQ: unbound identifier '{}'", name);
}   


// serialize - takes a Value and returns a serialized String
fn serialize(v: &Value) -> String {
    match v {
        Value::Real(n) => format!("{}", n),
        Value::Boolean(true) => "true".into(),
        Value::Boolean(false) => "false".into(),
        Value::String(s) => format!("{:?}", s),  // :? is same as ~v from Racket
        Value::CloV(_) => "#<procedure>".into(),
        Value::PrimV(_) => "#<primop>".into(),
    }
}

// create_env - takes a list of params, list of vals, and an Env to return a new extended Env
fn create_env(params: &[String], vals: &[Value], base_env: &Env) -> Env {
    if params.len() < vals.len() {
        panic!("SHEQ: too many values were passed into the applicatoin {:?} {:?}", params, vals);
    }
    if params.len() > vals.len() {
        panic!("SHEQ: too few values were passed in application {:?} {:?}", params, vals);
    }

    let mut new_env = base_env.clone();
    for(p, v) in params.iter().zip(vals.iter()) {
        new_env.push(Binding {
            name: p.clone(),
            val: Box::new(v.clone()),
        });
    }
    new_env
}

fn main() {
    let env = top_env(); // copy of top_env

    println!("{:?}", serialize(&interp( &ExprC::IdC(IdC {name : "+".into()}), &env)));
    let expr1 = ExprC::AppC(AppC {
        expr: Box::new(ExprC:: IdC(IdC {name: "+".into()})),
        args: vec![Box::new(ExprC:: NumC(NumC {n: 1.0})), 
        Box::new(ExprC:: NumC(NumC {n: 2.0}))]});
    
    println!("(AppC (IdC \"+\") (list (NumC 1.0) (NumC 2.0))) => {:?}", serialize(&interp(&expr1, &env)));
    
}

// TESTS - run tests in terminal with "cargo test"
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reserved_keywords_work() {
        assert!(is_reserved("if"));
        assert!(is_reserved("lambda"));
        assert_eq!(is_reserved("x"), false);
        assert_eq!(is_reserved("foo"), false);
    }

    #[test]
    fn serialize_work() {
        assert_eq!(serialize(&Value::Real(32.0)), "32");
        assert_eq!(serialize(&Value::Boolean(true)), "true");
        assert_eq!(serialize(&Value::Boolean(false)), "false");
        assert_eq!(serialize(&Value::String("hello".into())), "\"hello\"");

        let env = top_env();
        let clo = Value::CloV(CloV {
            params: vec!["x".into()],
            body: Box::new(ExprC:: NumC(NumC {n: 112.0})),
            env: env,
        });
        assert_eq!(serialize(&clo), "#<procedure>");
        assert_eq!(serialize(&Value::PrimV(PrimV {op: "equal?".into()})), "#<primop>"); 

    }

    #[test]
    fn interp_works() {
        let env = top_env();

        // test for PrimV + inside regular interp function
        let expr1 = ExprC::AppC(AppC {
            expr: Box::new(ExprC:: IdC(IdC {name: "+".into()})),
            args: vec![Box::new(ExprC:: NumC(NumC {n: 1.0})), 
            Box::new(ExprC:: NumC(NumC {n: 2.0}))]});
        
        assert_eq!(serialize(&interp(&expr1, &env)), "3");
    }

    #[test]
    fn interp_if() {
        let expr = ExprC::IfC(IfC {
            v: Box::new(ExprC::IdC(IdC {name: "true".into()})),
            iftrue: Box::new(ExprC::NumC(NumC {n: 1.0})),
            iffalse: Box::new(ExprC::NumC(NumC {n: 2.0})),
        });
        let env = top_env();
        assert!(matches!( interp(&expr, &env), Value::Real(1.0) ))
    }

    #[test]
    fn interp_lamda() {
        let env = top_env();

        let lam_expr = ExprC::LamC(LamC {
            args: vec!["x".into()],
            body: Box::new(ExprC::NumC(NumC { n: 5.0})),
        });

        let v = interp(&lam_expr, &env);

        match v {
            Value::CloV(clo) => {
                assert_eq!(clo.params, vec!["x".to_string()]);
            }
            other => panic!("SHEQ: Expected closure, got {:?}", other),
        }
    }
    #[test]
    fn interp_prim_add() {
        let prim_add = PrimV {op: "+".into() };
        let v_add = interp_prim(&prim_add, vec![Value::Real(3.0), Value::Real(10.0)]);
        
        assert!(matches!(v_add, Value::Real(13.0)));
    }

    #[test]
    fn interp_prim_sub() {
        let prim_sub = PrimV {op: "-".into() };
        let v_sub = interp_prim(&prim_sub, vec![Value::Real(10.0), Value::Real(2.0)]);

        assert!(matches!(v_sub, Value::Real(8.0)));
    }

    #[test]
    fn interp_prim_mult() {
        let prim_mult = PrimV {op: "*".into() };
        let v_mult = interp_prim(&prim_mult, vec![Value::Real(4.0), Value::Real(3.0)]);

        assert!(matches!(v_mult, Value::Real(12.0)));
    }

    #[test]
    fn interp_prim_div() {
        let prim_div = PrimV {op: "/".into() };
        let v_div = interp_prim(&prim_div, vec![Value::Real(12.0), Value::Real(2.0)]);

        assert!(matches!(v_div, Value::Real(6.0)));
    }

    #[test]
    fn interp_prim_leq() {
        let prim_leq = PrimV {op: "<=".into() };
        let v_leq1 = interp_prim(&prim_leq, vec![Value::Real(1.0), Value::Real(2.0)]);
        assert!(matches!(v_leq1, Value::Boolean(true)));

        let v_leq2 = interp_prim(&prim_leq, vec![Value::Real(2.0), Value::Real(1.0)]);
        assert!(matches!(v_leq2, Value::Boolean(false)));
    }

    #[test]
    fn interp_eq() {
        let prim_eq = PrimV {op: "equal?".into() };
        let v_eq1 = interp_prim(&prim_eq, vec![Value::Real(3.0), Value::Real(3.0)]);
        assert!(matches!(v_eq1, Value::Boolean(true)));

        let v_eq2 = interp_prim(&prim_eq, vec![Value::Real(2.0), Value::Real(3.0)]);
        assert!(matches!(v_eq2, Value::Boolean(false)));

        let v_eq3 = interp_prim(&prim_eq, vec![Value::String("hi".into()), Value::String("hi".into())]);
        assert!(matches!(v_eq3, Value::Boolean(true)));
    }

    #[test]
    fn interp_substr() {
        let prim_substr = PrimV {op: "substring".into() };
        let v_substr = interp_prim(&prim_substr, vec![Value::String("hello".into()), Value::Real(1.0), Value::Real(4.0)]);
        assert_eq!(v_substr, Value::String("ell".to_string()));
    }

    #[test]
    fn interp_strl() {
        let prim_strl = PrimV {op: "strlen".into()};
        let v_strl = interp_prim(&prim_strl, vec![Value::String("hello".into())]);
        assert_eq!(v_strl, Value::Real(5.0));
    }

    #[test]
    #[should_panic(expected = "Primv error expected string")]
    fn interp_prim_error_type() {
        let prim_et = PrimV {op: "error".into()};
        let _ = interp_prim(&prim_et, vec![Value::Real(1.0)]);
    }

    #[test]
    #[should_panic(expected = "Incorrect number of arguments")]
    fn interp_prim_error_arity() {
        let prim_ea = PrimV {op: "error".into()};
        let _ = interp_prim(&prim_ea, vec![]);
    }

    #[test]
    fn interp_appc() {
        let env = top_env();
        // this is going to be ( (lambda (x) x) 42)
        let expr = ExprC::AppC(AppC {
            expr: Box::new(ExprC::LamC(LamC {
                args: vec!["x".into()],
                body: Box::new(ExprC::IdC(IdC {
                    name: "x".into(),
                })),
            })),
            args: vec![Box::new(ExprC::NumC(NumC {
                n: 42.0
            }))],
        });

        let v_appc = interp(&expr, &env);
        assert_eq!(v_appc, Value::Real(42.0));
    }


}