#!/bin/bash

echo "Preparing Environment"

sudo apt-get update && sudo apt-get -y install pkg-config \
			       libssl-dev \
			       build-essential \
			       openssl

sudo mkdir /release

# For building a static binary.
export OPENSSL_STATIC=1

echo "Building binary"

cargo test
cargo build --release

echo "Pushing to release folder"

sudo cp target/release/lagoon /release

echo "Spitting out to github release"
echo "Crate Name: ${CRATE_NAME}"
echo "Tag Info: ${TRAVIS_TAG}"

# Tag information available

cd /release

echo "Building a tar ball"
if [ -z $TRAVIS_TAG ];
then
    val=$(date +%s)
    export TRAVIS_TAG="stable-$val"
fi

sudo tar -cvf ${CRATE_NAME}-${TRAVIS_TAG}.tar.gz *