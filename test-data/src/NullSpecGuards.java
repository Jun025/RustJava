import java.io.DataInputStream;
import java.io.File;
import java.io.FileInputStream;
import java.io.FileOutputStream;
import java.io.RandomAccessFile;
import java.util.zip.ZipEntry;
import java.util.zip.ZipFile;

class NullSpecGuards {
    public static void main(String[] args) throws Exception {
        // Sites the JDK spec requires to throw NullPointerException, measured against a
        // reference JVM. Before the guards these aborted the host process instead.
        System.out.println("DataInputStream.readUTF(DataInput):");
        try {
            DataInputStream.readUTF(null);
            System.out.println("should not reach");
        } catch (NullPointerException e) {
            System.out.println("caught NPE");
        }

        System.out.println("new FileInputStream(File):");
        try {
            new FileInputStream((File) null);
            System.out.println("should not reach");
        } catch (NullPointerException e) {
            System.out.println("caught NPE");
        } catch (Throwable t) {
            System.out.println("wrong: " + t.getClass().getName());
        }

        System.out.println("new FileOutputStream(File,boolean):");
        try {
            new FileOutputStream((File) null, true);
            System.out.println("should not reach");
        } catch (NullPointerException e) {
            System.out.println("caught NPE");
        } catch (Throwable t) {
            System.out.println("wrong: " + t.getClass().getName());
        }

        System.out.println("new RandomAccessFile(File,String):");
        try {
            new RandomAccessFile((File) null, "r");
            System.out.println("should not reach");
        } catch (NullPointerException e) {
            System.out.println("caught NPE");
        } catch (Throwable t) {
            System.out.println("wrong: " + t.getClass().getName());
        }

        System.out.println("new ZipEntry(ZipEntry):");
        try {
            new ZipEntry((ZipEntry) null);
            System.out.println("should not reach");
        } catch (NullPointerException e) {
            System.out.println("caught NPE");
        }

        System.out.println("new ZipFile(File):");
        try {
            new ZipFile((File) null);
            System.out.println("should not reach");
        } catch (NullPointerException e) {
            System.out.println("caught NPE");
        } catch (Throwable t) {
            System.out.println("wrong: " + t.getClass().getName());
        }
    }
}
