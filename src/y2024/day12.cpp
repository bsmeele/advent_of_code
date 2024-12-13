#include <iostream>
#include <fstream>
#include <string>
#include <sstream>
#include <vector>
#include <array>
#include <unordered_set>

// Alternative solution: count corners

struct ArrayHash {
    std::size_t operator()(const std::array<int, 2>& arr) const {
        // Combine the two integers into a single hash value
        return std::hash<int>()(arr[0]) ^ (std::hash<int>()(arr[1]) << 1);
    }
};

int count_sides(std::unordered_set<std::array<int, 2>, ArrayHash> dir) {
    int sides = 0;
    std::unordered_set<std::array<int, 2>, ArrayHash> used;

    for (std::array<int, 2> init_plot : dir) {
        if (used.count(init_plot) != 0) { continue; }

        std::vector<std::array<int, 2>> stack;
        stack.push_back(init_plot);

        while (stack.size() > 0) {
            std::array<int, 2> plot = stack[stack.size()-1];
            stack.pop_back();

            if (used.count(plot) != 0) { continue; }

            used.insert(plot);

            if (dir.count({plot[0], plot[1]-1}) != 0) { stack.push_back({plot[0], plot[1]-1}); }
            if (dir.count({plot[0]+1, plot[1]}) != 0) { stack.push_back({plot[0]+1, plot[1]}); }
            if (dir.count({plot[0], plot[1]+1}) != 0) { stack.push_back({plot[0], plot[1]+1}); }
            if (dir.count({plot[0]-1, plot[1]}) != 0) { stack.push_back({plot[0]-1, plot[1]}); }
        }
        sides += 1;
    }
    return sides;
}

void day12() {
    std::ifstream input("input/y2024/day12/input.txt");
    if (!input) {
        std::cerr << "Failed to open input file" << std::endl;
        return;
    }

    std::vector<std::vector<char>> garden;

    std::string line;
    while (std::getline(input, line)) {
        std::vector<char> row;
        for (int i = 0; i < line.length(); i++) {
            row.push_back(line[i]);
        }
        garden.push_back(row);
    }

    std::vector<std::vector<std::array<int, 2>>> regions;
    std::unordered_set<std::array<int, 2>, ArrayHash> used;
    for (int y = 0; y < garden.size(); y++) {
        for (int x = 0; x < garden[y].size(); x++) {
            if (used.count({x, y}) != 0) { continue; }
            std::vector<std::array<int, 2>> region;
            std::vector<std::array<int, 2>> stack;
            stack.push_back({x, y});
            while (stack.size() > 0) {
                std::array<int, 2> plot = stack[stack.size()-1];
                stack.pop_back();

                if (used.count({plot[0], plot[1]}) != 0) { continue; }

                region.push_back(plot);
                used.insert({plot[0], plot[1]});

                if (plot[1] > 0 && garden[plot[1]][plot[0]] == garden[plot[1]-1][plot[0]]) { stack.push_back({plot[0], plot[1]-1}); }
                if (plot[0] < garden[plot[1]].size()-1 && garden[plot[1]][plot[0]] == garden[plot[1]][plot[0]+1]) { stack.push_back({plot[0]+1, plot[1]}); }
                if (plot[1] < garden.size()-1 && garden[plot[1]][plot[0]] == garden[plot[1]+1][plot[0]]) { stack.push_back({plot[0], plot[1]+1}); }
                if (plot[0] > 0 && garden[plot[1]][plot[0]] == garden[plot[1]][plot[0]-1]) { stack.push_back({plot[0]-1, plot[1]}); }
            }
            regions.push_back(region);
        }
    }


    int part1 = 0;
    int part2 = 0;
    for (int i = 0; i < regions.size(); i++) {
        int perimiter = 0;
        std::unordered_set<std::array<int, 2>, ArrayHash> up;
        std::unordered_set<std::array<int, 2>, ArrayHash> right;
        std::unordered_set<std::array<int, 2>, ArrayHash> down;
        std::unordered_set<std::array<int, 2>, ArrayHash> left;
        for (int j = 0; j < regions[i].size(); j++) {
            std::array<int, 2> plot = regions[i][j];
            if (plot[1] <= 0 || garden[plot[1]][plot[0]] != garden[plot[1]-1][plot[0]]) {
                perimiter += 1;
                up.insert({plot[0], plot[1]});
            }
            if (plot[0] >= garden[plot[1]].size()-1 || garden[plot[1]][plot[0]] != garden[plot[1]][plot[0]+1]) {
                perimiter += 1;
                left.insert({plot[0], plot[1]});
            }
            if (plot[1] >= garden.size()-1 || garden[plot[1]][plot[0]] != garden[plot[1]+1][plot[0]]) {
                perimiter += 1;
                down.insert({plot[0], plot[1]});
            }
            if (plot[0] <= 0 || garden[plot[1]][plot[0]] != garden[plot[1]][plot[0]-1]) {
                perimiter += 1;
                right.insert({plot[0], plot[1]});
            }
        }
        int sides = count_sides(up) + count_sides(right) + count_sides(down) + count_sides(left);
        
        // std::cout << garden[regions[i][0][1]][regions[i][0][0]] << ": " << regions[i].size() << " area, " << perimiter << " perimiter, " << sides << " sides" << std::endl;
        part1 += regions[i].size() * perimiter;
        part2 += regions[i].size() * sides;
    }

    std::cout << "Year 2024 day 12 part 1: " << part1 << std::endl;
    std::cout << "Year 2024 day 12 part 2: " << part2 << std::endl;
}
