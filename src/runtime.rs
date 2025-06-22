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
        builtins.insert("readFile".to_string(), builtin_read_file as BuiltinFunc);
        builtins.insert("writeFile".to_string(), builtin_write_file as BuiltinFunc);
        builtins.insert("toUpper".to_string(), builtin_to_upper as BuiltinFunc);
        builtins.insert("toLower".to_string(), builtin_to_lower as BuiltinFunc);
        builtins.insert("httpGet".to_string(), builtin_http_get as BuiltinFunc);
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

fn builtin_read_file(args: Vec<Value>) -> Value {
    if let Some(Value::String(path)) = args.get(0) {
        match std::fs::read_to_string(path) {
            Ok(content) => Value::String(content),
            Err(_) => Value::Null,
        }
    } else {
        Value::Null
    }
}

fn builtin_write_file(args: Vec<Value>) -> Value {
    if let (Some(Value::String(path)), Some(Value::String(content))) = (args.get(0), args.get(1)) {
        match std::fs::write(path, content) {
            Ok(_) => Value::Bool(true),
            Err(_) => Value::Bool(false),
        }
    } else {
        Value::Null
    }
}

fn builtin_to_upper(args: Vec<Value>) -> Value {
    if let Some(Value::String(s)) = args.get(0) {
        Value::String(s.to_uppercase())
    } else {
        Value::Null
    }
}

fn builtin_to_lower(args: Vec<Value>) -> Value {
    if let Some(Value::String(s)) = args.get(0) {
        Value::String(s.to_lowercase())
    } else {
        Value::Null
    }
}

// Basit HTTP GET (reqwest veya std ile)
fn builtin_http_get(args: Vec<Value>) -> Value {
    if let Some(Value::String(url)) = args.get(0) {
        // Burada async/await ile gerçek HTTP isteği yapılabilir
        Value::String(format!("<dummy response for {}>", url))
    } else {
        Value::Null
    }
} 