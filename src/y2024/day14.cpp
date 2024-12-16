#include <iostream>
#include <fstream>
#include <string>
#include <sstream>
#include <utility>
#include <vector>
#include <regex>

void draw_robots(std::vector<std::pair<std::pair<int, int>, std::pair<int, int>>> robots, std::pair<int, int> map_size) {
    for (int y = 0; y < map_size.second; y++) {
        for (int x = 0; x < map_size.first; x++) {
            int robot = 0;
            for (int i = 0; i < robots.size(); i++) {
                if (robots[i].first.first == x && robots[i].first.second == y) { robot += 1; }
            }
            if (robot == 0) { std::cout << '.'; }
            else { std::cout << robot; }
        }
        std::cout << std::endl;
    }
}

void day14(bool test) {
    std::string filepath;
    std::pair<int, int> map_size;
    if (test) {
        map_size = std::pair(11, 7);
        filepath = "input/y2024/day14/test.txt";
    }
    else {
        map_size = std::pair(101, 103);
        filepath = "input/y2024/day14/input.txt";
    }

    std::ifstream input(filepath);
    if (!input) {
        std::cerr << "Failed to open input file" << std::endl;
        return;
    }

    std::regex pattern(R"(p=(\d+),(\d+)\sv=(-?\d+),(-?\d+))");
    std::smatch match;

    std::vector<std::pair<std::pair<int, int>, std::pair<int, int>>> robots;

    std::string line;
    while (std::getline(input, line)) {
        if (std::regex_match(line, match, pattern)) {
            if (match[1].matched && match[2].matched && match[3].matched && match[4].matched) {
                int x = std::stoi(match[1].str());
                int y = std::stoi(match[2].str());
                int vx = std::stoi(match[3].str());
                int vy = std::stoi(match[4].str());

                robots.push_back(std::pair(std::pair(x, y), std::pair(vx, vy)));
            }
        }
    }

    int q1 = 0;
    int q2 = 0;
    int q3 = 0;
    int q4 = 0;
    for (int i = 0; i < robots.size(); i++) {
        int x = ((robots[i].first.first + robots[i].second.first * 100) % map_size.first + map_size.first) % map_size.first;
        int y = ((robots[i].first.second + robots[i].second.second * 100) % map_size.second + map_size.second) % map_size.second;
        if (x < map_size.first/2 && y < map_size.second/2) { q1 += 1; }
        else if (x > map_size.first/2 && y < map_size.second/2) { q2 += 1; }
        else if (x < map_size.first/2 && y > map_size.second/2) { q3 += 1; }
        else if (x > map_size.first/2 && y > map_size.second/2) { q4 += 1; }
    }

    std::cout << "Year 2024 day 14 part 1: " << q1 * q2 * q3 * q4 << std::endl;

    // Opservation:
    //   Starting at 43 and every 103 seconds after that, the robots are somewhat horizontally clustered
    //   Starting at 68 and every 101 seconds after that, the robots are somewhat vertically clustered
    //   Coincidentally, 101 and 103 are the dimensions of the space
    // Hypothesis: The christmas tree appears when these two cycles overlap
    // Solution:
    //   68 + k*101 = 43 + n*103 has to have an integer solution for n and k
    //   Earliest solution: k = 64, n = 63 -> after 6532 seconds
    // Other solving methods:
    //   Calculate the variance after each second. The image should have the lowest variance

    if (!test) {
        int seconds = 6532;
        for (int i = 0; i < robots.size(); i++) {
            robots[i].first.first = ((robots[i].first.first + robots[i].second.first * seconds) % map_size.first + map_size.first) % map_size.first;
            robots[i].first.second = ((robots[i].first.second + robots[i].second.second * seconds) % map_size.second + map_size.second) % map_size.second;
        }
        // draw_robots(robots, map_size);

        std::cout << "Year 2024 day 14 part 2: " << seconds << std::endl;
    } else { std::cout << "Year 2024 day 14 part 2 has no solution for the test case" << std::endl; }
}
