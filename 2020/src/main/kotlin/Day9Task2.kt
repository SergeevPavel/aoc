package day9.task2

import readInput

fun main() {
    val data = readInput("day9.txt").lines().map { it.toLong() }.toList()
    val invalidNumber = 1124361034L
    for (s in 2 until data.size) {
        for (i in 0..(data.size - s)) {
            val slice = data.subList(i, i + s)
            if (slice.sum() == invalidNumber) {
                println("Result: ${slice.minOrNull()!! + slice.maxOrNull()!!}")
                return
            }
        }
    }
}