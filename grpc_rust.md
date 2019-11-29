grpc-rust
=========

<!-- https://travis-ci.org/stepancheg/rust-protobuf.png -->
[![Build Status](https://img.shields.io/travis/stepancheg/grpc-rust.svg)](https://travis-ci.org/stepancheg/grpc-rust)
[![License](https://img.shields.io/crates/l/grpc.svg)](https://github.com/stepancheg/grpc-rust/blob/master/LICENSE.txt)
[![crates.io](https://img.shields.io/crates/v/grpc.svg)](https://crates.io/crates/grpc) 

Rust implementation of [gRPC](http://www.grpc.io/) protocol, under development.

Some development questions in [FAQ](/docs/FAQ.md).

## Current status

It basically works. See `grpc-examples/src/bin/greeter_{client,server}.rs`. It can be tested
for example with [go client](https://github.com/grpc/grpc-go/tree/master/examples/helloworld):

```
# start greeter server implemented in rust
$ cargo run --bin greeter_server

# ... or start greeter server implemented in go
$ go get -u google.golang.org/grpc/examples/helloworld/greeter_client
$ greeter_server

# start greeter client implemented in rust
$ cargo run --bin greeter_client rust
> message: "Hello rust"

# ... or start greeter client implemented in go
$ go get -u google.golang.org/grpc/examples/helloworld/greeter_client
$ greeter_client rust
> 2016/08/19 05:44:45 Greeting: Hello rust
```

Client and server are implemented asynchronously.

## How to generate rust code

There are two ways to generate rust code from .proto files

### Invoke protoc programmatically with `protoc-rust-grpc` crate

(Recommended)

Have a look at readme in
[protoc-rust-grpc crate](https://github.com/stepancheg/grpc-rust/tree/master/protoc-rust-grpc).

### With `protoc` command and `protoc-gen-rust-grpc` plugin

[Readme](https://github.com/stepancheg/grpc-rust/tree/master/grpc-compiler)

### Use generated protos in your project:

In Cargo.toml:

```ini
[dependencies]
grpc            = "~0.5"
protobuf        = "~2"
futures         = "~0.1"
futures-cpupool = "~0.1"
```

In `lib.rs` or `main.rs` (or any other submodule):

```rust
extern crate protobuf;
extern crate grpc;
extern crate futures;
extern crate futures_cpupool;

pub mod myproto;
pub mod myproto_grpc;
```

## TODO

* Fix performance
* More tests
* In particular, add more compatibility tests, they live in `interop` directory
* Fix all TODO in sources

## Related projects

* [grpc-rs](https://github.com/pingcap/grpc-rs) — alternative implementation of gRPC in Rust,
  a wrapper to C++ implementation
* [httpbis](https://github.com/stepancheg/rust-http2) — implementation of HTTP/2,
  which is used by this implementation of gRPC
* [rust-protobuf](https://github.com/stepancheg/rust-protobuf/) — implementation of Google Protocol Buffers
