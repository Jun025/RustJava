// Scala 3 shapes most likely to put CONSTANT_MethodHandle (15), MethodType (16) or Dynamic (17)
// at an `ldc` site. Scala routes lambdas, SAM conversion and structural calls through
// invokedynamic, which is where those tags appear in compiler output at all.
//
//   scalac -release 21 -d <out> Targets.scala
import java.util.function.{Function => JFunction, Supplier}
import scala.reflect.Selectable.reflectiveSelectable

trait Op:
  def apply(x: Int): Int

enum Color:
  case Red, Green, Blue

class Holder(val n: Int):
  def twice: Int = n * 2

object Targets:
  def topLevel(x: Int): Int = x + 1
  inline def inlined(x: Int): Int = x * 2
  lazy val lazyValue: String = "computed"

  type Named = { def twice: Int }

  def main(args: Array[String]): Unit =
    val h = Holder(21)
    val lambda: Int => Int = _ * 3                              // lambda
    val eta: Int => Int = topLevel                              // eta-expansion
    val bound: () => Int = () => h.twice
    val sam: Op = (x: Int) => x - 1                             // SAM conversion, Scala trait
    val jfun: JFunction[String, Integer] = (s: String) => s.length  // SAM conversion, Java interface
    val sup: Supplier[String] = () => "s"
    val c: Color = Color.Green
    val w = c match                                             // enum subject
      case Color.Red   => "r"
      case Color.Green => "g"
      case Color.Blue  => "b"
    val struct: Named = h                                       // structural type -> reflective call site
    val s = s"a=${lambda(1)} b=${eta(2)} c=${bound()} d=${sam.apply(5)} " +
      s"e=${jfun.apply("ab")} f=${sup.get()} g=$w h=${struct.twice} i=${inlined(4)} j=$lazyValue"
    println(s)
