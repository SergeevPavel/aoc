package day10.task1

import readInput

fun main() {
    val input = readInput("day10.txt").lines().map {
        it.toInt()
    }
    val adaptors = (input + listOf(0, input.maxOrNull()!! + 3)).sorted()
    println("Adaptors: $adaptors")
    var oneJolt = 0
    var threeJolts = 0
    for (i in 1 until adaptors.size) {
        when (val d = adaptors[i] - adaptors[i - 1]) {
            0 -> {}
            1 -> {
                oneJolt += 1
            }
            2 -> {}
            3 -> {
                threeJolts += 1
            }
            else -> println("Malformed input: $adaptors \n difference: $d between: ${adaptors[i]} ${adaptors[i - 1]}")
        }
    }
    println("Result: $oneJolt $threeJolts ${oneJolt * threeJolts}")
}