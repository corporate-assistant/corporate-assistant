#!/bin/bash
for f in `tail -n +2 $1 | cut -d ',' -f 1` 
do
	if [ ! -f $f ]; then
		echo "ERROR: not existing entry: $f"
		exit 1 
	fi
done


tail -n +2  $1 | sed --expression='s:,.*$::g' | awk '{!seen[$0]++};END{ok=1; for(i in seen) { if(seen[i]>1) { print"ERROR: duplicated audio: " i ; ok = 0;}};  if(ok==1){print"Ok. Database consistent.";}}' 



