#include <iostream>
#include <fstream>
#include <string>
#include <sstream>
#include <utility>
#include <vector>
#include <regex>
#include <cstdint>

int64_t play_crane(std::pair<int64_t, int64_t> A, std::pair<int64_t, int64_t> B, std::pair<int64_t, int64_t> target) {
    int64_t B_sol = (target.second * A.first - target.first * A.second) / (A.first * B.second - B.first * A.second);
    int64_t A_sol = (target.first - B_sol * B.first) / A.first;

    if (A_sol * A.first + B_sol * B.first == target.first && A_sol * A.second + B_sol * B.second == target.second) {
        return A_sol * 3 + B_sol;
    } else { return -1; }
}

void day13(bool test) {
    std::string filepath;
    if (test) { filepath = "input/y2024/day13/test.txt"; }
    else { filepath = "input/y2024/day13/input.txt"; }

    std::ifstream input(filepath);
    if (!input) {
        std::cerr << "Failed to open input file" << std::endl;
        return;
    }

    std::vector<std::pair<int64_t, int64_t>> A;
    std::vector<std::pair<int64_t, int64_t>> B;
    std::vector<std::pair<int64_t, int64_t>> target;
    std::vector<std::pair<int64_t, int64_t>> target2;

    std::regex pattern(R"(Button [AB]: X\+(\d+), Y\+(\d+)|Prize: X=(\d+), Y=(\d+))");
    std::smatch match;

    int i = 0;
    std::string line;
    while (std::getline(input, line)) {
        if (std::regex_match(line, match, pattern)) {
            if (i % 4 == 0 && match[1].matched && match[2].matched) {
                int64_t x = std::stoi(match[1].str());
                int64_t y = std::stoi(match[2].str());
                A.push_back(std::pair(x, y));
            }
            else if (i % 4 == 1 && match[1].matched && match[2].matched) {
                int64_t x = std::stoi(match[1].str());
                int64_t y = std::stoi(match[2].str());
                B.push_back(std::pair(x, y));
            }
            else if (i % 4 == 2 && match[3].matched && match[4].matched) {
                int64_t x = std::stoi(match[3].str());
                int64_t y = std::stoi(match[4].str());
                target.push_back(std::pair(x, y));
                target2.push_back(std::pair(x + 10000000000000, y + 10000000000000));
            }
        }

        i += 1;
    }

    int64_t part1 = 0;
    int64_t part2 = 0;
    for (int i = 0; i < A.size(); i++) {
        int64_t num = play_crane(A[i], B[i], target[i]);
        if (num > 0) {
            part1 += num;
        }
        num = play_crane(A[i], B[i], target2[i]);
        if (num > 0) {
            part2 += num;
        }
    }

    std::cout << "Year 2024 day 13 part 1: " << part1 << std::endl;
    std::cout << "Year 2024 day 13 part 2: " << part2 << std::endl;
}
