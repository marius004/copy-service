# !/bin/bash

sudo kill -09 $(cat /tmp/csd.pid)
cargo run
python3 lib/samples.py
sudo kill -09 $(cat /tmp/csd.pid)