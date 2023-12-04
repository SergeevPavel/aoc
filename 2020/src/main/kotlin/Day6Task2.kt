

fun main() {
    val result = readInput("day6.txt").split("\n\n").map { group ->
        val common = group.lines().map { ans ->
            ans.toSet()
        }.reduce { s1, s2 -> s1.intersect(s2) }
        println("Group:\n$group")
        println("Common: $common")
        common.size
    }.sum()
    println("Result: $result")
}