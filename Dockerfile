# Setup for ArchLinux Docker

FROM archlinux:latest

## Dependencies

### Update system

RUN pacman -Syu --noconfirm

### Base

RUN pacman -S --noconfirm --needed gcc git rustup

RUN rustup default nightly  # 'rocket' requires nightly

### n3-net-api

RUN pacman -S --noconfirm --needed sqlite

### n3-torch-server

#### archlinuxcn repo
# see: https://github.com/archlinuxcn/repo

RUN printf "[archlinuxcn]\nServer = https://repo.archlinuxcn.org/\$arch" >> /etc/pacman.conf
RUN rm -rf /etc/pacman.d/gnupg && pacman-key --init
RUN pacman-key --populate archlinux # && pacman-key --populate archlinuxcn
RUN pacman -Syy && pacman -S --noconfirm --needed archlinuxcn-keyring

#### Base

RUN pacman -S --noconfirm --needed python python-pip python-inflection tensorboard python-tensorboardx python-tqdm

#### Only CPUs

RUN pacman -S --noconfirm --needed python-pytorch python-torchvision

#### GPUs (nvidia CUDA) Support

# RUN pacman -S --noconfirm --needed python-torch-cuda python-torchvision-cuda

### Cleanup

RUN pacman -Sc --noconfirm

## Build

### Get sources

RUN cd /root && git clone https://github.com/kerryeon/n3.git
WORKDIR /root/n3

RUN mkdir /root/worker

### Add PATH of binaries

ENV PATH="/root/.cargo/bin:${PATH}"

### n3-net-api

RUN cargo install diesel_cli --no-default-features --features sqlite
RUN cargo install --path n3-net/api

RUN cp -r n3-net/api/Rocket.toml /root/worker
RUN diesel setup --database-url /root/worker/n3_net_api.sqlite --migration-dir n3-net/api/migrations

### n3-torch-server

RUN cargo install --path n3-torch/client
RUN cargo install --path n3-torch/server

### Cleanup

RUN mv n3-torch/ffi/python /root/worker
ENV N3_SOURCE_ROOT=/root/worker/python/n3
ENV PYTHONPATH=/root/worker/python

WORKDIR /root/worker
RUN rm -r /root/n3
