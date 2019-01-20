Wally
======

A server to change wallpapers dynamically written in rust.

The server is controller via a config file.
Please see the file config.example.toml to see how to configure
the wally server.

### Requirements

* Rust `>= v1.31.0 (need edition=2018) only needed if building from source`
* [Unsplash Developer API](https://unsplash.com/developers)
* feh

### Installation

#### From prebuild binary
Get latest release of binary from [here](https://github.com/xk0nsid/wally-rs/releases).
Move the binary to your `$PATH`.

Use the provided `config.example.toml` for base configuration.

#### From source
```sh
$ git clone https://github.com/xk0nsid/wally-rs.git
$ cd wally-rs
$ cargo build --release
```

### Tasks

* [x] Add search/query support to be able to fetch query based results
* [x] Add pagination support to fully use the search results
* [x] Make temporary file path configurable via `config` file
* [ ] Maintain state between server restarts by storing state data in `/cache` or `/var`
* [ ] Add pre-built binary releases to github
