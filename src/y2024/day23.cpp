#include <iostream>
#include <fstream>
#include <string>
#include <sstream>
#include <cstdint>
#include <unordered_map>
#include <unordered_set>
#include <algorithm>
#include <vector>

// Part 1 and 2 could probably be combined
// Both parts doe a bunch of extra work
// Apperantly this is a np-complete problem (maximum clique problem?), but I dont'know the solution so I just check every permutation

void day23(bool test) {
    std::string filepath;
    if (test) { filepath = "input/y2024/day23/test.txt"; }
    else { filepath = "input/y2024/day23/input.txt"; }

    std::ifstream input(filepath);
    if (!input) {
        std::cerr << "Failed to open input file" << std::endl;
        return;
    }

    std::unordered_map<std::string, std::unordered_set<std::string>> graph;

    std::string line;
    while (std::getline(input, line)) {
        std::string c1 = line.substr(0, 2);
        std::string c2 = line.substr(3);

        if (graph.count(c1) == 0) {
            graph[c1] = {c2};
        } else {
            graph[c1].insert(c2);
        }

        if (graph.count(c2) == 0) {
            graph[c2] = {c1};
        } else {
            graph[c2].insert(c1);
        }
    }

    // This check is dependent on order, so it finds 6 times as many sets
    uint32_t part1 = 0;
    for (auto [c1, v1] : graph) {
        for (std::string c2 : v1) {
            for (std::string c3 : v1) {
                if (graph[c2].count(c3) != 0) {
                    if (c1[0] == 't' || c2[0] == 't' || c3[0] == 't') {
                        part1 += 1;
                    }
                }
            }
        }
    }
    part1 /= 6;

    std::cout << "Year 2024 day 23 part 1: " << part1 << std::endl;

    // Similar to part1, this is order dependent so does a bunch of unnecessary work
    std::unordered_set<std::string> biggest_lan;
    for (auto [c1, v1] : graph) {
        if (v1.size()+1 < biggest_lan.size()) { continue; }
        for (std::string c2 : v1) {
            std::unordered_set<std::string> lan = {c2};
            for (std::string c3 : v1) {
                bool connected = true;
                for (auto c4 : lan) {
                    if (graph[c3].count(c4) == 0) {
                        connected = false;
                        break;
                    }
                }
                if (connected) {
                    lan.insert(c3);
                }
            }
            lan.insert(c1);
            if (lan.size() > biggest_lan.size()) {
                biggest_lan = lan;
            }
        }
    }

    std::vector<std::string> sorted_lan(biggest_lan.begin(), biggest_lan.end());
    std::sort(sorted_lan.begin(), sorted_lan.end());
    std::cout << "Year 2024 day 23 part 2: ";
    for (int pc = 0; pc < sorted_lan.size(); pc++) {
        std::cout << sorted_lan[pc];
        if (pc != sorted_lan.size()-1) {
            std::cout << ',';
        }
    }
    std::cout << std::endl;
}
