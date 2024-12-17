#include <iostream>
#include <fstream>
#include <string>
#include <sstream>
#include <cstdint>
#include <vector>
#include <regex>
#include <cmath>

uint64_t combo(uint64_t op, uint64_t A, uint64_t B, uint64_t C) {
    switch(op) {
        case 0:
        case 1:
        case 2:
        case 3:
            return op;
        case 4:
            return A;
        case 5:
            return B;
        case 6:
            return C;
    }
    std::cout << "Unreachable" << std::endl;
    return -1;
}

std::vector<uint64_t> run_program(std::vector<uint64_t> program, uint64_t A, uint64_t B, uint64_t C) {
    std::vector<uint64_t> out;
    uint64_t pc = 0;
    while(pc < program.size()) {
        uint64_t op;
        switch (program[pc]) {
            case 0: // adv
                op = combo(program[pc+1], A, B, C);
                A = A >> op;
                break;
            case 1: // bxl
                B = B ^ program[pc+1];
                break;
            case 2: // bst
                B = combo(program[pc+1], A, B, C) % 8;
                break;
            case 3: // jnz
                if (A != 0) {
                    pc = program[pc+1];
                    continue;
                }
                break;
            case 4: // bxc
                B = B ^ C;
                break;
            case 5: // out
                out.push_back(combo(program[pc+1], A, B, C) % 8);
                break;
            case 6: // bdv
                op = combo(program[pc+1], A, B, C);
                B = A >> op;
                break;
            case 7: // cdv
                op = combo(program[pc+1], A, B, C);
                C = A >> op;
                break;
        }
        pc += 2;
    }

    return out;
}

// The program is as follows:
//   B = A % 8
//   B = B ^4
//   C = A >> B
//   B = B ^ C
//   B = B ^ 4
//   out = B & 8
//   A = A >> 3
//   if (A != 0) jump 0
// This program has the properties that we can find the appropriate A by working backwards in groupings of 3 bits
// First we find the bottom 3 bits of A so that matches the last instruction of the program
// The we shift A 3 to the right and find the next bottom 3 bits of A such that the output matche the last two instructions
// Repeat untill out is equal in length and contents to the program
// If at any point there is no set of bottom 3 bits of A that matches the output, a prevous set of 3 bits was wrong
// In this case, backtrack through A untill the wrong 3 bit has been corrected
uint64_t find_A(std::vector<uint64_t> program, uint64_t A, uint64_t B, uint64_t C) {
    for (int i = 0; i < 8; i++) {
        std::vector<uint64_t> out = run_program(program, A + i, B, C);

        bool equal = true;
        for (int j = 0; j < out.size(); j++) {
            if (out[j] != program[program.size() - out.size() + j]) {
                equal = false;
                break;
            }
        }

        if (equal) {
            if (out.size() == program.size()) { return A + i; }
            else if (out.size() > program.size()) { return -1; }
            else {
                uint64_t res = find_A(program, (A + i) << 3, B, C);
                if (res != -1) {
                    return res;
                }
            }
        }
    }
    return -1;
}

void day17(bool test) {
    std::string filepath;
    if (test) { filepath = "input/y2024/day17/test2.txt"; }
    else { filepath = "input/y2024/day17/input.txt"; }

    std::ifstream input(filepath);
    if (!input) {
        std::cerr << "Failed to open input file" << std::endl;
        return;
    }

    uint64_t A;
    uint64_t B;
    uint64_t C;
    std::vector<uint64_t> program;
    bool p = true;

    std::regex reg_pattern(R"(Register ([A-C]): (\d+))");
    std::regex prog_pattern(R"(^Program:\s*([\d,]+)$)");
    std::regex num_pattern(R"(\d+)");
    std::smatch matches;

    std::string line;
    while (std::getline(input, line)) {
        if (line.length() == 0) { p = false; }
        if (p) {
            if (std::regex_match(line, matches, reg_pattern)) {
                if (matches[1].str()[0] == 'A') { A = std::stoul(matches[2].str()); }
                if (matches[1].str()[0] == 'B') { B = std::stoul(matches[2].str()); }
                if (matches[1].str()[0] == 'C') { C = std::stoul(matches[2].str()); }
            }
        } else {
            if (std::regex_match(line, matches, prog_pattern)) {
                std::string num_str = matches[1];

                auto begin = std::sregex_iterator(num_str.begin(), num_str.end(), num_pattern);
                auto end = std::sregex_iterator();

                for (auto it = begin; it != end; it++) {
                    program.push_back(std::stoul(it->str()));
                }
            }
        }
    }

    std::vector<uint64_t> out = run_program(program, A, B, C);
    std::cout << "Year 2024 day 17 part 1: ";
    for (int i = 0; i < out.size(); i++) {
        std::cout << out[i];
        if (i < out.size()-1) { std::cout << ','; }
    }
    std::cout << std::endl;

    A = find_A(program, 1, B, C);

    std::cout << "Year 2024 day 17 part 2: " << A << std::endl;
}
