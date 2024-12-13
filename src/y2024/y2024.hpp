#pragma once

#include <iostream>

void day1();
void day2();
void day3();
void day4();
void day5();
void day6();
void day7();
void day8();
void day9();
void day10();
void day11();
void day12();
void day13();

void y2024(int day) {
    switch (day) {
        case 1:
            day1();
            break;
        case 2:
            day2();
            break;
        case 3:
            day3();
            break;
        case 4:
            day4();
            break;
        case 5:
            day5();
            break;
        case 6:
            day6();
            break;
        case 7:
            day7();
            break;
        case 8:
            day8();
            break;
        case 9:
            day9();
            break;
        case 10:
            day10();
            break;
        case 11:
            day11();
            break;
        case 12:
            day12();
            break;
        case 13:
            day13();
            break;
        default:
            std::cout << "Unsupported day: " << day << std::endl;
    }
}
