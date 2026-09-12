import java.io.File;
import java.io.FileInputStream;
import java.io.FileOutputStream;
import java.io.RandomAccessFile;

// Locks the six null-buffer guards that need a live file handle, so they cannot be
// deleted silently. The scratch file is created relative to the working directory and
// removed at the end; the last two lines assert that removal actually happened.
class NullFileIoGuards {
    public static void main(String[] args) throws Exception {
        File f = new File("null-file-io-guards.tmp");

        FileOutputStream seed = new FileOutputStream(f);
        seed.write(new byte[] {1, 2, 3, 4}, 0, 4);
        seed.close();

        System.out.println("FileInputStream.read(byte[],int,int):");
        FileInputStream in = new FileInputStream(f);
        try {
            in.read(null, 0, 0);
            System.out.println("should not reach");
        } catch (NullPointerException e) {
            System.out.println("caught NPE");
        }
        in.close();

        System.out.println("FileOutputStream.write(byte[],int,int):");
        FileOutputStream out = new FileOutputStream(f, true);
        try {
            out.write(null, 0, 0);
            System.out.println("should not reach");
        } catch (NullPointerException e) {
            System.out.println("caught NPE");
        }
        out.close();

        RandomAccessFile raf = new RandomAccessFile(f, "rw");

        System.out.println("RandomAccessFile.read(byte[]):");
        try {
            raf.read(null);
            System.out.println("should not reach");
        } catch (NullPointerException e) {
            System.out.println("caught NPE");
        }

        System.out.println("RandomAccessFile.read(byte[],int,int):");
        try {
            raf.read(null, 0, 0);
            System.out.println("should not reach");
        } catch (NullPointerException e) {
            System.out.println("caught NPE");
        }

        System.out.println("RandomAccessFile.write(byte[]):");
        try {
            raf.write(null);
            System.out.println("should not reach");
        } catch (NullPointerException e) {
            System.out.println("caught NPE");
        }

        System.out.println("RandomAccessFile.write(byte[],int,int):");
        try {
            raf.write(null, 0, 0);
            System.out.println("should not reach");
        } catch (NullPointerException e) {
            System.out.println("caught NPE");
        }

        raf.close();

        // The cleanup is part of the test: a leftover scratch file fails the run.
        System.out.println("scratch removed:");
        f.delete();
        System.out.println(f.exists() ? "STILL THERE" : "gone");
    }
}
