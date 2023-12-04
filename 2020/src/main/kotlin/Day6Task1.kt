

fun main() {
    val result = readInput("day6.txt").split("\n\n").map { group ->
        group.lines().map { ans ->
            ans.toSet()
        }.reduce { s1, s2 -> s1.union(s2) }.size
    }.sum()
    println("Result: $result")
}