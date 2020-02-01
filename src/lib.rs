//! Document ready listener for browsers.
//! 
//! # Examples
//! 
//! ```
//! use wasm_bindgen::prelude::*;
//! 
//! #[wasm_bindgen(start)]
//! pub fn main() {
//!     println!("waiting on document to load");
//!     document_ready::ready().await;
//!     println!("document loaded!");
//! }
//! ```

#![forbid(unsafe_code, future_incompatible, rust_2018_idioms)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(missing_docs, missing_doc_code_examples, unreachable_pub)]

use async_std::sync::channel;
use gloo_events::EventListener;
use std::time::Duration;

/// Wait for the DOM to be loaded.
pub async fn ready() {
    let document = web_sys::window()
        .expect("Window not found")
        .document()
        .unwrap();

    match document.ready_state().as_str() {
        "complete" | "interactive" => {
            futures_timer::Delay::new(Duration::from_secs(0)).await;
        }
        _ => {
            let (sender, receiver) = channel(1);
            EventListener::new(&document, "DOMContentLoaded", move |_| {
                sender.try_send(()).unwrap();
            });
            receiver.recv().await;
        }
    };
}
