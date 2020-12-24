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

WORKDIR /root
ADD . n3
WORKDIR /root/n3

RUN mkdir /workspace
ENV N3_ROOT=/workspace/n3
ENV N3_SOURCE_ROOT=/workspace/python/n3
ENV PYTHONPATH=/workspace/python

### Add PATH of binaries

ENV PATH="/root/.cargo/bin:${PATH}"

### n3-net-api

RUN cargo install diesel_cli --no-default-features --features sqlite
RUN cargo install --path n3-net/api

RUN cp -r n3-net/api/Rocket.toml /workspace
RUN diesel setup --database-url /workspace/n3_net_api.sqlite --migration-dir n3-net/api/migrations

### n3-torch-server

RUN cargo install --path n3-torch/client
RUN cargo install --path n3-torch/server

### Cleanup

RUN mv n3-torch/ffi/python /workspace

WORKDIR /workspace
RUN rm -r /root/n3
