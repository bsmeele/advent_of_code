#include <iostream>
#include <fstream>
#include <string>
#include <sstream>
#include <vector>

bool check_report(std::vector<int> report) {
    if (report.size() == 0) { return true; }

    int dir = 0;
    int prev = report[0];

    for (int i = 1; i < report.size(); i++) {
        int next = report[i];

        int dif = next - prev;

        if (abs(dif) == 0 || abs(dif) > 3) {
            return false;
        }
        if ((dif > 0 && dir < 0) || (dif < 0 && dir > 0)) {
            return false;
        }
        dir = dif;
        prev = next;
    }

    return true;
}

void day2() {
    std::ifstream input("input/y2024/day02/input.txt");
    if (!input) {
        std::cerr << "Failed to open input file" << std::endl;
        return;
    }

    int part1 = 0;
    int part2 = 0;

    std::string line;
    while (std::getline(input, line)) {
        std::stringstream ss(line);

        std::vector<int> list;
        int num;
        while (ss >> num) { list.push_back(num); }

        if (check_report(list)) {
            part1 += 1;
            part2 += 1;
            continue;
        }

        for (int i = 0; i < list.size(); i++) {
            std::vector<int> list2 = list;
            list2.erase(list2.begin() + i);
            if (check_report(list2)) {
                part2 += 1;
                break;
            }
        }
    }

    std::cout << "Year 2024 day 2 part 1: " << part1 << std::endl;
    std::cout << "Year 2024 day 2 part 2: " << part2 << std::endl;
}
