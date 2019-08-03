FROM alpine:3.8

RUN apk add --no-cache gcc musl musl-dev g++ make ca-certificates cmake
RUN apk add --no-cache bash nano wget git curl openssl openssl-dev perl
RUN apk add tar curl
RUN mkdir /workspace

ENV PREFIX=/opt/rust-musl

RUN mkdir -p ${PREFIX}
WORKDIR ${PREFIX}

# Build musl
RUN curl -O http://www.musl-libc.org/releases/musl-1.1.10.tar.gz && \
    tar xf musl-1.1.10.tar.gz && \
    cd musl-1.1.10 && \
    ./configure --disable-shared --prefix=$PREFIX && \
    make && \
    make install

RUN du -h ${PREFIX}/lib/libc.a

RUN apk add xz tar xz bzip2

RUN echo -e '#!/bin/bash\n\
echo #!/bin/sh\n\
echo std=${0##*/}\n\
echo exec gcc -std=$std "$@"'\
>> /usr/local/bin/gcc-wrapper

RUN cat /usr/local/bin/gcc-wrapper
RUN ln -s /usr/local/bin/gcc-wrapper /usr/bin/c99
RUN ln -s /usr/local/bin/gcc-wrapper /usr/bin/c89
RUN chmod +x /usr/bin/c99 /usr/bin/c89
# Build libunwind.a
RUN curl -O http://releases.llvm.org/3.7.0/llvm-3.7.0.src.tar.xz&& \
    tar xf llvm-3.7.0.src.tar.xz && \
    cd llvm-3.7.0.src/projects/ && \
    curl http://releases.llvm.org/3.7.0/libunwind-3.7.0.src.tar.xz | tar xJf - && \
    mv libunwind-3.7.0.src libunwind && \
    mkdir libunwind/build && \
    cd libunwind/build && \
    cmake -DLLVM_PATH=../../.. -DLIBUNWIND_ENABLE_SHARED=0 .. && \
    make
#cp lib/libunwind.a $PREFIX/lib/


# du -h musldist/lib/libunwind.a
