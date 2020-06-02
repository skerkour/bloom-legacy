FROM rust:1.35-stretch

RUN useradd -ms /bin/bash bloom

RUN mkdir /bitflow && chown -R bloom:bloom /bitflow && chmod 700 /bitflow


# install dependencies
RUN echo "deb http://deb.debian.org/debian unstable main" >> /etc/apt/sources.list
RUN apt update -y && apt dist-upgrade -y && apt upgrade -y
RUN apt-get install --no-install-recommends -y ca-certificates libssl-dev libpq-dev aria2

USER bloom

RUN rustup default nightly-2019-05-10
RUN cargo install --force cargo-watch

WORKDIR /bitflow

CMD ["bash"]
