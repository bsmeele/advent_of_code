#include <iostream>
#include <fstream>
#include <string>
#include <sstream>
#include <cstdint>
#include <unordered_set>
#include <utility>
#include <vector>

struct PairHash {
    template <typename T1, typename T2>
    std::size_t operator()(const std::pair<T1, T2>& p) const {
        auto h1 = std::hash<T1>()(p.first);
        auto h2 = std::hash<T2>()(p.second);
        return h1 ^ (h2 << 1);
    }
};

void draw_map(std::unordered_set<std::pair<int, int>, PairHash> walls, std::unordered_set<std::pair<int, int>, PairHash> boxes, std::pair<int, int> robot, std::pair<int, int> map_size, bool part2) {
    bool skip = false;
    for (int y = 0; y < map_size.second; y++) {
        for (int x = 0; x < map_size.first; x++) {
            if (skip) {
                skip = false;
                continue;
            }
            if (walls.count(std::pair(x, y)) != 0) { std::cout << '#'; }
            else if (boxes.count(std::pair(x, y)) != 0) {
                if (!part2) { std::cout << 'O'; }
                else {
                    std::cout << "[]";
                    skip = true;
                }
            }
            else if (std::pair(x, y) == robot) { std::cout << '@'; }
            else { std::cout << '.'; }
        }
        std::cout << std::endl;
    }
}

void day15(bool test) {
    std::string filepath;
    if (test) { filepath = "input/y2024/day15/test.txt"; }
    else { filepath = "input/y2024/day15/input.txt"; }

    std::ifstream input(filepath);
    if (!input) {
        std::cerr << "Failed to open input file" << std::endl;
        return;
    }

    std::unordered_set<std::pair<int, int>, PairHash> walls;
    std::unordered_set<std::pair<int, int>, PairHash> walls2;
    std::unordered_set<std::pair<int, int>, PairHash> boxes;
    std::unordered_set<std::pair<int, int>, PairHash> boxes2;
    std::pair<int, int> robot;
    std::pair<int, int> robot2;
    bool parse_map = true;
    std::pair<int, int> map_size;
    
    int y = 0;
    std::string line;
    while (std::getline(input, line)) {
        if (parse_map) {
            if (line.length() == 0) {
                parse_map = false;
                continue;
            }
            for (int x = 0; x < line.length(); x++) {
                if (line[x] == '#') {
                    walls.insert(std::pair(x, y));
                    walls2.insert(std::pair(2*x, y));
                    walls2.insert(std::pair(2*x+1, y));
                }
                else if (line[x] == 'O') {
                    boxes.insert(std::pair(x, y));
                    boxes2.insert(std::pair(2*x, y));
                }
                else if (line[x] == '@') {
                    robot = std::pair(x, y);
                    robot2 = std::pair(2*x, y);
                }
            }
            y += 1;
            map_size = std::pair(line.length(), y);
        } else {
            for (int c = 0; c < line.length(); c++) {
                std::pair<int, int> dir;
                if (line[c] == '^') { dir = std::pair(0, -1); }
                else if (line[c] == '>') { dir = std::pair(1, 0); }
                else if (line[c] == 'v') { dir = std::pair(0, 1); }
                else if (line[c] == '<') { dir = std::pair(-1, 0); }
                
                // ----- Part 1 -----
                std::vector<std::pair<int, int>> to_move;
                std::pair<int, int> to_check = std::pair(robot.first + dir.first, robot.second + dir.second);
                bool can_move = true;
                while (true) {
                    if (walls.count(to_check) != 0) {
                        can_move = false;
                        break;
                    }
                    if (boxes.count(to_check) == 0) { break; }
                    to_move.push_back(to_check);
                    to_check = std::pair(to_check.first + dir.first, to_check.second + dir.second);
                }

                if (can_move) {
                    robot.first += dir.first;
                    robot.second += dir.second;
                    for (int i = to_move.size()-1; i >= 0; i--) {
                        boxes.erase(to_move[i]);
                        boxes.insert(std::pair(to_move[i].first + dir.first, to_move[i].second + dir.second));
                    }
                }

                // ----- Part 2 -----
                to_move.clear();
                std::vector<std::pair<int, int>> to_check_vec;
                to_check_vec.push_back(std::pair(robot2.first + dir.first, robot2.second + dir.second));
                can_move = true;
                while (to_check_vec.size() > 0) {
                    to_check = to_check_vec[to_check_vec.size()-1];
                    to_check_vec.pop_back();

                    if (walls2.count(to_check) != 0) {
                        can_move = false;
                        break;
                    }

                    if (boxes2.count(to_check) == 0 && boxes2.count(std::pair(to_check.first - 1, to_check.second)) == 0) { continue; }
                    if (line[c] != '<') { to_check_vec.push_back(std::pair(to_check.first + dir.first, to_check.second + dir.second)); }
                    if (boxes2.count(to_check) != 0) {
                        to_move.push_back(to_check);
                        if (line[c] != '>') { to_check_vec.push_back(std::pair(to_check.first + dir.first + 1, to_check.second + dir.second)); }
                    } else {
                        if (line[c] != '>') { to_move.push_back(std::pair(to_check.first-1, to_check.second)); }
                        if (line[c] != '>') { to_check_vec.push_back(std::pair(to_check.first + dir.first - 1, to_check.second + dir.second)); }
                    }
                }

                if (can_move) {
                    robot2.first += dir.first;
                    robot2.second += dir.second;
                    for (int i = to_move.size()-1; i >= 0; i--) {
                        boxes2.erase(to_move[i]);
                        boxes2.insert(std::pair(to_move[i].first + dir.first, to_move[i].second + dir.second));
                    }
                }
            }
        }
    }

    int part1 = 0;
    for (auto box : boxes) {
        part1 += box.first + box.second*100;
    }

    int part2 = 0;
    for (auto box : boxes2) {
        part2 += box.first + box.second*100;
    }

    std::cout << "Year 2024 day 15 part 1: " << part1 << std::endl;
    std::cout << "Year 2024 day 15 part 2: " << part2 << std::endl;
}
