#!/bin/bash

echo "Builing the debian package."
mkdir -p lagoon/usr/bin
cp ../target/$1/lagoon lagoon/usr/bin/
dpkg-deb --build lagoon
sudo cp lagoon.deb /release/