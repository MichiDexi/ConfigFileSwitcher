#!/bin/bash

# Check if commit message was provided
if [ -z "$1" ]; then
    echo "Usage: ./gitpush.sh \"Your commit message here\""
    exit 1
fi

COMMIT_MSG="$1"

# Show Git status
echo "=== Git Status ==="
git status
echo "================="

# Add all changes
git add .

# Commit changes
git commit -m "$COMMIT_MSG"

# Push to the current branch
CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD)
git push origin "$CURRENT_BRANCH"

echo "✅ Done! Changes pushed to branch '$CURRENT_BRANCH'."
