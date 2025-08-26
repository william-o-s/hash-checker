#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::TemplateApp;

mod hashing;
pub use hashing::hash_sha256;
