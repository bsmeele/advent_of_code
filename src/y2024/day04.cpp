#include <iostream>
#include <fstream>
#include <string>
#include <sstream>
#include <vector>

void day4(bool test) {
    std::string filepath;
    if (test) { filepath = "input/y2024/day04/test.txt"; }
    else { filepath = "input/y2024/day04/input.txt"; }

    std::ifstream input(filepath);
    if (!input) {
        std::cerr << "Failed to open input file" << std::endl;
        return;
    }

    std::vector<std::vector<char>> wordsearch;

    std::string line;
    while (std::getline(input, line)) {
        std::vector<char> row;
        for (int i = 0; i < line.length(); i++) {
            row.push_back(line[i]);
        }
        wordsearch.push_back(row);
    }

    int part1 = 0;
    int part2 = 0;
    for (int i = 0; i < wordsearch.size(); i++) {
        for (int j = 0; j < wordsearch[i].size(); j++) {
            // Up
            if (i >= 3 && wordsearch[i][j] == 'X' && wordsearch[i-1][j] == 'M' && wordsearch[i-2][j] == 'A' && wordsearch[i-3][j] == 'S') { part1 += 1; }
            
            // Up right
            if (i >= 3 && j < wordsearch[i].size()-3 && wordsearch[i][j] == 'X' && wordsearch[i-1][j+1] == 'M' && wordsearch[i-2][j+2] == 'A' && wordsearch[i-3][j+3] == 'S') { part1 += 1; }

            // Right
            if (j < wordsearch[i].size()-3 && wordsearch[i][j] == 'X' && wordsearch[i][j+1] == 'M' && wordsearch[i][j+2] == 'A' && wordsearch[i][j+3] == 'S') { part1 += 1; }

            // Down right
            if (i < wordsearch.size()-3 && j < wordsearch[i].size()-3 && wordsearch[i][j] == 'X' && wordsearch[i+1][j+1] == 'M' && wordsearch[i+2][j+2] == 'A' && wordsearch[i+3][j+3] == 'S') { part1 += 1; }

            // Down
            if (i < wordsearch.size()-3 && wordsearch[i][j] == 'X' && wordsearch[i+1][j] == 'M' && wordsearch[i+2][j] == 'A' && wordsearch[i+3][j] == 'S') { part1 += 1; }

            // Down left
            if (i < wordsearch.size()-3 && j >= 3 && wordsearch[i][j] == 'X' && wordsearch[i+1][j-1] == 'M' && wordsearch[i+2][j-2] == 'A' && wordsearch[i+3][j-3] == 'S') { part1 += 1; }

            // Left
            if (j >= 3 && wordsearch[i][j] == 'X' && wordsearch[i][j-1] == 'M' && wordsearch[i][j-2] == 'A' && wordsearch[i][j-3] == 'S') { part1 += 1; }

            // Up left
            if (i >= 3 && j >= 3 && wordsearch[i][j] == 'X' && wordsearch[i-1][j-1] == 'M' && wordsearch[i-2][j-2] == 'A' && wordsearch[i-3][j-3] == 'S') { part1 += 1; }

            // M.M
            // .A.
            // S.S
            if (i > 0 && i < wordsearch.size()-1 && j > 0 && j < wordsearch[i].size()-1
            && wordsearch[i][j] == 'A' && wordsearch[i-1][j-1] == 'M' && wordsearch[i-1][j+1] == 'M' && wordsearch[i+1][j+1] == 'S' && wordsearch[i+1][j-1] == 'S') { part2 += 1; }

            // M.S
            // .A.
            // M.S
            if (i > 0 && i < wordsearch.size()-1 && j > 0 && j < wordsearch[i].size()-1
            && wordsearch[i][j] == 'A' && wordsearch[i-1][j-1] == 'M' && wordsearch[i-1][j+1] == 'S' && wordsearch[i+1][j+1] == 'S' && wordsearch[i+1][j-1] == 'M') { part2 += 1; }

            // S.M
            // .A.
            // S.M
            if (i > 0 && i < wordsearch.size()-1 && j > 0 && j < wordsearch[i].size()-1
            && wordsearch[i][j] == 'A' && wordsearch[i-1][j-1] == 'S' && wordsearch[i-1][j+1] == 'M' && wordsearch[i+1][j+1] == 'M' && wordsearch[i+1][j-1] == 'S') { part2 += 1; }
            
            // S.S
            // .A.
            // M.M
            if (i > 0 && i < wordsearch.size()-1 && j > 0 && j < wordsearch[i].size()-1
            && wordsearch[i][j] == 'A' && wordsearch[i-1][j-1] == 'S' && wordsearch[i-1][j+1] == 'S' && wordsearch[i+1][j+1] == 'M' && wordsearch[i+1][j-1] == 'M') { part2 += 1; }
        }
    }

    std::cout << "Year 2024 day 4 part 1: " << part1 << std::endl;
    std::cout << "Year 2024 day 4 part 2: " << part2 << std::endl;
}
