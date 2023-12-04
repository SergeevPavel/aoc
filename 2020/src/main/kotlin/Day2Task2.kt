

fun main() {
    val result = readInput("day2.txt").lines().map { line ->
        val result = Regex("(\\d*)-(\\d*)\\s(\\w):\\s(\\w*)").find(line)!!
        val firstIndex = result.groupValues[1].toInt() - 1
        val secondIndex = result.groupValues[2].toInt() - 1
        val letter = result.groupValues[3]
        val password = result.groupValues[4]
        val isValid = (password.elementAtOrNull(firstIndex).toString() == letter) xor (password.elementAtOrNull(secondIndex).toString() == letter)
        println("Result: ${result.groupValues} IsValid: $isValid")
        isValid
    }.filter { it }.count()
    println(result)
}