package day16.task1

import readInput

data class Constraint(val fieldName: String, val ranges: List<IntRange>)

data class InputData(val constraints: List<Constraint>,
                     val ourTicket: List<Int>,
                     val otherTickets: List<List<Int>>)

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
    val errorRate = inputData.otherTickets.flatMap { ticket ->
        ticket.filterNot { field ->
            inputData.constraints.any { constraint ->
                constraint.ranges.any { range ->
                    range.contains(field)
                }
            }
        }
    }.sum()
    println("Error rate: $errorRate")
}