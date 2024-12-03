package days

import (
	"fmt"

	"aoc/aoc"
)

type day2 struct {
	year        int
	day         int
	linesFirst  []string
	linesSecond []string
}

func (d *day2) Parse() error {
	return nil
}

// First implements aoc.Solution.
func (d *day2) First() ([]string, error) {
	var safeReports int

	for _, line := range d.linesFirst {
		var nums = aoc.Collect(line, " ")

		if isSafe(nums) {
			safeReports++
		}
	}

	return aoc.Output(safeReports), nil
}

func isSafe(nums []int) bool {
	return isMonothonic(nums, func(a, b int) bool {
		return a > b && a-b >= 1 && a-b <= 3
	}) || isMonothonic(nums, func(a, b int) bool {
		return a < b && b-a >= 1 && b-a <= 3
	})
}

func isMonothonic(nums []int, fn func(int, int) bool) bool {
	for i := len(nums) - 1; i > 0; i-- {
		if !fn(nums[i], nums[i-1]) {
			return false
		}
	}

	return true
}

// Second implements aoc.Solution.
func (d *day2) Second() ([]string, error) {
	var safeReports int

	for _, line := range d.linesSecond {
		var nums = aoc.Collect(line, " ")

		if isSafe(nums) {
			safeReports++
			continue
		}

		for i := 0; i < len(nums); i++ {
			subNums := aoc.CopyExcept(nums, i)

			if isSafe(subNums) {
				safeReports++
				break
			}
		}
	}

	return aoc.Output(safeReports), nil
}

func (d *day2) Name() (int, int) {
	return d.year, d.day
}

func Day2(y int, d int) aoc.Solution {
	day2 := &day2{
		year:        y,
		day:         d,
		linesFirst:  nil,
		linesSecond: nil,
	}

	var err error
	day2.linesFirst, err = aoc.ReadInput(fmt.Sprintf("input/%d/%d/input.txt", y, d))
	if err != nil {
		panic(err)
	}
	day2.linesSecond, err = aoc.ReadInput(fmt.Sprintf("input/%d/%d/input.txt", y, d))
	if err != nil {
		panic(err)
	}

	return day2
}
