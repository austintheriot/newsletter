#!/bin/sh

# This script allows applying transformations as a pre-commit hook and then staging them
# again after edit them.

# If any command fails, exit immediately with that command's exit status
set -eo pipefail

# Find all changed files for this commit
# Compute the diff only once to save a small amount of time.
CHANGED_FILES=$(git diff --name-only --cached --diff-filter=ACMR)

# Get only changed files that match our file suffix pattern
get_pattern_files() {
    pattern=$(echo "$*" | sed "s/ /\$\\\|/g")
    echo "$CHANGED_FILES" | { grep "$pattern$" || true; }
}

# Get all changed RUST files
FILES=$(get_pattern_files .rs)

if [[ -n "$FILES" ]]
then
    # Run the specified script file in the `script` directory for each staged file only
    source scripts/pre-commit.sh
    git add $files
    echo "re-staging: $($FILES)"
fi