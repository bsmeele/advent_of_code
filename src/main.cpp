#include <iostream>

#include "y2024/y2024.hpp"

int main(int argc, char* argv[]) {
    if (argc != 3) {
        std::cout << "Please provide a year and day" << std::endl;
        return 1;
    }

    int year = std::atoi(argv[1]);
    int day = std::atoi(argv[2]);

    switch (year) {
        case 2024:
            y2024(day);
            break;
        default:
            std::cout << "Unspported year: " << year << std::endl;
    }
}
