// Unboxing followed by widening, which this runtime does not insert. The interface method takes an
// erased `Object`; the instantiated type says it is a `Short`, and the implementation takes an
// `int`. OpenJDK 26.0.2.1 runs this and prints 42; here the class is refused.
//
// This is the case that makes the instantiated type matter. Reading only the erased `Object`, an
// `int` <-> `Integer` adapter would unbox this argument as an `Integer` and throw
// ClassCastException where the real factory answers 42 — a refusal turned into a wrong answer.
//
// Compiled with: javac --release 21 -d test-data/indy test-data/src/indy/LambdaUnboxingShort.java
public class LambdaUnboxingShort {
    interface Fn<T, R> {
        R apply(T t);
    }

    static int inc(int x) {
        return x + 1;
    }

    public static void main(String[] args) {
        Fn<Short, Integer> f = LambdaUnboxingShort::inc;
        System.out.println(f.apply((short) 41));
    }
}
