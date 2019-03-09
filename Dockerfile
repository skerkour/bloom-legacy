FROM rust:1.33-stretch AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update
RUN apt install -y git ca-certificates make libssl-dev libpq-dev \
    build-essential cmake curl file \
    musl-dev musl-tools \
    libsqlite-dev pkgconf \
    sudo xutils-dev
#    gcc-4.7-multilib-arm-linux-gnueabihf
RUN useradd -ms /bin/bash bloom

WORKDIR /kernel
COPY . ./
ENV PKG_CONFIG_ALLOW_CROSS=1
RUN make build_static

####################################################################################################
## Image
####################################################################################################

FROM alpine:latest
RUN apk --no-cache add ca-certificates postgresql-dev libressl-dev openssl-dev libldap \
  libsasl musl

RUN adduser -D -g '' bloom
RUN ln -s /lib/libc.musl-x86_64.so.1 /lib/ld64.so.1

RUN mkdir /kernel && chown -R bloom:bloom /kernel && chmod 700 /kernel
COPY --from=builder /kernel/dist/kernel /kernel/kernel
COPY --from=builder /kernel/dist/assets /kernel/assets

USER bloom
WORKDIR /kernel

EXPOSE 8000
CMD ["./kernel"]
