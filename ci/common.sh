#!/bin/bash
TARGET="$MACHINE-$VENDOR-$OS"

USE_CROSS=true


# if [ "$TARGET" == "x86_64-unknown-linux-musl" ]; then
#     USE_CROSS=false
#     DOCKER="docker run -it --rm -v $PWD:/workspace freemountain/alpine-rs:0.6"
# fi


run() {
    if [ "$USE_CROSS" = true ] ; then
        cross $1 --target "$TARGET" ${@:2}
    else
        $DOCKER cargo $1 ${@:2}
    fi
}
