

private fun parse(line: String): Pair<String, List<Pair<Long, String>>> {
    val headAndContent = line.split(" bags contain ")
    val head = headAndContent[0]
    val content = headAndContent[1]
    if (content == "no other bags.") {
        return Pair(head, emptyList())
    }
    val descendants = content.split(", ").map { descendant ->
        val parts = descendant.split(" ")
        val count = parts[0].toLong()
        val color = parts[1] + " " + parts[2]
        Pair(count, color)
    }.toList()
    return Pair(head, descendants)
}

private fun dfs(connections: Map<String, List<Pair<Long, String>>>, current: String): Long {
    connections[current]?.let { descendants ->
        return descendants.map { (count, color) ->
            count * dfs(connections, color)
        }.sum() + 1
    } ?: return 1
}

fun main() {
    val connections = readInput("day7.txt").lines().map { line ->
        parse(line)
    }.toMap()
    
    println("$connections")
    println("Result: ${dfs(connections, "shiny gold")}")
}