# archive_my_stuff

![Rust CI](https://github.com/Celeo/archive_my_stuff/workflows/Rust%20CI/badge.svg?branch=master)

A CLI program for interactively archiving _your_ GitHub repos.

## Why

I had a bunch of old repos I wanted to archive, and doing it through the GitHub website was tedious.

## Installing

Download the latest release from the [releases tab](https://github.com/Celeo/archive_my_stuff/releases). If you don't run Linux, you'll need to clone the repo and build the binary yourself using [Rust](https://www.rust-lang.org/),

## Using

Run the binary. Supply the `--help` flag to get information on parameters. You'll need a [personal access token](https://help.github.com/en/github/authenticating-to-github/creating-a-personal-access-token-for-the-command-line), which you can generate at [https://github.com/settings/tokens](https://github.com/settings/tokens).

## Developing

### Building

### Requirements

* Git
* A recent version of [Rust](https://www.rust-lang.org/tools/install)

### Steps

```sh
git clone https://github.com/Celeo/archive_my_stuff
cd archive_my_stuff
cargo build
```

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE))
* MIT license ([LICENSE-MIT](LICENSE-MIT))

## Contributing

Please feel free to contribute. Please open an issue first (or comment on an existing one) so that I know that you want to add/change something.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
