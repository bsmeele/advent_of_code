#include <iostream>
#include <fstream>
#include <string>
#include <sstream>
#include <unordered_set>
#include <utility>

// Improvement: Only check for loops when placing an object that is on an originally visited tile

void print_map(std::unordered_set<int> map, std::unordered_set<int> visited, std::pair<int, int> map_size, std::pair<int, int> guard, std::pair<int, int> orientation) {
    std::cout << "Map size: " << map_size.first << " " << map_size.second << std::endl;
    for (int y = 0; y < map_size.second; y++) {
        for (int x = 0; x < map_size.first; x++) {
            if (x == guard.first && y == guard.second) {
                if (orientation.first == 0 && orientation.second == -1) { std::cout << '^'; }
                else if (orientation.first == 1 && orientation.second == 0) { std::cout << '>'; }
                else if (orientation.first == 0 && orientation.second == 1) { std::cout << 'V'; }
                else if (orientation.first == -1 && orientation.second == 0) { std::cout << '<'; }
            } else if (map.count(x + y*map_size.first)) { std::cout << '#'; }
            else if (visited.count(x + y*map_size.first + -2*map_size.first*map_size.second) || visited.count(x + y*map_size.first + -1*map_size.first*map_size.second)
            || visited.count(x + y*map_size.first + map_size.first*map_size.second) || visited.count(x + y*map_size.first + 2*map_size.first*map_size.second)) {
                std::cout << 'X';
            }
            else { std::cout << '.'; }
        }
        std::cout << std::endl;
    }
    std::cout << std::endl;
}

std::pair<std::unordered_set<int>, bool> play_map(std::unordered_set<int> map, std::pair<int, int> map_size, std::pair<int, int> guard, std::pair<int, int> orientation) {
    std::unordered_set<int> visited;
    while(true) {
        if (visited.count(guard.first + guard.second*map_size.first + (orientation.first + 2*orientation.second)*map_size.first*map_size.second)) { return std::pair(visited, true); }
        visited.insert(guard.first + guard.second*map_size.first + (orientation.first + 2*orientation.second)*map_size.first*map_size.second);

        // Check space in front
        if ((guard.first + orientation.first) >= map_size.first || (guard.first + orientation.first) < 0
        || (guard.second + orientation.second) >= map_size.second || (guard.second + orientation.second) < 0) { return std::pair(visited, false); }
        else if (map.count(guard.first + orientation.first + (guard.second + orientation.second)*map_size.first) == 0) {
            // Step forward
            guard.first += orientation.first;
            guard.second += orientation.second;
        } else {
            // Rotate
            if (orientation.first == 0 && orientation.second == -1) {
                orientation.first = 1;
                orientation.second = 0;
            } else if (orientation.first == 1 && orientation.second == 0) {
                orientation.first = 0;
                orientation.second = 1;
            } else if (orientation.first == 0 && orientation.second == 1) {
                orientation.first = -1;
                orientation.second = 0;
            } else if (orientation.first == -1 && orientation.second == 0) {
                orientation.first = 0;
                orientation.second = -1;
            }
        }
    }
}

void day6() {
    std::ifstream input("input/y2024/day06/input.txt");
    if (!input) {
        std::cerr << "Failed to open input file" << std::endl;
        return;
    }

    std::unordered_set<int> map;
    std::pair<int, int> guard;
    std::pair<int, int> orientation;
    std::pair<int, int> map_size;

    int y = 0;
    std::string line;
    while (std::getline(input, line)) {
        map_size = std::pair(line.length(), y+1);
        for (int x = 0; x < line.length(); x++) {
            switch (line[x]) {
                case '#':
                    map.insert(x + y*map_size.first);
                    break;
                case '^':
                    guard = std::pair(x, y);
                    orientation = std::pair(0, -1);
                    break;
                case '>':
                    guard = std::pair(x, y);
                    orientation = std::pair(1, 0);
                    break;
                case 'V':
                    guard = std::pair(x, y);
                    orientation = std::pair(0, 1);
                    break;
                case '<':
                    guard = std::pair(x, y);
                    orientation = std::pair(-1, 0);
                    break;
            }
        }
        y += 1;
    }

    std::unordered_set<int> visited;
    bool loop;
    // print_map(map, visited, map_size, guard, orientation);

    std::pair<std::unordered_set<int>, bool> res = play_map(map, map_size, guard, orientation);
    visited = res.first;
    loop = res.second;

    // print_map(map, visited, map_size, guard, orientation);

    int part1 = 0;
    for (int y = 0; y < map_size.second; y++) {
        for (int x = 0; x < map_size.first; x++) {
            if (visited.count(x + y*map_size.first + -2*map_size.first*map_size.second) || visited.count(x + y*map_size.first + -1*map_size.first*map_size.second)
            || visited.count(x + y*map_size.first + map_size.first*map_size.second) || visited.count(x + y*map_size.first + 2*map_size.first*map_size.second)) {
                part1 += 1;
            }
        }
    }
    std::cout << "Year 2024 day 6 part 1: " << part1 << std::endl;

    int part2 = 0;
    for (int y = 0; y < map_size.second; y++) {
        for (int x = 0; x < map_size.first; x++) {
            if (map.count(x + y*map_size.first) == 0 && !(x == guard.first && y == guard.second)) {
                map.insert(x + y*map_size.first);
                std::pair<std::unordered_set<int>, bool> res = play_map(map, map_size, guard, orientation);
                if (res.second) {
                    // print_map(map, res.first, map_size, guard, orientation);
                    part2 += 1;
                }
                map.erase(x + y*map_size.first);
            }
        }

        if (y%10 == 0) { std::cout << "Completed row " << y << std::endl; }
    }
    std::cout << "Year 2024 day 6 part 2: " << part2 << std::endl;
}
