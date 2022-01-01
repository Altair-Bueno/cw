#!/usr/bin/env bash

# Simple bash script that runs some benchmarks using hyperfine
# USAGE
# ./bench.sh "path/to/files"

cd "$1" || exit

export ARGS_LIST_HYPERFINE=$(ls | xargs)
echo "Byte count"
hyperfine -m 100 -w 2 -s basic "cw -b $ARGS_LIST_HYPERFINE" "wc -c $ARGS_LIST_HYPERFINE"
echo "********************************************************************************"
echo "Word count"
hyperfine -m 100 -w 2 -s basic "cw -w $ARGS_LIST_HYPERFINE" "wc -w $ARGS_LIST_HYPERFINE"
echo "********************************************************************************"
echo "Char count"
hyperfine -m 100 -w 2 -s basic "cw -c $ARGS_LIST_HYPERFINE" "wc -m $ARGS_LIST_HYPERFINE"
echo "********************************************************************************"
echo "Line count"
hyperfine -m 100 -w 2 -s basic "cw -l $ARGS_LIST_HYPERFINE" "wc -l $ARGS_LIST_HYPERFINE"
echo "********************************************************************************"
echo "max length"
hyperfine -m 100 -w 2 -s basic "cw -L $ARGS_LIST_HYPERFINE" "wc -L $ARGS_LIST_HYPERFINE"
echo "********************************************************************************"
echo "Default mode"
hyperfine -m 100 -w 2 -s basic "cw $ARGS_LIST_HYPERFINE" "wc $ARGS_LIST_HYPERFINE"
echo "********************************************************************************"
echo "High load"
hyperfine -m 100 -w 2 -s basic "cw $ARGS_LIST_HYPERFINE $ARGS_LIST_HYPERFINE $ARGS_LIST_HYPERFINE" "wc $ARGS_LIST_HYPERFINE $ARGS_LIST_HYPERFINE $ARGS_LIST_HYPERFINE"