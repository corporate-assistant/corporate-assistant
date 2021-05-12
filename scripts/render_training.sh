#!/bin/bash

function get_epoch_size()
{

local epoch=0
local epoch_size=`awk -v pattern=$2 '\
BEGIN {prev_step=0+0;}\
$0 ~ pattern {\
  for(i=1;i<=NF;++i) {\
    if (tolower($i) == "steps:" && ($(i+1) ~ /[0-9]+/)) { \
       step=$(i+1)+0;\
       if(step < prev_step) {\
         exit 0;\
       } else {\
         prev_step = step+0;\
       }\
    }\
  }\
}\
END {print prev_step; }\
' $1`
echo $epoch_size
}

function get_loss_data()
{

local loss_data=`awk -v pattern=$2 '\
BEGIN {step=0+0; }\
$0 ~ pattern { \
  for(i=1;i<=NF;++i) {\
  if(tolower($i) == "loss:"){ \
    printf("%d %f\\\n",step,$(i+1));\
  }\
  }\
  step+=1+0;\
}\
' $1`
echo -e $loss_data
}

function get_saved_position()
{
local saved_val_index=`awk '\
BEGIN {epoch=0+0; saved_val_epoch=0+0;}\
/Saved/ {\
saved_val_epoch=epoch;\
}\
/Epoch|Validation/ {\
epoch=$2+0;\
}\
END {print saved_val_epoch+1;}\
' $1`
echo $saved_val_index
}

############### TRAINING #######################

training_epoch_size=$(get_epoch_size $1 Training)
echo "Training Epoch Size: ${training_epoch_size}"

training_loss_data=$(get_loss_data $1 Training)

training_epochs_strings=""
for x in {0..200}
do
training_epochs_strings=$training_epochs_strings"set arrow from $x*$training_epoch_size,graph 0 to $x*$training_epoch_size,graph 1 nohead dt \"-\";"
done

training_data_file=$(mktemp)
echo "$training_loss_data" >> $training_data_file
echo "LOSS DATA: $training_loss_data"

gnuplot -e " set terminal pngcairo dashed size 1024, 768;
             set output \"deepspeech-training.png\";
             set title \"Deepspeech training loss\";
             set yrange [0:5];
             set xlabel \"Iterations\";
             $training_epochs_strings
             plot \"$training_data_file\" using 1:2 with lines title \"Loss\", 1/0 dt \"-\" title \"End of Epoch\"; "
             
rm $training_data_file


############### VALIDATION #######################

validation_epoch_size=$(get_epoch_size $1 "Validation")
echo "Validation Epoch Size: ${validation_epoch_size}"

validation_loss_data=$(get_loss_data $1 "Validation")

validation_epochs_strings=""
for x in {0..200}
do
validation_epochs_strings=$validation_epochs_strings"set arrow from $x*$validation_epoch_size,graph 0 to $x*$validation_epoch_size,graph 1 nohead dt \"-\";"
done

saved_val_index=$(get_saved_position $1)

validation_data_file=$(mktemp)
echo "$validation_loss_data" >> $validation_data_file
echo "LOSS DATA: $validation_loss_data"




gnuplot -e " set terminal pngcairo dashed size 1024, 768;
             set output \"deepspeech-validation.png\";
             set title \"Deepspeech validation loss\";
             set yrange [0:5];
             set xlabel \"Iterations\";
             set object 3 circle at $saved_val_index*$validation_epoch_size,scr 0.07+0.008 size scr 0.008 fc  rgb \"black\" fs solid;
             set label \"best model\" at $saved_val_index*$validation_epoch_size,scr 0.014 textcolor \"black\";
             set arrow from  $saved_val_index*$validation_epoch_size,scr 0.014 to $saved_val_index*$validation_epoch_size,scr 0.07; 
             $validation_epochs_strings
             plot \"$validation_data_file\" using 1:2 with lines title \"Loss\", 1/0 dt \"-\" title \"End of Epoch\"; "
             
rm $validation_data_file

