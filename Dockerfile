# FROM rust:1.34-stretch AS builder

# RUN rustup target add x86_64-unknown-linux-musl
# # to fix rustc opaque type compiler error (actix web).
# RUN rustup install nightly-2019-05-10 && rustup default nightly-2019-05-10
# RUN apt update
# RUN apt install -y git ca-certificates make libssl-dev libpq-dev \
#     build-essential cmake curl file \
#     musl-dev musl-tools \
#     libsqlite-dev pkgconf \
#     sudo xutils-dev
# #    gcc-4.7-multilib-arm-linux-gnueabihf

# WORKDIR /kernel
# COPY . ./
# # ENV PKG_CONFIG_ALLOW_CROSS=1
# RUN make build

####################################################################################################
## Image
####################################################################################################

FROM debian:stretch-slim

RUN apt-get update && apt-get upgrade -y

RUN apt-get install --no-install-recommends -y ca-certificates libssl-dev libpq-dev \
  && apt-get remove -y --allow-remove-essential gzip \
  && apt-get autoremove -y \
  && rm -rf /var/lib/apt/lists/*

RUN useradd -ms /bin/bash bloom

RUN mkdir /bloom && chown -R bloom:bloom /bloom && chmod 700 /bloom
# COPY --from=builder /kernel/dist/kernel /kernel/kernel
# COPY --from=builder /kernel/dist/assets /kernel/assets
COPY ./dist/ /bloom/
COPY ./bloom.default.sane /bloom/bloom.sane

# RUN chown -R bloom:bloom /bloom && chmod 700 /bloom

WORKDIR /bloom
USER bloom

EXPOSE 8000
CMD ["./bloom", "server"]
