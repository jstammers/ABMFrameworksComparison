#!/bin/bash

SEED=42
RANDOM=$SEED
N_RUN=${N_RUN:-100}

run_model() {
    local width=$1
    local height=$2
    local n_sheep=$3
    local n_wolves=$4
    local sheep_reproduce=$5
    local wolf_reproduce=$6
    local regrowth_time=$7
    local steps=100
    local times=()
    for ((i=0; i<N_RUN; i++)); do
        local run_seed=$((RANDOM % 10000 + 1))
        local output
        output=$(cargo run --quiet --manifest-path WolfSheep/KrABMaga/Cargo.toml --release -- "$width" "$height" "$n_sheep" "$n_wolves" "$sheep_reproduce" "$wolf_reproduce" "$regrowth_time" "$steps" "$run_seed")
        local t=${output##*$'\n'}
        times+=("$t")
    done
    sorted=($(printf '%s\n' "${times[@]}" | sort -n))
    echo "${sorted[$((N_RUN / 2))]}"
}

small=$(run_model 25 25 60 40 0.2 0.1 20)
echo "KrABMaga WolfSheep-small (ms): $small"

large=$(run_model 100 100 1000 500 0.4 0.2 10)
echo "KrABMaga WolfSheep-large (ms): $large"
