#!/bin/bash

python linker_v2.py bouncing_ball_v2
cp ./bouncing_ball_v2.rom ./data/boot.rom
cargo run