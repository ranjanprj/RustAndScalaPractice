

object Euler1 {
    def euler1_sum(): Int = {
    var a = 0;
    var s = 0;
    for(a <- 0 to 1000){
        if( a % 3 == 0 || a % 5 == 0){
            s += a;
        }
    }
    return s;
}
   def main(args: Array[String]) {
      println(euler1_sum)
   }
}

