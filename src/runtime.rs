// src/runtime.rs
// NovaLang için kapsamlı runtime ve yerleşik fonksiyonlar

use crate::vm::Value;
use std::collections::HashMap;

pub type BuiltinFunc = fn(Vec<Value>) -> Value;

pub struct Runtime {
    pub builtins: HashMap<String, BuiltinFunc>,
}

impl Runtime {
    pub fn new() -> Self {
        let mut builtins = HashMap::new();
        builtins.insert("print".to_string(), builtin_print as BuiltinFunc);
        builtins.insert("len".to_string(), builtin_len as BuiltinFunc);
        builtins.insert("toString".to_string(), builtin_to_string as BuiltinFunc);
        builtins.insert("keys".to_string(), builtin_keys as BuiltinFunc);
        // Diğer yerleşik fonksiyonlar buraya eklenebilir
        Runtime { builtins }
    }

    pub fn load_std_module(name: &str) -> Option<String> {
        match name {
            "math" => Some("func sqrt(x) { /* ... */ }".to_string()),
            "string" => Some("func upper(s) { /* ... */ }".to_string()),
            "file" => Some("func read(path) { /* ... */ }".to_string()),
            _ => None,
        }
    }
}

fn builtin_print(args: Vec<Value>) -> Value {
    for arg in args {
        match arg {
            Value::Int(i) => print!("{} ", i),
            Value::Float(f) => print!("{} ", f),
            Value::String(s) => print!("{} ", s),
            Value::Bool(b) => print!("{} ", b),
            Value::Null => print!("null "),
            Value::List(list) => {
                print!("[");
                for (i, v) in list.iter().enumerate() {
                    if i > 0 { print!", "; }
                    builtin_print(vec![v.clone()]);
                }
                print!("] ");
            }
            Value::Map(map) => {
                print!("{{");
                let mut first = true;
                for (k, v) in map.iter() {
                    if !first { print!", "; } else { first = false; }
                    print!("{}: ", k);
                    builtin_print(vec![v.clone()]);
                }
                print!("}} ");
            }
            _ => print!("<fn> "),
        }
    }
    println!("");
    Value::Null
}

fn builtin_len(args: Vec<Value>) -> Value {
    if let Some(Value::String(s)) = args.get(0) {
        Value::Int(s.len() as i64)
    } else {
        Value::Null
    }
}

fn builtin_to_string(args: Vec<Value>) -> Value {
    if let Some(val) = args.get(0) {
        Value::String(format!("{:?}", val))
    } else {
        Value::Null
    }
}

fn builtin_keys(args: Vec<Value>) -> Value {
    if let Some(Value::Map(map)) = args.get(0) {
        let keys = map.keys().cloned().map(Value::String).collect();
        Value::List(keys)
    } else {
        Value::Null
    }
} 