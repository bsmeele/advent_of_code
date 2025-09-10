package aoc.y2015;

import aoc.utils.InputLoader;
import aoc.utils.Point;

import java.util.List;
import java.util.ArrayList;
import java.io.*;
import java.util.HashSet;

public class Day3 {
    public static void run(boolean test) {
        List<String> lines = new ArrayList<>();
        
        try {
            if (test) {
                lines = InputLoader.readLines("y2015/day3/test.txt");
            } else {
                lines = InputLoader.readLines("y2015/day3/input.txt");
            }
        } catch (IOException e) {
            System.err.println("Failed to load input: " + e.getMessage());
            return;
        }

        for (String line : lines) {
            System.out.printf("Day 3 part 1: %d%n", part1(line));
            System.out.printf("Day 3 part 2: %d%n", part2(line));
        }
    }

    private static int part1(String line) {
        Point point = new Point();

        HashSet<Point> set = new HashSet<>();
        set.add(point);

        for (char c : line.toCharArray()) {
            switch (c) {
                case '^':
                    point.y += 1;
                    break;
                case 'v':
                    point.y -= 1;
                    break;
                case '<':
                    point.x -= 1;
                    break;
                case '>':
                    point.x += 1;
                    break;
                default:
                    System.out.println("Unrecognized character: " + c);
                    continue;
            }

            set.add(point);
        }

        return set.size();
    }

    private static int part2(String line) {
        Point santa = new Point();
        Point robot = new Point();

        HashSet<Point> set = new HashSet<>();
        set.add(new Point(santa));

        for (int i = 0; i < line.length(); i++) {
            char c = line.charAt(i);
            switch (c) {
                case '^':
                    if (i%2 == 0) { santa.y += 1; }
                    else { robot.y += 1; }
                    break;
                case 'v':
                    if (i%2 == 0) { santa.y -= 1; }
                    else { robot.y -= 1; }
                    break;
                case '<':
                    if (i%2 == 0) { santa.x += 1; }
                    else { robot.x += 1; }
                    break;
                case '>':
                    if (i%2 == 0) { santa.x -= 1; }
                    else { robot.x -= 1; }
                    break;
                default:
                    System.out.println("Unrecognized character: " + c);
                    continue;
            }

            if (i%2 == 0) { set.add(new Point(santa)); }
            else { set.add(new Point(robot)); }
        }
        
        return set.size();
    }
}
