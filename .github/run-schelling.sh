#!/bin/bash

RESULT_FILE="${RESULT_FILE:-benchmark_results.txt}"
SKIP_TABLE="${SKIP_TABLE:-0}"

(

echo "Benchmarking Agents.jl"
julia --project=@. ./Schelling/Agents/benchmark_schelling.jl

echo "Benchmarking Ark.jl"
julia --project=@. ./Schelling/Ark/benchmark_schelling.jl

echo "Benchmarking Mason"
bash ./Schelling/Mason/benchmark_schelling.sh

echo "Benchmarking Mesa"
python3 ./Schelling/Mesa/benchmark_schelling.py

echo "Benchmarking Mesa-Frames"
python3 ./Schelling/MesaFrames/benchmark_schelling_mesaframes.py

echo "Benchmarking KrABMaga"
bash ./Schelling/KrABMaga/benchmark_schelling.sh

echo "Benchmarking NetLogo"
bash Schelling/NetLogo/benchmark_schelling.sh

) | tee "$RESULT_FILE"

if [[ "$SKIP_TABLE" != "1" ]]; then
  julia --project=@. create_benchmark_table.jl
fi
