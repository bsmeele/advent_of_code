#include <iostream>
#include <fstream>
#include <string>
#include <sstream>
#include <cstdint>
#include <unordered_map>
#include <unordered_set>
#include <array>
#include <vector>
#include <utility>
#include <algorithm>

struct ArrayHash {
    std::size_t operator()(const std::array<std::string, 4>& arr) const {
        std::hash<std::string> hashString;
        std::size_t hash = 0;

        for (const auto& str : arr) {
            hash ^= hashString(str) + 0x9e3779b9 + (hash << 6) + (hash >> 2);
        }
        return hash;
    }
};

void day24(bool test) {
    std::string filepath;
    if (test) { filepath = "input/y2024/day24/test2.txt"; }
    else { filepath = "input/y2024/day24/input.txt"; }

    std::ifstream input(filepath);
    if (!input) {
        std::cerr << "Failed to open input file" << std::endl;
        return;
    }

    std::unordered_map<std::string, bool> wires;
    std::unordered_set<std::array<std::string, 4>, ArrayHash> gates;

    bool flag = true;

    std::string line;
    while (std::getline(input, line)) {
        if (line.length() == 0) {
            flag = false;
            continue;
        }
        if (flag) {
            wires[line.substr(0, 3)] = line[5] == '1';
        } else {
            std::stringstream ss(line);
            std::string in1, gate, in2, arrow, out;
            ss >> in1 >> gate >> in2 >> arrow >> out;

            gates.insert({in1, in2, gate, out});
        }
    }

    while (gates.size() > 0) {
        std::vector<std::array<std::string, 4>> to_remove;
        for (auto e : gates) {
            if (wires.count(e[0]) != 0 && wires.count(e[1]) != 0) {
                if (e[2] == "AND") {
                    wires[e[3]] = wires[e[0]] & wires[e[1]];
                } else if (e[2] == "OR") {
                    wires[e[3]] = wires[e[0]] | wires[e[1]];
                } else if (e[2] == "XOR") {
                    wires[e[3]] = wires[e[0]] ^ wires[e[1]];
                }
                to_remove.push_back(e);
            }
        }
        for (auto e : to_remove) {
            gates.erase(e);
        }
    }

    std::vector<std::pair<std::string, bool>> out;
    for (auto [k, v] : wires) {
        if (k[0] == 'z') {
            out.push_back(std::pair(k, v));
        }
    }

    std::sort(out.begin(), out.end(), [](const auto &a, const auto &b) {
        return a.first < b.first;
    });

    uint64_t part1 = 0;
    for (int i = 0; i < out.size(); i++) {
        if (out[i].second) {
            part1 = part1 | ((uint64_t) 1 << i);
        }
    }

    std::cout << "Year 2024 day 24 part 1: " << part1 << std::endl;

    // Input is 45 bit, there are 222 gates: ripple carry adder with 44 full adders and 1 half adder
    // Solved by manual inspection:
    //   z14 - vss
    //   kdh - hjf
    //   z31 - kpp
    //   z35 - sgj
    std::vector<std::string> swapped = {"z14", "vss", "kdh", "hjf", "z31", "kpp", "z35", "sgj"};
    std::sort(swapped.begin(), swapped.end());

    std::cout << "Year 2024 day 24 part 2: ";
    for (int i = 0; i < swapped.size(); i++) {
        std::cout << swapped[i];
        if (i < swapped.size()-1) {
            std::cout << ",";
        }
    }
    std::cout << std::endl;
}
