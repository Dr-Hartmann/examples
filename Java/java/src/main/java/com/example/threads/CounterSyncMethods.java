package com.example.threads;

public class CounterSyncMethods implements ICounter {
    private int value = 0;

    @Override
    public synchronized void increment() {
        for (int n = 0; n < 100_000; n++) {
            ++value;
        }
    }

    @Override
    public synchronized void decrement() {
        for (int n = 0; n < 100_000; n++) {
            --value;
        }
    }

    @Override
    public int getValue() {
        return value;
    }
}
