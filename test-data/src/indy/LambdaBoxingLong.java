// A boxing pair this runtime does not insert: `long` into `Object`. OpenJDK 26.0.2.1 runs this and
// prints 5000000000; here the call site is left unlinked and the class is refused.
//
// The adapter in `jvm-bytecode/src/lambda.rs` is the `int` <-> `Integer` pair and nothing wider, so
// this fixture is what notices a change that generalises it to "any primitive meets a reference".
//
// Compiled with: javac --release 21 -d test-data/indy test-data/src/indy/LambdaBoxingLong.java
public class LambdaBoxingLong {
    interface Gen {
        Object get();
    }

    static long big() {
        return 5000000000L;
    }

    public static void main(String[] args) {
        Gen gen = LambdaBoxingLong::big;
        System.out.println(gen.get());
    }
}
