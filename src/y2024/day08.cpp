#include <iostream>
#include <fstream>
#include <string>
#include <sstream>
#include <vector>
#include <unordered_map>
#include <unordered_set>
#include <utility>

void print_map(std::unordered_map<char, std::vector<std::pair<int, int>>> antennas, std::unordered_set<int> antinodes, std::pair<int, int> map_size) {
    // Todo: add antennas to print
    for (int y = 0; y < map_size.second; y++) {
        for (int x = 0; x < map_size.first; x++) {
            if (antinodes.count(x + y*map_size.first)) { std::cout << '#'; }
            else { std::cout << '.'; }
        }
        std::cout << std::endl;
    }
}

void day8(bool test) {
    std::string filepath;
    if (test) { filepath = "input/y2024/day08/test.txt"; }
    else { filepath = "input/y2024/day08/input.txt"; }

    std::ifstream input(filepath);
    if (!input) {
        std::cerr << "Failed to open input file" << std::endl;
        return;
    }

    std::unordered_map<char, std::vector<std::pair<int, int>>> antennas;
    std::pair<int, int> map_size;

    int y = 0;
    std::string line;
    while (std::getline(input, line)) {
        map_size = std::pair(line.length(), y+1);
        for (int i = 0; i < line.length(); i++) {
            if (line[i] != '.') {
                antennas[line[i]].push_back(std::pair(i, y));
            }
        }
        y += 1;
    }

    std::unordered_set<int> antinodes_1;
    std::unordered_set<int> antinodes_2;
    for (const auto& pair : antennas) {
        for (int i = 0; i < pair.second.size(); i++) {
            for (int j = i+1; j < pair.second.size(); j++) {
                std::pair<int, int> dist = std::pair(pair.second[i].first - pair.second[j].first, pair.second[i].second - pair.second[j].second);

                std::pair<int, int> antinode_1 = std::pair(pair.second[i].first + dist.first, pair.second[i].second + dist.second);
                std::pair<int, int> antinode_2 = std::pair(pair.second[j].first - dist.first, pair.second[j].second - dist.second);

                if (antinode_1.first >= 0 && antinode_1.first < map_size.first && antinode_1.second >= 0 && antinode_1.second < map_size.second) {
                    antinodes_1.insert(antinode_1.first + antinode_1.second*map_size.first);
                }
                if (antinode_2.first >= 0 && antinode_2.first < map_size.first && antinode_2.second >= 0 && antinode_2.second < map_size.second) {
                    antinodes_1.insert(antinode_2.first + antinode_2.second*map_size.first);
                }

                antinodes_2.insert(pair.second[i].first + pair.second[i].second*map_size.second);
                antinodes_2.insert(pair.second[j].first + pair.second[j].second*map_size.second);

                while(true) {
                    if (antinode_1.first < 0 || antinode_1.first >= map_size.first || antinode_1.second < 0 || antinode_1.second >= map_size.second) { break; }

                    antinodes_2.insert(antinode_1.first + antinode_1.second*map_size.first);

                    antinode_1.first += dist.first;
                    antinode_1.second += dist.second;
                }
                while(true) {
                    if (antinode_2.first < 0 || antinode_2.first >= map_size.first || antinode_2.second < 0 || antinode_2.second >= map_size.second) { break; }

                    antinodes_2.insert(antinode_2.first + antinode_2.second*map_size.first);

                    antinode_2.first -= dist.first;
                    antinode_2.second -= dist.second;
                }
            }
        }
    }

    std::cout << "Year 2024 day 8 part 1: " << antinodes_1.size() << std::endl;
    std::cout << "Year 2024 day 8 part 2: " << antinodes_2.size() << std::endl;
}
