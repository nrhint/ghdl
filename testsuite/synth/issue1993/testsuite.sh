#! /bin/sh

. ../../testenv.sh

GHDL_STD_FLAGS=--std=08
synth_only repro1
synth_only bug

echo "Test successful"