# This script takes care of building your crate and packaging it for release

set -ex
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null && pwd )"
source "$DIR/common.sh"

main() {
    TARGET="$MACHINE-$VENDOR-$OS"

    local src=$(pwd) \
          stage=

    # case $TRAVIS_OS_NAME in
    #     linux)
    #         stage=$(mktemp -d)
    #         ;;
    #     osx)
    #         stage=$(mktemp -d -t tmp)
    #         ;;
    # esac

    test -f Cargo.lock || cargo generate-lockfile

    # TODO Update this to build the artifacts that matter to you
    run rustc --bin jason --release -- -C lto

    # TODO Update this to package the right artifacts
    local binary="jason"
    local artifact="jason-$OS-$MACHINE"
    if [ "$TARGET" != "${TARGET%"windows"*}" ]; then
        binary="$binary.exe"
        artifact="$artifact.exe"
    fi

    if [ "$USE_CROSS" = true ] ; then
        cp "target/$TARGET/release/"$binary $src/$artifact
    else
        cp "target/release/"$binary $src/$artifact
    fi


    #cd $stage
    #tar czf $src/$CRATE_NAME-$TRAVIS_TAG-$TARGET.tar.gz *
    cd $src

    #rm -rf $stage
}

main
