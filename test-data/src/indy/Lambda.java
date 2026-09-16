// javac lowers a lambda to an invokedynamic bound to
// java.lang.invoke.LambdaMetafactory#metafactory, whose three static bootstrap arguments are
// CONSTANT_MethodType (16), CONSTANT_MethodHandle (15) and CONSTANT_MethodType again. That is
// the shape StringConcat.java does not have: its single argument is an ordinary String.
//
// Compiled with: javac --release 21 -d test-data/indy test-data/src/indy/Lambda.java
public class Lambda {
    interface Op {
        int apply(int x);
    }

    public static void main(String[] args) {
        Op op = x -> x + 1;
        System.out.println(op.apply(args.length));
    }
}
