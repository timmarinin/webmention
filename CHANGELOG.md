# webmention changelog

# 0.4.0

- added some docs
- no more errors when no target endpoint is found
- new enum variant `WebmentionAcceptance::NoTargetEndpoint` returned from `send`

# 0.3.0

- switched from `surf` to `reqwest`
- switched from `async_std` to `tokio` and tried to make it minimal
- added success reporting in cli
- used `tokio_test` for testing

# 0.2.0

- default-features no longer include `cli` to help with library usage
- Removed usage of default-features of surf, used hyper-client by default

# 0.1.3

- Added feature `hyper`, which enables `surf/hyper-client`.

