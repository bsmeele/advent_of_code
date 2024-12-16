#include <iostream>
#include <fstream>
#include <string>
#include <sstream>
#include <vector>
#include <array>
#include <unordered_set>

void day10(bool test) {
    std::string filepath;
    if (test) { filepath = "input/y2024/day10/test.txt"; }
    else { filepath = "input/y2024/day10/input.txt"; }

    std::ifstream input(filepath);
    if (!input) {
        std::cerr << "Failed to open input file" << std::endl;
        return;
    }

    std::vector<std::vector<int>> map;
    std::vector<std::array<int, 2>> trailheads;

    std::string line;
    while (std::getline(input, line)) {
        std::vector<int> row;
        for (int i = 0; i < line.length(); i++) {
            if (line[i] - '0' == 0) { trailheads.push_back({i, (int) map.size()}); }
            row.push_back(line[i] - '0');
        }
        map.push_back(row);
    }

    int part1 = 0;
    int part2 = 0;
    for (int i = 0; i < trailheads.size(); i++) {
        std::vector<std::array<int, 2>> q;
        std::unordered_set<int> reached;

        q.push_back(trailheads[i]);

        while (q.size() > 0) {
            std::array<int, 2> loc = q[q.size()-1];
            q.pop_back();

            if (loc[1] > 0 && map[loc[1]-1][loc[0]] == map[loc[1]][loc[0]] + 1) {
                if (map[loc[1]-1][loc[0]] == 9) {
                    if (reached.count(loc[0] + (loc[1]-1)*map[loc[1]].size()) == 0) { reached.insert(loc[0] + (loc[1]-1)*map[loc[1]].size()); }
                    part2 += 1;
                }
                else { q.push_back({loc[0], loc[1]-1}); }
            }
            if (loc[0] < map[loc[1]].size()-1 && map[loc[1]][loc[0]+1] == map[loc[1]][loc[0]] + 1) {
                if (map[loc[1]][loc[0]+1] == 9) {
                    if (reached.count(loc[0]+1 + loc[1]*map[loc[1]].size()) == 0) { reached.insert(loc[0]+1 + loc[1]*map[loc[1]].size()); }
                    part2 += 1;
                }
                else { q.push_back({loc[0]+1, loc[1]}); }
            }
            if (loc[1] < map.size()-1 && map[loc[1]+1][loc[0]] == map[loc[1]][loc[0]] + 1) {
                if (map[loc[1]+1][loc[0]] == 9) {
                    if (reached.count(loc[0] + (loc[1]+1)*map[loc[1]].size()) == 0) { reached.insert(loc[0] + (loc[1]+1)*map[loc[1]].size()); }
                    part2 += 1;
                }
                else { q.push_back({loc[0], loc[1]+1}); }
            }
            if (loc[0] > 0 && map[loc[1]][loc[0]-1] == map[loc[1]][loc[0]] + 1) {
                if (map[loc[1]][loc[0]-1] == 9) {
                    if (reached.count(loc[0]-1 + loc[1]*map[loc[1]].size()) == 0) { reached.insert(loc[0]-1 + loc[1]*map[loc[1]].size()); }
                    part2 += 1;
                }
                else { q.push_back({loc[0]-1, loc[1]}); }
            }
        }
        part1 += reached.size();
    }

    std::cout << "Year 2024 day 10 part 1: " << part1 << std::endl;
    std::cout << "Year 2024 day 10 part 2: " << part2 << std::endl;
}
