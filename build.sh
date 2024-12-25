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
  "$SRC_DIR/y2024/day09.cpp" \
  "$SRC_DIR/y2024/day10.cpp" \
  "$SRC_DIR/y2024/day11.cpp" \
  "$SRC_DIR/y2024/day12.cpp" \
  "$SRC_DIR/y2024/day13.cpp" \
  "$SRC_DIR/y2024/day14.cpp" \
  "$SRC_DIR/y2024/day15.cpp" \
  "$SRC_DIR/y2024/day16.cpp" \
  "$SRC_DIR/y2024/day17.cpp" \
  "$SRC_DIR/y2024/day18.cpp" \
  "$SRC_DIR/y2024/day19.cpp" \
  "$SRC_DIR/y2024/day20.cpp" \
  "$SRC_DIR/y2024/day21.cpp" \
  "$SRC_DIR/y2024/day22.cpp" \
  "$SRC_DIR/y2024/day23.cpp" \
  "$SRC_DIR/y2024/day24.cpp" \
  "$SRC_DIR/y2024/day25.cpp" \
  -o "$OUTPUT"

# Check for success
if [ $? -ne 0 ]; then
    echo "Build failed."
    exit 1
fi

echo "Build succeeded! Executable: $OUTPUT"
