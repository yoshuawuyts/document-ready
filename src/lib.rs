#![cfg_attr(feature = "nightly", deny(missing_docs))]
#![cfg_attr(feature = "nightly", feature(external_doc))]
#![cfg_attr(feature = "nightly", doc(include = "../README.md"))]
#![cfg_attr(test, deny(warnings))]
#![forbid(unsafe_code, missing_debug_implementations)]
#![feature(futures_api)]

#[macro_use]
extern crate failure;

extern crate wasm_bindgen;
extern crate web_sys;

mod error;

pub use error::{Error, ErrorKind, Result};

use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::{Document, EventTarget};

/// A function to call once the DOM has loaded. If the DOM is already loaded,
/// it will return on next tick.
pub fn ready() -> Result<()> {
  let document = Document::new().map_err(|_| ErrorKind::Document)?;
  let target: EventTarget = document.into();

  let cb = Closure::wrap(Box::new(move || {
    println!("hello world from WASM");
  }) as Box<Fn()>);

  let event_name = "DOMContentLoaded";
  target
    .add_event_listener_with_callback(event_name, cb.as_ref().unchecked_ref())
    .map_err(|_| ErrorKind::Document)?;
  Ok(())
}
