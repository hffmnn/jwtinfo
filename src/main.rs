#![crate_name = "jwtinfo"]

//! # jwtinfo
//!
//! `jwt` is a command line utility and a small library to parse JWT tokens
//!
//! ## Installation
//!
//! ```bash
//! cargo install jwtinfo
//! ```
//!
//! ## Usage
//!
//! ```bash
//! $ jwtinfo eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c
//! ```
//!
//! Which will print:
//!
//! ```json
//! {"sub":"1234567890","name":"John Doe","iat":1516239022}
//! ```
//!
//! ## Programmatic usage
//!
//! Install with cargo:
//!
//! ```toml
//! [dependencies]
//! jwtinfo = "*"
//! ```
//!
//! Then use it in your code
//!
//! ```rust
//! use jwtinfo::{jwt};
//!
//! let token = jwt::parse("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c").unwrap();
//! assert_eq!(token.header.alg, "HS256");
//! assert_eq!(token.body, "{\"sub\":\"1234567890\",\"name\":\"John Doe\",\"iat\":1516239022}");
//! ```

pub mod jwt;

#[macro_use]
extern crate lazy_static;
