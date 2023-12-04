

fun main() {
    val numbers = readInput("day15.txt").split(",").map { it.toInt() }
    val lastIndexes = numbers.dropLast(1).mapIndexed { index, number ->
        number to (index + 1)
    }.toMap().toMutableMap()
    var lastNumber = numbers.last()
    for (i in (numbers.size + 1)..30000000) {
//        println("i: $i prevLastNumber: $lastNumber indexes: $lastIndexes")
        val lastIndex = lastIndexes[lastNumber]
        lastIndexes[lastNumber] = i - 1
        lastNumber = if (lastIndex != null) {
            i - lastIndex - 1
        } else {
            0
        }
    }
    println(lastNumber)
}