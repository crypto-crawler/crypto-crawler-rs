# protobuf schema

The schema file is `message.proto`.

## Build

First, install the `protoc`, i.e., the protobuf compiler:

```bash
# Install protoc on macOS
xcode-select --install && brew install protobuf gcc
# Install protc on Ubuntu
sudo apt install protobuf-compiler
```

Second, compile `message.proto` to language specific files:

```bash
protoc -I=. message.proto --cpp_out=./cpp
protoc -I=. message.proto --python_out=./python
protoc -I=. message.proto --rust_out=./rust # Need to cargo install protobuf-codegen
```

## Libraries

- [Python delimited-protobuf](https://pypi.org/project/delimited-protobuf/)
- [Rust delimited-protobuf](https://crates.io/crates/delimited-protobuf)
