#include <iostream>
#include <fstream>
#include <string>
#include <sstream>
#include <cstdint>
#include <vector>
#include <utility>
#include <algorithm>
#include <iomanip>
#include <unordered_set>

struct PairHash {
    template <typename T1, typename T2>
    std::size_t operator()(const std::pair<T1, T2>& p) const {
        auto h1 = std::hash<T1>()(p.first);
        auto h2 = std::hash<T2>()(p.second);
        return h1 ^ (h2 << 1);
    }
};

std::pair<int, int> dir_rotate(std::pair<int, int> dir, bool cw) {
    if (cw) {
        if (dir.first == 0 && dir.second == -1) { return std::pair(1, 0); }
        else if (dir.first == 1 && dir.second == 0) { return std::pair(0, 1); }
        else if (dir.first == 0 && dir.second == 1) { return std::pair(-1, 0); }
        else if (dir.first == -1 && dir.second == 0) { return std::pair(0, -1); }
    } else {
        if (dir.first == 0 && dir.second == -1) { return std::pair(-1, 0); }
        else if (dir.first == 1 && dir.second == 0) { return std::pair(0, -1); }
        else if (dir.first == 0 && dir.second == 1) { return std::pair(1, 0); }
        else if (dir.first == -1 && dir.second == 0) { return std::pair(0, 1); }
    }
    return dir;
}

int dir_to_num(std::pair<int, int> dir) {
    if (dir.first == 0 && dir.second == -1) { return 0; }
    else if (dir.first == 1 && dir.second == 0) { return 1; }
    else if (dir.first == 0 && dir.second == 1) { return 2; }
    else if (dir.first == -1 && dir.second == 0) { return 3; }
    return -1;
}

char dir_to_char(std::pair<int, int> dir) {
    if (dir.first == 0 && dir.second == -1) { return '^'; }
    else if (dir.first == 1 && dir.second == 0) { return '>'; }
    else if (dir.first == 0 && dir.second == 1) { return 'v'; }
    else if (dir.first == -1 && dir.second == 0) { return '<'; }
    return '.';
}

void draw_map(std::vector<std::vector<bool>> map, std::pair<int, int> start, std::pair<int, int> end, std::unordered_set<std::pair<int, int>, PairHash> tiles) {
    for (int y = 0; y < map.size(); y++) {
        for (int x = 0; x < map[y].size(); x++) {
            if (std::pair(x, y) == start) { std::cout << 'S'; }
            else if (std::pair(x, y) == end) { std::cout << 'E'; }
            else if (map[y][x]) { std::cout << '#'; }
            else if (tiles.count(std::pair(x, y)) != 0) { std::cout << 'O'; }
            else { std::cout << '.'; }
        }
        std::cout << std::endl;
    }
}

std::unordered_set<std::pair<int, int>, PairHash> best_path_tiles(std::vector<std::vector<std::vector<uint32_t>>> score, std::pair<int, int> end) {
    std::unordered_set<std::pair<int, int>, PairHash> tiles;
    std::vector<std::pair<std::pair<int, int>, std::pair<int, int>>> stack;

    int lowest = std::min(score[0][end.second][end.first], std::min(score[1][end.second][end.first], std::min(score[2][end.second][end.first], score[3][end.second][end.first])));
    if (score[0][end.second][end.first] == lowest) { stack.push_back(std::pair(end, std::pair(0, -1))); }
    else if (score[1][end.second][end.first] == lowest) { stack.push_back(std::pair(end, std::pair(1, 0))); }
    else if (score[2][end.second][end.first] == lowest) { stack.push_back(std::pair(end, std::pair(0, 1))); }
    else if (score[3][end.second][end.first] == lowest) { stack.push_back(std::pair(end, std::pair(-1, 0))); }

    while (stack.size() > 0) {
        auto [loc, dir] = stack.back();
        stack.pop_back();

        if (score[dir_to_num(dir)][loc.second - dir.second][loc.first - dir.first] == score[dir_to_num(dir)][loc.second][loc.first] - 1) {
            stack.push_back(std::pair(std::pair(loc.first - dir.first, loc.second - dir.second), dir));
            tiles.insert(loc);
        }

        if (score[dir_to_num(dir_rotate(dir, true))][loc.second][loc.first] == score[dir_to_num(dir)][loc.second][loc.first] - 1000) {
            stack.push_back(std::pair(loc, dir_rotate(dir, true)));
            tiles.insert(loc);
        }

        if (score[dir_to_num(dir_rotate(dir, false))][loc.second][loc.first] == score[dir_to_num(dir)][loc.second][loc.first] - 1000) {
            stack.push_back(std::pair(loc, dir_rotate(dir, false)));
            tiles.insert(loc);
        }
    }

    return tiles;
}

void day16(bool test) {
    std::string filepath;
    if (test) { filepath = "input/y2024/day16/test.txt"; }
    else { filepath = "input/y2024/day16/input.txt"; }

    std::ifstream input(filepath);
    if (!input) {
        std::cerr << "Failed to open input file" << std::endl;
        return;
    }

    std::vector<std::vector<bool>> map;
    std::pair<int, int> start;
    std::pair<int, int> end;

    int y = 0;
    std::string line;
    while (std::getline(input, line)) {
        std::vector<bool> row;
        for (int x = 0; x < line.length(); x++) {
            if (line[x] == '#') { row.push_back(true); }
            else if (line[x] == 'S') {
                start = std::pair(x, y);
                row.push_back(false);
            }
            else if (line[x] == 'E') {
                end = std::pair(x, y);
                row.push_back(false);
            }
            else { row.push_back(false); }
        }
        map.push_back(row);
        y += 1;
    }

    // draw_map(map, start, end);

    std::vector<std::vector<std::vector<uint32_t>>> score(4, std::vector<std::vector<uint32_t>>(map.size(), std::vector<uint32_t>(map[0].size(), UINT32_MAX)));
    score[1][start.second][start.first] = 0;
    std::vector<std::pair<std::pair<int, int>, std::pair<int, int>>> stack;
    stack.push_back(std::pair(start, std::pair(1, 0)));

    while (stack.size() > 0) {
        auto [loc, dir] = stack.back();
        stack.pop_back();

        if (loc.first == end.first && loc.second == end.second) { continue; }

        if (loc.first + dir.first >= 0 && loc.first + dir.first < map[0].size()
        && loc.second + dir.second >= 0 && loc.second + dir.second < map.size()
        && !map[loc.second + dir.second][loc.first + dir.first]) {
            if (score[dir_to_num(dir)][loc.second][loc.first] + 1 < score[dir_to_num(dir)][loc.second + dir.second][loc.first + dir.first]) {
                score[dir_to_num(dir)][loc.second + dir.second][loc.first + dir.first] = score[dir_to_num(dir)][loc.second][loc.first] + 1;
                stack.push_back(std::pair(std::pair(loc.first + dir.first, loc.second + dir.second), dir));
            }
        }

        if (score[dir_to_num(dir)][loc.second][loc.first] + 1000 < score[dir_to_num(dir_rotate(dir, true))][loc.second][loc.first]) {
            score[dir_to_num(dir_rotate(dir, true))][loc.second][loc.first] = score[dir_to_num(dir)][loc.second][loc.first] + 1000;
            stack.push_back(std::pair(loc, dir_rotate(dir, true)));
        }

        if (score[dir_to_num(dir)][loc.second][loc.first] + 1000 < score[dir_to_num(dir_rotate(dir, false))][loc.second][loc.first]) {
            score[dir_to_num(dir_rotate(dir, false))][loc.second][loc.first] = score[dir_to_num(dir)][loc.second][loc.first] + 1000;
            stack.push_back(std::pair(loc, dir_rotate(dir, false)));
        }
    }

    // int max_width = 6;
    // for (int d = 0; d < 4; d++) {
    //     for (int y = 0; y < map.size(); y++) {
    //         for (int x = 0; x < map[y].size(); x++) {
    //             if (score[d][y][x] == UINT32_MAX) { std::cout << std::setw(max_width) << '#'; }
    //             else { std::cout << std::setw(max_width) << score[d][y][x]; }
    //         }
    //         std::cout << std::endl;
    //     }
    //     std::cout << std::endl;
    // }

    // draw_map(map, start, end);

    int part1 = std::min(score[0][end.second][end.first], std::min(score[1][end.second][end.first], std::min(score[2][end.second][end.first], score[3][end.second][end.first])));

    std::cout << "Year 2024 day 16 part 1: " << part1 << std::endl;

    std::unordered_set<std::pair<int, int>, PairHash> tiles = best_path_tiles(score, end);
    std::cout << "Year 2024 day 16 part 2: " << tiles.size() << std::endl;

    // draw_map(map, start, end, tiles);

    // for (auto e : tiles) {
    //     std::cout << e.first << ' ' << e.second << std::endl;
    // }
}
