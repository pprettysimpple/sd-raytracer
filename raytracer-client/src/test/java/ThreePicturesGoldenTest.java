import org.junit.After;
import org.junit.Before;
import org.junit.Test;
import org.raytracer.client.RaytracerClient;

import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.Objects;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertTrue;

// You need to manually run local server on port 4242
public class ThreePicturesGoldenTest {
    private RaytracerClient client;

    @Before
    public void setup() {
        this.client = new RaytracerClient("localhost", 4242);
    }

    private void assertPictures(String expectedPath, String actualPath) throws IOException {
        byte[] expected = Files.readAllBytes(Path.of(expectedPath + ".png"));
        byte[] actual = Files.readAllBytes(Path.of(actualPath + ".png"));

        assertEquals(expected.length, actual.length);

        int different = 0;
        for (int i = 0; i < expected.length; i++) {
            if (expected[i] != actual[i]) {
                different++;
            }
        }

        double diff = (double) different / expected.length * 100;
        assertTrue(diff <= 5);
    }

    @Test
    public void goldenSession() throws IOException {
        client.setResolution(1000, 500);
        client.setFov(80);
        client.setFileName("golden/actual1");
        client.render();
        assertPictures("golden/expected1", "golden/actual1");

        client.setFileName("golden/actual2");
        client.setViewDirection(5, 5, -5);
        client.render();
        assertPictures("golden/expected2", "golden/actual2");

        client.setOrigin(3, 2, 1);
        client.setFileName("golden/actual3");
        client.render();
        assertPictures("golden/expected3", "golden/actual3");
    }

    @After
    public void clean() {
        for (File file : Objects.requireNonNull(new File("golden").listFiles())) {
            if (file.getName().startsWith("actual")) {
                //noinspection ResultOfMethodCallIgnored
                file.delete();
            }
        }
    }
}
