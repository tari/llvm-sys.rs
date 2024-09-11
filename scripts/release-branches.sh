#!/bin/bash
#
# release-branches.sh llvm-15.0 llvm-16.0 -- --features prefer-dynamic -- minor
set -ex

if [ "$#" -eq 0 ]
then
    echo "Usage: $0 [branch...] -- [cargo-release args]" >&2
    echo "Example: $0 llvm-19 -- patch --features no-llvm-linking --execute"
fi

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
