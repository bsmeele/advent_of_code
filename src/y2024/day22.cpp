#include <iostream>
#include <fstream>
#include <string>
#include <sstream>
#include <cstdint>
#include <vector>
#include <array>
#include <unordered_map>
#include <unordered_set>

struct ArrayHash {
    std::size_t operator()(const std::array<int16_t, 4>& key) const {
        std::hash<int> hashInt;
        std::size_t hash = 0;
        for (int i = 0; i < 4; i++) {
            hash ^= hashInt(key[i]) + 0x9e3779b9 + (hash << 6) + (hash >> 2);
        }
        return hash;
    }
};

uint64_t secret_number(uint64_t num) {
    uint64_t mult = num << 6;
    num = num ^ mult;
    num = num & 0xffffff;

    mult = num >> 5;
    num = num ^ mult;
    num = num & 0xffffff;

    mult = num << 11;
    num = num ^ mult;
    num = num & 0xffffff;

    return num;
}

void day22(bool test) {
    std::string filepath;
    if (test) { filepath = "input/y2024/day22/test.txt"; }
    else { filepath = "input/y2024/day22/input.txt"; }

    std::ifstream input(filepath);
    if (!input) {
        std::cerr << "Failed to open input file" << std::endl;
        return;
    }

    std::unordered_map<std::array<int16_t,4>, uint16_t, ArrayHash> banana;

    uint64_t part1 = 0;
    std::string line;
    while (std::getline(input, line)) {

        std::vector<uint8_t> price;
        uint64_t num = std::stoull(line);
        for (int i = 0; i < 2000; i++) {
            num = secret_number(num);
            price.push_back(num%10);
        }

        part1 += num;

        std::unordered_set<std::array<int16_t, 4>, ArrayHash> checked_changes;

        for (int i = 4; i < price.size(); i++) {
            int16_t c1 = price[i-3] - price[i-4];
            int16_t c2 = price[i-2] - price[i-3];
            int16_t c3 = price[i-1] - price[i-2];
            int16_t c4 = price[i] - price[i-1];
            if (checked_changes.count({c1, c2, c3, c4}) == 0) {
                checked_changes.insert({c1, c2, c3, c4});
                if (banana.count({c1, c2, c3, c4}) != 0) {
                    banana[{c1, c2, c3, c4}] += price[i];
                } else {
                    banana[{c1, c2, c3, c4}] = price[i];
                }
            }
        }
    }

    std::cout << "Year 2024 day 22 part1: " << part1 << std::endl;

    uint16_t biggest_banana = 0;
    for (auto [c, b] : banana) {
        if (b > biggest_banana) {
            biggest_banana = b;
        }
    }

    std::cout << "Year 2024 day 22 part2: " << biggest_banana << std::endl;
}
