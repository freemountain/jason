TARGET="$MACHINE-$VENDOR-$OS"

USE_CROSS=true

DOCKER="docker run -it --rm -v $PWD:/workspace freemountain/alpine-rs:0.3"

if [ "$TARGET" == "86_64-unknown-linux-musl" ]; then
    USE_CROSS=false
fi


run() {
    if [ "$USE_CROSS" = true ] ; then
        cross $1 --target "$TARGET" ${@:2}
    else
        $DOCKER cargo $1 ${@:2}
    fi
}
