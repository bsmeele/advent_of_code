#pragma once

#include <iostream>

void day1(bool);
void day2(bool);
void day3(bool);
void day4(bool);
void day5(bool);
void day6(bool);
void day7(bool);
void day8(bool);
void day9(bool);
void day10(bool);
void day11(bool);
void day12(bool);
void day13(bool);
void day14(bool);
void day15(bool);
void day16(bool);
void day17(bool);
void day18(bool);
void day19(bool);
void day20(bool);
void day21(bool);
void day22(bool);
void day23(bool);
void day24(bool);
void day25(bool);

void y2024(int day, bool test) {
    switch (day) {
        case 1:
            day1(test);
            break;
        case 2:
            day2(test);
            break;
        case 3:
            day3(test);
            break;
        case 4:
            day4(test);
            break;
        case 5:
            day5(test);
            break;
        case 6:
            day6(test);
            break;
        case 7:
            day7(test);
            break;
        case 8:
            day8(test);
            break;
        case 9:
            day9(test);
            break;
        case 10:
            day10(test);
            break;
        case 11:
            day11(test);
            break;
        case 12:
            day12(test);
            break;
        case 13:
            day13(test);
            break;
        case 14:
            day14(test);
            break;
        case 15:
            day15(test);
            break;
        case 16:
            day16(test);
            break;
        case 17:
            day17(test);
            break;
        case 18:
            day18(test);
            break;
        case 19:
            day19(test);
            break;
        case 20:
            day20(test);
            break;
        case 21:
            day21(test);
            break;
        case 22:
            day22(test);
            break;
        case 23:
            day23(test);
            break;
        case 24:
            day24(test);
            break;
        case 25:
            day25(test);
            break;
        case -1:
            day1(test);
            day2(test);
            day3(test);
            day4(test);
            day5(test);
            day6(test);
            day7(test);
            day8(test);
            day9(test);
            day10(test);
            day11(test);
            day12(test);
            day13(test);
            day14(test);
            day15(test);
            day16(test);
            day17(test);
            day18(test);
            day19(test);
            day20(test);
            day21(test);
            day22(test);
            day23(test);
            day24(test);
            day25(test);
        default:
            std::cout << "Unsupported day: " << day << std::endl;
    }
}
