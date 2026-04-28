//! # glossa
//!
//! `glossa` is a lightweight DSL toolkit for building expressive function pipelines in Rust.
//!
//! It provides a macro-based system that enables pipeline-style composition similar to:
//!
//! ```rust
//! use glossa::prelude::*;
//!
//! #[glossa]
//! fn main() {
//!     "hello world";
//! }
//! ```
//!
//! ## ✨ Features
//!
//! - Pipe operator style: `a >> b >> c`
//! - String-based DSL parsing
//! - Function composition macros
//! - Custom type parsing via `GlossaFrom`
//! - Lightweight prelude system
//!
//! ---
//!
//! ## 🚀 Design Philosophy
//!
//! `glossa` does NOT aim to replace Rust’s type system.
//! Instead, it provides a *syntactic layer* for expressing pipelines more naturally.
//!
//! ---
//!
//! ## 📦 Modules
//!
//! - `prelude` — recommended imports
//! - `GlossaFrom` — string parsing trait
//! - `glossa_macro` — procedural macro backend

// ============================================================
// TRAIT: GlossaFrom
// ============================================================

/// Trait used by `glossa` to convert string inputs into typed values.
///
/// This enables custom parsing logic for user-defined types.
///
/// ## Example
///
/// ```rust
/// use glossa::GlossaFrom;
///
/// struct Age(u8);
///
/// impl GlossaFrom for Age {
///     fn glossa_parse(s: &str) -> Self {
///         Age(s.parse().unwrap())
///     }
/// }
/// ```
pub trait GlossaFrom: Sized {
    fn glossa_parse(s: &str) -> Self;
}

/// Blanket implementation for all `FromStr` types.
impl<T> GlossaFrom for T
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    fn glossa_parse(s: &str) -> Self {
        s.parse().expect("glossa: failed to parse argument")
    }
}

// ============================================================
// MACRO RE-EXPORTS
// ============================================================

pub use glossa_macro::glossa;
pub use glossa_macro::{compose, pipe, glossa_fn};

// ============================================================
// PRELUDE
// ============================================================

/// A convenience module that re-exports the most commonly used items.
///
/// This is the recommended import path for most users.
///
/// ## Example
///
/// ```rust
/// use glossa::prelude::*;
/// ```
pub mod prelude {
    pub use crate::glossa;
    pub use crate::compose;
    pub use crate::pipe;
    pub use crate::glossa_fn;
}