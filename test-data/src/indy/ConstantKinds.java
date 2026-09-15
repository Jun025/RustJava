// One class carrying all four of the constant pool tags this parser learned in
// rustjava-cp-tags-15-18-parse-and-honest-diagnosis, emitted by javac rather than assembled by
// hand. The unit tests in classfile/src/constant_pool.rs fix each tag's operand width in
// isolation; this fixture is the same tags read in sequence, where a slot-accounting mistake
// shifts every later entry — a failure shape those unit tests cannot see.
//
// What produces what:
//   - `qualified()`  a switch whose case labels are qualified enum constants on a non-enum
//                    selector (JEP 441). javac describes each constant as a java.lang.Enum$EnumDesc
//                    through ConstantBootstraps.invoke, which is CONSTANT_Dynamic (tag 17).
//                    Measured rarity: of the 27,902 classes in OpenJDK 26's own jmods, exactly
//                    ONE carries tag 17 (jdk/jpackage/internal/PackageBuilder), and a plain enum
//                    switch or a sealed-interface pattern switch emits none.
//   - `lambda()`     LambdaMetafactory.metafactory, whose bootstrap arguments are
//                    CONSTANT_MethodType (16) and CONSTANT_MethodHandle (15).
//   - both           CONSTANT_InvokeDynamic (18).
//
// Compiled with: javac --release 21 -d test-data/indy test-data/src/indy/ConstantKinds.java
public class ConstantKinds {
    enum Suit { HEARTS, SPADES }

    interface Op {
        int apply(int x);
    }

    static String qualified(Object o) {
        return switch (o) {
            case Suit.HEARTS -> "h";
            case Suit.SPADES -> "s";
            default -> "?";
        };
    }

    static int lambda(int x) {
        Op op = v -> v + 1;
        return op.apply(x);
    }

    public static void main(String[] args) {
        System.out.println(qualified(Suit.HEARTS) + lambda(args.length));
    }
}
