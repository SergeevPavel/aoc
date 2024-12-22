import Foundation

enum Command {
    case Mul(a: Int, b: Int)
    case Do
    case Dont
}

func parseCommands() throws -> [Command] {
    let commandPattern = /(?:don't\(\))|(?:do\(\))|(?:mul\((?<a>\d+)\,(?<b>\d+)\))/
    let s = try String(contentsOfFile: "data/day03.txt")
    let matches = s.matches(of: commandPattern)
    return matches.compactMap { m -> Command? in
        let command = m.output.0
        return if command.starts(with: "mul") {
            if let a = m.output.a,
                let b = m.output.b,
                let a = Int(a),
                let b = Int(b)
            {
                Command.Mul(a: a, b: b)
            } else {
                nil
            }
        } else if command.starts(with: "don't") {
            Command.Dont
        } else if command.starts(with: "do") {
            Command.Do
        } else {
            nil
        }
    }
}

func part1() throws {
    let commands = try parseCommands()
    var result = 0
    for command in commands {
        switch command {
        case .Mul(let a, let b):
        result += a * b

        case .Do: break

        case .Dont: break

        }
    }
    print("Result1: \(result)")
}

func part2() throws {
    let commands = try parseCommands()
    print(commands)
    var result = 0
    var skipNext = false
    for command in commands {
        switch command {
        case .Mul(let a, let b):
        if !skipNext {
            result += a * b
        }

        case .Do:
        skipNext = false

        case .Dont:
        skipNext = true

        }
    }
    print("Result1: \(result)")
}

try part1()
try part2()
