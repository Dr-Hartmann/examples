package com.example;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import com.example.threads.CounterInterference;
import com.example.threads.CounterSyncInstructions;
import com.example.threads.CounterSyncMethods;
import com.example.threads.ICounter;
import com.example.recurs.Utils;;

public class Main {
    private static Logger logger = LoggerFactory.getLogger(Main.class);

    public static void main(String[] args) {
        logger.info("Hello world!");

        ICounter.test(new CounterInterference());
        ICounter.test(new CounterSyncInstructions());
        ICounter.test(new CounterSyncMethods());

        logger.info("{}", Utils.factorial(5));
        logger.info("{}", Utils.fib(10));
    }
}