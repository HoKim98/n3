<div align="center">
  <img src="https://github.com/kerryeon/n3/blob/master/assets/logo.png">
</div>

# N3: Neural Network Notation

[![Rust](https://github.com/kerryeon/n3/workflows/Rust/badge.svg)](https://travis-ci.com/kerryeon/n3)

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

# Quick Start

```bash
# install dependencies (apt)
sudo apt update
sudo apt install -y \
    gcc git
    sqlite3 libsqlite3-dev

# create an conda environment (can vary on your settings)
conda create -n n3 -c pytorch -c nvidia \
    python=3 pip \
    pytorch torchvision torchaudio cudatoolkit=11.1 \
    tqdm
conda install -n n3 -c conda-forge \
    inflection tensorboard tensorboardx
conda activate n3

# build
cargo b --all

# set environment variables
export N3_SOURCE_ROOT=$PWD/n3-torch/ffi/python/n3/
export PYTHONPATH=$PYTHONPATH:$N3_SOURCE_ROOT/../
export PATH=$PATH:$PWD/target/debug/

# set environment variables (by manual)
export PATH=$PATH:$(dirname $(which python)/../bin/)
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$(dirname $(which python)/../lib/)

# set environment variables (can vary on your settings)
export N3_MACHINES=cuda

# spawn a daemon (in the seperated tty)
n3-torchd

# train a model (in the seperated tty)
n3 train image_classification --model LeNet5 --data MNIST \
    --epoch 1 --batch_size 50

# monitor the progress (in the seperated tty)
n3 monitor
```

# Usage

## Server

```bash
$ sudo systemctl start n3-torchd
```

## Client

### Training

```bash
$ n3 train image_classification --model LeNet5 --data MNIST --machines cuda
```

### Evaluating

```bash
$ n3 eval image_classification --model LeNet5 --data MNIST --machines cuda
```

### Publishing

```bash
$ n3 publish image_classification --model LeNet5 --target android:java
```

* android: java, flutter
* ios: flutter
* universal: c++, python

### Monitoring using Tensorboard

```bash
$ n3 monitor # or, browse http://localhost::xxxx/
```

### Distributed Training

```bash
$ n3 train image_classification --model LeNet5 --data MNIST --machines w:180:cuda:0 w:192.168.0.181 cpu
```

* "w:180:cuda:0": the "cuda:0" machine in "xxx.xxx.xxx.180" (local)
* "w:192.168.0.181": automatically choose machines in "192.168.0.181"
* These can be defined as environment variables (N3_MACHINES)

## Docker

```bash
$ docker build --tag n3:1.0 .
$ docker run -it --rm n3:1.0 bash -c "n3-torchd & n3-net-api"
```
