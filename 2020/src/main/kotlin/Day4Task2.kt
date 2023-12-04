
//    byr (Birth Year) - four digits; at least 1920 and at most 2002.
//    iyr (Issue Year) - four digits; at least 2010 and at most 2020.
//    eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
//    hgt (Height) - a number followed by either cm or in:
//        If cm, the number must be at least 150 and at most 193.
//        If in, the number must be at least 59 and at most 76.
//    hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
//    ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
//    pid (Passport ID) - a nine-digit number, including leading zeroes.
//    cid (Country ID) - ignored, missing or not.

fun isValid(passport: Map<String, String>): Boolean {
    passport.get("byr")?.let { year ->
        year.toIntOrNull()?.let { year ->
            if (year < 1920 || 2002 < year) return false            
        } ?: return false
    } ?: return false

    passport.get("iyr")?.let { year ->
        year.toIntOrNull()?.let { year ->
            if (year < 2010 || 2020 < year) return false            
        } ?: return false
    } ?: return false

    passport.get("eyr")?.let { year ->
        year.toIntOrNull()?.let { year ->
            if (year < 2020 || 2030 < year) return false            
        } ?: return false
    } ?: return false

    passport.get("hgt")?.let { height ->
        val cmMatch = Regex("(\\d+)cm").find(height)
        if (cmMatch != null) {
            val heightInCm = cmMatch.groupValues[1].toIntOrNull() ?: return false
            if (heightInCm < 150 || heightInCm > 193) return false
        }
        else {
            val inMatch = Regex("(\\d+)in").find(height)
            if (inMatch != null) {
                val heightInIn = inMatch.groupValues[1].toIntOrNull() ?: return false
                if (heightInIn < 59 || heightInIn > 76) return false
            } else {
                return false
            }
        }
    } ?: return false
    
    passport.get("hcl")?.let { hairColor ->
        Regex("^#([0-9]|[a-f]){6}$").find(hairColor) ?: return false
    } ?: return false
    
    passport.get("ecl")?.let { eyeColor ->
        if (!setOf("amb", "blu", "brn", "gry", "grn", "hzl", "oth").contains(eyeColor)) {
            return false
        }
    } ?: return false
    
    passport.get("pid")?.let { passportId ->
        Regex("^\\d{9}$").find(passportId) ?: return false
    } ?: return false

    return true
}

fun main() {
    val validCount = readInput("day4.txt").split("\n\n").map() { passport ->
        val keyValues = passport.split("\\s+".toRegex()).map() { keyValueString ->
            keyValueString.split(":").let {
                Pair(it[0], it[1])
            }
        }.toMap()
        isValid(keyValues)
    }.filter { it }.count()
    println("Result: $validCount")
}