sealed class Instruction {
    data class Acc(val arg: Int): Instruction()
    data class Jmp(val arg: Int): Instruction()
    data class Nop(val arg: Int): Instruction()
}

private fun run(program: List<Instruction>): Int {
    var acc = 0
    var pc = 0
    val visitedStates = mutableSetOf<Int>()
    while (0 <= pc && pc < program.size) {
        visitedStates.add(pc)
        val instr = program[pc]
        when (instr) {
            is Instruction.Acc -> {
                acc += instr.arg
                pc += 1
            }
            is Instruction.Jmp -> {
                pc += instr.arg
            }
            is Instruction.Nop -> {
                pc += 1
            }
        }
        if (visitedStates.contains(pc)) break 
    }
    return acc
}

fun main() {
    val program = readInput("day8.txt").lines().map { line ->
        val instr = line.split(" ")
        when (instr[0]) {
            "acc" -> {
                Instruction.Acc(instr[1].toInt())
            }
            "jmp" -> {
                Instruction.Jmp(instr[1].toInt())
            }
            "nop" -> {
                Instruction.Nop(instr[1].toInt())
            }
            else -> throw Error("Illegal instruction: $line")
        }
    }.toList()
    println(run(program))
}