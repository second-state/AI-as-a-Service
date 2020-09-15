# Face detection model wrapper

This application is designed to be used as a `command` from a WebAssembly function running inside the SSVM to perform native tensorflow operations.
It is NOT designed for direct command line use.

## Install tensorflow

[Follow the instructions](https://www.tensorflow.org/install/lang_c). On most Linux systems, just do the following.

```bash
$ wget https://storage.googleapis.com/tensorflow/libtensorflow/libtensorflow-gpu-linux-x86_64-1.15.0.tar.gz
$ sudo tar -C /usr/ -xzf libtensorflow-gpu-linux-x86_64-1.15.0.tar.gz
```

## Build and install

To install from the public repo, do this.

```bash
$ cargo install face_detection_mtcnn
```

To install from the local source, do this.

```bash
$ cargo install --path .
```

Note: If error occurs when building, try to update `rustup`.

```bash
$ rustup update nightly
$ rustup update stable
```
