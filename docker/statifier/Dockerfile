FROM ubuntu:16.04

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    cmake \
    gcc \
    libc6-dev \
    make \
    pkg-config

RUN apt-get install -y --no-install-recommends wget g++
RUN apt-get install -y --no-install-recommends g++-multilib
RUN dpkg --print-architecture
RUN mkdir /workspace
WORKDIR /workspace
RUN wget -O statifier-1.7.4.tar.gz "https://downloads.sourceforge.net/project/statifier/statifier/1.7.4/statifier-1.7.4.tar.gz?r=https%3A%2F%2Fsourceforge.net%2Fprojects%2Fstatifier%2Ffiles%2Fstatifier%2F1.7.4%2Fstatifier-1.7.4.tar.gz%2Fdownload&ts=1532071132"
RUN tar xf statifier-1.7.4.tar.gz
RUN cd statifier-1.7.4 && make all && make install
RUN cat /usr/bin/statifier

FROM alpine:3.6

COPY --from=0 /usr/lib/statifier /opt/statifier