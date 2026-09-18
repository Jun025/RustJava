// The boundary: a call site this runtime refuses to link, and why that is a decision rather than
// an oversight.
//
// `LambdaMetafactory` is allowed to insert adapters. Here the interface method returns `Object`
// and the implementation returns `int`, so the real factory boxes on the way out — measured:
// OpenJDK 26.0.1 runs this and prints 3. This runtime has no boxing to insert at that point, so
// it does not link the call site at all and the class is refused as an unsupported feature.
//
// A refusal is worse than an adapter and much better than a wrong answer, and this fixture is what
// keeps that choice honest: delete the signature check in `jvm-bytecode/src/lambda.rs` and this
// class stops being refused.
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
