package days

import (
	"aoc/aoc"
)

type day3 struct {
	year int
	day  int
}

func (d *day3) Parse() error {
	return nil
}

// First implements aoc.Solution.
func (d *day3) First() ([]string, error) {
	return aoc.Output("unsolved"), nil
}

// Second implements aoc.Solution.
func (d *day3) Second() ([]string, error) {
	return aoc.Output("unsolved"), nil
}

func (d *day3) Name() (int, int) {
	return d.year, d.day
}

func Day3(y int, d int) aoc.Solution {
	day3 := &day3{
		year: y,
		day:  d,
	}

	return day3
}
