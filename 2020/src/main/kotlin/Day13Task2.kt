package day13.task2

import readInput

data class Route(val id: Long, val pos: Long)

fun parseInput(input: String): List<Route> {
    return input.lines()[1].split(",").mapIndexedNotNull { index, id ->
        id.toLongOrNull()?.let {
            Route(it, index.toLong())
        }
    }
}

data class BezoutIdentity(val gcd: Long, val x: Long, val y: Long)

fun egcd(a: Long, b: Long): BezoutIdentity {
    if (a == 0L) {
        return BezoutIdentity(b, 0L, 1L)
    }
    val bi1 = egcd(b % a, a)
    val x = bi1.y - (b / a) * bi1.x
    val y = bi1.x
    return BezoutIdentity(bi1.gcd, x, y)
}

fun inverse(x: Long, mod: Long): Long {
    return (egcd(x, mod).x + mod) % mod
}

data class Equation(val r: Long, val a: Long)

fun crt(eqs: List<Equation>): Long {
    val m = eqs.map { it.a }.reduce { a, b -> a * b }
    val ms = eqs.map { m / it.a }
    val msInv = ms.zip(eqs).map { (m, eq) ->
        val m = m % eq.a
        val invM = inverse(m, eq.a)
        println("m: $m invM: $invM a: ${eq.a}")
        invM
    }
    println("ms: $ms msInv: $msInv ")
    var result = 0L
    for (i in eqs.indices) {
        result += ms[i] * msInv[i] * eqs[i].r
        result %= m
    }
    return result
}

fun negate(x: Long, mod: Long): Long {
    var x = - x
    while (x < 0L) x += mod
    return x % mod
}

fun main() {
    val parsed = parseInput(readInput("day13.txt"))
    val eqs = parsed.map { Equation(r = negate(it.pos, it.id), a = it.id) }
    println(eqs)
    println(crt(eqs))
}