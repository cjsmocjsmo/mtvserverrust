FROM rust:buster AS builder

RUN mkdir /usr/src/mtvserver
RUN mkdir /usr/src/mtvserver/src


RUN mkdir /usr/src/mtvserver/target
WORKDIR /usr/src/mtvserver

COPY Cargo.toml .

COPY src/main.rs ./src
COPY src/lib.rs ./src

RUN cargo install --path .

# FROM arm32v6/alpine:latest
FROM alpine:latest
# RUN apk --no-cache add ca-certificates
WORKDIR /root/

COPY --from=builder /usr/local/cargo/bin/mtvserver /usr/local/bin/mtvserver
RUN \
  mkdir ./static && \
  chmod -R +rwx ./static && \
  mkdir ./fsData && \
  chmod -R +rwx ./fsData
  # mkdir ./logs && \
  # chmod -R +rwx ./logs && \
  # echo "Creating log file" > ./logs/mtvServer.log && \
  # echo "Creating log file" > ./logs/mtvTV.log && \
  # echo "Creating log file" > ./logs/mtvMOV.log && \
  # chmod -R +rwx ./logs/mtvServer.log && \
  # chmod -R +rwx ./logs/mtvTV.log && \
  # chmod -R +rwx ./logs/mtvMOV.log

STOPSIGNAL SIGINT
CMD ["mtvserver"]
