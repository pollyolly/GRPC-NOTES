## Streaming Response
```
Sending data without waiting for client request
Used stream keyword in rpc to call stream of messages in SayResponse.
```
### Run
Server
```
$cargo run --bin server
```
Client
```
$cargo run --bin client
```
### Troubleshooting
Problem:
```
there is no reactor running, must be called from the context of Tokio runtime
```
Solution: Update tokio version to supported version
```
[dependencies]
tokio = {version="0.2", features=["stream","macros"]}
```

### Reference
[Beginner Guide to gRPC](https://dev.to/anshulgoyal15/a-beginners-guide-to-grpc-with-rust-3c7o)
