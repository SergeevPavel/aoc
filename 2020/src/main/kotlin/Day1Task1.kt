
// identation in lambdas
// chaotic backend

fun main() {
    val input = object {}.javaClass.getResource("day1.txt").readText()
    val numbers = input.lines().map { number ->
        number.toInt()
    }
    for (a in numbers) {
        for (b in numbers) {
            if (a + b == 2020) {
                println("a = $a b = $b result = ${a * b}")
            }
        }
    }
}