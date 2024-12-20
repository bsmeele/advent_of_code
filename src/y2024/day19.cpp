#include <iostream>
#include <fstream>
#include <string>
#include <sstream>
#include <cstdint>
#include <vector>
#include <unordered_map>
#include <regex>

// Alternate solution:
//   Regex lol

uint64_t make_design(std::string design, std::vector<std::string> towels, std::unordered_map<std::string, uint64_t> &cache) {
    if (design.length() == 0) { return 1; }

    if (cache.count(design) != 0) { return cache[design]; }

    uint64_t num_designs = 0;
    for (int i = 0; i < towels.size(); i++) {
        if (towels[i].length() > design.length()) { continue; }

        bool equal = true;
        for (int j = 0; j < towels[i].length(); j++) {
            if (towels[i][j] != design[j]) {
                equal = false;
                break;
            }
        }

        if (equal) { num_designs += make_design(design.substr(towels[i].length()), towels, cache); }
    }

    cache[design] = num_designs;
    
    return num_designs;
}

void day19(bool test) {
    std::string filepath;
    if (test) { filepath = "input/y2024/day19/test.txt"; }
    else { filepath = "input/y2024/day19/input.txt"; }

    std::ifstream input(filepath);
    if (!input) {
        std::cerr << "Failed to open input file" << std::endl;
        return;
    }

    std::vector<std::string> towels;
    std::unordered_map<std::string, uint64_t> cache;
    bool read_towels = true;
    uint64_t part1 = 0;
    uint64_t part2 = 0;

    std::string line;
    while (std::getline(input, line)) {
        if (line.length() == 0) {
            read_towels = false;
            continue;
        }
        if (read_towels) {
            std::replace(line.begin(), line.end(), ',', ' ');
            std::stringstream ss(line);
            std::string towel;

            while (ss >> towel) {
                towels.push_back(towel);
            }
        } else {
            uint64_t num_designs = make_design(line, towels, cache);
            if (num_designs > 0) {
                part1 += 1;
            }
            part2 += num_designs;
        }
    }

    std::cout << "Year 2024 day 19 part 1: " << part1 << std::endl;
    std::cout << "Year 2024 day 19 part 2: " << part2 << std::endl;
}
