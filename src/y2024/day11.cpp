#include <iostream>
#include <fstream>
#include <string>
#include <sstream>
#include <vector>
#include <unordered_map>

// Step 1: Define a struct to represent the custom key (pair of unsigned long long and unsigned int)
struct Key {
    unsigned long long key1;
    unsigned int key2;

    // Define the equality operator for Key to compare two keys
    bool operator==(const Key& other) const {
        return key1 == other.key1 && key2 == other.key2;
    }
};

// Step 2: Define a custom hash function for Key
struct KeyHash {
    std::size_t operator()(const Key& k) const {
        // Combine the hashes of key1 and key2
        // Use std::hash to hash each key, and combine them using bit shifting
        std::size_t h1 = std::hash<unsigned long long>{}(k.key1);
        std::size_t h2 = std::hash<unsigned int>{}(k.key2);
        return h1 ^ (h2 << 1);  // XOR the hashes and shift to combine them
    }
};

unsigned long long blink(unsigned long long stone, unsigned int depth, std::unordered_map<Key, unsigned long long, KeyHash> &cache) {
    if (depth == 0) { return 1; }

    if (cache.count({stone, depth}) != 0) {
        return cache[{stone, depth}];
    }

    if (stone == 0) {
        unsigned long long res = blink(1, depth-1, cache);
        cache[{stone, depth}] = res;
        return res;
    } else if (std::to_string(stone).length()%2 == 0) {
        std::string stone_str = std::to_string(stone);
        unsigned long long stone_left = std::stoull(stone_str.substr(0, stone_str.length()/2));
        unsigned long long stone_right = std::stoull(stone_str.substr(stone_str.length()/2));

        unsigned long long res = blink(stone_left, depth-1, cache) + blink(stone_right, depth-1, cache);
        cache[{stone, depth}] = res;
        return res;
    } else {
        if (stone > stone*2024) { std::cout << "overflow = bad" << std::endl; }
        unsigned long long res = blink(stone*2024, depth-1, cache);
        cache[{stone, depth}] = res;
        return res;
    }
}

void day11(bool test) {
    std::string filepath;
    if (test) { filepath = "input/y2024/day11/test.txt"; }
    else { filepath = "input/y2024/day11/input.txt"; }

    std::ifstream input(filepath);
    if (!input) {
        std::cerr << "Failed to open input file" << std::endl;
        return;
    }

    std::vector<unsigned long long> stones;

    std::string line;
    while (std::getline(input, line)) {
        std::stringstream ss(line);
        unsigned long long num;
        while (ss >> num) {
            stones.push_back(num);
        }
    }

    std::unordered_map<Key, unsigned long long, KeyHash> cache; 
    unsigned long long part1 = 0;
    for (int i = 0; i < stones.size(); i++) {
        part1 += blink(stones[i], 25, cache);
    }

    std::cout << "Year 2024 day 11 part 1: " << part1 << std::endl;

    unsigned long long part2 = 0;
    for (int i = 0; i < stones.size(); i++) {
        part2 += blink(stones[i], 75, cache);
    }

    std::cout << "Year 2024 day 11 part 2: " << part2 << std::endl;
}
