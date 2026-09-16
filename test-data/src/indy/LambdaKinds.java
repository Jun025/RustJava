// `Lambda.java` next door is the simplest possible metafactory call site: no capture, and an
// implementation method that is `REF_invokeStatic`. That single shape leaves the rest of the
// linker unobserved — the reference kind is a five-way branch, and a capture is the difference
// between "the object holds something" and "the object holds nothing".
//
// So each line below is a different one of those. What `javac --release 21` actually emitted for
// them is recorded in `tests/test_class_format.rs`, where the test reads the reference kinds back
// out of the class file rather than trusting this comment.
//
// Compiled with: javac --release 21 -d test-data/indy test-data/src/indy/LambdaKinds.java
public class LambdaKinds {
    interface IntOp {
        int apply(int x);
    }

    interface Sink {
        void accept(int x);
    }

    interface Maker {
        Box make(int x);
    }

    interface Describer {
        String describe(Box box);
    }

    interface Getter {
        String get();
    }

    interface Named {
        String name();
    }

    interface NameOf {
        String of(Named named);
    }

    static class Box {
        final int value;

        Box(int value) {
            this.value = value;
        }

        int doubled() {
            return value * 2;
        }

        String describe() {
            return "Box:" + value;
        }
    }

    static class NamedBox implements Named {
        public String name() {
            return "named";
        }
    }

    static class Base {
        String describe() {
            return "base";
        }
    }

    static class Derived extends Base {
        String describe() {
            return "derived";
        }

        // `super::` is the only way to ask javac for REF_invokeSpecial: the reference has to name
        // the method non-virtually, which is exactly what that kind means.
        Getter superReference() {
            return super::describe;
        }
    }

    static int twice(int x) {
        return x + x;
    }

    // A `void` interface method whose implementation returns something is not a curiosity: the
    // value has to be *dropped*, and if it is not, it stays on the operand stack. Nothing in
    // ordinary bytecode ever pops it again, so the only way to see the mistake is to hand the
    // object to something that calls the method from outside the interpreter — `Thread.run()`
    // invokes `Runnable.run()V` from Rust and converts the result to `()`, which a stray value
    // cannot be.
    static int tick() {
        System.out.println("tick");
        return 7;
    }

    static int report(int x) {
        System.out.println("sink:" + x);
        return x;
    }

    public static void main(String[] args) {
        int base = 100;

        IntOp plain = x -> x + 1;                 // no capture, REF_invokeStatic
        IntOp captured = x -> x + base;           // captures `base`
        IntOp staticRef = LambdaKinds::twice;     // REF_invokeStatic, method reference
        Maker constructorRef = Box::new;          // REF_newInvokeSpecial
        Describer unbound = Box::describe;        // REF_invokeVirtual, receiver from the argument
        Sink discarding = LambdaKinds::report;    // the interface method is void, `report` is not

        NameOf named = Named::name;              // REF_invokeInterface
        Box bound = new Box(21);
        IntOp boundRef = x -> bound.doubled() + x; // captures an object

        System.out.println(plain.apply(args.length));
        System.out.println(captured.apply(1));
        System.out.println(staticRef.apply(3));
        System.out.println(constructorRef.make(7).doubled());
        System.out.println(unbound.describe(bound));
        System.out.println(boundRef.apply(0));
        System.out.println(named.of(new NamedBox()));
        System.out.println(new Derived().superReference().get());
        discarding.accept(9);

        Runnable ticking = LambdaKinds::tick;
        new Thread(ticking).run();
    }
}
