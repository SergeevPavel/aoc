
fun main() {
    val fragment = readInput("day3.txt").lines().map { line ->
        line.toCharArray().toList()
    }.toList()
    val height = fragment.size
    val width = fragment[0].size
    
    var x = 0
    var y = 0
    val stepX = 3
    val stepY = 1
    var trees = 0
    while (y < height) {
        if (fragment[y][x] == '#') {
            trees += 1
        }
        x += stepX
        x = x.rem(width)
        y += stepY
    }
    println("Tries: $trees")
}