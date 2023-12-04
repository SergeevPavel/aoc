package day10.task2

import readInput

fun main() {
    val input = readInput("day10.txt").lines().map {
        it.toInt()
    }
    val adaptors = (input + listOf(0, input.maxOrNull()!! + 3)).sorted()
    val counts = MutableList(adaptors.size) { 0L }
    counts[0] = 1
    for (i in counts.indices) {
        var j = i + 1
        while (j < counts.size && adaptors[j] - adaptors[i] <= 3) {
            counts[j] += counts[i]
            j += 1
        }
    }
    println("Result: ${counts.lastOrNull()}")
}