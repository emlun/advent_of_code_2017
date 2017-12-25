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

  def validContinuations(prefix: List[Part], parts: Multiset[Part]): Iterator[List[Part]] = {
    val startValue = prefix.headOption map { _.b } getOrElse 0
    val starts = parts filter { p => p.a == startValue || p.b == startValue }

    if (starts.isEmpty)
      List(prefix).toIterator
    else
      for {
        start <- starts.toIterator
        result <- validContinuations(start.align(startValue) +: prefix, parts - start)
      } yield result
  }

  def score(bridge: List[Part]): Int = bridge.map(p => p.a + p.b).sum

  def solveA(parts: List[Part]): Int =
    (validContinuations(Nil, Multiset(parts)) map score).max

  def solveB(parts: List[Part]): Int = {
    val bridge = validContinuations(Nil, Multiset(parts)) maxBy { b => (b.length, score(b)) }
    score(bridge)
  }

  println(s"A: ${solveA(parts)}")
  println(s"B: ${solveB(parts)}")
}
