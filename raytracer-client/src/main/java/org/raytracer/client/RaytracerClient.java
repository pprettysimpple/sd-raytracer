package org.raytracer.client;

import com.google.protobuf.ByteString;
import io.grpc.ManagedChannel;
import io.grpc.ManagedChannelBuilder;
import org.raytracer.proto.*;

import javax.imageio.ImageIO;
import java.awt.*;
import java.awt.image.BufferedImage;
import java.io.File;
import java.io.IOException;
import java.util.Iterator;

public class RaytracerClient {
    private final RendererGrpc.RendererBlockingStub client;
    private final Resolution resolution = new Resolution(1000, 500);
    private final Vec3 origin = new Vec3(0, 0, 0);
    private final Vec3 viewDirection = new Vec3(0, 0, -1);
    private double fov = 90;
    private String fileName = "picture";

    public RaytracerClient(String host, int port) {
        ManagedChannel channel = ManagedChannelBuilder.forAddress(host, port).usePlaintext().build();
        this.client = RendererGrpc.newBlockingStub(channel);
    }

    public void setFov(double fov) {
        this.fov = fov;
    }

    public void setResolution(int width, int height) {
        this.resolution.setWidth(width);
        this.resolution.setHeight(height);
    }

    public void setOrigin(double x, double y, double z) {
        this.origin.setX(x);
        this.origin.setY(y);
        this.origin.setZ(z);
    }

    public void setViewDirection(double x, double y, double z) {
        this.viewDirection.setX(x);
        this.viewDirection.setY(y);
        this.viewDirection.setZ(z);
    }

    public void setFileName(String fileName) {
        this.fileName = fileName;
    }

    public void render() throws IOException {
        Fov fov = Fov.newBuilder().setFov(this.fov).build();
        org.raytracer.proto.Resolution resolution =
                org.raytracer.proto.Resolution
                        .newBuilder()
                        .setWidth(this.resolution.getWidth())
                        .setHeight(this.resolution.getHeight())
                        .build();

        org.raytracer.proto.Vec3 originVec =
                org.raytracer.proto.Vec3
                        .newBuilder()
                        .setX(origin.getX())
                        .setY(origin.getY())
                        .setZ(origin.getZ())
                        .build();

        Origin origin = Origin.newBuilder().setOrigin(originVec).build();

        org.raytracer.proto.Vec3 viewDirVec =
                org.raytracer.proto.Vec3
                        .newBuilder()
                        .setX(viewDirection.getX())
                        .setY(viewDirection.getY())
                        .setZ(viewDirection.getZ())
                        .build();

        ViewDirection viewDirection = ViewDirection.newBuilder().setDirection(viewDirVec).build();

        RenderRequest request =
                RenderRequest
                        .newBuilder()
                        .addOperations(Operation.newBuilder().setSetFov(fov))
                        .addOperations(Operation.newBuilder().setSetResolution(resolution))
                        .addOperations(Operation.newBuilder().setSetOrigin(origin))
                        .addOperations(Operation.newBuilder().setSetViewDirection(viewDirection))
                        .build();

        Iterator<RenderResponse> responseIterator = client.render(request);

        ByteString result = ByteString.EMPTY;
        while (responseIterator.hasNext()) {
            RenderResponse part = responseIterator.next();
            result = result.concat(part.getPictureData());
        }

        BufferedImage bufferedImage = new BufferedImage(this.resolution.getWidth(), this.resolution.getHeight(), BufferedImage.TYPE_3BYTE_BGR);
        Graphics graphics = bufferedImage.getGraphics();
        byte[] bytes = result.toByteArray();
        int bytesPos = 0;
        for (int i = 0; i < this.resolution.getHeight(); i++) {
            for (int j = 0; j < this.resolution.getWidth(); j++) {
                int red = bytes[bytesPos] & 0xff;
                int green = bytes[bytesPos + 1] & 0xff;
                int blue = bytes[bytesPos + 2] & 0xff;
                bytesPos += 3;

                Color color = new Color(red, green, blue);
                graphics.setColor(color);
                graphics.drawLine(j, i, j, i);
            }
        }

        ImageIO.write(bufferedImage, "png", new File(fileName + ".png"));
    }
}
