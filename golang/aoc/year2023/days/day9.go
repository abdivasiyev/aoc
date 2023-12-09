package days

import (
	"fmt"

	"aoc/aoc"
)

type day9 struct {
	year        int
	day         int
	linesFirst  []string
	linesSecond []string
}

func findDiffs(history []int, s *[][]int) {
	isAllZero := true
	for i := 0; i < len(history); i++ {
		if history[i] != 0 {
			isAllZero = false
			break
		}
	}

	if isAllZero {
		return
	}

	var diff []int
	for i := 1; i < len(history); i++ {
		diff = append(diff, history[i]-history[i-1])
	}

	*s = append(*s, diff)

	findDiffs(diff, s)
}

// First implements aoc.Solution.
func (d *day9) First() ([]string, error) {
	var total int

	for _, line := range d.linesFirst {
		var s [][]int

		history := aoc.Collect(line, " ")
		s = append(s, history)
		findDiffs(history, &s)

		last := s[len(s)-1]
		next := last[len(last)-1]
		s = s[:len(s)-1]
		for len(s) > 0 {
			last = s[len(s)-1]
			next += last[len(last)-1]
			s = s[:len(s)-1]
		}

		total += next
	}

	return aoc.Output(total), nil
}

// Second implements aoc.Solution.
func (d *day9) Second() ([]string, error) {
	var total int

	for _, line := range d.linesSecond {
		var s [][]int

		history := aoc.Collect(line, " ")
		s = append(s, history)
		findDiffs(history, &s)

		last := s[len(s)-1]
		prev := last[0]
		s = s[:len(s)-1]
		for len(s) > 0 {
			last = s[len(s)-1]
			prev = last[0] - prev
			s = s[:len(s)-1]
		}

		total += prev
	}

	return aoc.Output(total), nil
}

// String implements aoc.Solution.
func (d *day9) String() string {
	return fmt.Sprintf("***AOC %d***\n___DAY %d___\n", d.year, d.day)
}

func Day9(y int, d int) aoc.Solution {
	day := &day9{
		year:        y,
		day:         d,
		linesFirst:  nil,
		linesSecond: nil,
	}

	var err error
	day.linesFirst, err = aoc.ReadInput(fmt.Sprintf("./input/%d/%d/1/input.txt", y, d))
	if err != nil {
		panic(err)
	}
	day.linesSecond, err = aoc.ReadInput(fmt.Sprintf("./input/%d/%d/2/input.txt", y, d))
	if err != nil {
		panic(err)
	}

	return day
}
