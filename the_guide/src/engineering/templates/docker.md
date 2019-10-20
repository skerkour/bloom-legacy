# Docker


## Go

```dockerfile
####################################################################################################
## Builder                                                                                         #
####################################################################################################
FROM golang:1.11-alpine AS builder

RUN apk update && apk add git ca-certificates make
RUN adduser -D -g '' bloom

WORKDIR /{PROJECT}
COPY . ./
RUN make static


####################################################################################################
## Image                                                                                           #
####################################################################################################
FROM scratch

COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /{PROJECT}/dist/{PROJECT} /{PROJECT}/{PROJECT}
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

# copy assets
COPY assets /{PROJECT}/assets

USER bloom
WORKDIR /{PROJECT}

EXPOSE 8080
CMD ["./{PROJECT}"]
```
