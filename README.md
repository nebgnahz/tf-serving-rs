TensorFlow Serving
---

An attempt to build tensorflow serving client with Rust:

- [grpc](https://github.com/stepancheg/grpc-rust) for protocol
- [tokio](https://tokio.rs/) for runtime


## Instructions

```sh
## Update submodule inside submodule.
git submodule update --init --recursive

## Build protobuf
protoc -I=serving -I serving/tensorflow --rust-grpc_out=src \
       serving/tensorflow_serving/apis/*.proto
protoc -I=serving -I serving/tensorflow --rust_out=src \
       serving/tensorflow_serving/apis/*.proto
protoc -I=serving/tensorflow --rust_out=src \
       serving/tensorflow/tensorflow/core/framework/*.proto
protoc -I=serving/tensorflow --rust_out=src \
       serving/tensorflow/tensorflow/core/protobuf/{saver,meta_graph}.proto
protoc -I=serving/tensorflow --rust_out=src \
       serving/tensorflow/tensorflow/core/example/*.proto
```