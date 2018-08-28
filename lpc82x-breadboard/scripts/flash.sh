#!/usr/bin/env bash

# Script for building an example and uploading it to an LPC82x microcontroller.
# It assumes that a version of lpc21isp that supports LPC82x is installed. This
# fork is known to work: https://github.com/hannobraun/lpc21isp
#
# Example:
# ./scripts/flash.sh /dev/ttyUSB0

DEVICE=$1
BINARY=lpc82x-breadboard

if [ $# -ne 1 ]
then
    echo "Usage: $0 DEVICE"
    exit 1
fi

TARGET_DIR=target/thumbv6m-none-eabi/release
ELF_FILE=$TARGET_DIR/$BINARY
BIN_FILE=$TARGET_DIR/$BINARY.bin

cargo build --release &&

arm-none-eabi-objcopy \
    -O binary \
    $ELF_FILE \
    $BIN_FILE &&

lpc21isp -bin -verify -term $BIN_FILE $DEVICE 115200 0
