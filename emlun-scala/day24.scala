object Day24 extends App {

  object Multiset {
    def apply[A](items: Iterable[A]): Multiset[A] = new Multiset(items)
  }
  case class Multiset[A](items: Map[A, Int]) extends Iterable[A] {
    def this(items: Iterable[A]) =
      this(items.foldLeft(Map.empty[A, Int]) { (result, item) =>
        result.updated(item, (result get item getOrElse 0) + 1)
      })

    def -(item: A): Multiset[A] = {
      val count: Int = items get item getOrElse 0
      Multiset(
        if (count > 1)
          items.updated(item, count - 1)
        else
          items - item
      )
    }

    override def filter(pred: A => Boolean): Multiset[A] =
      Multiset(items filter { case (item, _) => pred(item) })

    override def isEmpty: Boolean = items.isEmpty

    override def iterator: Iterator[A] =
      items.toIterator flatMap { case (item, count) => Iterator.fill(count)(item) }
  }

  val parts: List[Part] = (for {
    line <- io.Source.stdin.getLines()
    Array(a, b) = line.trim.split("/") map { _.toInt }
  } yield Part(a, b)).toList

  case class Part(a: Int, b: Int) {
    def align(prevEnd: Int): Part =
      if (a == prevEnd) this
      else if (b == prevEnd) Part(b, a)
      else ???
  }

  def maxValues(priority: ((Int, Int)) => (Int, Int))(strength: Int, length: Int, last: Int, parts: Multiset[Part]): (Int, Int) = {
    val nexts = parts filter { p => p.a == last || p.b == last }

    if (nexts.isEmpty)
      (strength, length)
    else
      nexts
        .map { next =>
          maxValues(priority)(strength + next.a + next.b, length + 1, next.align(last).b, parts - next)
        }
        .maxBy(priority)
  }

  def solveA(parts: List[Part]): Int = maxValues(a => a)(0, 0, 0, Multiset(parts))._1
  def solveB(parts: List[Part]): Int = maxValues({ case (w, l) => (l, w) })(0, 0, 0, Multiset(parts))._1

  println(s"A: ${solveA(parts)}")
  println(s"B: ${solveB(parts)}")
}
