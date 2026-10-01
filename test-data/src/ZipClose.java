import java.io.File;
import java.util.zip.ZipFile;

// Locks ZipFile.close(): both the explicit call and the try-with-resources path, which javac
// compiles to the same invokevirtual. Reuses test-data/test.jar (a jar is a zip).
class ZipClose {
    public static void main(String[] args) throws Exception {
        ZipFile zf = new ZipFile(new File("test-data/test.jar"));
        zf.close();
        System.out.println("close ok");

        try (ZipFile twr = new ZipFile(new File("test-data/test.jar"))) {
            System.out.println("entry " + (twr.getEntry("test.txt") != null));
        }
        System.out.println("try-with-resources ok");
    }
}
