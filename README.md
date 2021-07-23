# webmention

This crate helps deal with [Webmentions](https://www.w3.org/TR/webmention/): sending, receiving, checking the validity.

## Installation

If you have `cargo` installed, then you can easily get webmention by running

```
cargo install webmention --bin webmention --features="cli"
```

Or (if you want receiving functionality):

```
cargo install webmention --bin webmention --features="cli receive"
```

## CLI Usage

Send a webmention:

```
webmention send --from my_url --to other_url
```

Try to send webmentions for all linked URLs:

```
webmention send --from my_url
```

Start a receiver server:

```
webmention receive --domain my_domain
```

## Use cases

1. CLI tool for sending webmentions from your posts manually (endpoint-discovery, sending)
2. Simple web server for receiving webmentions (receiving, storage, validating, querying)
3. Infrastructure for embedding webmention endpoint into a larger web server (receiving, storage)

## License

This project is dually licensed under [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0.html) and [MIT license](https://opensource.org/licenses/MIT) and maintained by [marinintim.com](https://marinintim.com).
