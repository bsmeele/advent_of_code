#include <iostream>
#include <fstream>
#include <string>
#include <sstream>

void day_() {
    std::ifstream input("input/y2024/day__/test.txt");
    if (!input) {
        std::cerr << "Failed to open input file" << std::endl;
        return;
    }

    std::string line;
    while (std::getline(input, line)) {
        std::cout << line << std::endl;
    }
}
