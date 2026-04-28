//! # glossa
//!
//! `glossa` provides a small DSL for writing functional-style pipelines in Rust.
//!
//! ## Features
//!  
//! - Function pipelines:
//!   ```rust
//!   use glossa_macro::glossa_fn;
//!   fn double(x: u8) -> u8 { x * 2 }
//!   fn add(a: u8, b: u8) -> u8 { a + b }
//!   let f = glossa_fn!(|x, y| add >> double);
//!   ```
//!
//! - Composition:
//!   ```rust
//!   fn add(a: u8, b: u8) -> u8 { a + b }
//!   fn double(x: u8) -> u8 { x * 2 }
//!   use glossa_macro::compose;
//!   let f = compose!(add(5), double);
//!   ```
//!
//! - Inline print shortcuts:
//!   ```rust
//!   "Hello";
//!   "n:No newline";
//!   "e:Error";
//!   ```
//!
//! ## Notes
//!
//! - Parsing uses `GlossaFrom`
//! - Errors panic at runtime
//! - Designed for DSL ergonomics, not strict validation

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, parse::Parser, BinOp, Expr, ExprCall, ExprClosure, ExprLit, ItemFn, Lit,
    Stmt,
};

/// Trait used to convert string inputs into typed values.
/// 
/// pub trait GlossaFrom {
///     fn glossa_parse(s: &str) -> Self;
/// }

/// Default implementation for all `FromStr`
/// impl<T> GlossaFrom for T
/// where
///    T: std::str::FromStr,
///    T::Err: std::fmt::Debug,
/// {
///     fn glossa_parse(s: &str) -> Self {
///         s.parse().unwrap()
///     }
/// }

//////////////////////////////////////////////////////////////////
/// ================= INTERNAL HELPERS ==========================
//////////////////////////////////////////////////////////////////

/// Extract `"1,2"` >> f >> g → ("1,2", [f, g])
fn extract_chain(expr: &Expr) -> Option<(String, Vec<Expr>)> {
    let mut funcs = Vec::new();
    let mut current = expr;

    loop {
        if let Expr::Binary(bin) = current {
            if matches!(bin.op, BinOp::Shr(_)) {
                funcs.push((*bin.right).clone());
                current = &bin.left;
                continue;
            }
        }
        break;
    }

    if let Expr::Lit(ExprLit {
        lit: Lit::Str(ref s),
        ..
    }) = current
    {
        funcs.reverse();
        return Some((s.value(), funcs));
    }

    None
}

/// Extract f >> g >> h (no string)
fn extract_funcs(expr: &Expr) -> Option<Vec<Expr>> {
    let mut funcs = Vec::new();
    let mut current = expr;

    loop {
        if let Expr::Binary(bin) = current {
            if matches!(bin.op, BinOp::Shr(_)) {
                funcs.push((*bin.right).clone());
                current = &bin.left;
                continue;
            }
        }

        funcs.push(current.clone());
        break;
    }

    funcs.reverse();
    Some(funcs)
}

/// Detect placeholder `__`
fn is_placeholder(expr: &Expr) -> bool {
    matches!(expr, Expr::Path(p) if p.path.is_ident("__"))
}

/// Build partial application
fn build_partial_call(func: &Expr, call: &ExprCall, input: Expr) -> Expr {
    let mut has_placeholder = false;

    let args: Vec<Expr> = call
        .args
        .iter()
        .map(|arg| {
            if is_placeholder(arg) {
                has_placeholder = true;
                return input.clone();
            }
            arg.clone()
        })
        .collect();

    if has_placeholder {
        syn::parse_quote! {
            #func(#(#args),*)
        }
    } else {
        syn::parse_quote! {
            #func(#input, #(#args),*)
        }
    }
}

/// Convert expression into callable closure
fn to_callable(expr: &Expr) -> Expr {
    match expr {
        Expr::Call(call) => {
            let func = &call.func;
            let args: Vec<_> = call.args.iter().cloned().collect();

            syn::parse_quote! {
                |__x| #func(__x, #(#args),*)
            }
        }
        Expr::Closure(_) => expr.clone(),
        _ => syn::parse_quote! {
            |__x| #expr(__x)
        },
    }
}

/// Checks presence of 2nd usecase before-hand. 
fn contains_pipeline(expr: &Expr) -> bool {
    match expr {
        Expr::Binary(bin) => matches!(bin.op, BinOp::Shr(_)),
        Expr::Call(_) => false,
        Expr::Lit(_) => false,
        Expr::Closure(_) => false,
        _ => false,
    }
}

//////////////////////////////////////////////////////////////////
/// ================= ATTRIBUTE MACRO ===========================
//////////////////////////////////////////////////////////////////

/// Main DSL transformer
#[proc_macro_attribute]
pub fn glossa(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemFn);

    for stmt in &mut input.block.stmts {
        transform_stmt(stmt);
    }

    TokenStream::from(quote! { #input })
}

/// Transform statements (glossa DSL entry point)
fn transform_stmt(stmt: &mut Stmt) {
    match stmt {

        // =====================================================
        // let bindings: let x = ...
        // =====================================================
        Stmt::Local(local) => {
            if let Some(init) = &mut local.init {
                transform_expr(&mut init.expr);
            }
        }

        // =====================================================
        // expression with semicolon
        // =====================================================
        Stmt::Expr(expr, Some(_)) => {
            transform_expr(expr);

            // Only handle string literals AFTER expression transform
            if let Expr::Lit(expr_lit) = expr {
                if let Lit::Str(ref s) = expr_lit.lit {
                    let content = s.value();

                    // =================================================
                    // ESCAPE SYSTEM
                    // =================================================
                    // "!n:hello" → disables DSL, prints literal string
                    if content.starts_with("!") {
                        let raw = &content[1..];
                        *stmt = syn::parse_quote! {
                            println!(#raw);
                        };
                        return;
                    }

                    // =================================================
                    // PRINT MODES
                    // =================================================

                    // n: → no newline print!
                    if content.starts_with("n:") {
                        let val = &content[2..];
                        *stmt = syn::parse_quote! {
                            print!(#val);
                        };
                    }

                    // e: → stderr
                    else if content.starts_with("e:") {
                        let val = &content[2..];
                        *stmt = syn::parse_quote! {
                            eprintln!(#val);
                        };
                    }

                    // f: → formatted println!
                    // allows: f:hello {x}
                    else if content.starts_with("f:") {
                        let val = &content[2..];
                        *stmt = syn::parse_quote! {
                            println!(#val);
                        };
                    }

                    // b: → debug-style print (useful for raw inspection)
                    else if content.starts_with("b:") {
                        let val = &content[2..];
                        *stmt = syn::parse_quote! {
                            println!("{:?}", #val);
                        };
                    }

                    // r: → raw string print (no formatting interpretation)
                    else if content.starts_with("r:") {
                        let val = &content[2..];
                        *stmt = syn::parse_quote! {
                            println!("{}", #val);
                        };
                    }

                    // default → println!
                    else {
                        *stmt = syn::parse_quote! {
                            println!(#content);
                        };
                    }
                }
            }
        }

        // =====================================================
        // expression without semicolon
        // =====================================================
        Stmt::Expr(expr, None) => {
            transform_expr(expr);
        }

        _ => {}
    }
}
fn transform_expr(expr: &mut Expr) {

    // =========================================================
    // 1. ONLY handle pipeline expressions containing >>
    // =========================================================
    if contains_pipeline(expr) {
        if let Some((content, funcs)) = extract_chain(expr) {

            let args: Vec<Expr> = content
                .split(',')
                .map(|arg| {
                    let arg = arg.trim();
                    syn::parse_quote! {
                        ::glossa::GlossaFrom::glossa_parse(#arg)
                    }
                })
                .collect();

            let mut funcs_iter = funcs.into_iter();

            // SAFE FIRST FUNCTION (NO unwrap)
            let mut current = match funcs_iter.next() {
                Some(first) => syn::parse_quote! {
                    #first(#(#args),*)
                },
                None => return,
            };

            for func in funcs_iter {
                current = match func {
                    Expr::Call(call) => build_partial_call(&call.func, &call, current),
                    Expr::Closure(c) => syn::parse_quote! { (#c)(#current) },
                    _ => syn::parse_quote! { #func(#current) },
                };
            }

            *expr = current;
        }

        return;
    }

    // =========================================================
    // 2. DO NOT TOUCH non-pipeline expressions
    // =========================================================
}

//////////////////////////////////////////////////////////////////
/// ================= FUNCTION MACROS ===========================
//////////////////////////////////////////////////////////////////

/// Function composition
#[proc_macro]
pub fn compose(input: TokenStream) -> TokenStream {
    let exprs = syn::punctuated::Punctuated::<Expr, syn::Token![,]>::parse_terminated
        .parse(input)
        .unwrap();

    let mut iter = exprs.into_iter();

    let first = to_callable(&iter.next().unwrap());
    let mut current = first;

    for func in iter {
        let callable = to_callable(&func);

        current = syn::parse_quote! {
            |__x| (#callable)((#current)(__x))
        };
    }

    TokenStream::from(quote! { #current })
}

/// Build pipeline into closure
#[proc_macro]
pub fn glossa_fn(input: TokenStream) -> TokenStream {
    let closure: ExprClosure = syn::parse(input).unwrap();

    let inputs = closure.inputs;
    let body = *closure.body;

    if let Some(funcs) = extract_funcs(&body) {
        let mut iter = funcs.into_iter();

        let first = iter.next().unwrap();
        let args: Vec<_> = inputs.iter().cloned().collect();

        let mut current: Expr = syn::parse_quote! {
            #first(#(#args),*)
        };

        for func in iter {
            current = match func {
                Expr::Call(call) => build_partial_call(&call.func, &call, current),
                Expr::Closure(c) => syn::parse_quote! { (#c)(#current) },
                _ => syn::parse_quote! { #func(#current) },
            };
        }

        return TokenStream::from(quote! {
            |#inputs| #current
        });
    }

    panic!("glossa_fn! expects a pipeline body");
}

/// Pipe macro (alias of glossa_fn without args)
/// Builds a function pipeline from a closure-like DSL.
///
/// # Syntax
///
/// ```rust
/// use glossa_macro::pipe;
/// fn sum(a: u8, b: u8) -> u8 { a + b }
/// fn double(x: u8) -> u8 { x * 2 }
/// let f = pipe!(|a, b| sum >> double);
/// ```
///
/// Expands roughly to:
///
/// ```rust
/// fn sum(a: u8, b: u8) -> u8 { a + b }
/// fn double(x: u8) -> u8 { x * 2 }
/// let f = |a, b| double(sum(a, b));
/// ```
///
/// # Features
///
/// - Supports any number of input arguments (defined explicitly)
/// - Supports chaining with `>>`
/// - Supports:
///   - plain functions: `sum >> double`
///   - partial application: `add(5)`
///   - placeholders: `add(__, 5)`
///   - closures: `>> (|x| x * 2)`
///
/// # Important
///
/// - The input **must be a closure** (`|args| ...`)
/// - Function arity is **not inferred** — it comes from the closure
/// - This macro only builds the pipeline; type checking is done by Rust
///
/// # Example
///
/// ```rust
/// use glossa_macro::pipe;
/// fn sum3(a: u8, b: u8, c: u8) -> u8 { a + b + c }
/// fn double(x: u8) -> u8 { x * 2 }
/// fn print(_x: u8) { /* side-effect only */ }
/// let f = pipe!(|a, b, c| sum3 >> double >> print);
/// f(1, 2, 3);
/// ```
///
/// # Panics
///
/// Panics if the body is not a valid `>>` pipeline.
#[proc_macro]
pub fn pipe(input: TokenStream) -> TokenStream {
    // Parse input as a closure: |a, b, c| ...
    let closure: syn::ExprClosure = syn::parse(input).unwrap();

    // Extract closure inputs (arguments)
    let inputs = closure.inputs;

    // Extract closure body (pipeline expression)
    let body = *closure.body;

    // Extract pipeline functions from body: f >> g >> h
    if let Some(funcs) = extract_funcs(&body) {
        let mut iter = funcs.into_iter();

        // First function in pipeline (entry point)
        let first = iter.next().unwrap();

        // Convert closure inputs into a Vec for expansion
        let args: Vec<_> = inputs.iter().cloned().collect();

        // Build initial call:
        // e.g. sum(a, b, c)
        let mut current: syn::Expr = syn::parse_quote! {
            #first(#(#args),*)
        };

        // Chain remaining functions:
        // e.g. double(sum(...)) → print(double(...))
        for func in iter {
            current = match func {
                // Handle partial application: add(5), add(__, 5), etc.
                syn::Expr::Call(call) => {
                    build_partial_call(&call.func, &call, current)
                }

                // Handle inline closures: >> (|x| x * 2)
                syn::Expr::Closure(c) => syn::parse_quote! {
                    (#c)(#current)
                },

                // Handle plain functions: >> double
                _ => syn::parse_quote! {
                    #func(#current)
                },
            };
        }

        // Return final closure:
        // |a, b, c| print(double(sum(a, b, c)))
        return TokenStream::from(quote! {
            |#inputs| #current
        });
    }

 
    return syn::Error::new_spanned(
        body,
        "pipe! expects a closure like |args| f >> g"
    )
    .to_compile_error()
    .into();
}