# This script takes care of building your crate and packaging it for release

set -ex

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
    cross rustc --bin jason --target $TARGET --release -- -C lto

    # TODO Update this to package the right artifacts
    local binary="jason"
    local artifact="jason-$TARGET"
    if [ "$TARGET" != "${TARGET%"windows"*}" ]; then
        binary="$binary.exe"
        artifact="$artifact.exe"
    fi

    cp "target/$TARGET/release/"$binary $src/$artifact

    #cd $stage
    #tar czf $src/$CRATE_NAME-$TRAVIS_TAG-$TARGET.tar.gz *
    cd $src

    #rm -rf $stage
}

main
