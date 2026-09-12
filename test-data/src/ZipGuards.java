import java.io.File;
import java.util.zip.ZipFile;

// Locks the ZipFile.getInputStream(ZipEntry) null guard. A live ZipFile instance is the
// only hard part, so this reuses the archive that already ships as a fixture — a jar is a
// zip. Nothing here depends on the archive's contents, only that it opens.
class ZipGuards {
    public static void main(String[] args) throws Exception {
        ZipFile zf = new ZipFile(new File("test-data/test.jar"));

        System.out.println("ZipFile.getInputStream(ZipEntry):");
        try {
            zf.getInputStream(null);
            System.out.println("should not reach");
        } catch (NullPointerException e) {
            System.out.println("caught NPE");
        }
        // ZipFile.close() is not registered in this runtime, so there is nothing to release.
    }
}
