

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

private fun dfs(connections: HashMap<String, MutableSet<String>>, visited: MutableSet<String>, current: String): Set<String> {
    connections[current]?.let { descendants ->
        for (node in descendants) {
            if (!visited.contains(node)) {
                visited.add(node)
                dfs(connections, visited, node)
            }
        }
    }
    return visited
}

fun main() {
    val connections = HashMap<String, MutableSet<String>>()
    readInput("day7.txt").lines().map { line ->
        val (head, descendants) = parse(line)
        for ((count, color) in descendants) {
            val ancestors = connections.getOrPut(color, { mutableSetOf() })
            ancestors.add(head)
        }
    }
    
    println("$connections")
    println("Result: ${dfs(connections, mutableSetOf(), "shiny gold").size}")    
}