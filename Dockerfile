FROM ubuntu:20.04

RUN apt-get update && \
    apt-get upgrade -y && \
    rm -rf /var/lib/apt/lists/*

COPY target/release/pid-tester /pid-tester

ENTRYPOINT [ "/pid-tester" ]