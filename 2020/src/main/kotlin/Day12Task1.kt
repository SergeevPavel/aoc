package day12.task1

import readInput
import kotlin.math.absoluteValue

fun parseCommand(line: String): Pair<Char, Int> {
    return Pair(line[0], line.drop(1).toInt())
}

fun main() {

    var x = 0
    var y = 0
    var dir = 0
    readInput("day12.txt").lines().forEach { line ->
        val (cmd, arg) = parseCommand(line)
        when (cmd) {
            'N' -> {
                y += arg
            }
            'S' -> {
                y -= arg
            }
            'E' -> {
                x += arg
            }
            'W' -> {
                x -= arg
            }
            'L' -> {
                dir = ((dir + arg) + 360) % 360
            }
            'R' -> {
                dir = ((dir - arg) + 360) % 360
            }
            'F' -> {
                when (dir) {
                    0   -> x += arg
                    90  -> y += arg
                    180 -> x -= arg
                    270 -> y -= arg
                }
            }
        }
    }
    println("Result: ${x.absoluteValue + y.absoluteValue}")
}