
// missing quotes overtype
// bracket overtype is broken
// create file in current dir in file tree
// cmd-s doesn't work

fun main() {
    val result = readInput("day2.txt").lines().map { line ->
        val result = Regex("(\\d*)-(\\d*)\\s(\\w):\\s(\\w*)").find(line)!!
        println("Result: ${result.groupValues}")
        val minConstraint = result.groupValues[1].toInt()
        val maxConstraint = result.groupValues[2].toInt()
        val letter = result.groupValues[3]
        val password = result.groupValues[4]
        val letterCount = letter.toRegex().findAll(password).count()
        minConstraint <= letterCount && letterCount <= maxConstraint
    }.filter { it }.count()
    println(result)
}