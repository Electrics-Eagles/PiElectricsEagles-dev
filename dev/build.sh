#!/bin/sh
whoami

#cargo install cross

cross build --target arm-unknown-linux-musleabi
