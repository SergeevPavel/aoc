package day11.task1

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

fun Field.neighbours(x: Int, y: Int): List<Char> {
    val result = mutableListOf<Char>()
    for (dx in -1..1) {
        for (dy in -1..1) {
            if (dx == 0 && dy == 0) continue
            val nx = x + dx
            val ny = y + dy
            if (contains(nx, ny)) {
                result.add(this[nx, ny])
            }
        }
    }
    return result
}

fun step(field: Field): Field {
    val newField = field.copy()
    for (x in 0 until field.width) {
        for (y in 0 until field.height) {
            when (field[x, y]) {
                '#' -> {
                    val neighbours = field.neighbours(x, y).count { it == '#' }
                    if (neighbours >= 4) {
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
    val x = 23
    
    
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