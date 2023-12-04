package day14.task2

import readInput

sealed class Instruction {
    data class UpdateMask(val newMask: String): Instruction()
    data class SetValue(val addr: Long, val newValue: Long): Instruction()
}

fun parse(input: String): List<Instruction> {
    val maskPattern = Regex("mask = (.*)")
    val memPattern = Regex("mem\\[(\\d*)] = (\\d*)")
    return input.lines().map { line ->
        maskPattern.find(line)?.let { match ->
            Instruction.UpdateMask(match.groupValues[1])
        } ?: memPattern.find(line)?.let { match ->
            Instruction.SetValue(addr = match.groupValues[1].toLong(), newValue = match.groupValues[2].toLong())
        } ?: throw Error("Unexpected input format $line")
    }
}

fun applyMask(mask: String, v: Long): Set<Long> {
    val result = mutableSetOf<Long>()
    var withOnes = v
    Regex("1").findAll(mask.reversed()).map {
        it.range.first
    }.forEach {
        withOnes = withOnes or (1L shl it)
    }
    val floatingBitPositions = Regex("X").findAll(mask.reversed()).map {
        it.range.first
    }.toList()
    val floatingBitsCount = floatingBitPositions.size
    for (n in 0..(1L shl floatingBitsCount)) {
        floatingBitPositions.forEachIndexed { index, position ->
            val b = (1L shl index) and n > 0
            if (b) {
                withOnes = withOnes or (1L shl position)
            } else {
                withOnes = withOnes and (1L shl position).inv()
            }
        }
        result.add(withOnes)
    }
    return result
}

fun main() {
    var mask: String? = null
    val memory = HashMap<Long, Long>()
    parse(readInput("day14.txt")).forEach { instr ->
        when (instr) {
            is Instruction.UpdateMask -> {
                mask = instr.newMask
            }
            is Instruction.SetValue -> {
                for (addr in applyMask(mask ?: throw Error("No initial mask"), instr.addr)) {
                    memory[addr] = instr.newValue
                }
            }
        }
    }
    println("Result: ${memory.values.sum()}")
}