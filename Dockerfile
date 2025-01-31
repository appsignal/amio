FROM rust:1.84.1

RUN apt-get update
RUN apt-get upgrade -y

ENV CARGO_HOME /amio/target

WORKDIR /amio
