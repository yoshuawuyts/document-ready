# document-ready
[![crates.io version][1]][2] [![build status][3]][4]
[![downloads][5]][6] [![docs.rs docs][7]][8]

Document ready listener for browsers.

- [Documentation][8]
- [Crates.io][2]
- [Releases][releases]

## Usage
```rust
use document_ready::document_ready;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
  println!("waiting on document to load");
  await!{document_ready()};
  println!("document loaded!");
}
```

## Installation
```sh
$ cargo add document-ready
```

## Contributing
Want to join us? Check out our [The "Contributing" section of the
guide][contributing] and take a look at some of these issues:

- [Issues labeled "good first issue"][good-first-issue]
- [Issues labeled "help wanted"][help-wanted]

## License
[MIT](./LICENSE-MIT) OR [Apache-2.0](./LICENSE-APACHE)

[1]: https://img.shields.io/crates/v/document-ready.svg?style=flat-square
[2]: https://crates.io/crates/document-ready
[3]: https://img.shields.io/travis/yoshuawuyts/document-ready.svg?style=flat-square
[4]: https://travis-ci.org/yoshuawuyts/document-ready
[5]: https://img.shields.io/crates/d/document-ready.svg?style=flat-square
[6]: https://crates.io/crates/document-ready
[7]: https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square
[8]: https://docs.rs/document-ready

[releases]: https://github.com/yoshuawuyts/document-ready/releases
[contributing]: https://github.com/yoshuawuyts/document-ready/.github/CONTRIBUTING.md
[good-first-issue]: https://github.com/yoshuawuyts/document-ready/labels/good%20first%20issue
[help-wanted]: https://github.com/yoshuawuyts/document-ready/labels/help%20wanted
