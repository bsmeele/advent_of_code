#include <iostream>
#include <fstream>
#include <string>
#include <sstream>
#include <cstdint>
#include <vector>
#include <utility>

void day25(bool test) {
    std::string filepath;
    if (test) { filepath = "input/y2024/day25/test.txt"; }
    else { filepath = "input/y2024/day25/input.txt"; }

    std::ifstream input(filepath);
    if (!input) {
        std::cerr << "Failed to open input file" << std::endl;
        return;
    }

    std::vector<std::vector<uint8_t>> locks;
    std::vector<std::vector<uint8_t>> keys;
    int flag = 0;
    std::vector<uint8_t> buf;
    int y = 0;

    std::string line;
    while (std::getline(input, line)) {
        if (line.length() == 0) {
            if (flag == -1) {
                locks.push_back(buf);
            } else if (flag == 1) {
                keys.push_back(buf);
            }
            buf.clear();
            flag = 0;
            y = 0;
            continue;
        }
        if (flag == 0) {
            if (line[0] == '#') {
                flag = -1;
            } else {
                flag = 1;
            }
            for (int i = 0; i < line.length(); i++) {
                buf.push_back(0);
            }
        } else {
            if (y >= 6) { continue; }

            for (int c = 0; c < line.length(); c++) {
                if (line[c] == '#') {
                    buf[c] += 1;
                }
            }
            y += 1;
        }
    }
    if (flag == -1) {
        locks.push_back(buf);
    } else if (flag == 1) {
        keys.push_back(buf);
    }

    uint32_t part1 = 0;
    for (int l = 0; l < locks.size(); l++) {
        for (int k = 0; k < keys.size(); k++) {
            bool fits = true;
            for (int i = 0; i < locks[l].size(); i++) {
                if (locks[l][i] + keys[k][i] > 6) {
                    fits = false;
                    break;
                }
            }
            if (fits) { part1 += 1; }
        }
    }

    std::cout << " Year 2024 day 25 part 1: " << part1 << std::endl;
}
