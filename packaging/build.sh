#!/bin/bash

echo "Builing the debian package."
mkdir -p peerstore/usr/bin
cp ../target/$1/peerstore peerstore/usr/bin/
dpkg-deb --build peerstore
sudo cp peerstore.deb /release/