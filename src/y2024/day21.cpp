#include <iostream>
#include <fstream>
#include <string>
#include <sstream>
#include <cstdint>
#include <vector>
#include <unordered_map>
#include <utility>

// A sequence that will be expanded into the shortest sequence has the following two properties
// Prefers to keep moving in the same direction
// Prefers to move to the furthest digit first (so preferred order is < v ^ < (it seems to prefer ^ over < even though they are the same distance, I have no idea why))

// Every digit in a code will require moving to that digit, and pressing A
// That means every digit can be independently evaluated

const std::unordered_map<char, std::pair<int, int>> KEYPAD_TO_COORD = {
    {'A', std::pair(0, 0)},
    {'0', std::pair(-1, 0)},
    {'1', std::pair(-2, -1)},
    {'2', std::pair(-1, -1)},
    {'3', std::pair(0, -1)},
    {'4', std::pair(-2, -2)},
    {'5', std::pair(-1, -2)},
    {'6', std::pair(0, -2)},
    {'7', std::pair(-2, -3)},
    {'8', std::pair(-1, -3)},
    {'9', std::pair(0, -3)},
    {'^', std::pair(-1, 0)},
    {'>', std::pair(0, 1)},
    {'v', std::pair(-1, 1)},
    {'<', std::pair(-2, 1)},
};

struct PairHash {
    template <typename T1, typename T2>
    std::size_t operator()(const std::pair<T1, T2>& p) const {
        return std::hash<T1>{}(p.first) ^ (std::hash<T2>{}(p.second) << 1);
    }
};

struct DoublePairHash {
    std::size_t operator()(const std::pair<std::pair<int, int>, std::pair<int, int>>& p) const {
        PairHash pairHash;
        return pairHash(p.first) ^ (pairHash(p.second) << 1);
    }
};

struct StringPairHash {
    std::size_t operator()(const std::pair<std::string, uint8_t>& key) const {
        std::hash<std::string> hashString;
        std::hash<uint8_t> hashUint8;

        return hashString(key.first) ^ (hashUint8(key.second) << 1);
    }
};

struct StringPairEqual {
    bool operator()(const std::pair<std::string, uint8_t>& lhs, const std::pair<std::string, uint8_t>& rhs) const {
        return lhs.first == rhs.first && lhs.second == rhs.second;
    }
};

std::string keypad(std::pair<int, int> start, std::pair<int, int> end) {
    if (start.first == end.first && start.second == end.second) {
        return "";
    }

    if (start.first == -2 && start.second == 0) {
        std::cout << "Unreachable (hopefully): pointing at empty space" << std::endl;
        return "";
    }

    std::pair<int, int> dir = std::pair(end.first - start.first, end.second - start.second);

    if (dir.first < 0 && !(start.first + dir.first == -2 && start.second == 0) ) {
        return std::string(abs(dir.first), '<')  + keypad(std::pair(start.first + dir.first, start.second),  end);
    } else if (dir.second > 0 && !(start.first == -2 && start.second + dir.second == 0)) {
        return std::string(abs(dir.second), 'v') + keypad(std::pair(start.first, start.second + dir.second), end);
    } else if (dir.second < 0 && !(start.first == -2 && start.second + dir.second == 0)) {
        return std::string(abs(dir.second), '^') + keypad(std::pair(start.first, start.second + dir.second), end);
    } else if (dir.first > 0) {
        return std::string(abs(dir.first), '>')  + keypad(std::pair(start.first + dir.first, start.second),  end);
    } else if (dir.second < 0) {
        return std::string(abs(dir.second), '^') + keypad(std::pair(start.first, start.second + dir.second), end);
    } else if (dir.first < 0) {
        return std::string(abs(dir.first), '<')  + keypad(std::pair(start.first + dir.first, start.second),  end);
    } else if (dir.second > 0) {
        return std::string(abs(dir.second), 'v') + keypad(std::pair(start.first, start.second + dir.second), end);
    }

    std::cout << "Unreachable (hopefully): no dir match" << std::endl;
    return "";
}

uint64_t encode_keypad(std::string code, uint16_t depth, std::unordered_map<std::pair<std::pair<int, int>, std::pair<int, int>>, std::string, DoublePairHash> &cache, std::unordered_map<std::pair<std::string, uint8_t>, uint64_t, StringPairHash> &cache2) {
    if (depth == 0) { return code.length(); }

    if (cache2.count(std::pair(code, depth)) != 0) { return cache2[std::pair(code, depth)]; }

    uint64_t length = 0;

    char start = 'A';
    for (int c = 0; c < code.length(); c++) {
        std::string encoded;
        if (cache.count(std::pair(KEYPAD_TO_COORD.at(start),  KEYPAD_TO_COORD.at(code[c]))) != 0) {
            encoded = cache[std::pair(KEYPAD_TO_COORD.at(start),  KEYPAD_TO_COORD.at(code[c]))];
        } else {
            encoded = keypad(KEYPAD_TO_COORD.at(start), KEYPAD_TO_COORD.at(code[c])) + "A";
            cache[std::pair(KEYPAD_TO_COORD.at(start),  KEYPAD_TO_COORD.at(code[c]))] = encoded;
        }
        length += encode_keypad(encoded, depth-1, cache, cache2);
        start = code[c];
    }

    cache2[std::pair(code, depth)] = length;

    return length;
}

void day21(bool test) {
    std::string filepath;
    if (test) { filepath = "input/y2024/day21/test.txt"; }
    else { filepath = "input/y2024/day21/input.txt"; }

    std::ifstream input(filepath);
    if (!input) {
        std::cerr << "Failed to open input file" << std::endl;
        return;
    }

    uint64_t part1 = 0;
    uint64_t part2 = 0;

    std::unordered_map<std::pair<std::pair<int, int>, std::pair<int, int>>, std::string, DoublePairHash> cache;
    std::unordered_map<std::pair<std::string, uint8_t>, uint64_t, StringPairHash> cache2;

    std::string line;
    while (std::getline(input, line)) {
        uint64_t length1 = encode_keypad(line, 3, cache, cache2);
        uint64_t length2 = encode_keypad(line, 26, cache, cache2);

        line.pop_back();
        part1 += length1 * std::stoull(line);
        part2 += length2 * std::stoull(line);
    }

    std::cout << "Year 2024 day 21 part 1: " << part1 << std::endl;
    std::cout << "Year 2024 day 21 part 2: " << part2 << std::endl;
}
