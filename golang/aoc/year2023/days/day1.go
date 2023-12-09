package days

import (
	"fmt"
	"strconv"

	"aoc/aoc"
)

type day1 struct {
	year        int
	day         int
	linesFirst  []string
	linesSecond []string
}

// First implements aoc.Solution.
func (d *day1) First() ([]string, error) {
	var total int

	for _, line := range d.linesFirst {
		var nums []int
		for _, ch := range line {
			n, err := strconv.Atoi(string(ch))
			if err != nil {
				continue
			}
			nums = append(nums, n)
		}

		total += nums[0]*10 + nums[len(nums)-1]
	}

	return aoc.Output(total), nil
}

// Second implements aoc.Solution.
func (d *day1) Second() ([]string, error) {
	var (
		total        int
		spelled_nums = []string{
			"one", "1",
			"two", "2",
			"three", "3",
			"four", "4",
			"five", "5",
			"six", "6",
			"seven", "7",
			"eight", "8",
			"nine", "9",
		}
	)

	for _, line := range d.linesSecond {
		var number int

	outFirst:
		for i := range line {
			for j, num := range spelled_nums {
				if i+len(num) >= len(line) {
					continue
				}

				found := line[i : i+len(num)]
				if found == num {
					number = number*10 + (j/2 + 1)
					break outFirst
				}
			}
		}

	outSecond:
		for i := len(line); i >= 0; i-- {
			for j, num := range spelled_nums {
				if i+len(num) > len(line) {
					continue
				}

				found := line[i : i+len(num)]
				if found == num {
					number = number*10 + (j/2 + 1)
					break outSecond
				}
			}
		}

		if number < 10 {
			number = number*10 + number
		}

		total += number
	}

	return aoc.Output(total), nil
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
	day1.linesFirst, err = aoc.ReadInput(fmt.Sprintf("./input/%d/%d/1/input.txt", y, d))
	if err != nil {
		panic(err)
	}
	day1.linesSecond, err = aoc.ReadInput(fmt.Sprintf("./input/%d/%d/2/input.txt", y, d))
	if err != nil {
		panic(err)
	}

	return day1
}
