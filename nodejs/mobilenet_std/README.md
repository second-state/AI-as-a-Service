# The imagenet mobilenet v2 image classification example

## Set up

Follow the instructions to [setup Rust and WebAssembly tools in Node.js](https://www.secondstate.io/articles/setup-rust-nodejs/)

Next, install the necessary NPM packages.

```
$ npm i express express-fileupload
```

Finally, you need to install dependencies for the face detection Tensorflow model library.

```
$ wget https://storage.googleapis.com/tensorflow/libtensorflow/libtensorflow-gpu-linux-x86_64-1.15.0.tar.gz
$ sudo tar -C /usr/ -xzf libtensorflow-gpu-linux-x86_64-1.15.0.tar.gz

$ cargo install mobilenet_v2
```

## Build the WASM bytecode

```
# Go to the nodejs/mobilenet_std directory
$ ssvmup build --enable-aot
```

## Test

```
$ cd test
$ node test.js
```

## Web service

```
$ cd node
$ node server.js
```

Then you can access the service at `http://host:8080/`. Try upload an image and see the result in your browser!


