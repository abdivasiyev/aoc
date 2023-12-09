package days

import (
	"fmt"
	"strconv"
	"strings"

	"aoc/aoc"
)

type day4 struct {
	year        int
	day         int
	linesFirst  []string
	linesSecond []string
}

func (d *day4) Parse() error {
	return nil
}

func (d *day4) parseLine(line string) (id int, winning []int, nums []int) {
	var err error

	if parsed := strings.Split(line, ":"); len(parsed) >= 2 {
		id, err = strconv.Atoi(strings.TrimSpace(strings.TrimLeft(parsed[0], "Card")))
		if err != nil {
			panic(err)
		}

		if parsed := strings.Split(strings.TrimSpace(parsed[1]), "|"); len(parsed) >= 2 {
			for _, str := range strings.Split(strings.TrimSpace(parsed[0]), " ") {
				str = strings.TrimSpace(str)
				if str == "" {
					continue
				}
				n, err := strconv.Atoi(str)
				if err != nil {
					panic(err)
				}
				winning = append(winning, n)
			}

			for _, str := range strings.Split(strings.TrimSpace(parsed[1]), " ") {
				str = strings.TrimSpace(str)
				if str == "" {
					continue
				}
				n, err := strconv.Atoi(str)
				if err != nil {
					panic(err)
				}
				nums = append(nums, n)
			}
		}
	}

	return id, winning, nums
}

// First implements aoc.Solution.
func (d *day4) First() ([]string, error) {
	var total int

	for _, line := range d.linesFirst {
		score := 0
		winningMap := make(map[int]bool)
		_, winning, nums := d.parseLine(line)

		for _, num := range winning {
			winningMap[num] = true
		}

		for _, num := range nums {
			if winningMap[num] {
				if score == 0 {
					score = 1
				} else {
					score *= 2
				}
			}
		}

		total += score
	}

	return aoc.Output(total), nil
}

type card struct {
	id        int
	winning   []int
	nums      []int
	instances int
}

func (c *card) checkWinners() []int {
	var r []int
	m := make(map[int]int)

	for i, num := range c.winning {
		m[num] = i
	}

	for _, num := range c.nums {
		if i, ok := m[num]; ok {
			r = append(r, i+1)
		}
	}

	return r
}

// Second implements aoc.Solution.
func (d *day4) Second() ([]string, error) {
	var (
		total int
		cards []*card
		stack []*card
	)

	for _, line := range d.linesSecond {
		id, winning, nums := d.parseLine(line)
		c := card{
			id:        id,
			winning:   winning,
			nums:      nums,
			instances: 1,
		}
		cards = append(cards, &c)
	}

	for i, c := range cards {
		winners := c.checkWinners()

		for j := i + 1; j < i+len(winners)+1; j++ {
			if j >= len(cards) {
				break
			}
			cards[j].instances++
			stack = append(stack, cards[j])
		}

		for len(stack) > 0 {
			last := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			winners = last.checkWinners()
			for j := last.id; j < last.id+len(winners); j++ {
				if j >= len(cards) {
					break
				}
				cards[j].instances++
				stack = append(stack, cards[j])
			}
		}
	}

	for _, c := range cards {
		total += c.instances
	}

	return aoc.Output(total), nil
}

// Name implements aoc.Solution.
func (d *day4) Name() (int, int) {
	return d.year, d.day
}
func Day4(y int, d int) aoc.Solution {
	day := &day4{
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
