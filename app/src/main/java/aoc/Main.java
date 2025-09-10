package aoc;
import aoc.y2015.Y2015;

public class Main {
    public static void main(String[] args) {
        int year = 2015;
        int day = 1;
        boolean test = false;

        System.out.println(args);

        for (int i = 0; i < args.length; i++) {
            switch (args[i]) {
                case "-y":
                    if (i + 1 < args.length) year = Integer.parseInt(args[++i]);
                    break;
                case "-d":
                    if (i + 1 < args.length) day = Integer.parseInt(args[++i]);
                    break;
                case "-t":
                    if (i + 1 < args.length) test = Boolean.parseBoolean(args[++i]);
                    break;
            }
        }

        System.out.printf("Running Aoc %d day %d | Test mode: %b%n", year, day, test);

        switch (year) {
            case 2015:
                Y2015.run(day, test);
                break;
            default:
                System.out.printf("Unspported year: %d%n", year);
        }
    }
}
