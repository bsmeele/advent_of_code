package aoc.utils;

import java.io.*;
import java.util.List;
import java.util.stream.Collectors;

public class InputLoader {
    public static String read(String path) throws IOException {
        InputStream inputStream = InputLoader.class.getClassLoader().getResourceAsStream(path);
        if (inputStream == null) {
            throw new FileNotFoundException("Resource not found: " + path);
        }
        return new String(inputStream.readAllBytes());
    }

    public static List<String> readLines(String path) throws IOException {
        InputStream inputStream = InputLoader.class.getClassLoader().getResourceAsStream(path);
        if (inputStream == null) {
            throw new FileNotFoundException("Resource not found: " + path);
        }
        try (BufferedReader reader = new BufferedReader(new InputStreamReader(inputStream))) {
            return reader.lines().collect(Collectors.toList());
        }
    }
}
