import java.net.JarURLConnection;
import java.net.URL;
import java.net.URLConnection;
import java.net.URLStreamHandler;
import java.util.jar.JarFile;

// Locks the two null guards that sit behind `protected` entry points on abstract runtime
// classes. Neither is reachable without a subclass, so the fixture supplies the smallest
// one that compiles: implement the abstract methods, then call the protected entry point.
class NullNetGuards {
    static class Handler extends URLStreamHandler {
        protected URLConnection openConnection(URL u) {
            return null;
        }

        void callSetUrl() {
            setURL(null, "http", "host", 1, "file", "ref");
        }
    }

    static class JarConn extends JarURLConnection {
        JarConn(URL u) throws Exception {
            super(u);
        }

        public JarFile getJarFile() {
            return null;
        }

        public void connect() {
        }
    }

    public static void main(String[] args) throws Exception {
        System.out.println("URLStreamHandler.setURL(URL,...):");
        try {
            new Handler().callSetUrl();
            System.out.println("should not reach");
        } catch (NullPointerException e) {
            System.out.println("caught NPE");
        }

        System.out.println("JarURLConnection(URL):");
        try {
            new JarConn(null);
            System.out.println("should not reach");
        } catch (NullPointerException e) {
            System.out.println("caught NPE");
        }
    }
}
