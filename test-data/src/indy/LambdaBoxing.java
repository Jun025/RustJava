// The first adapter this runtime inserts, in its simplest place. The interface method returns
// `Object` and the implementation returns `int`, so `LambdaMetafactory` boxes on the way out —
// measured: OpenJDK 26.0.1 runs this and prints 3.
//
// This fixture used to be the boundary: the call site was refused because nothing here could box.
// `jvm-bytecode/src/lambda.rs` now inserts `Integer.valueOf` for exactly this pair, and the pairs
// that are still refused have their own fixtures (LambdaBoxingLong, LambdaUnboxingShort).
//
// Compiled with: javac --release 21 -d test-data/indy test-data/src/indy/LambdaBoxing.java
public class LambdaBoxing {
    interface Gen {
        Object get();
    }

    static int size() {
        return 3;
    }

    public static void main(String[] args) {
        Gen gen = LambdaBoxing::size;
        System.out.println(gen.get());
    }
}
