package days

import (
	"aoc/aoc"
)

type day struct {
	year int
	day  int
}

func (d *day) Parse() error {
	return nil
}

// First implements aoc.Solution.
func (d *day) First() ([]string, error) {
	return aoc.Output("unsolved"), nil
}

// Second implements aoc.Solution.
func (d *day) Second() ([]string, error) {
	return aoc.Output("unsolved"), nil
}

func (d *day) Name() (int, int) {
	return d.year, d.day
}

func Day(y int, d int) aoc.Solution {
	day := &day{
		year: y,
		day:  d,
	}

	return day
}
