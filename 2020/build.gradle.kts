import org.jetbrains.kotlin.gradle.tasks.KotlinCompile

plugins {
    kotlin("jvm") version "1.7.10"
    java
}
group = "me.pavel"
version = "1.0-SNAPSHOT"

repositories {
    mavenCentral()
}
dependencies {
    testImplementation(kotlin("test-junit"))
}
tasks.withType<KotlinCompile>() {
    kotlinOptions.jvmTarget = "1.8"
}

task("day1_1", JavaExec::class) {
    mainClass = "Day1Task1Kt"
    classpath = sourceSets["main"].runtimeClasspath
}

task("day1_2", JavaExec::class) {
    mainClass = "Day1Task2Kt"
    classpath = sourceSets["main"].runtimeClasspath
}

task("day2_1", JavaExec::class) {
    mainClass = "Day2Task1Kt"
    classpath = sourceSets["main"].runtimeClasspath
}

task("day2_2", JavaExec::class) {
    mainClass = "Day2Task2Kt"
    classpath = sourceSets["main"].runtimeClasspath
}

task("day3_1", JavaExec::class) {
    mainClass = "Day3Task1Kt"
    classpath = sourceSets["main"].runtimeClasspath
}

task("day3_2", JavaExec::class) {
    mainClass = "Day3Task2Kt"
    classpath = sourceSets["main"].runtimeClasspath
}

task("day4_1", JavaExec::class) {
    mainClass = "Day4Task1Kt"
    classpath = sourceSets["main"].runtimeClasspath
}

task("day4_2", JavaExec::class) {
    mainClass = "Day4Task2sKt"
    classpath = sourceSets["main"].runtimeClasspath
}

task("day5_1", JavaExec::class) {
    mainClass = "Day5Task1Kt"
    classpath = sourceSets["main"].runtimeClasspath
}

task("day5_2", JavaExec::class) {
    mainClass = "Day5Task2Kt"
    classpath = sourceSets["main"].runtimeClasspath
}

task("day7_1", JavaExec::class) {
    mainClass = "Day7Task1Kt"
    classpath = sourceSets["main"].runtimeClasspath
}


task("day7_2", JavaExec::class) {
    mainClass = "Day7Task2Kt"
    classpath = sourceSets["main"].runtimeClasspath
}

task("day8_1", JavaExec::class) {
    mainClass = "Day8Task1Kt"
    classpath = sourceSets["main"].runtimeClasspath
}

task("day8_2", JavaExec::class) {
    mainClass = "day8.task2.Day8Task2Kt"
    classpath = sourceSets["main"].runtimeClasspath
}

task("day11_1", JavaExec::class) {
    mainClass = "day11.task1.Day11Task1Kt"
    classpath = sourceSets["main"].runtimeClasspath
}

task("day14_1", JavaExec::class) {
    mainClass = "day14.task1.Day14Task1Kt"
    classpath = sourceSets["main"].runtimeClasspath
}

task("day16_1", JavaExec::class) {
    mainClass = "day16.task1.Day16Task1Kt"
    classpath = sourceSets["main"].runtimeClasspath
}

task("day16_2", JavaExec::class) {
    mainClass = "day16.task2.Day16Task2Kt"
    classpath = sourceSets["main"].runtimeClasspath
}