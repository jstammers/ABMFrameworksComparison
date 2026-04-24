#!/bin/bash

(

echo "Benchmarking Agents.jl"
julia --project=@. ./Flocking/Agents/benchmark_flocking.jl

echo "Benchmarking Ark.jl"
julia --project=@. ./Flocking/Ark/benchmark_flocking.jl

echo "Benchmarking Mason"
bash ./Flocking/Mason/benchmark_flocking.sh

echo "Benchmarking Mesa"
python3 ./Flocking/Mesa/benchmark_flocking.py

echo "Benchmarking KrABMaga"
bash ./Flocking/KrABMaga/benchmark_flocking.sh

echo "Benchmarking NetLogo"
bash Flocking/NetLogo/benchmark_flocking.sh

) | tee benchmark_results.txt

julia --project=@. create_benchmark_table.jl
