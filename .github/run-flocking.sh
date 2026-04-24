#!/bin/bash

RESULT_FILE="${RESULT_FILE:-benchmark_results.txt}"
SKIP_TABLE="${SKIP_TABLE:-0}"

(

echo "Benchmarking Agents.jl"
julia --project=@. ./Flocking/Agents/benchmark_flocking.jl

echo "Benchmarking Ark.jl"
julia --project=@. ./Flocking/Ark/benchmark_flocking.jl

echo "Benchmarking Mason"
bash ./Flocking/Mason/benchmark_flocking.sh

echo "Benchmarking Mesa"
python3 ./Flocking/Mesa/benchmark_flocking.py

echo "Benchmarking Mesa-Frames"
python3 ./Flocking/MesaFrames/benchmark_flocking_mesaframes.py

echo "Benchmarking KrABMaga"
bash ./Flocking/KrABMaga/benchmark_flocking.sh

echo "Benchmarking NetLogo"
bash Flocking/NetLogo/benchmark_flocking.sh

) | tee "$RESULT_FILE"

if [[ "$SKIP_TABLE" != "1" ]]; then
  julia --project=@. create_benchmark_table.jl
fi
