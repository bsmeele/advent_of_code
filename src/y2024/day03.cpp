#include <iostream>
#include <fstream>
#include <string>
#include <sstream>
#include <regex>

void day3(bool test) {
    std::string filepath;
    if (test) { filepath = "input/y2024/day03/test.txt"; }
    else { filepath = "input/y2024/day03/input.txt"; }

    std::ifstream input(filepath);
    if (!input) {
        std::cerr << "Failed to open input file" << std::endl;
        return;
    }

    // This regex consists of three patterns separated by pipes
    // The pipes ensure that the regex matches with either of three patterns
    // The R and the opening brackets denote a raw string literal, which treats the contents as without escape sequences like \n, \t, or \\
    // The first pattern matches mul(num1,num2): (mul\((\d+),(\d+)\))
    //   \d+ matches a sequence of one or more digits
    //   The entire pattern is match group 1
    //   num1 is match group 2
    //   num2 is match group 3
    // The second pattern matches do(): (do\(\))
    //   The entire pattern is match group 4
    // The third pattern matches don't(): (don't\(\))
    //   The entire pattern is match gropu 5
    std::regex pattern(R"((mul\((\d+),(\d+)\))|(do\(\))|(don't\(\)))");

    int part1 = 0;
    int part2 = 0;
    bool do_mul = true;

    std::string line;
    while (std::getline(input, line)) {
        std::sregex_iterator begin(line.begin(), line.end(), pattern);
        std::sregex_iterator end;
        for (auto it = begin; it != end; it++) {
            std::smatch match = *it;
            
            if (match[1].matched) {
                int num1 = std::stoi(match[2].str());
                int num2 = std::stoi(match[3].str());

                if (num1 > 999 || num1 < 0 || num2 > 999 || num2 < 0) { continue; }
                part1 += num1 * num2;
                if (do_mul) { part2 += num1 * num2; }
            } else if (match[4].matched) { do_mul = true; }
            else if (match[5].matched) { do_mul = false; }
        }
    }

    std::cout << "Year 2024 day 3 part 1: " << part1 << std::endl;
    std::cout << "Year 2024 day 3 part 2: " << part2 << std::endl;
}