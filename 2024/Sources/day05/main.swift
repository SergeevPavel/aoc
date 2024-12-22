import Foundation

typealias Rules = [Int:[Int]]
struct MalformedInput: Error {
    let s: String
}

func data() throws -> (rules: Rules, orders: [[Int]]) {
    let input = try String(contentsOfFile: "data/day05.txt", encoding: String.Encoding.ascii)
    let parts = input.split(separator: "\n\n", maxSplits: 2, omittingEmptySubsequences: true)
    let rulesText = parts[0]
    let ordersText = parts[1]
    let rules = try rulesText.split(whereSeparator: \.isNewline).map { rule in
        let rule = rule.split(separator: "|", maxSplits: 1)
        return if let before = Int(rule[0]), let after = Int(rule[1]) {
            (before: before, after: after)
        } else {
            throw MalformedInput(s: "Broken rule \(rule)")
        }
    }.reduce(into: [Int:[Int]](), { result, rule in
        let (before, after) = rule
        result[before, default: []].append(after)
    })
    let orders = try ordersText.split(whereSeparator: \.isNewline).map { order in
        try order.split(separator: ",").map { n in
            if let x = Int(n) {
                x
            } else {
                throw MalformedInput(s: "Broken order: \(order)")
            }
        }
    }

    return (rules, orders)
}

func isConforming(order: [Int], rules: Rules) -> Bool {
    var visitedBefore: Set<Int> = Set()
    for e in order {
        for shouldBeAfter in rules[e] ?? [] {
            if visitedBefore.contains(shouldBeAfter) {
                return false
            }
        }
        visitedBefore.insert(e)
    }
    return true
}

func dfs(_ el: Int, _ visited: inout Set<Int>, _ rules: Rules, _ newOrder: inout [Int]) {
    if !visited.contains(el) {
        visited.insert(el)
        for neighbors in rules[el] ?? [] {
            dfs(neighbors, &visited, rules, &newOrder)
        }
        newOrder.append(el)
    }
}

func filterRules(order: Set<Int>, rules: Rules) -> Rules {
    rules.filter { el in
        order.contains(el.key)
    }.mapValues { value in
        value.filter { el in
            order.contains(el)
        }
    }
}

func fixedOrder(order: [Int], rules: Rules) -> [Int] {
    let rules = filterRules(order: Set(order), rules: rules)
    var newOrder: [Int] = []
    var visited: Set<Int> = Set()
    for el in order.reversed() {
        dfs(el, &visited, rules, &newOrder)
    }
    return newOrder.reversed()
}

func part1() throws {
    let (rules, orders) = try data()
    let result = orders.filter { order in
        isConforming(order: order, rules: rules)
    }.map { order in
        let mid = order.count / 2
        return order[mid]
    }.reduce(0, +)
    print("Result1: \(result)")
    
}

func part2() throws {
    let (rules, orders) = try data()
    let result = orders.filter { order in
        !isConforming(order: order, rules: rules)
    }.map { order in
        let fixedOrder = fixedOrder(order: order, rules: rules)
        assert(fixedOrder.count == order.count)
        assert(Set(fixedOrder) == Set(order))
        assert(isConforming(order: fixedOrder, rules: rules), "\(order)\n\(fixedOrder)")

        let mid = fixedOrder.count / 2
        return fixedOrder[mid]
    }.reduce(0, +)
    print("Result2: \(result)")
    
}

try part1()
try part2()