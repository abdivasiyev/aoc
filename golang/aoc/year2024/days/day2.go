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
	if len(nums) == 0 {
		return false
	}

	var (
		prev       = nums[0]
		isNegative bool
	)

	for i := 1; i < len(nums); i++ {
		diff := aoc.Abs(nums[i] - prev)
		if diff < 1 || diff > 3 {
			return false
		}
		if i == 1 {
			isNegative = nums[i]-prev < 0
		}
		if nums[i]-prev < 0 != isNegative {
			return false
		}
		prev = nums[i]
	}

	return true
}

// Second implements aoc.Solution.
func (d *day2) Second() ([]string, error) {
	var safeReports int

	for _, line := range d.linesSecond {
		var (
			nums = aoc.Collect(line, " ")
		)

		if isSingleSafe(nums) {
			safeReports++
		}
		// fmt.Println("========================")
	}

	return aoc.Output(safeReports), nil
}

func isSingleSafe(nums []int) bool {
	if isSafe(nums) {
		return true
	}

	for i := 0; i < len(nums); i++ {
		newNums := aoc.CopyExcept(nums, i)
		// fmt.Println("newNums", newNums, "nums", nums)
		if isSafe(newNums) {
			return true
		}
	}

	return false
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
	day2.linesFirst, err = aoc.ReadInput(fmt.Sprintf("../input/%d/%d/input.txt", y, d))
	if err != nil {
		panic(err)
	}
	day2.linesSecond, err = aoc.ReadInput(fmt.Sprintf("../input/%d/%d/input.txt", y, d))
	if err != nil {
		panic(err)
	}

	return day2
}
