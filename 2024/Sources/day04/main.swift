import Foundation

struct Field {
    let width: Int
    let height: Int
    let field: [[Character]]

    func get(_ x: Int, _ y: Int) -> Character? {
        guard x >= 0 && x < width && y >= 0 && y < width else {
            return nil
        }
        return field[y][x]
    }
}

let word: [Character] = ["X", "M", "A", "S"]

func data() throws -> Field {
    let letters: Set<Character> = Set(word)
    let s = try String(contentsOfFile: "data/day04.txt")
    let lines = s.split(whereSeparator: \.isNewline)
    let field = lines.map { line in
        line.compactMap { c in
            if letters.contains(c) {
                c
            } else {
                nil
            }
        }
    }
    let height = field.count
    let width = field[0].count
    return Field(width: width, height: height, field: field)
}

func checkWord(field: Field, x: Int, y: Int, dx: Int, dy: Int) -> Bool {
    var x = x
    var y = y
    for i in 0..<word.count {
        if field.get(x, y) != word[i] {
            return false
        }
        x += dx
        y += dy
    }
    return true
}

func part1() throws {
    let field = try data()
    var result = 0
    for y in 0..<field.height {
        for x in 0..<field.width {
            for dx in -1...1 {
                for dy in -1...1 {
                    if checkWord(field: field, x: x, y: y, dx: dx, dy: dy) {
                        result += 1
                    }
                }
            }
        }
    }
    print("Result1: \(result)")
}

func checkXMas(field: Field, x: Int, y: Int) -> Bool {
    let cond1 = field.get(x, y) == "A" 
    let cond2 = Set([field.get(x - 1, y - 1), field.get(x + 1, y + 1)]) == Set(["M", "S"])
    let cond3 = Set([field.get(x - 1, y + 1), field.get(x + 1, y - 1)]) == Set(["M", "S"])
    return cond1 && cond2 && cond3
}

func part2() throws {
    let field = try data()
    var result = 0
    for y in 0..<field.height {
        for x in 0..<field.width {
            if checkXMas(field: field, x: x, y: y) {
                result += 1
            }
        }
    }
    print("Result2: \(result)")
}

try part1()
try part2()
