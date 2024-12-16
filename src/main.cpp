#include <iostream>

#include "y2024/y2024.hpp"

int main(int argc, char* argv[]) {
    if (argc < 2) {
        std::cout << "Please provide a year and day" << std::endl;
        return 1;
    }

    int year = std::atoi(argv[1]);
    int day = -1;
    if (argc >= 3) { day = std::atoi(argv[2]); }

    bool test = false;
    if (argc >= 4) { test = std::atoi(argv[3]); }

    switch (year) {
        case 2024:
            y2024(day, test);
            break;
        default:
            std::cout << "Unspported year: " << year << std::endl;
    }
}
