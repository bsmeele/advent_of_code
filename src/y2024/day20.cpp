#include <iostream>
#include <fstream>
#include <string>
#include <sstream>
#include <cstdint>

void day20(bool test) {
    std::string filepath;
    if (test) { filepath = "input/y2024/day20/test.txt"; }
    else { filepath = "input/y2024/day20/input.txt"; }

    std::ifstream input(filepath);
    if (!input) {
        std::cerr << "Failed to open input file" << std::endl;
        return;
    }

    std::string line;
    while (std::getline(input, line)) {
        std::cout << line << std::endl;
    }
    std::cout << "Year 2024 day 20 not implemented yet" << std::endl;
}
