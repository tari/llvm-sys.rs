#!/bin/bash
#
# Generate a C API diff between LLVM versions.
#
# Version numbers may be specified as normally written, like "3.6.0".
# "trunk" refers to the current latest development version.

branch_url() {
    typeset -r svnroot=https://llvm.org/svn/llvm-project/llvm

    if [[ $1 == trunk ]]
    then
        echo $svnroot/trunk
    else
        echo $svnroot/tags/RELEASE_$(echo $1 | sed s/\\.//g)/final
    fi
}

if [[ $# -ne 2 ]]
then
    echo "Usage: $0 <version1> <version2>" >&2
    echo "Emit a diff between LLVM C API releases <version1> and <version2>" >&2
    return 1
else
    typeset -r R1=$1
    typeset -r R2=$2
fi

if which colordiff >/dev/null
then
    colordiff="colordiff"
else
    colordiff="cat"
fi

svn diff $(branch_url $R1)/include/llvm-c $(branch_url $R2)/include/llvm-c | $colordiff

echo "For new major verions, check llvm/Config/ for new target functions."
