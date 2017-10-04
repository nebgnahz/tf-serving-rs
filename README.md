TensorFlow Serving with Rust
---

[![Build Status][travis-badge]](https://travis-ci.com/nebgnahz/tf-serving-rs)

[Documentation](https://nebgnahz.github.io/tf-serving-rs/tf_serving/)

`src` already contains generated Rust from Tensorflow serving protos. If you
want to generate from scratch, use the following instructions:

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
