package aoc.y2015;

import aoc.utils.InputLoader;

import java.util.List;
import java.util.ArrayList;
import java.io.*;

public class Day1 {
    public static void run(boolean test) {
        List<String> lines = new ArrayList<>();
        
        try {
            if (test) {
                lines = InputLoader.readLines("y2015/day1/test.txt");
            } else {
                lines = InputLoader.readLines("y2015/day1/input.txt");
            }
        } catch (IOException e) {
            System.err.println("Failed to load input: " + e.getMessage());
            return;
        }

        for (String line : lines) {
            if (line.isEmpty()) { continue; }
            System.out.printf("Day 1 part 1: %d%n", part1(line));
            System.out.printf("Day 1 part 2: %d%n", part2(line));
        }
    }

    private static int part1(String input) {
        int floor = 0;

        for (char c : input.toCharArray()) {
            if (c == '(') { floor += 1; }
            else if (c == ')') { floor -= 1; }
            else { System.out.println("Unrecognized character: " + c); }
        }

        return floor;
    }

    private static int part2(String input) {
        int floor = 0;

        for (int i = 0; i < input.length(); i++) {
            char c = input.charAt(i);
            if (c == '(') { floor += 1; }
            else if (c == ')') { floor -= 1; }
            else { System.out.println("Unrecognized character: " + c); }

            if (floor < 0) { return  i + 1; }
        }

        return 0;
    }
}
