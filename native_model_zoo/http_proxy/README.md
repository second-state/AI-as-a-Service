# HTTP proxy for WebAssembly

This application is designed to be used as a `command` from a WebAssembly function running inside the SSVM to perform native network operations.

## Examples

Simple GET

```bash
$ echo "" | target/debug/http_proxy get http://scooterlabs.com/echo
```

GET with header

```bash
$ echo "" | target/debug/http_proxy get http://scooterlabs.com/echo '{"X-My-Custom-Header":"foo"}'
```

POST with header and body

```bash
$ echo "123456789" | target/debug/http_proxy post http://scooterlabs.com/echo '{"X-My-Custom-Header":"foo"}'
```
