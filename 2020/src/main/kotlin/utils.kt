

fun readInput(name: String): String {
    return object {}.javaClass.getResource(name).readText()
}