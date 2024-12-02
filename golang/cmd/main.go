package main

import (
	"aoc/aoc"
	days2024 "aoc/aoc/year2024/days"
)

func main() {
	solutions := []aoc.Solution{
		days2024.Day1(2024, 1),
		days2024.Day2(2024, 2),
	}

	runner := aoc.New(aoc.Last)
	runner.Add(solutions...)
	runner.Run()

}
