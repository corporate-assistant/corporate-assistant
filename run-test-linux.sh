#!/bin/bash
MODEL=/home/jczaja/DEEPSPEECH/jacek-04-02-2021.pbmm
SCORER=/home/jczaja/DEEPSPEECH/deepspeech-0.9.3-models.scorer
SCRIPT_DIR=$(dirname "$0")
pushd "$SCRIPT_DIR"
LIBRARY_PATH=/home/jczaja/DEEPSPEECH/ LD_LIBRARY_PATH=/home/jczaja/DEEPSPEECH/ cargo test 
popd
