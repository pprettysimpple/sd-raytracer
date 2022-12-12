package org.raytracer.repl;

import io.grpc.StatusRuntimeException;
import org.raytracer.client.RaytracerClient;
import org.raytracer.repl.exception.BadArgumentsException;
import org.raytracer.repl.exception.BadCommandException;

import java.io.IOException;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.Scanner;

public class REPL {
    private final Scanner scanner;
    private final RaytracerClient client;

    public REPL(String host, int port) {
        this.client = new RaytracerClient(host, port);
        this.scanner = new Scanner(System.in);
    }

    public void run() {
        while (true) {
            try {
                System.out.print("raytracer-client> ");
                String[] input = scanner.nextLine().trim().split(" ");
                Command command;
                if (input.length == 0 || (command = fromString(input[0])) == null) {
                    throw new BadCommandException(input.length == 0 ? "" : input[0]);
                }

                List<String> args = new ArrayList<>(Arrays.asList(input).subList(1, input.length));

                if (command == Command.EXIT) {
                    System.out.println("Good bye!");
                    break;
                }

                switch (command) {
                    case SET_FOV -> {
                        if (args.size() != 1) {
                            throw new BadArgumentsException(args);
                        }

                        try {
                            client.setFov(Double.parseDouble(args.get(0)));
                        } catch (NumberFormatException e) {
                            throw new BadArgumentsException(args);
                        }
                    }

                    case SET_RESOLUTION -> {
                        if (args.size() != 2) {
                            throw new BadArgumentsException(args);
                        }

                        try {
                            int width = Integer.parseInt(args.get(0));
                            int height = Integer.parseInt(args.get(1));

                            client.setResolution(width, height);
                        } catch (NumberFormatException e) {
                            throw new BadArgumentsException(args);
                        }
                    }
                    case SET_ORIGIN -> {
                        if (args.size() != 3) {
                            throw new BadArgumentsException(args);
                        }

                        try {
                            double x = Double.parseDouble(args.get(0));
                            double y = Double.parseDouble(args.get(1));
                            double z = Double.parseDouble(args.get(2));

                            client.setOrigin(x, y, z);
                        } catch (NumberFormatException e) {
                            throw new BadArgumentsException(args);
                        }
                    }
                    case SET_VIEW_DIRECTION -> {
                        if (args.size() != 3) {
                            throw new BadArgumentsException(args);
                        }

                        try {
                            double x = Double.parseDouble(args.get(0));
                            double y = Double.parseDouble(args.get(1));
                            double z = Double.parseDouble(args.get(2));

                            client.setViewDirection(x, y, z);
                        } catch (NumberFormatException e) {
                            throw new BadArgumentsException(args);
                        }
                    }
                    case RENDER -> {
                        client.render();
                        System.out.println("Done!");
                    }
                    case SET_FILEPATH -> {
                        if (args.size() != 1) {
                            throw new BadArgumentsException(args);
                        }

                        client.setFileName(args.get(0));
                    }
                }
            } catch (BadCommandException e) {
                System.out.println("Unexpected command: " + e.getCommand());
            } catch (BadArgumentsException e) {
                System.out.println("Bad arguments: " + e.getArgs().toString());
            } catch (StatusRuntimeException e) {
                System.out.println(e.getMessage());
                break;
            } catch (IOException e) {
                System.out.println("Can't write image: " + e.getMessage());
            }
        }
    }

    private Command fromString(String str) {
        return switch (str) {
            case "set_fov" -> Command.SET_FOV;
            case "set_resolution" -> Command.SET_RESOLUTION;
            case "set_origin" -> Command.SET_ORIGIN;
            case "set_view_direction" -> Command.SET_VIEW_DIRECTION;
            case "render" -> Command.RENDER;
            case "exit" -> Command.EXIT;
            case "set_filepath" -> Command.SET_FILEPATH;
            default -> null;
        };
    }

    private enum Command {
        SET_FOV, SET_RESOLUTION, SET_ORIGIN, SET_VIEW_DIRECTION, RENDER, EXIT, SET_FILEPATH,
    }
}
