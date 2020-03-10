####################################################################################################
## Builder
####################################################################################################

FROM golang:latest AS builder

RUN apt update && apt install -y make git ca-certificates

WORKDIR /build
COPY . ./
RUN make build

####################################################################################################
## Image
####################################################################################################

FROM alpine:latest

RUN apk update && apk upgrade && apk add ca-certificates

RUN adduser -D -g '' bloom
RUN mkdir /bitflow && chown -R bloom:bloom /bitflow && chmod 700 /bitflow

COPY --from=builder /build/dist/bitflow /bitflow/bitflow

USER bloom
WORKDIR /bitflow

CMD ["./bitflow", "worker"]
