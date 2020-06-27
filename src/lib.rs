//! Provides two macros to time your execution
//! 
//! Usage
//! ```
//! 
//! extern crate timed;
//! use timed::timed_macro::timed_block;
//! use timed::timed_macro::timed_fn;
//! 
//! fn main() {
//!     timed_block_fn();
//!     timed_attribute_fn_with_custom_name();
//!     timed_attribute_fn_with_no_name();
//! }
//! 
//! fn timed_block_fn() {
//!     timed_block!("timed_fn_name");
//!     println!("A timed block function");
//! }
//! 
//! #[timed_fn(name = "custom_name")]
//! fn timed_attribute_fn_with_custom_name() {
//!     println!("A timed attribute function with custom name");
//! }
//! 
//! #[timed_fn]
//! fn timed_attribute_fn_with_no_name() {
//!     println!("A timed attribute function no name");
//! }
//! ```
//! 
//! When the above code is run 
//! ```ignore
//! TIMED_ENABLED=1 cargo run;
//! 
//! //Outputs
//! 
//! A timed block function
//![timed]:[function:timed_fn_name]:[0 ms, 42 us]
//!
//!A timed attribute function with custom name
//![timed]:[function:timed_attribute_fn_with_custom_name]:[0 ms, 2 us]
//!
//!A timed attribute function no name
//![timed]:[function:timed_attribute_fn_with_no_name]:[0 ms, 1 us]
//! ```
//! 
//! Note: By default the **timed is disabled**, you need to run it with `TIMED_ENABLED=1`
pub use timed_macro;
pub mod timed_execution;
