#include <iostream>
#include <fstream>
#include <string>
#include <sstream>
#include <vector>
#include <algorithm>

void day1() {
    std::ifstream input("input/y2024/day01/input");
    if (!input) {
        std::cerr << "Failed to open input file" << std::endl;
        return;
    }

    std::vector<int> list1;
    std::vector<int> list2;

    std::string line;
    while (std::getline(input, line)) {
        std::stringstream ss(line);
        int num1, num2;
        ss >> num1 >> num2;

        list1.push_back(num1);
        list2.push_back(num2);
    }

    input.close();

    std::sort(list1.begin(), list1.end());
    std::sort(list2.begin(), list2.end());

    int dist = 0;
    int sim = 0;
    for (int i = 0; i < list1.size(); i++) {
        dist += abs(list1[i] - list2[i]);

        int count = 0;
        for (int j = 0; j < list2.size(); j++) {
            if (list1[i] == list2[j]) { count += 1; }
        }
        sim += list1[i] * count;
    }

    std::cout << "Year 2024 day 1 part 1: " << dist << std::endl;
    std::cout << "Year 2024 day 1 part 2: " << sim << std::endl;
}
