// javac 9+ lowers `+` on strings to an invokedynamic call site bound to
// java.lang.invoke.StringConcatFactory#makeConcatWithConstants, so this ordinary
// one-liner is enough to put CONSTANT_InvokeDynamic (tag 18), CONSTANT_MethodHandle
// (tag 15) and CONSTANT_MethodType (tag 16) into the constant pool.
//
// Compiled with: javac --release 21 -d test-data/indy test-data/src/indy/StringConcat.java
public class StringConcat {
    public static void main(String[] args) {
        System.out.println("a" + args.length);
    }
}
