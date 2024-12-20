#include <iostream>
#include <fstream>
#include <string>
#include <sstream>
#include <cstdint>
#include <vector>
#include <utility>

// Note on input:
//   The 'maze' is a single path with no branches or dead ends
//   That means to solve the maze you visit every cell
// This solution uses that observation
// A general solution would need to maps:
//   One to track the distance from the start to a tile
//   One to track the distance from a tile to the end
//   Time saved is then tile2_dist_to_end - tile1_dist_to_start - cheat_dist

void draw_map(std::vector<std::vector<bool>> map, std::pair<uint32_t, uint32_t> start, std::pair<uint32_t, uint32_t> end) {
    for (int y = 0; y < map.size(); y++) {
        for (int x = 0; x < map[0].size(); x++) {
            if (map[y][x]) { std::cout << '#'; }
            else if (x == start.first && y == start.second) { std::cout << 'S'; }
            else if (x == end.first && y == end.second) { std::cout << 'E'; }
            else { std::cout << '.'; }
        }
        std::cout << std::endl;
    }
}

std::vector<std::vector<uint32_t>> solve_maze(std::vector<std::vector<bool>> map, std::pair<uint32_t, uint32_t> start, std::pair<uint32_t, uint32_t> end) {
    std::vector<std::vector<uint32_t>> dist(map.size(), std::vector<uint32_t>(map[0].size(), UINT32_MAX));
    dist[start.second][start.first] = 0;

    std::vector<std::pair<uint32_t, uint32_t>> stack;
    stack.push_back(start);

    while (stack.size() > 0) {
        auto [x, y] = stack.back();
        stack.pop_back();

        if (x == end.first && y == end.second) { continue; }

        if (y > 0 && !map[y-1][x] && dist[y-1][x] > dist[y][x] + 1) {
            dist[y-1][x] = dist[y][x] + 1;
            stack.push_back(std::pair(x, y-1));
        }
        if (x < map[0].size()-1 && !map[y][x+1] && dist[y][x+1] > dist[y][x] + 1) {
            dist[y][x+1] = dist[y][x] + 1;
            stack.push_back(std::pair(x+1, y));
        }
        if (y < map.size()-1 && !map[y+1][x] && dist[y+1][x] > dist[y][x] + 1) {
            dist[y+1][x] = dist[y][x] + 1;
            stack.push_back(std::pair(x, y+1));
        }
        if (x > 0 && !map[y][x-1] && dist[y][x-1] > dist[y][x] + 1) {
            dist[y][x-1] = dist[y][x] + 1;
            stack.push_back(std::pair(x-1, y));
        }
    }

    return dist;
}

void day20(bool test) {
    std::string filepath;
    if (test) { filepath = "input/y2024/day20/test.txt"; }
    else { filepath = "input/y2024/day20/input.txt"; }

    std::ifstream input(filepath);
    if (!input) {
        std::cerr << "Failed to open input file" << std::endl;
        return;
    }

    std::vector<std::vector<bool>> map;
    std::pair<uint32_t, uint32_t> start;
    std::pair<uint32_t, uint32_t> end;

    std::string line;
    while (std::getline(input, line)) {
        std::vector<bool> row;
        for (int x = 0; x < line.length(); x++) {
            if (line[x] == '#') { row.push_back(true); }
            else { row.push_back(false); }

            if (line[x] == 'S') { start = std::pair(x, map.size()); }
            else if (line[x] == 'E') { end = std::pair(x, map.size()); }
        }
        map.push_back(row);
    }

    std::vector<std::vector<uint32_t>> dist = solve_maze(map, start, end);

    uint32_t part1 = 0;
    uint32_t part2 = 0;
    for (int y = 0; y < map.size(); y++) {
        for (int x = 0; x < map[0].size(); x++) {
            if (map[y][x]) { continue; }
            for (int dy = 0; dy <= 20; dy++) {
                for (int dx = -20; dx <= 20; dx++) {
                    if (x + dx < 0 || x + dx > map[0].size()-1 || y + dy < 0 || y + dy > map.size()-1 || abs(dx) + abs(dy) <= 1  || abs(dx) + abs(dy) > 20 || map[y + dy][x + dx] || (dx < 0 && dy == 0)) { continue; }
                    uint32_t a = dist[y][x];
                    uint32_t b = dist[y + dy][x + dx];
                    int32_t cheat = std::max(a, b) - std::min(a, b) - (abs(dx) + abs(dy));
                    if (test && cheat >= 50) {
                        if (abs(dx) + abs(dy) == 2) { part1 += 1; }
                        part2 += 1;
                    } else if (!test && cheat >= 100) {
                        if (abs(dx) + abs(dy) == 2) { part1 += 1; }
                        part2 += 1;
                    }
                }
            }
        }
    }

    std::cout << "Year 2024 day 20 part 1: " << part1 << std::endl;
    std::cout << "Year 2024 day 20 part 2: " << part2 << std::endl;
}
