#!/bin/bash
# Build script for Advent of Code project on Linux

# Paths to source files
SRC_DIR="src"
OUTPUT="aoc"

# Compile command
echo "Compiling..."
g++ -std=c++17 \
  "$SRC_DIR/main.cpp" \
  "$SRC_DIR/y2024/day01.cpp" \
  "$SRC_DIR/y2024/day02.cpp" \
  "$SRC_DIR/y2024/day03.cpp" \
  "$SRC_DIR/y2024/day04.cpp" \
  "$SRC_DIR/y2024/day05.cpp" \
  "$SRC_DIR/y2024/day06.cpp" \
  "$SRC_DIR/y2024/day07.cpp" \
  "$SRC_DIR/y2024/day08.cpp" \
  -o "$OUTPUT"

# Check for success
if [ $? -ne 0 ]; then
    echo "Build failed."
    exit 1
fi

echo "Build succeeded! Executable: $OUTPUT"
