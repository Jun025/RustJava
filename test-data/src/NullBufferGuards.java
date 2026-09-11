import java.io.BufferedOutputStream;
import java.io.ByteArrayInputStream;
import java.io.ByteArrayOutputStream;
import java.io.DataInputStream;
import java.io.DataOutputStream;
import java.io.InputStreamReader;
import java.io.OutputStreamWriter;

class NullBufferGuards {
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

        System.out.println("String.getChars(int,int,char[],int):");
        try {
            "ab".getChars(0, 2, null, 0);
            System.out.println("should not reach");
        } catch (NullPointerException e) {
            System.out.println("caught NPE");
        }
    }
}
