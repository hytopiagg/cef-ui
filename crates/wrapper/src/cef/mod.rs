mod accessibility_handler;
mod app;
mod browser;
mod callbacks;
mod client;
mod color;
mod command_line;
mod context;
mod drag;
mod events;
mod frame;
mod main_args;
mod process;
mod refcounted;
mod render_handler;
mod request;
mod request_context;
mod request_context_handler;
mod request_handler;
mod resource_request_handler;
mod response;
mod settings;
mod shared_memory_region;
mod ssl;
mod string;
mod time;
mod types;
mod url_request;
mod values;
mod window;
mod x509_certificate;

pub use accessibility_handler::*;
pub use app::*;
pub use browser::*;
pub use callbacks::*;
pub use client::*;
pub use color::*;
pub use command_line::*;
pub use context::*;
pub use drag::*;
pub use events::*;
pub use frame::*;
pub use main_args::*;
pub use process::*;
pub use refcounted::*;
pub use render_handler::*;
pub use request::*;
pub use request_context::*;
pub use request_context_handler::*;
pub use request_handler::*;
pub use resource_request_handler::*;
pub use response::*;
pub use settings::*;
pub use shared_memory_region::*;
pub use ssl::*;
pub use string::*;
pub use time::*;
pub use types::*;
pub use url_request::*;
pub use values::*;
pub use window::*;
pub use x509_certificate::*;

// TODO: Evaluate that your code is actually save! You were doing some unsafe
//  things getting pointers to CefString's and then letting the original value
//  be dropped before you used the pointer. This is because you were using the
//  original value by value instead of by reference.

// TODO: Add doc comments to all new(..) functions!

// TODO: Put doc comments on both trait for callbacks and the type!
