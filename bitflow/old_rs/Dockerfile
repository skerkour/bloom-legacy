FROM debian:stretch-slim

RUN apt-get update && apt-get upgrade -y

RUN apt-get install --no-install-recommends -y ca-certificates libssl-dev libpq-dev aria2

RUN useradd -ms /bin/bash bloom

RUN mkdir /bitflow && chown -R bloom:bloom /bitflow && chmod 700 /bitflow

# Add build
COPY ./dist/ /bitflow/

# copy default configuration file
COPY ./bitflow.default.sane /bitflow/bitflow.sane

WORKDIR /bitflow
USER bloom
RUN mkdir downloads

CMD ["./bitflow", "worker"]
