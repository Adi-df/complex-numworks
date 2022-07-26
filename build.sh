#!/bin/sh

rm -r dust
mkdir dust

cargo build --target=thumbv7em-none-eabihf --lib --release
cp target/thumbv7em-none-eabihf/debug/libapp.a dust/libapp.a
ar x dust/libapp.a --output=dust

arm-none-eabi-gcc -Os -Wl,-Ur --specs=nosys.specs \
 -nostartfiles -lm -I. -Isrc -Os -Wall -MD -MP \
 -ggdb3 -mthumb -mfloat-abi=hard -mcpu=cortex-m7 \
 -mfloat-abi=hard -mfpu=fpv5-sp-d16 -fno-common \
 -fdata-sections -ffunction-sections -fno-exceptions \
 $(ls dust/*.o | grep -v "divxc3\.o") -o dust/app.nwa

nwlink install-nwa dust/app.nwa