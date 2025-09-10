package aoc.y2015;

import aoc.utils.InputLoader;

import java.util.List;
import java.util.ArrayList;
import java.io.*;

public class Day2 {
    public static void run(boolean test) {
        List<String> lines = new ArrayList<>();
        
        try {
            if (test) {
                lines = InputLoader.readLines("y2015/day2/test.txt");
            } else {
                lines = InputLoader.readLines("y2015/day2/input.txt");
            }
        } catch (IOException e) {
            System.err.println("Failed to load input: " + e.getMessage());
            return;
        }

        System.out.printf("Day 2 part 1: %d%n", part1(lines));
        System.out.printf("Day 2 part 2: %d%n", part2(lines));
    }

    private static int part1(List<String> lines) {
        int total = 0;
        
        for (String line : lines) {
            String[] parts = line.split("x");
            if (parts.length != 3) {
                System.out.println("Unrecognized input: " + line);
                continue;
            }

            int l = Integer.parseInt(parts[0]);
            int w = Integer.parseInt(parts[1]);
            int h = Integer.parseInt(parts[2]);

            int lw = l*w;
            int wh = w*h;
            int lh = l*h;

            total += 2*lw + 2*wh + 2*lh + Math.min(Math.min(lw, wh), lh);
        }

        return total;
    }

    private static int part2(List<String> lines) {
        int total = 0;
        
        for (String line : lines) {
            String[] parts = line.split("x");
            if (parts.length != 3) {
                System.out.println("Unrecognized input: " + line);
                continue;
            }

            int l = Integer.parseInt(parts[0]);
            int w = Integer.parseInt(parts[1]);
            int h = Integer.parseInt(parts[2]);

            int lw = l+w;
            int wh = w+h;
            int lh = l+h;

            total += l*w*h + 2*Math.min(Math.min(lw, wh), lh);
        }

        return total;
    }
}
