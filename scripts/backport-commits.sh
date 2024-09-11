#!/bin/bash

declare -a commits
declare -a branches

if [ "$#" -eq 0 ]
then
    echo "Usage: $0 [branch...] -- [commit...]" >&2
    exit 1
fi

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
    git cherry-pick -x "${commits[@]}" || (
        echo "Cherry-pick failed: dropping to a shell to investigate." >&2
        echo "When done, type 'exit' to continue." >&2
        $SHELL
    )
done
