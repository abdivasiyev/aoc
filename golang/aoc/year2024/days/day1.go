package days

import (
	"fmt"
	"slices"
	"strconv"
	"strings"

	"aoc/aoc"
)

type day1 struct {
	year        int
	day         int
	linesFirst  []string
	linesSecond []string
}

func (d *day1) Parse() error {
	return nil
}

// First implements aoc.Solution.
func (d *day1) First() ([]string, error) {
	var left, right []int

	for _, line := range d.linesFirst {
		var nums = strings.Split(line, " ")

		l, err := strconv.Atoi(nums[0])
		if err != nil {
			panic(err)
		}

		r, err := strconv.Atoi(nums[len(nums)-1])
		if err != nil {
			panic(err)
		}

		left = append(left, l)
		right = append(right, r)
	}

	slices.Sort(left)
	slices.Sort(right)

	var totalDistance int

	for i := 0; i < len(left); i++ {
		totalDistance += aoc.Abs(right[i] - left[i])
	}

	return aoc.Output(totalDistance), nil
}

// Second implements aoc.Solution.
func (d *day1) Second() ([]string, error) {
	var left, right []int

	for _, line := range d.linesFirst {
		var nums = strings.Split(line, " ")

		l, err := strconv.Atoi(nums[0])
		if err != nil {
			panic(err)
		}

		r, err := strconv.Atoi(nums[len(nums)-1])
		if err != nil {
			panic(err)
		}

		left = append(left, l)
		right = append(right, r)
	}

	var (
		freq            = make(map[int]int)
		similarityScore int
	)

	for i := range right {
		freq[right[i]]++
	}

	for i := range left {
		similarityScore += left[i] * freq[left[i]]
	}

	return aoc.Output(similarityScore), nil
}

func (d *day1) Name() (int, int) {
	return d.year, d.day
}

func Day1(y int, d int) aoc.Solution {
	day1 := &day1{
		year:        y,
		day:         d,
		linesFirst:  nil,
		linesSecond: nil,
	}

	var err error
	day1.linesFirst, err = aoc.ReadInput(fmt.Sprintf("input/%d_%d.txt", y, d))
	if err != nil {
		panic(err)
	}
	day1.linesSecond, err = aoc.ReadInput(fmt.Sprintf("input/%d_%d.txt", y, d))
	if err != nil {
		panic(err)
	}

	return day1
}
