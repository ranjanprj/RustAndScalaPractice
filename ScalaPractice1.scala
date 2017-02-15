/*
    Scala program which takes two input and sums them.
*/

object SumInputs{
    def main(args : Array[String]) {
        println(io.Source.stdin.getLines().take(2).map(_.toInt).sum)
    }
}
