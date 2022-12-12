package org.raytracer.repl.exception;

public class BadCommandException extends Exception {
    private final String command;

    public BadCommandException(String command) {
        super();
        this.command = command;
    }

    public String getCommand() {
        return command;
    }
}
