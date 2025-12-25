package com.example.threads;

public class CounterInterference implements ICounter {
    private int value = 0;

    @Override
    public void increment() {
        for (int n = 0; n < 100_000; n++) {
            value++;
        }
    }

    @Override
    public void decrement() {
        for (int n = 0; n < 100_000; n++) {
            value--;
        }
    }

    @Override
    public int getValue() {
        return value;
    }
}
