#include <iostream>
#include <fstream>
#include <string>
#include <sstream>
#include <regex>
#include <vector>
#include <cstdint>

bool check_calibration(uint64_t target, uint64_t current, std::vector<uint64_t> rest, bool part2) {
    if (rest.size() == 0) {
        if (target == current) { return true; }
        else { return false; }
    }

    uint64_t num = rest[0];
    rest.erase(rest.begin());
    if (check_calibration(target, current + num, rest, part2)) { return true; }
    if (check_calibration(target, current * num, rest, part2)) { return true; };
    if (part2) {
        std::string str1 = std::to_string(current);
        std::string str2 = std::to_string(num);

        std::string concatenated = str1 + str2;

        uint64_t concatenatedNumber = std::stoull(concatenated);

        if (check_calibration(target, concatenatedNumber, rest, part2)) { return true; };
    }
    return false;
}

void day7(bool test) {
    std::string filepath;
    if (test) { filepath = "input/y2024/day07/test.txt"; }
    else { filepath = "input/y2024/day07/input.txt"; }

    std::ifstream input(filepath);
    if (!input) {
        std::cerr << "Failed to open input file" << std::endl;
        return;
    }

    uint64_t part1 = 0;
    uint64_t part2 = 0;

    std::regex pattern(R"((\d+):\s(\d+(\s+\d+)*)?)");
    std::smatch matches;

    std::string line;
    while (std::getline(input, line)) {
        if (std::regex_match(line, matches, pattern)) {
            uint64_t target = std::stoull(matches[1]);

            std::vector<uint64_t> operands;

            std::string rest_of_string = matches[2];
            std::regex number_pattern(R"(\d+)");

            auto numbers_begin = std::sregex_iterator(rest_of_string.begin(), rest_of_string.end(), number_pattern);
            auto numbers_end = std::sregex_iterator();

            // Extract all numbers into the vector
            for (auto it = numbers_begin; it != numbers_end; ++it) {
                operands.push_back(std::stoull(it->str()));
            }

            uint64_t current = operands[0];
            operands.erase(operands.begin());

            if (check_calibration(target, current, operands, false)) { part1 += target; }
            if (check_calibration(target, current, operands, true)) { part2 += target; }

        }
    }

    std::cout << "Year 2024 day 7 part 1: " << part1 << std::endl;
    std::cout << "Year 2024 day 7 part 2: " << part2 << std::endl;
}
