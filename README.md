# bitsors
A Rust wrapper for the [Bitso API](https://bitso.com/api_info/).



## Testing

Tests are independent of each other, but since Bitso's API's authentication method depends on a *nonce* based on a UNIX timestamp, the private API tests should be run sequentially, instead of concurrently (as tests are run by default with Rust). Thus, to run the tests use the following: 

```bash
cargo test -- --test-threads=1
```

