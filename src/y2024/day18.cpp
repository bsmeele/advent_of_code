#include <iostream>
#include <fstream>
#include <string>
#include <sstream>
#include <cstdint>
#include <vector>
#include <regex>
#include <utility>
#include <unordered_set>

struct PairHash {
    template <typename T1, typename T2>
    std::size_t operator()(const std::pair<T1, T2>& p) const {
        auto h1 = std::hash<T1>()(p.first);
        auto h2 = std::hash<T2>()(p.second);
        return h1 ^ (h2 << 1);
    }
};

void draw_map(std::unordered_set<std::pair<uint32_t, uint32_t>, PairHash> map, std::pair<uint32_t, uint32_t> map_size, std::unordered_set<std::pair<uint32_t, uint32_t>, PairHash> path) {
    for (int y = 0; y < map_size.second; y++) {
        for (int x = 0; x < map_size.first; x++) {
            if (map.count(std::pair(x, y)) != 0) { std::cout << '#'; }
            else if (path.count(std::pair(x, y)) != 0) { std::cout << 'O'; }
            else { std::cout << '.'; }
        }
        std::cout << std::endl;
    }
}

std::unordered_set<std::pair<uint32_t, uint32_t>, PairHash> find_exit(std::unordered_set<std::pair<uint32_t, uint32_t>, PairHash> map, std::pair<uint32_t, uint32_t> map_size) {
    std::vector<std::pair<uint32_t, uint32_t>> stack;
    std::vector<std::vector<uint32_t>> dist(map_size.second, std::vector<uint32_t>(map_size.first, UINT32_MAX));
    stack.push_back(std::pair(0, 0));
    dist[0][0] = 0;
    while (stack.size() > 0) {
        auto [x, y] = stack.back();
        stack.pop_back();

        if (x == map_size.first-1 && y == map_size.second-1) { continue; }

        if (y > 0 && map.count(std::pair(x, y-1)) == 0 && dist[y-1][x] > dist[y][x] + 1) {
            stack.push_back(std::pair(x, y-1));
            dist[y-1][x] = dist[y][x] + 1;
        }
        if (x < map_size.second-1 && map.count(std::pair(x+1, y)) == 0 && dist[y][x+1] > dist[y][x] + 1) {
            stack.push_back(std::pair(x+1, y));
            dist[y][x+1] = dist[y][x] + 1;
        }
        if (y < map_size.second-1 && map.count(std::pair(x, y+1)) == 0 && dist[y+1][x] > dist[y][x] + 1) {
            stack.push_back(std::pair(x, y+1));
            dist[y+1][x] = dist[y][x] + 1;
        }
        if (x > 0 && map.count(std::pair(x-1, y)) == 0 && dist[y][x-1] > dist[y][x] + 1) {
            stack.push_back(std::pair(x-1, y));
            dist[y][x-1] = dist[y][x] + 1;
        }
    }


    std::unordered_set<std::pair<uint32_t, uint32_t>, PairHash> path;
    if (dist[map_size.second-1][map_size.first-1] == UINT32_MAX) { return path; }
    std::pair<uint32_t, uint32_t> loc = std::pair(map_size.first-1, map_size.second-1);

    while (true) {
        path.insert(loc);

        if (loc.first == 0 && loc.second == 0) { break; }

        if (loc.second > 0 && dist[loc.second-1][loc.first] == dist[loc.second][loc.first] - 1) { loc = std::pair(loc.first, loc.second-1); }
        else if (loc.first < map_size.first-1 && dist[loc.second][loc.first+1] == dist[loc.second][loc.first] - 1) { loc = std::pair(loc.first+1, loc.second); }
        else if (loc.second < map_size.second-1 && dist[loc.second+1][loc.first] == dist[loc.second][loc.first] - 1) { loc = std::pair(loc.first, loc.second+1); }
        else if (loc.first > 0 && dist[loc.second][loc.first-1] == dist[loc.second][loc.first] - 1) { loc = std::pair(loc.first-1, loc.second); }
    }

    return path;
}

void day18(bool test) {
    std::string filepath;
    if (test) { filepath = "input/y2024/day18/test.txt"; }
    else { filepath = "input/y2024/day18/input.txt"; }

    std::ifstream input(filepath);
    if (!input) {
        std::cerr << "Failed to open input file" << std::endl;
        return;
    }

    std::pair<uint32_t, uint32_t> map_size;
    if (test) { map_size = std::pair(7, 7); }
    else { map_size = std::pair(71, 71); }

    std::regex pattern(R"((\d+),(\d+))");
    std::smatch matches;

    std::vector<std::pair<uint32_t, uint32_t>> bytes;

    std::string line;
    while (std::getline(input, line)) {
        if (std::regex_match(line, matches, pattern)) {
            uint32_t x = std::stoul(matches[1].str());
            uint32_t y = std::stoul(matches[2].str());
            bytes.push_back(std::pair(x, y));
        }
    }
     
    std::unordered_set<std::pair<uint32_t, uint32_t>, PairHash> map;
    int lim;
    if (test) { lim = 12; }
    else { lim = 1024; }
    for (int i = 0; i < lim; i++) {
        map.insert(bytes[i]);
    }

    std::unordered_set<std::pair<uint32_t, uint32_t>, PairHash> path = find_exit(map, map_size);

    std::cout << "Year 2024 day 18 part 1: " << path.size()-1 << std::endl; // Start is not included in the step count

    std::pair<uint32_t, uint32_t> part2;
    for (int i = lim; i < bytes.size(); i++) {
        map.insert(bytes[i]);

        if (path.count(bytes[i]) == 0) { continue; }

        path = find_exit(map, map_size);
        if (path.size() == 0) {
            part2 = bytes[i];
            break;
        }
    }

    std::cout << "Year 2024 day 18 part 2: " << part2.first << ',' << part2.second << std::endl;
}
