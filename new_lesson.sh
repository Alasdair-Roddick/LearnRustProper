#!/usr/bin/env bash

set -e

# Ensure a number was provided
if [ -z "$1" ]; then
  echo "Usage: $0 <lesson_number>"
  exit 1
fi

LESSON_NUM="$1"

# Zero-pad numbers < 10
PADDED_NUM=$(printf "%02d" "$LESSON_NUM")

BRANCH_NAME="lesson-${PADDED_NUM}"
DIR_NAME="lesson-${PADDED_NUM}"
NOTES_FILE="notesLesson${LESSON_NUM}.md"

# Create and switch to the git branch
git checkout -b "$BRANCH_NAME"

# Create the Rust project
cargo new "$DIR_NAME"

# Enter the directory
cd "$DIR_NAME"

# Create notes file
touch "$NOTES_FILE"

echo """# Lesson $LESSON_NUM - 




## Lesson $LESSON_NUM Objectives:

> Theme:



# End-of-Lesson Reflection

## What I Leaned.

## What Confused Me?

## What Mental Model Shifted?

""" > "$NOTES_FILE"

echo "Created $BRANCH_NAME with project $DIR_NAME"
