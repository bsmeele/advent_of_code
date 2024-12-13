@echo off
REM Build script for Advent of Code project

REM Paths to source files
set SRC_DIR=src
set OUTPUT=aoc.exe

REM Compile command
echo Compiling...
g++ -std=c++17 %SRC_DIR%\main.cpp ^
%SRC_DIR%\y2024\day01.cpp ^
%SRC_DIR%\y2024\day02.cpp ^
%SRC_DIR%\y2024\day03.cpp ^
%SRC_DIR%\y2024\day04.cpp ^
%SRC_DIR%\y2024\day05.cpp ^
%SRC_DIR%\y2024\day06.cpp ^
%SRC_DIR%\y2024\day07.cpp ^
%SRC_DIR%\y2024\day08.cpp ^
%SRC_DIR%\y2024\day09.cpp ^
%SRC_DIR%\y2024\day10.cpp ^
%SRC_DIR%\y2024\day11.cpp ^
%SRC_DIR%\y2024\day12.cpp ^
%SRC_DIR%\y2024\day13.cpp ^
-o %OUTPUT%

REM Check for success
if %errorlevel% neq 0 (
    echo Build failed.
    exit /b %errorlevel%
)

echo Build succeeded! Executable: %OUTPUT%
