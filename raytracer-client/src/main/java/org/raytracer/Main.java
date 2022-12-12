package org.raytracer;

import org.raytracer.repl.REPL;

public class Main {
    public static void main(String[] args) {
        new REPL("localhost", 4242).run();
    }
}