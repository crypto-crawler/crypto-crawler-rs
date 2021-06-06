# crypto-crawler-rs

[![](https://img.shields.io/github/workflow/status/soulmachine/crypto-crawler-rs/CI/main)](https://github.com/soulmachine/crypto-crawler-rs/actions?query=branch%3Amain)

This project contains a list of useful cryptocurrency librarys:

- [carbonbot](./carbonbot) `carbonbot` is a CLI tool to run crawlers, which is composed of `crypto-crawler` and `crypto-msg-parser`.
- [crypto-crawler](./crypto-crawler) is the crawler library to crawl trade and orderbook messages from exchanges
- [crypto-msg-parser](./crypto-msg-parser) is the parser library to parse the output of `crypto-crawler`.
- [crypto-crawler-py](https://github.com/soulmachine/crypto-crawler-py) is Python bindings for the `crypto-crawler` library.
- [crypto-msg-parser-py](https://github.com/soulmachine/crypto-msg-parser-py) is Python bindings for the `crypto-msg-parser` library.

Rust developers will mainly use `crypto-crawler` and `crypto-msg-parser`, and Python developers will use `crypto-crawler-py` and `crypto-msg-parser-py`.

**Dependency Relationship**:

![](./dependency-tree.svg)
