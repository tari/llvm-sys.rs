#!/bin/bash
#
# release-branches.sh llvm-15.0 llvm-16.0 -- --features prefer-dynamic -- minor
set -ex

branches=()

while [ "$1" != "--" ]
do
    branches+=("$1")
    shift
done
shift

for branch in "${branches[@]}"
do
    git checkout "$branch"
    cargo release "$@"
done
