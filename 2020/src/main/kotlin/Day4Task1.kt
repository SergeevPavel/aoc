

fun main() {
    val requiredKeys = setOf("byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid") // "cid"
    val validCount = readInput("day4.txt").split("\n\n").map() { passport ->
        val keyValues = passport.split("\\s+".toRegex()).map() { keyValueString ->
            keyValueString.split(":").let {
                Pair(it[0], it[1])
            }
        }        
        val keys = keyValues.map() {
           it.first
        }.toSet()
        
        keys.containsAll(requiredKeys)        
    }.filter { it }.count()
    println("Result: $validCount")
}