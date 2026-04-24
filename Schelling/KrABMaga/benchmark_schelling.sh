#!/bin/bash

SEED=42
RANDOM=$SEED
N_RUN=${N_RUN:-100}

run_model() {
    local width=$1
    local height=$2
    local num_agents=$3
    local min_to_be_happy=$4
    local radius=$5
    local steps=20
    local times=()
    for ((i=0; i<N_RUN; i++)); do
        local run_seed=$((RANDOM % 10000 + 1))
        local output
        output=$(cargo run --quiet --manifest-path Schelling/KrABMaga/Cargo.toml --release -- "$width" "$height" "$num_agents" "$min_to_be_happy" "$radius" "$steps" "$run_seed")
        local t=${output##*$'\n'}
        times+=("$t")
    done
    sorted=($(printf '%s\n' "${times[@]}" | sort -n))
    echo "${sorted[$((N_RUN / 2))]}"
}

small=$(run_model 40 40 1000 3 1)
echo "KrABMaga Schelling-small (ms): $small"

large=$(run_model 100 100 8000 8 2)
echo "KrABMaga Schelling-large (ms): $large"
