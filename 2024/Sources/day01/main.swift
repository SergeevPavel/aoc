import Foundation

func data() throws -> ([Int], [Int]) {
    let s = try String(contentsOfFile: "data/day01.txt")
    let lines = s.split(whereSeparator: \.isNewline)
    let pairs = lines.map { s in
        let nums = s.split(maxSplits: 2, whereSeparator: \.isWhitespace)
        let a = Int(nums[0])!
        let b = Int(nums[1])!
        return (a, b)
    }
    let nums1 = pairs.map(\.0)
    let nums2 = pairs.map(\.1)
    return (nums1, nums2)
}

func part1() throws {
    var (nums1, nums2) = try data()
    nums1.sort()
    nums2.sort()
    let result = zip(nums1, nums2).map { (a: Int, b: Int) in
        abs(a - b)
    }.reduce(0, +)
    print("Result1: \(result)")
}

func part2() throws {
    let (nums1, nums2) = try data()
    let result = nums1.map { n in
        let count = nums2.count { m in
            m == n
        }
        return count * n
    }.reduce(0, +)
    print("Result2: \(result)")
}

try part1()
try part2()
