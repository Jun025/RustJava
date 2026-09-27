// Every place the int <-> Integer adapter goes, and the exceptions it throws. The expected output
// is OpenJDK 26.0.2.1's, taken by running this fixture there:
//   4 / 42 / 7 / obj:5 / integer:6 / npe in / cce / npe out
//
// Each line is one position: boxing on the way out through a generic interface, unboxing on the
// way in and boxing on the way out, unboxing on the way out, boxing on the way in to `Object` and
// to `Integer`. The last three are what the real adapter throws: `null` unboxed on the way in,
// something that is not an `Integer` passed through a raw type, and `null` unboxed on the way out.
//
// Compiled with: javac --release 21 -d test-data/indy test-data/src/indy/LambdaBoxingKinds.java
public class LambdaBoxingKinds {
    interface Src<T> {
        T get();
    }

    interface Fn<T, R> {
        R apply(T t);
    }

    interface IntGen {
        int get();
    }

    interface IntSink {
        void accept(int x);
    }

    static int size() {
        return 3;
    }

    static int inc(int x) {
        return x + 1;
    }

    static Integer boxed() {
        return 7;
    }

    static Integer none() {
        return null;
    }

    static void show(Object o) {
        System.out.println("obj:" + o);
    }

    static void showInteger(Integer o) {
        System.out.println("integer:" + o);
    }

    @SuppressWarnings({"rawtypes", "unchecked"})
    public static void main(String[] args) {
        Src<Integer> s = LambdaBoxingKinds::size;
        System.out.println(s.get() + 1);

        Fn<Integer, Integer> f = LambdaBoxingKinds::inc;
        System.out.println(f.apply(41));

        IntGen g = LambdaBoxingKinds::boxed;
        System.out.println(g.get());

        IntSink k = LambdaBoxingKinds::show;
        k.accept(5);

        IntSink ki = LambdaBoxingKinds::showInteger;
        ki.accept(6);

        try {
            f.apply(null);
        } catch (NullPointerException e) {
            System.out.println("npe in");
        }

        Fn raw = f;
        try {
            raw.apply("x");
        } catch (ClassCastException e) {
            System.out.println("cce");
        }

        IntGen n = LambdaBoxingKinds::none;
        try {
            n.get();
        } catch (NullPointerException e) {
            System.out.println("npe out");
        }
    }
}
