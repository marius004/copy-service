# !/bin/bash

sh ./install.sh

kill -09 $(cat /tmp/csd.pid)
rm -rf daemon/tmp/*
cargo run
python3 lib/samples.py
# kill -09 $(cat /tmp/csd.pid)