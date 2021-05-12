#!/bin/bash

scriptdir=`dirname $0`

$scriptdir/record.sh "$1" && $scriptdir/record-dev.sh "$1" && $scriptdir/record-test.sh "$1"



