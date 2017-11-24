Query parser for Noise
======================

This is a query parser for the [Noise Query Language](https://github.com/pipedown/noise/blob/master/query_language_reference.md).

### Build

    cargo build


### Running tests

    cargo test


### Running against queries

You can run the parser on file that contains one query per line:

    cargo run <the-file>

It returns “ok” for every successfull parsed line or the error if there was any.


Contributing
------------

### Commit messages

Commit messages are important as soon as you need to dig into the history
of certain parts of the system. Hence please follow the guidelines of
[How to Write a Git Commit Message](http://chris.beams.io/posts/git-commit/).


License
-------

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
