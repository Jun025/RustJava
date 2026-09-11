import java.io.BufferedOutputStream;
import java.io.ByteArrayInputStream;
import java.io.ByteArrayOutputStream;
import java.io.DataInputStream;
import java.io.DataOutputStream;
import java.io.InputStreamReader;
import java.io.OutputStreamWriter;
import java.io.Reader;
import java.io.Writer;

class NullBufferGuards {
    // Reader/Writer only expose the array overloads through their own base implementation,
    // so a subclass that overrides just the abstract (char[],int,int) form reaches them.
    // Nested on purpose: the fixture harness skips class files whose name contains '$'.
    static class BareReader extends Reader {
        public int read(char[] cbuf, int off, int len) {
            return -1;
        }

        public void close() {
        }
    }

    static class BareWriter extends Writer {
        public void write(char[] cbuf, int off, int len) {
        }

        public void flush() {
        }

        public void close() {
        }
    }

    public static void main(String[] args) throws Exception {
        // Each case passes a null buffer to a java.io data-transfer method. The JDK spec
        // mandates NullPointerException; the runtime used to dereference and abort the host.
        ByteArrayOutputStream sink = new ByteArrayOutputStream();
        ByteArrayInputStream source = new ByteArrayInputStream(new byte[4]);

        System.out.println("BufferedOutputStream.write(byte[],int,int):");
        try {
            new BufferedOutputStream(sink).write(null, 0, 0);
            System.out.println("should not reach");
        } catch (NullPointerException e) {
            System.out.println("caught NPE");
        }

        System.out.println("DataInputStream.readFully(byte[]):");
        try {
            new DataInputStream(source).readFully(null);
            System.out.println("should not reach");
        } catch (NullPointerException e) {
            System.out.println("caught NPE");
        }

        System.out.println("DataOutputStream.writeBytes(String):");
        try {
            new DataOutputStream(sink).writeBytes(null);
            System.out.println("should not reach");
        } catch (NullPointerException e) {
            System.out.println("caught NPE");
        }

        System.out.println("DataOutputStream.writeChars(String):");
        try {
            new DataOutputStream(sink).writeChars(null);
            System.out.println("should not reach");
        } catch (NullPointerException e) {
            System.out.println("caught NPE");
        }

        System.out.println("DataOutputStream.writeUTF(String):");
        try {
            new DataOutputStream(sink).writeUTF(null);
            System.out.println("should not reach");
        } catch (NullPointerException e) {
            System.out.println("caught NPE");
        }

        System.out.println("InputStreamReader.read(char[]):");
        try {
            new InputStreamReader(source).read((char[]) null);
            System.out.println("should not reach");
        } catch (NullPointerException e) {
            System.out.println("caught NPE");
        }

        System.out.println("OutputStreamWriter.write(char[]):");
        try {
            new OutputStreamWriter(sink).write((char[]) null);
            System.out.println("should not reach");
        } catch (NullPointerException e) {
            System.out.println("caught NPE");
        }

        System.out.println("Writer.write(String):");
        try {
            new OutputStreamWriter(sink).write((String) null);
            System.out.println("should not reach");
        } catch (NullPointerException e) {
            System.out.println("caught NPE");
        }

        System.out.println("Writer.write(String,int,int):");
        try {
            new OutputStreamWriter(sink).write((String) null, 0, 0);
            System.out.println("should not reach");
        } catch (NullPointerException e) {
            System.out.println("caught NPE");
        }

        System.out.println("Reader.read(char[]):");
        try {
            new BareReader().read((char[]) null);
            System.out.println("should not reach");
        } catch (NullPointerException e) {
            System.out.println("caught NPE");
        }

        System.out.println("Writer.write(char[]):");
        try {
            new BareWriter().write((char[]) null);
            System.out.println("should not reach");
        } catch (NullPointerException e) {
            System.out.println("caught NPE");
        }

        System.out.println("String.getChars(int,int,char[],int):");
        try {
            "ab".getChars(0, 2, null, 0);
            System.out.println("should not reach");
        } catch (NullPointerException e) {
            System.out.println("caught NPE");
        }

        // JDK getChars checks the range before it touches dst, so a bad range wins over a
        // null dst. Locks the guard's position, not just its presence.
        System.out.println("String.getChars bad range beats null dst:");
        try {
            "ab".getChars(0, 99, null, 0);
            System.out.println("should not reach");
        } catch (StringIndexOutOfBoundsException e) {
            System.out.println("caught SIOOBE");
        } catch (NullPointerException e) {
            System.out.println("caught NPE");
        }
    }
}
