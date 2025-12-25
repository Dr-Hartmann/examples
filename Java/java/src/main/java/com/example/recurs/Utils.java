package com.example.recurs;

public interface Utils {
    public static int factorial(int n) {
        return n <= 1
                ? 1
                : n * factorial(n - 1);
    }

    public static int fib(int n) {
        return n == 0 || n == 1
                ? n
                : fib(n - 1) + fib(n - 2);
    }
}
