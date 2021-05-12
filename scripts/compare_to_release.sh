#!/bin/bash

audio_files=/home/jczaja/DEEPSPEECH/jacek-corpo/*.wav
release_model=/home/jczaja/DEEPSPEECH/deepspeech-0.8.2-models.pbmm
candidate_model=/home/jczaja/DEEPSPEECH/jacek-16-11-2020.pb
scorer=/home/jczaja/DEEPSPEECH/deepspeech-0.8.2-models.scorer

for sample in $audio_files
do
	transcription=`deepspeech --model $candidate_model --scorer=$scorer --audio $sample 2>/dev/null`
	release_transcription=`deepspeech --model $release_model --scorer=$scorer --audio $sample 2>/dev/null`
	echo "sample: "$sample" rel_out:"$release_transcription" out:"$transcription
done
