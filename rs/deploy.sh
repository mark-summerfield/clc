#!/bin/bash
if [ $(pwd) != $HOME/app/clc/rs ]
then
    cd rs
fi
cargo build --release
cp -f $HOME/app/clc/rs/target/release/clc $HOME/opt/bin/clc
