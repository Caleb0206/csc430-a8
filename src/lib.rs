// SHEQ4
// WIP implementation of SHEQ4.

// Data definitions

// Value - Numbers, Booleans, String, CloV, PrimV
#[derive(Debug, Clone)]
pub enum Value {
    Real(f64),
    Boolean(bool),
    String(String),
    CloV(CloV),
    PrimV(PrimV)
}

// CloV - Closures contain list of symbol params, body of ExprC, Env
#[derive(Debug, Clone)]
pub struct CloV {
    pub params: Vec<String>,
    pub body: Box<ExprC>,
    pub env: Env
}
// PrimV - Represents a primitive operator by its symbol
#[derive(Debug, Clone)]
pub struct PrimV {
    pub op: String
}

// Binding : pair of a Symbol and a Value
#[derive(Debug, Clone)]
pub struct Binding {
    pub name : String,
    pub val : Box<Value>
}

// Env : a list of Bindings
pub type Env = Vec<Binding>;

// ExprC type : NumC, IfC, IdC, AppC, LamC, StringC
#[derive(Debug, Clone)]
pub enum ExprC {
    NumC(NumC),
    StringC(StringC),
    IdC(IdC),
    IfC(IfC),
    AppC(AppC),
    LamC(LamC),
}

// NumC : a Real
#[derive(Debug, Clone)]
pub struct NumC {
    pub n : f64
}

// StringC : a String
#[derive(Debug, Clone)]
pub struct StringC {
    pub s : String
}

// IdC : a symbol representing an ID
#[derive(Debug, Clone)]
pub struct IdC {
    pub name : String
}

// IfC : an if statement of ExprC, and ExprC's to act on if true or false
#[derive(Debug, Clone)]
pub struct IfC {
    pub v : Box<ExprC>,
    pub iftrue : Box<ExprC>,
    pub iffalse : Box<ExprC>
}

// AppC : Represents a function application.function ExprC with a list of arg ExprC's
#[derive(Debug, Clone)]
pub struct AppC {
    pub expr : Box<ExprC>,
    pub args : Vec<Box<ExprC>>
}

// LamC - Lambdas contain a list of symbol args, and a body of ExprC
#[derive(Debug, Clone)]
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
        Value::String(s) => format!("{:?}", s),
        Value::CloV(_) => "#<procedure>".into(),
        Value::PrimV(_) => "#<primop>".into(),
    }
}

// interp - takes the complete AST (ExprC) with an Env, returning a Value
fn interp(e: &ExprC, env: &Env) -> Value {
    match e {
        ExprC::NumC(NumC {n}) => Value::Real(*n),
        ExprC::StringC(StringC {s}) => Value::String(s.clone()),
        ExprC::IdC(IdC {name}) => get_binding_val(name, env),
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
        ExprC::AppC(AppC {expr, args}) => todo!(),
        ExprC::LamC(LamC {args, body}) => todo!()
    }
}

fn main() {
    let env = top_env(); // copy of top_env

    println!("{:?}", serialize(&interp( &ExprC::IdC(IdC {name : "+".into()}), &env)));

    println!("Hello world!");
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
    fn interp_if() {
        let expr = ExprC::IfC(IfC {
            v: Box::new(ExprC::IdC(IdC {name: "true".into()})),
            iftrue: Box::new(ExprC::NumC(NumC {n: 1.0})),
            iffalse: Box::new(ExprC::NumC(NumC {n: 2.0})),
        });
        let env = top_env();
        assert!(matches!( interp(&expr, &env), Value::Real(1.0) ))
    }

}