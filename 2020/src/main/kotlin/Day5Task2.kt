

fun main() {
    val seatIds = readInput("day5.txt").lines().map { line ->
        val rowCode = line.substring(0, 7)
        val columnCode = line.substring(7, 10)

        val row = rowCode.replace('B', '1').replace('F', '0').toInt(2)
        val column = columnCode.replace('L', '0').replace('R', '1').toInt(2)
        val seatId = row * 8 + column

        println("row: $rowCode row: $row columnCode: $columnCode column: $column seatId: $seatId")
        seatId

    }
    val minSeatId = seatIds.minOrNull()!!
    val maxSeatId = seatIds.maxOrNull()!!
    println("Result: ${(minSeatId..maxSeatId).toSet().minus(seatIds)}")
}