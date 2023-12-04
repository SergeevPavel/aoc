package day14.task1

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

fun applyMask(mask: String, v: Long): Long {
    var result = v
    mask.reversed().forEachIndexed { i, c ->
        when(c) {
            '0' -> {
                result = result and (1L shl i).inv()
            }
            '1' -> {
                result = result or (1L shl i)
            }
        }
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
                memory[instr.addr] = applyMask(mask ?: throw Error("No initial mask"), instr.newValue)
            }
        }
    }
    println("Result: ${memory.values.sum()}")
}