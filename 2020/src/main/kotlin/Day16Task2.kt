package day16.task2

import readInput

data class Constraint(val fieldName: String, val ranges: List<IntRange>) {
    fun satisfy(v: Int): Boolean {
        return ranges.any { range ->
            range.contains(v)
        }
    }
}

data class InputData(val constraints: List<Constraint>,
                     val ourTicket: List<Int>,
                     val otherTickets: List<List<Int>>,)

fun parse(input: String): InputData {
    val (constraintsPart, ourTicketPart, otherTicketsPart) = input.split("\n\n")
    val constraints = constraintsPart.lines().map {
        val (name, rangesString) = it.split(": ")
        val ranges = rangesString.split(" or ").map { range ->
            val (minValue, maxValue) = range.split("-")
            IntRange(minValue.toInt(), maxValue.toInt())
        }
        Constraint(name, ranges)
    }
    val ourTicket = ourTicketPart.lines()[1].split(",").map { it.toInt() }
    val otherTickets = otherTicketsPart.lines().drop(1).map { line ->
        line.split(",").map { it.toInt() }
    }
    return InputData(constraints, ourTicket, otherTickets)
}

fun main() {
    val inputData = parse(readInput("day16.txt"))
    val filteredTickets = inputData.otherTickets.filter { ticket ->
        ticket.all { field ->
            inputData.constraints.any { constraint ->
                constraint.satisfy(field)
            }
        }
    }
    val correspondingConstraints = filteredTickets.map { ticket ->
        ticket.map { field ->
            inputData.constraints.filter { constraint -> constraint.satisfy(field) }.toSet()
        }
    }
    for (i in correspondingConstraints.first().indices) {
        val iThFieldConstraints = correspondingConstraints.map { constraintsPerTicket ->
            constraintsPerTicket[i]
        }
        println("i: $i constraints: ${iThFieldConstraints.reduce { s1, s2 -> s1.intersect(s2) }}")
    }
}