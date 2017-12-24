object Day24 extends App {

  val input = (for {
    line <- io.Source.stdin.getLines()
    Array(a, b) = line.trim.split("/") map { _.toInt }
  } yield Part(a, b)).toList

  case class Part(a: Int, b: Int) {
    def align(prevEnd: Int): Part =
      if (a == prevEnd) this
      else if (b == prevEnd) Part(b, a)
      else ???
  }

  def validContinuations(prefix: List[Part], parts: Set[Part]): Iterator[List[Part]] = {
    val startValue = (prefix.headOption map { _.b } getOrElse 0)
    val (starts, rests) = parts partition { p => p.a == startValue || p.b == startValue }

    if (starts.isEmpty) {
      List(prefix).toIterator
    } else {

      for {
        start <- starts.toIterator
        result <- validContinuations(start.align(startValue) +: prefix, parts - start)
      } yield result
    }
  }

  def score(bridge: List[Part]): Int = bridge.map(p => p.a + p.b).sum

  def solveA(parts: List[Part]) = {
    if (parts.toSet.size != parts.size) {
      println("WARNIG: DUPLICATE PARTS!")
    }

    score(validContinuations(Nil, parts.toSet) maxBy score)
  }

  def solveB(parts: List[Part]) = {
    if (parts.toSet.size != parts.size) {
      println("WARNIG: DUPLICATE PARTS!")
    }

    val bridge = validContinuations(Nil, parts.toSet) reduce { (a, b) =>
      if (a.length > b.length)
        a
      else if (b.length > a.length)
        b
      else if (score(a) > score(b))
        a
      else
        b
    }

    score(bridge)
  }

  println(s"A: ${solveA(input)}")
  println(s"B: ${solveB(input)}")
}