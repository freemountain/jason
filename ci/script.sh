#!/bin/bash

# This script takes care of testing your crate

set -ex
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null && pwd )"
source "$DIR/common.sh"

# TODO This is the "test phase", tweak it as you see fit
main() {
    run build
    run build --release

    if [ ! -z $DISABLE_TESTS ]; then
        return
    fi

    run test
    run test --release

    run run -- --help
    run run --release -- --help
}

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
   main
fi
