#!/bin/bash

SEED=42
RANDOM=$SEED
N_RUN=${N_RUN:-100}

run_model() {
    local population=$1
    local width=$2
    local height=$3
    local steps=100
    local times=()
    for ((i=0; i<N_RUN; i++)); do
        local run_seed=$((RANDOM % 10000 + 1))
        local output
        output=$(cargo run --quiet --manifest-path Flocking/KrABMaga/Cargo.toml --release -- "$population" "$width" "$height" "$steps" "$run_seed")
        local t=${output##*$'\n'}
        times+=("$t")
    done
    sorted=($(printf '%s\n' "${times[@]}" | sort -n))
    echo "${sorted[$((N_RUN / 2))]}"
}

small=$(run_model 200 100 100)
echo "KrABMaga Flocking-small (ms): $small"

large=$(run_model 400 150 150)
echo "KrABMaga Flocking-large (ms): $large"
