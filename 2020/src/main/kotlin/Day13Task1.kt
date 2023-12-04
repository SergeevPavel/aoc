package day13.task1

import readInput

fun parse(input: String): Pair<Long, List<Long>> {
    val (departureLine, idsLine) = input.lines()
    val departure = departureLine.toLong()
    val ids = idsLine.split(",").mapNotNull { it.toLongOrNull() }
    return departure to ids
}

fun main() {
    val (departure, ids) = parse(readInput("day13.txt"))
    val result = ids.map { id ->
        val minTime = if (departure % id == 0L) departure
        else {
            ((departure / id) + 1) * id
        }
        val waitingTime = minTime - departure
        waitingTime * id to minTime
    }.minByOrNull { it.second }!!.first
    println("result: $result")
}