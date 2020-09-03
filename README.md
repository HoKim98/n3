# N3: Neural Network Notation

[![travis-ci](https://api.travis-ci.com/kerryeon/n3-rs.svg?token=EwDa73MhCmpxV2ZhCUmb&branch=master)](https://travis-ci.com/github/kerryeon/n3-rs)
[![Coverage Status](https://coveralls.io/repos/github/kerryeon/n3-rs/badge.svg?branch=master&t=bHgSyR)](https://coveralls.io/github/kerryeon/n3-rs?branch=master)

This project is in construction. Please be aware of using it.

```
node LeNet5:
    let K: kernel size = int 5

    let W: width = int 28
    let H: height = int 28

    with Conv2D:
        set kernel size = K
        set padding = K / 2
        set stride = 2

    node MyConv:
        1. Conv2D
        2. Relu

    0. Input                   =  1, W  , H
    1. MyConv                  = 32, W/2, H/2
    2. MyConv                  = 64, W/4, H/4
    3. Transform               = 64* W/4* H/4
    4. Linear + Relu + Dropout = 1024
    5. Linear + Softmax(D=-1)  = 10
```

## Usage
* Training
    ```bash
    $ n3 train image-classification --model LeNet5 --data MNIST --devices cuda:0 cpu
    ```
* Evaluating
    ```bash
    $ n3 eval image-classification --model LeNet5 --data MNIST --devices cuda:0 cpu
    ```
* Publish
    ```bash
    $ n3 publish image-classification --model LeNet5 --target android:java
    ```
    * android: java, flutter
    * ios: flutter
    * universal: c++, python
* Monitoring using Tensorboard
    ```bash
    $ n3 monitor # or, browse http://localhost::xxxx/
    ```
* Clustering with `n3-clu`
    ```bash
    $ n3 eval image-classification --model LeNet5 --data MNIST --devices w:180:cuda:0 w:192.168.0.181 cpu
    ```
    * "w:180:cuda:0": the "cuda:0" device in "xxx.xxx.xxx.180" (local)
    * "w:192.168.0.181": automatically choose devices in "192.168.0.181"
    * These can be defined as environment variables (N3_DEVICES)
