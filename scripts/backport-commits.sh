#!/bin/bash

declare -a commits
declare -a branches

while [ "$1" != "--" ]
do
    branches+=("$1")
    shift
done

shift

while [ $# -gt 0 ]
do
    commits+=("$1")
    shift
done

for branch in "${branches[@]}"
do
    git checkout "$branch" || exit 1
    git cherry-pick -x "${commits[@]}" || $SHELL
done
