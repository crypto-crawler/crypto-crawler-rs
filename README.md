# crypto-crawler-rs

[![](https://img.shields.io/github/workflow/status/crypto-crawler/crypto-crawler-rs/CI/main)](https://github.com/crypto-crawler/crypto-crawler-rs/actions?query=branch%3Amain)

This project contains a list of useful cryptocurrency librarys:

- [carbonbot](https://github.com/crypto-crawler/carbonbot) is the main CLI tool to run crawlers.
- [crypto-crawler](./crypto-crawler) is the crawler library to crawl websocket and restful messages from exchanges
- [crypto-msg-parser](./crypto-msg-parser) is the parser library to parse messages from `crypto-crawler`.
- [crypto-client](./crypto-client) is a RESTful client library to place and cancel orders.
- [crypto-ws-client](./crypto-ws-client) is the underlying websocket client library, providing a set of universal APIs for different exchanges.
- [crypto-rest-client](./crypto-rest-client) is the underlying RESTful client library, providing universal APIs to get public data from different exchanges.
- [crypto-markets](./crypto-markets) is a RESTful library to retreive market meta data from cryptocurrency echanges.
- [crypto-pair](./crypto-pair) is an offline utility library to parse exchange-specific symbols to unified format.
- [crypto-contract-value](./crypto-pair) is an offline utility library that simply provides the contract values of a trading market.
- Support multiple languages. Some libraries support multiple languages, which is achieved by first providing a FFI binding, then a languge specific wrapper. For example, `crypto-crawler` provides a C-style FFI binding first, and then provides a Python wrapper and a C++ wrapper based on the FFI binding.

**Dependency Relationship**:

![](./dependency-tree.svg)
