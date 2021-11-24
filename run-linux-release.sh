#!/bin/bash
MODEL=/home/jczaja/DEEPSPEECH/jacek-18-11-2021.pbmm
SCORER=/home/jczaja/DEEPSPEECH/deepspeech-0.9.3-models.scorer
SCRIPT_DIR=$(dirname "$0")
pushd "$SCRIPT_DIR"
LD_LIBRARY_PATH=/home/jczaja/DEEPSPEECH/deepspeech-native ./target/release/corporate-assistant --model "$MODEL" --scorer "$SCORER" --organization=itp.toml --project=paddle.toml
popd
