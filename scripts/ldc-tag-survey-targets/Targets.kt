// Kotlin shapes most likely to put CONSTANT_MethodHandle (15), MethodType (16) or Dynamic (17)
// at an `ldc` site. Compiled with every flag that routes a feature through invokedynamic, because
// that is where those three tags live in compiler output at all.
//
//   kotlinc -jvm-target 21 -Xlambdas=indy -Xsam-conversions=indy \
//           -Xstring-concat=indy-with-constants -d <out> Targets.kt
import java.util.function.Function
import java.util.function.Supplier

fun interface Op {
    fun apply(x: Int): Int
}

enum class Color { RED, GREEN, BLUE }

class Holder(val n: Int) {
    fun twice(): Int = n * 2
}

fun topLevel(x: Int): Int = x + 1

val lazyValue: String by lazy { "computed" }

inline fun <reified T> typeName(): String = T::class.java.name

fun main() {
    val h = Holder(21)
    val lambda: (Int) -> Int = { it * 3 }                       // lambda
    val unbound: (Int) -> Int = ::topLevel                      // unbound callable reference
    val bound: () -> Int = h::twice                             // bound callable reference
    val sam: Op = Op { it - 1 }                                 // SAM conversion, Kotlin fun interface
    val jfun: Function<String, Int> = Function { it.length }    // SAM conversion, Java interface
    val sup: Supplier<String> = Supplier { "s" }
    val c = Color.GREEN
    val w = when (c) {                                          // enum subject
        Color.RED -> "r"
        Color.GREEN -> "g"
        Color.BLUE -> "b"
    }
    // string templates: the one construct javac routes through invokedynamic by default
    val s = "a=${lambda(1)} b=${unbound(2)} c=${bound()} d=${sam.apply(5)} " +
        "e=${jfun.apply("ab")} f=${sup.get()} g=$w h=${typeName<Holder>()} i=$lazyValue"
    println(s + Holder::class.java.simpleName)
}
