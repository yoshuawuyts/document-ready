#![forbid(unsafe_code, missing_debug_implementations)]
#![feature(futures_api)]

use futures::prelude::*;
use futures::sync::oneshot;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;

struct DocumentReady {
  receiver: oneshot::Receiver<()>,
  _cb: Closure<FnMut()>,
}

/// A function to call once the DOM has loaded. If the DOM is already loaded,
/// it will return on next tick.
impl Future for DocumentReady {
  type Item = ();
  type Error = ();

  fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
    match self.receiver.poll() {
      Ok(Async::Ready(_)) => Ok(Async::Ready(())),
      Ok(Async::NotReady) => Ok(Async::NotReady),
      Err(_) => {
        if cfg!(debug_assertions) {
          unreachable!("This future keeps the closure alive, the closer keeps the sender alive. Therefor it can't be cancelled.")
        }
        Err(())
      }
    }
  }
}

pub fn ready() -> impl Future<Item = (), Error = ()> {
  let doc = window()
    .document()
    .expect("should have a document on window");

  let (sender, receiver) = oneshot::channel();

  let mut sender = Some(sender);
  let cb = move || {
    sender.take().unwrap().send(()).unwrap();
  };

  let cb = Closure::wrap(Box::new(cb) as Box<FnMut()>);
  match doc.ready_state().as_str() {
    "complete" | "interactive" => {
      window()
        .set_timeout_with_callback(cb.as_ref().unchecked_ref())
        .unwrap();
    }
    _ => {
      doc
        .add_event_listener_with_callback(
          "DOMContentLoaded",
          cb.as_ref().unchecked_ref(),
        )
        .unwrap();
    }
  };

  DocumentReady { _cb: cb, receiver }
}

/// Access the window.
// todo: expect_throw
fn window() -> web_sys::Window {
  web_sys::window().expect("no global `window` exists")
}
