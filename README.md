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

# Usage

## Server

``` bash
$ sudo systemctl start n3-torchd
```

## Client

### Training

``` bash
$ n3 train image_classification --model LeNet5 --data MNIST --devices cuda:0 cpu
```

### Evaluating

``` bash
$ n3 eval image_classification --model LeNet5 --data MNIST --devices cuda:0 cpu
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
