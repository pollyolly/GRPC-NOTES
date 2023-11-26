### gRPC Authentication
#### Certificate CA
Generate Certificate Signing Key
```
$ openssl genrsa -des3 -out my_chat_ca.key 2048
```
Sign pem key (my_chat_ca.pem) using (my_chat_ca.key) for cetificate CA
```
$ openssl req -x509 -new -nodes -key my_chat_ca.key -sha256 -days 1825 -out my_chat_ca.pem
```
#### Genenerate for Server Key 
Generate key for server
```
$ openssl genrsa -out chat_server.key 2048
```
Sign server key to generate our chat_server.csr
```
$ openssl req -new -sha256 -key chat_server.key -out chat_server.csr
```
Create file chat_server.ext contains the text below
```
authorityKeyIdentifier=keyid,issuer
basicConstraints=CA:FALSE
keyUsage = digitalSignature, nonRepudiation, keyEncipherment, dataEncipherment
subjectAltName = @alt_names

[alt_names]
DNS.1 = localhost
```
Add our identity in a form of DNS 
```
$ openssl x509 -req -in chat_server.csr -CA my_chat_ca.pem -CAkey my_chat_ca.key -CAcreateserial -out chat_server.pem -days 1825 -sha256 -extfile - chat_server.ext
```
#### Generate for Client Key
Generate key for client
```
$ openssl genrsa -out chat_client.key 2048
```
Sign client key to generate our chat_client.csr
```
$ openssl req -new -sha256 -key chat_client.key -out chat_client.csr
```
Create file chat_client.ext contains the text below
```
authorityKeyIdentifier=keyid,issuer
basicConstraints=CA:FALSE
keyUsage = digitalSignature, nonRepudiation, keyEncipherment, dataEncipherment
subjectAltName = @alt_names

[alt_names]
DNS.1 = localhost
```
Add our identity in a form of DNS 
```
$ openssl x509 -req -in chat_client.csr -CA my_chat_ca.pem -CAkey my_chat_ca.key  -CAcreateserial -out chat_client.pem -days 1825 -sha256 -extfile chat_client.ext
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
When generating extension file: error loading the config file 'chat_server.ext'
```
make sure to create first the extfile chat_server.ext
```
### Reference
[Beginner Guide to gRPC](https://dev.to/anshulgoyal15/a-beginners-guide-to-grpc-with-rust-3c7o)