# N3: Neural Network Notation

[![travis-ci](https://api.travis-ci.com/kerryeon/n3.svg)](https://travis-ci.com/kerryeon/n3)

This project is WIP. Please be aware of using it.

```
node LeNet5:
    let K: kernel size = int 5

    let C: input channels = dim
    let W: width = dim
    let H: height = dim

    with Conv2D:
        set kernel size = K
        set padding = K / 2
        set stride = 2

    node MyConv:
        1. Conv2D
        2. Relu

    0. Input                    =  C, W  , H
    1. MyConv                   = 32, W/2, H/2
    2. MyConv                   = 64, W/4, H/4
    3. ToLinear
    4. Linear + Relu + Dropout  = 1024
    5. Linear                   = 10
```

# Milestone

## Meaning

### Progress

* ❌ Not Yet
* 🚧 Work In Progress
* ✅ Done

### Priority

* ❗ High Priority - Design is complete
* ❓ Medium Priority - Under design
* ❌ Low Priority - Too early to design
* 🚧 Low Priority - Already moderately implemented

## ✅ Core

* ✅ [Parser AST](n3-parser-ast) & [Parser](n3-parser)
* ✅ [Program AST](n3-program-ast) & [Program](n3-program)
* ✅ [Machine FFI](n3-machine-ffi) & [Machine](n3-machine)
* ✅ [Program Builder](n3-builder)

## ❌ ❓ Package Server & Client

* ❌ ❓ Server
* ❌ ❓ Client

## 🚧 PyTorch FFI

### ✅ [Core](n3-torch/ffi)

* ✅ [PyTorch Python FFI](n3-torch/ffi/python)
* ✅ [PyTorch Rust FFI](n3-torch/ffi/rust)

### 🚧 [Standard Library](n3-torch/ffi/python)

* 🚧 [Neural Networks (nn)](n3-torch/ffi/python/n3/std/nn)
* 🚧 [Optimizers (optim)](n3-torch/ffi/python/n3/std/optim)
* 🚧 [Datasets (data)](n3-torch/ffi/python/n3/std/data)
* 🚧 [Executable Programs (data)](n3-torch/ffi/python/n3/std/exec)
* 🚧 [Sample Models (models)](n3-torch/ffi/python/n3/std/models)

### 🚧 ❗ [Commands](n3-torch/ffi/python/n3/std/exec)

* 🚧 ❗ Train
* ❌ ❗ Eval
* ❌ ❗ Publish
* 🚧 Monitor

## 🚧 Distributed Server & Client

### ✅ [Common](n3-net)

* ✅ [Common Protocol](n3-net/protocol)
* ✅ [Common Server](n3-net/server)
* ✅ [Common Client](n3-net/client)

### 🚧 [PyTorch](n3-torch)

* ✅ [PyTorch Core](n3-torch/core)
* 🚧 [PyTorch Machines](n3-torch/core/src/device)
* ✅ [PyTorch Server (n3-torchd)](n3-torch/server)

## 🚧 ❗ Command-Line Interface

* 🚧 ❗ [CLI (n3)](n3-torch/server)

## ❌ Graphical User Interface

### ❌ Core - Shared Library

* ❌ Client
* ❌ Monitor

### ❌ Platforms

* ❌ Desktop - Windows, Linux (x11), macOS
* ❌ Web Browser via wasm
* ❌ Mobile - Android, iOS

# Usage

## Server

``` bash
$ sudo systemctl start n3-torchd
```

## Client

### Training

``` bash
$ n3 train image_classification --model LeNet5 --data MNIST --devices cuda
```

### Evaluating

``` bash
$ n3 eval image_classification --model LeNet5 --data MNIST --devices cuda
```

### Publishing

``` bash
$ n3 publish image_classification --model LeNet5 --target android:java
```

* android: java, flutter
* ios: flutter
* universal: c++, python

### Monitoring using Tensorboard

``` bash
$ n3 monitor # or, browse http://localhost::xxxx/
```

### Distributed Training

``` bash
$ n3 train image_classification --model LeNet5 --data MNIST --devices w:180:cuda:0 w:192.168.0.181 cpu
```

* "w:180:cuda:0": the "cuda:0" device in "xxx.xxx.xxx.180" (local)
* "w:192.168.0.181": automatically choose devices in "192.168.0.181"
* These can be defined as environment variables (N3_MACHINES)
