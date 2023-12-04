package day12.task2

import readInput
import kotlin.math.absoluteValue

fun parseCommand(line: String): Pair<Char, Int> {
    return Pair(line[0], line.drop(1).toInt())
}

data class Point(var x: Int, var y: Int)

fun Point.rotate(degree: Int) {
    fun rotate90(p: Point) {
        val newX = -p.y
        val newY = p.x
        p.x = newX
        p.y = newY
    }
    val rs = ((degree + 360) % 360) / 90
    for (i in 0 until rs) {
        rotate90(this)
    }
}

fun main() {
    val wayPoint = Point(10, 1)
    val ship = Point(0, 0)
    readInput("day12.txt").lines().forEach { line ->
        val (cmd, arg) = parseCommand(line)
        when (cmd) {
            'N' -> {
                wayPoint.y += arg
            }
            'S' -> {
                wayPoint.y -= arg
            }
            'E' -> {
                wayPoint.x += arg
            }
            'W' -> {
                wayPoint.x -= arg
            }
            'L' -> {
                wayPoint.rotate(arg)
            }
            'R' -> {
                wayPoint.rotate(360 - arg)
            }
            'F' -> {
                ship.x += wayPoint.x * arg
                ship.y += wayPoint.y * arg
            }
        }
    }
    println("Result: ${ship.x.absoluteValue + ship.y.absoluteValue}")
}