import Foundation

struct MalformedInput: Error {}

func data() throws -> [[Int]] {
    let s = try String(contentsOfFile: "data/day02.txt")
    let lines = s.split(whereSeparator: \.isNewline)
    return try lines.map { line in
        try line.split(whereSeparator: \.isWhitespace).map { num in
            guard let x = Int(num) else {
                throw MalformedInput()
            }
            return x
         }
    }
}

func isReportSafe(_ report: [Int]) -> Bool {
    let pairs = zip(report, report.dropFirst())
    let inc = pairs.allSatisfy { (a, b) in
        b > a
    }
    let dec = pairs.allSatisfy { (a, b) in
        b < a
    }
    let cond1 = inc || dec
    let cond2 = pairs.allSatisfy { (a, b) in
        let d = abs(a - b)
        return d >= 1 && d <= 3
    }
    return cond1 && cond2
}

func part1() throws {
    let reports = try data()
    let result = reports.count { report in
        isReportSafe(report)    
    }
    print("Result1: \(result)")
}

func part2() throws {
    let reports = try data()
    let result = reports.count { report in
        for i in 0..<report.count {
            var report = report
            report.remove(at: i)
            if isReportSafe(report) {
                return true
            }
        }
        return false
    }
    print("Result2: \(result)")
}

try part1()
try part2()