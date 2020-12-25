# Setup for Docker

FROM rust:1-buster as rust_builder

## Dependencies

### Update system

RUN apt-get update && apt-get upgrade -y

### Base

RUN apt-get install -y gcc git

RUN rustup default nightly  # 'rocket' requires nightly
RUN echo 'alias python=python3' >> ~/.bashrc

### n3-net-api

RUN apt-get install -y sqlite

### n3-torch-server

FROM continuumio/miniconda as conda_builder

RUN conda install python=3 inflection tensorboard tqdm
RUN conda install tensorboardx -c conda-forge

ENV N3_CONDA_DEVICE=cpuonly
# ENV N3_CONDA_DEVICE=cudatoolkit=10.2
RUN conda install pytorch torchvision $N3_CONDA_DEVICE -c pytorch

#### Cleanup

RUN conda clean -ya

FROM rust_builder
COPY --from=conda_builder /opt/conda /opt/conda

ENV PATH /opt/conda/bin:$PATH
ENV LD_LIBRARY_PATH /opt/conda/lib:$LD_LIBRARY_PATH

## Build

### Get sources

ENV N3_SRC /usr/src/n3
WORKDIR $N3_SRC
ADD . .

ENV N3_ROOT=/workspace
ENV N3_SOURCE_ROOT=$N3_ROOT/python/n3
ENV PYTHONPATH=$N3_ROOT/python
RUN mkdir $N3_ROOT

RUN mkdir $N3_ROOT/bin
ENV PATH $N3_ROOT/bin:$PATH

### n3-net-api

RUN cargo install diesel_cli --no-default-features --features sqlite
RUN cargo install --path n3-net/api

RUN cp n3-net/api/Rocket.toml $N3_ROOT
RUN diesel setup --database-url $N3_ROOT/n3_net_api.sqlite --migration-dir n3-net/api/migrations

### n3-torch-server

RUN cargo install --path n3-torch/client
RUN cargo install --path n3-torch/server

### Cleanup

RUN mv /usr/local/cargo/bin/n3* $N3_ROOT/bin

RUN mv n3-torch/ffi/python $N3_ROOT

WORKDIR $N3_ROOT
RUN rm -r $N3_SRC

CMD ["n3-apid", "n3-torchd"]
