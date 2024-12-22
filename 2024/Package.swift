// swift-tools-version: 6.0
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "aoc2024",
    platforms: [
        .macOS(.v13)
    ],
    products: [
        .executable(name: "day01", targets: ["day01"]),
        .executable(name: "day02", targets: ["day02"]),
        .executable(name: "day03", targets: ["day03"]),
        .executable(name: "day04", targets: ["day04"]),
    ],
    targets: [
        .executableTarget(name: "day01"),
        .executableTarget(name: "day02"),
        .executableTarget(name: "day03"),
        .executableTarget(name: "day04"),
    ]
)
