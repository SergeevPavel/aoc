package day9.task1

import readInput

fun main() {
    val data = readInput("day9.txt").lines().map { it.toLong() }.toList()
    val windowSize = 25
    for (i in windowSize until data.size) {
        var isValid = false
        loop@for (j in 1..windowSize) {
            for (k in (j + 1)..windowSize) {
                if (data[i - j] + data[i - k] == data[i]) {
                    isValid = true
                    break@loop
                }
            }
        }
        if (!isValid) {
            println("Result: ${data[i]}")
            break
        }
    }
}