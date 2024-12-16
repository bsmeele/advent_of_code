#include <iostream>
#include <fstream>
#include <string>
#include <sstream>
#include <vector>
#include <algorithm>
#include <utility>

void day5(bool test) {
    std::string filepath;
    if (test) { filepath = "input/y2024/day05/test.txt"; }
    else { filepath = "input/y2024/day05/input.txt"; }

    std::ifstream input(filepath);
    if (!input) {
        std::cerr << "Failed to open input file" << std::endl;
        return;
    }

    std::vector<std::vector<int>> rules(100);
    bool rules_parse = true;

    int part1 = 0;
    int part2 = 0;

    std::string line;
    while (std::getline(input, line)) {
        if (rules_parse) {
            if (line.empty()) {
                rules_parse = false;
                continue;
            }

            int num1;
            int num2;

            sscanf(line.c_str(), "%d|%d", &num1, &num2);

            rules[num1].push_back(num2);
        } else {
            for (char& ch : line) {
                if (ch == ',') {
                    ch = ' ';
                }
            }
            
            std::stringstream ss(line);
            int num;
            std::vector<int> pages;

            while(ss >> num) { pages.push_back(num); }

            bool correct = true;

            for (int i = 0; i < pages.size(); i++) {
                for (int j = i; j < pages.size(); j++) {
                    if (std::count(rules[pages[j]].begin(), rules[pages[j]].end(), pages[i])) {
                        correct = false;
                        std::swap(pages[i], pages[j]);
                    }
                }
            }

            if (correct) { part1 += pages[(int) pages.size()/2]; }
            else { part2 += pages[(int) pages.size()/2]; }
        }
    }

    std::cout << "Year 2024 day 5 part 1: " << part1 << std::endl;
    std::cout << "Year 2024 day 5 part 2: " << part2 << std::endl;
}
