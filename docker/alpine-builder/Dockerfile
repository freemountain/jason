FROM alpine:3.8

RUN apk add --no-cache gcc musl musl-dev g++ make ca-certificates cmake
RUN apk add --no-cache bash nano wget git curl openssl openssl-dev perl
RUN apk add tar curl coreutils alpine-sdk
RUN mkdir /workspace

RUN wget -O statifier-1.7.4.tar.gz "https://downloads.sourceforge.net/project/statifier/statifier/1.7.4/statifier-1.7.4.tar.gz?r=https%3A%2F%2Fsourceforge.net%2Fprojects%2Fstatifier%2Ffiles%2Fstatifier%2F1.7.4%2Fstatifier-1.7.4.tar.gz%2Fdownload&ts=1532071132"
RUN tar xf statifier-1.7.4.tar.gz
RUN cd statifier-1.7.4 && make all