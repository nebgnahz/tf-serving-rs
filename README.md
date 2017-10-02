TensorFlow Serving
---

[![Build Status][travis-badge]](https://travis-ci.com/nebgnahz/tf-serving-rs)

[Documentation](https://nebgnahz.github.io/tf-serving-rs/)

An attempt to build tensorflow serving client with Rust:

- [grpcio](https://github.com/pingcap/grpc-rs) for protocol
- [tokio](https://tokio.rs/) for runtime

## Instructions

```sh
## Update submodule inside submodule.
git submodule update --init --recursive

## Install the protobuf compiler and gRPC plugin
cargo install protobuf
cargo install grpcio-compiler

## Build protobuf
./proto_gen
```

<!-- links -->
[travis-badge]: https://travis-ci.com/nebgnahz/tf-serving-rs.svg?token=FtzQss73KSBwcHhSsrGQ&branch=master
