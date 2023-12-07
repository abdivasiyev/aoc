package main

import (
	"fmt"

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
	}

	solution := solutions[len(solutions)-1]

	fmt.Println(solution)
	part1, err := solution.First()
	if err != nil {
		panic(err)
	}
	fmt.Printf("Part 1: %v\n", part1)

	part2, err := solution.Second()
	if err != nil {
		panic(err)
	}
	fmt.Printf("Part 2: %v\n\n", part2)

}
