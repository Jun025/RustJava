// The one reference kind a modern javac will not give us: REF_invokeSpecial.
//
// A lambda whose body reads an instance field compiles to a private *instance* method, and before
// nestmates (JEP 181, Java 11) the only handle that could name a private method was
// REF_invokeSpecial. Java 11 onwards compiles the same source to REF_invokeVirtual on a synthetic
// method instead — measured on this file: `--release 8` emits kind 7, `--release 21` emits kind 5.
//
// So this fixture is pinned to 8 deliberately, and it is not a curiosity: class files that old are
// what this runtime exists to run. Without it the REF_invokeSpecial branch of the linker is code
// no test can reach.
//
// Compiled with: javac --release 8 -d test-data/indy test-data/src/indy/LambdaCapturingThis.java
public class LambdaCapturingThis {
    interface Op {
        int apply(int x);
    }

    private final int base;

    LambdaCapturingThis(int base) {
        this.base = base;
    }

    Op adder() {
        return x -> x + base;
    }

    public static void main(String[] args) {
        System.out.println(new LambdaCapturingThis(40).adder().apply(2));
    }
}
