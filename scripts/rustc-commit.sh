#!/bin/sh
set -e

if [ $# -lt 1 ]
then
    echo "Usage: $0 <llvm_version>" >&2
    echo "Example: $0 19" >&2
    exit 1
fi

LLVM_VERSION=$1

URL="https://api.github.com/repos/rust-lang/rust/commits?path=.gitmodules"
HEADERS="Accept: application/json"

curl -s -X GET "$URL" -H "$HEADERS" | \
    jq "[.[] | select(.commit.message | contains (\"LLVM\") and contains (\"$LLVM_VERSION\")) | {sha: .sha, message: .commit.message}]"
