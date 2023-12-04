package day11.task2

import readInput
import java.lang.StringBuilder

data class Field(private val data: MutableList<Char>,
                 val width: Int,
                 val height: Int) {

    companion object {
        fun fromString(s: String): Field {
            val lines = s.lines()
            val height = lines.size
            val width = lines.firstOrNull()!!.length
            val data = s.toCharArray().filter { it != '\n' }.toMutableList()
            return Field(data = data, width = width, height = height)
        }

    }

    operator fun get(x: Int, y: Int): Char {
        return data[y * width + x]
    }

    operator fun set(x: Int, y: Int, v: Char) {
        data[y * width + x] = v
    }

    fun data(): List<Char> {
        return data
    }

    override fun toString(): String {
        val result = StringBuilder()
        data.chunked(width).forEach { line ->
            result.append(String(line.toCharArray()))
            result.append("\n")
        }
        return result.toString()
    }

    fun copy(): Field {
        return Field(data.toMutableList(), width, height)
    }
}

fun Field.contains(x: Int, y: Int): Boolean {
    return x in 0 until width && y in 0 until height
}

fun Field.beam(x: Int, y: Int, dx: Int, dy: Int, cont: (Char) -> Boolean){
    var x = x + dx
    var y = y + dy
    while (contains(x, y) && cont(this[x, y])) {
        x += dx
        y += dy
    }
}

fun Field.neighbours(x: Int, y: Int): List<Char> {
    val result = mutableListOf<Char>()
    val cont: (Char) -> Boolean = { c ->
        when (c) {
            '.' -> true
            'L' -> {
                result.add('L')
                false
            }
            '#' -> {
                result.add('#')
                false
            }
            else -> throw Error("Unexpected symbol $c")
        }
    }
    beam(x, y, -1, 0, cont)
    beam(x, y, 1, 0, cont)
    beam(x, y, 0, -1, cont)
    beam(x, y, 0, 1, cont)

    beam(x, y, -1, -1, cont)
    beam(x, y, 1, -1, cont)
    beam(x, y, -1, 1, cont)
    beam(x, y, 1, 1, cont)
    return result
}

fun step(field: Field): Field {
    val newField = field.copy()
    for (x in 0 until field.width) {
        for (y in 0 until field.height) {
            when (field[x, y]) {
                '#' -> {
                    val neighbours = field.neighbours(x, y).count { it == '#' }
                    if (neighbours >= 5) {
                        newField[x, y] = 'L'
                    }
                }
                'L' -> {
                    val neighbours = field.neighbours(x, y).count { it == '#' }
                    if (neighbours == 0) {
                        newField[x, y] = '#'
                    }
                }
                else -> {}
            }
        }
    }
    return newField
}


fun main() {
    var field = Field.fromString(readInput("day11.txt"))
    println("Width: ${field.width} Height: ${field.height} \n$field")
    var round = 0
    while (round < 10000) {
        val newField = step(field)
        if (field == newField) break
        field = newField
//        println("Round: $round")
//        println(field)
        round += 1
    }
    val result = field.data().count { it == '#' }
    println("Result: $result")
}