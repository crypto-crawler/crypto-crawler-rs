# crypto-message

Unified data structures for all cryptocurrency exchanges.

This library contains all output data types of [`crypto-msg-parser`](https://crates.io/crates/crypto-msg-parser).

The `crypto_message::proto` module contains protobuf messages corresponding to message types in `lib.rs`.

The `crypto_message::compact` module contains compact messages corresponding to message types in `lib.rs`.

**Differences**:

* Message types in `lib.rs` are output data types of `crypto-msg-parser`, and they suitable for parsing.
* Message types in `crypto_message::proto` are protobuf messages, which are suitable for serialization and RPC.
* message types in `crypto_message::compact` are suitable for hight-performance processing.

    Messages types in `lib.rs` has string fields such as `exchange`, `symbol`, which causes a lot of memory allocation and copying, so these types are NOT suitable for high-performance processing.

    Message types in `crypto_message::proto` are compact and hight-performance, but they lack metadata fields such as `exchange`, `symbol` and `pair`.

    Message types in `crypto_message::compact` are equivalent to message types in `lib.rs`, with `exchange` changed to `enum`, `symbol` and `pair` changed to `u64` hash values.
