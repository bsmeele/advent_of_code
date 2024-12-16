#include <iostream>
#include <fstream>
#include <string>
#include <sstream>
#include <vector>

struct Block {
    int idx;
    int length;
    int prev;
    int next;

    Block(int idx, int length) : idx(idx), length(length), prev(-1), next(-1) {}
};

void day9(bool test) {
    std::string filepath;
    if (test) { filepath = "input/y2024/day09/test.txt"; }
    else { filepath = "input/y2024/day09/input.txt"; }

    std::ifstream input(filepath);
    if (!input) {
        std::cerr << "Failed to open input file" << std::endl;
        return;
    }

    int idx = 0;
    std::vector<int> filesystem;
    std::vector<Block> filesystem2;
    bool file = true;

    std::string line;
    while (std::getline(input, line)) {
        for (int i = 0; i < line.length(); i++) {
            int num = line[i] - '0';
            if (file) {
                filesystem2.push_back(Block((int) filesystem.size(), num));
                filesystem2[idx].prev = idx-1;
                if (filesystem2.size() >= 2) {
                    filesystem2[idx-1].next = idx;
                }
            }
            for (int j = 0; j < num; j++) {
                if (file) { filesystem.push_back(idx); }
                else { filesystem.push_back(-1); }
            }
            if (file && num != 0) { idx += 1; }
            file = !file;
        }
    }

    int r = filesystem.size()-1;
    while (filesystem[r] == -1) { r -= 1; }
    for (int l = 0; l < r; l++) {
        if (filesystem[l] == -1) {
            filesystem[l] = filesystem[r];
            filesystem[r] = -1;
            while (filesystem[r] == -1) { r -= 1; }
        }
    }

    unsigned long long part1 = 0;
    for (int i = 0; i < filesystem.size(); i++) {
        if (filesystem[i] == -1) { break; }
        part1 += i * filesystem[i];
    }

    std::cout << "Year 2024 day 9 part 1: " << part1 << std::endl;

    r = filesystem2.size()-1;
    int l = 0;
    while(r > 0) {
        int dist = filesystem2[filesystem2[l].next].idx - (filesystem2[l].idx + filesystem2[l].length);
        if (dist >= filesystem2[r].length) {
            filesystem2[filesystem2[r].prev].next = filesystem2[r].next;
            filesystem2[filesystem2[r].next].prev = filesystem2[r].prev;

            filesystem2[r].prev = l;
            filesystem2[r].next = filesystem2[l].next;
            filesystem2[r].idx = filesystem2[l].idx + filesystem2[l].length;

            filesystem2[filesystem2[l].next].prev = r;
            filesystem2[l].next = r;

            l = 0;
            r -= 1;
        } else {
            l = filesystem2[l].next;
            if (l == r) {
                l = 0;
                r -= 1;
            }
        }
    }
    
    unsigned long long part2 = 0;
    for (int i = 0; i < filesystem2.size(); i++) {
        for (int j = 0; j < filesystem2[i].length; j++) {
            part2 += i * (filesystem2[i].idx + j);
        }
    }

    std::cout << "Year 2024 day 9 part 2: " << part2 << std::endl;
}
