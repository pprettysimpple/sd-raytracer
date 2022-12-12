package org.raytracer.repl.exception;

import java.util.List;

public class BadArgumentsException extends Exception {
    private final List<String> args;

    public BadArgumentsException(List<String> args) {
        super();
        this.args = args;
    }

    public List<String> getArgs() {
        return args;
    }
}
