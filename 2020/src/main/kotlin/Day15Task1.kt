package day15.task1

import readInput

fun main() {
    val numbers = readInput("day15.txt").split(",").map { it.toInt() }.toMutableList()
    while (numbers.size < 2020) {
        val last = numbers.last()
        val lastIndex = numbers.dropLast(1).lastIndexOf(last)
        if (lastIndex == -1) {
            numbers.add(0)
        } else {
            numbers.add(numbers.size - lastIndex - 1)
        }
    }
    println(numbers.last())
}