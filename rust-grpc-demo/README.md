## gRPC Rust Demo
### Instruction 
Run Server
```
$ cd rust-grpc-demo
$ cargo run --bin server
```
Run Cli
```
$ cargo build --release --bin cli
$ cd rust-grpc-demo/target/release
$ ./cli
```
Commands:
Add Item
```
$ ./cli add --sku TESTSKU --price 1.99 --quantity 20 --name bananas --description "yellow fruit"
success: item was added to the inventory.
```
Retrieve Item
```
$ ./cli get --sku TESTSKU
found item: { sku: "TESTSKU" }, stock: { price: 1.99, quantity: 0 }, information: { name: "bananas", description: "yellow fruit" }
```
Rejected Duplicates
```
$ ./cli add --sku TESTSKU --price 2.99
Error: Status { code: AlreadyExists, message: "item already exists in inventory" }
```
Change Quantity
```
$ ./cli update-quantity --sku TESTSKU --change -17
success: quantity was updated. Quantity: 3 Price: 1.99
```
Update Price
```
$ ./cli update-price --sku TESTSKU --price 2.19
success: price was updated. Quantity: 3 Price: 2.19
```
Watch Activity
```
$ ./cli watch --sku TESTSKU
streaming changes to item TESTSKU
```
Show Help (Automatic Generation)
```
$ ./cli --help
Usage: cli <COMMAND>

Commands:
  add
  remove
  get
  update-quantity
  update-price
  watch
  help             Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```
### Todo

* adding [TLS and auth](https://grpc.io/docs/guides/auth/) to the client and server to protect data
* add command line flags for the cli and server, enable changing host:port

### Reference

[Rust gRPC Demo](https://konghq.com/blog/building-grpc-apis-with-rust)

