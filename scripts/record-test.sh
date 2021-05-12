#!/bin/bash

if [ ! -f test.csv ]; then
	echo "wav_filename,wav_filesize,transcript" > test.csv
fi

filename=`cat /dev/urandom | tr -cd 'a-f0-9' | head -c 12``date -I`
arecord -r 16000 -f S16_LE $filename.wav
filesize=`du -b $filename.wav | cut -f 1`
echo $filename.wav,$filesize,$1 >> test.csv
