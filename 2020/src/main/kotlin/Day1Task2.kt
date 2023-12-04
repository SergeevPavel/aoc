

fun main() {
    val input = object {}.javaClass.getResource("day1.txt").readText()
    val numbers = input.lines().map { number ->
        number.toInt()
    }
    for (a in numbers) {
        for (b in numbers) {
            for (c in numbers) {
                if (a + b + c == 2020) {
                    println("a = $a b = $b c = $c result = ${a * b * c}")
                }
            }
        }
    }
}