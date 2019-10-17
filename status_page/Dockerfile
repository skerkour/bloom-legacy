####################################################################################################
## Build
####################################################################################################

FROM rust:1.35-stretch AS build

WORKDIR /app

RUN git clone https://github.com/valeriansaliou/vigil.git

WORKDIR /app/vigil


RUN rustup --version
RUN rustup default nightly-2019-04-16

RUN rustc --version && \
    rustup --version && \
    cargo --version

RUN cargo clean && cargo build --release

####################################################################################################
## Image
####################################################################################################

FROM debian:stretch-slim

RUN apt-get update && apt-get install -y libssl-dev libcurl3

RUN useradd -ms /bin/bash bloom

RUN mkdir /vigil && chown -R bloom:bloom /vigil && chmod 700 /vigil

WORKDIR /vigil

COPY --from=build /app/vigil/res/assets/ ./res/assets/
COPY --from=build /app/vigil/target/release/vigil ./vigil


CMD [ "./vigil", "-c", "vigil.toml" ]

EXPOSE 8080
