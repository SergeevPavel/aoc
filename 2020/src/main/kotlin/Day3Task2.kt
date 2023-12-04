
fun countTrees(fragment: List<List<Char>>, stepX: Int, stepY: Int): Int {
    val height = fragment.size
    val width = fragment[0].size
    var x = 0
    var y = 0
    var trees = 0
    while (y < height) {
        if (fragment[y][x] == '#') {
            trees += 1
        }
        x += stepX
        x = x.rem(width)
        y += stepY
    }
    return trees
}

fun main() {
    val fragment = readInput("day3.txt").lines().map { line ->
        line.toCharArray().toList()
    }.toList()
    val slopes = listOf(Pair(1, 1), Pair(3, 1), Pair(5, 1), Pair(7, 1), Pair(1, 2))
    var result = 1L
    for ((stepX, stepY) in slopes) {
        val treesCount = countTrees(fragment, stepX, stepY)
        println("stepX: $stepX stepY: $stepY triesCount: $treesCount result: $result")
        result *= treesCount
    }
    println("Result: $result")
}