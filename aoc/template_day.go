package aoc

import (
	"fmt"
)

type day struct {
	year        int
	day         int
	linesFirst  []string
	linesSecond []string
}

// First implements aoc.Solution.
func (*day) First() (any, error) {
	return "unsolved", nil
}

// Second implements aoc.Solution.
func (*day) Second() (any, error) {
	return "unsolved", nil
}

// String implements aoc.Solution.
func (d *day) String() string {
	return fmt.Sprintf("***AOC %d***\n___DAY %d___\n", d.year, d.day)
}

func New(y int, d int) Solution {
	day := &day{
		year:        y,
		day:         d,
		linesFirst:  nil,
		linesSecond: nil,
	}

	var err error
	day.linesFirst, err = ReadInput(fmt.Sprintf("./input/%d/%d/1/input.txt", y, d))
	if err != nil {
		panic(err)
	}
	day.linesSecond, err = ReadInput(fmt.Sprintf("./input/%d/%d/2/input.txt", y, d))
	if err != nil {
		panic(err)
	}

	return day
}
