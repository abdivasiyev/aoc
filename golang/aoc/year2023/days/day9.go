package days

import (
	"fmt"

	"aoc/aoc"
)

type day9 struct {
	year int
	day  int

	history [][]int
}

func (d *day9) Parse() error {
	lines, err := aoc.ReadInput(fmt.Sprintf("./input/%04d-%02d.txt", d.year, d.day))
	if err != nil {
		return err
	}

	for _, line := range lines {
		h := aoc.Collect(line, " ")
		d.history = append(d.history, h)
	}

	return nil
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

	for _, h := range d.history {
		var s [][]int
		s = append(s, h)
		findDiffs(h, &s)

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

	for _, h := range d.history {
		var s [][]int
		s = append(s, h)
		findDiffs(h, &s)

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

// Name implements aoc.Solution.
func (d *day9) Name() (int, int) {
	return d.year, d.day
}

func Day9(y int, d int) aoc.Solution {
	return &day9{
		year: y,
		day:  d,
	}
}
