package com.example.threads;

import java.util.Arrays;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public interface ICounter {
    Logger logger = LoggerFactory.getLogger(ICounter.class);

    void increment();

    void decrement();

    int getValue();

    static void test(ICounter counter) {
        try {
            var threadIncrement = new Thread(counter::increment);
            var threadDecrement = new Thread(counter::decrement);
            threadIncrement.start();
            threadDecrement.start();
            threadIncrement.join();
            threadDecrement.join();
            logger.info("'{}': {}", counter.getClass().getName(), counter.getValue());
        } catch (InterruptedException e) {
            logger.error(Arrays.toString(e.getStackTrace()));
            Thread.currentThread().interrupt();
        }
    }
}
