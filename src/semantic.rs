// src/semantic.rs
// NovaLang için kapsamlı semantik analiz ve tip kontrolü

use crate::ast::{Expr, Stmt, Program};
use crate::error::{NovaError, NovaErrorKind};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
    String,
    Bool,
    Null,
    Func(Vec<Type>, Box<Type>),
    Unknown,
    List(Box<Type>),
    Map(Box<Type>, Box<Type>),
}

pub struct SemanticAnalyzer {
    pub variables: HashMap<String, Type>,
    pub functions: HashMap<String, (Vec<Type>, Type)>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        SemanticAnalyzer {
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    pub fn analyze(&mut self, program: &Program) -> Result<(), NovaError> {
        for stmt in &program.body {
            self.analyze_stmt(stmt)?;
        }
        Ok(())
    }

    fn analyze_stmt(&mut self, stmt: &Stmt) -> Result<(), NovaError> {
        match stmt {
            Stmt::Class { name, base, methods, .. } => {
                // Sınıf adı, üst sınıf ve metotları kaydet
                // ...
                Ok(())
            }
            Stmt::Export { name, value, .. } => {
                // Export edilen sembolü kaydet
                // ...
                Ok(())
            }
            Stmt::AsyncFunc { name, params, body, .. } => {
                // Async fonksiyon kaydı
                // ...
                Ok(())
            }
            Stmt::TryCatchFinally { try_block, err_name, catch_block, finally_block, .. } => {
                // Try, catch ve finally bloklarını analiz et
                // ...
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn analyze_expr(&mut self, expr: &Expr) -> Result<Type, NovaError> {
        match expr {
            Expr::List(items, _, _) => {
                // Listenin eleman tiplerini kontrol et
                let mut elem_type = Type::Unknown;
                for item in items {
                    let t = self.analyze_expr(item)?;
                    if elem_type == Type::Unknown {
                        elem_type = t;
                    } else if elem_type != t {
                        elem_type = Type::Unknown; // Farklı tipler varsa Unknown
                    }
                }
                Ok(Type::List(Box::new(elem_type)))
            }
            Expr::Map(pairs, _, _) => {
                // Map anahtarları string olmalı, değer tipleri çıkarılır
                let mut val_type = Type::Unknown;
                for (k, v) in pairs {
                    let kt = self.analyze_expr(k)?;
                    if kt != Type::String {
                        return Err(NovaError::new(NovaErrorKind::SemanticError, "Map anahtarı string olmalı", 0, 0, None));
                    }
                    let vt = self.analyze_expr(v)?;
                    if val_type == Type::Unknown {
                        val_type = vt;
                    } else if val_type != vt {
                        val_type = Type::Unknown;
                    }
                }
                Ok(Type::Map(Box::new(Type::String), Box::new(val_type)))
            }
            Expr::Index { collection, index, .. } => {
                let coll_type = self.analyze_expr(collection)?;
                let idx_type = self.analyze_expr(index)?;
                match coll_type {
                    Type::List(_) if idx_type == Type::Int => Ok(Type::Unknown),
                    Type::Map(_, v) if idx_type == Type::String => Ok(*v),
                    _ => Err(NovaError::new(NovaErrorKind::SemanticError, "Geçersiz index işlemi", 0, 0, None)),
                }
            }
            Expr::Object { class_name, fields } => {
                // Nesne tipi kontrolü
                // ...
                Ok(Type::Unknown)
            }
            Expr::MethodCall { object, method, args, .. } => {
                // Metot çağrısı tip kontrolü
                // ...
                Ok(Type::Unknown)
            }
            Expr::New { class_name, args, .. } => {
                // new ile nesne oluşturma tipi
                // ...
                Ok(Type::Unknown)
            }
            Expr::Await { expr, .. } => {
                // await edilen ifadenin tipi
                // ...
                Ok(Type::Unknown)
            }
            _ => Ok(Type::Unknown),
        }
    }

    // Diğer yardımcı analiz fonksiyonları buraya eklenecek
}
