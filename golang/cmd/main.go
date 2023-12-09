package main

import (
	"aoc/aoc"
	days2023 "aoc/aoc/year2023/days"
)

func main() {
	solutions := []aoc.Solution{
		days2023.Day1(2023, 1),
		days2023.Day2(2023, 2),
		days2023.Day3(2023, 3),
		days2023.Day4(2023, 4),
		days2023.Day5(2023, 5),
		days2023.Day6(2023, 6),
		days2023.Day7(2023, 7),
		days2023.Day8(2023, 8),
		days2023.Day9(2023, 9),
	}

	runner := aoc.New(aoc.Last)
	runner.Add(solutions...)
	runner.Run()

}
