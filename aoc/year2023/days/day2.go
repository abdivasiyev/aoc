package days

import (
	"aoc/aoc"
	"fmt"
	"slices"
	"strconv"
	"strings"
)

type day2 struct {
	year        int
	day         int
	linesFirst  []string
	linesSecond []string
}

func (d *day2) parseLine(s string) (id int, reds []int, greens []int, blues []int) {
	var (
		r   int
		err error
	)

	parsed := strings.Split(s, ":")

	if len(parsed) > 0 {
		id, err = strconv.Atoi(strings.TrimLeft(parsed[0], "Game "))

		if err != nil {
			panic(err)
		}

		for _, parsedAttempt := range strings.Split(parsed[1], ";") {
			for _, parsedColor := range strings.Split(strings.TrimSpace(parsedAttempt), ",") {
				var color string
				c := strings.TrimSpace(parsedColor)

				switch {
				case strings.Contains(c, "red"):
					color = "red"
				case strings.Contains(c, "green"):
					color = "green"
				case strings.Contains(c, "blue"):
					color = "blue"
				default:
					panic(err)
				}
				r, err = strconv.Atoi(strings.TrimSpace(strings.TrimRight(c, color)))
				if err != nil {
					panic(err)
				}

				switch color {
				case "red":
					reds = append(reds, r)
				case "green":
					greens = append(greens, r)
				case "blue":
					blues = append(blues, r)
				}
			}
		}
	}

	return
}

// First implements aoc.Solution.
func (d *day2) First() (any, error) {
	maxReds, maxGreens, maxBlues := 12, 13, 14
	total := 0

outer:
	for _, line := range d.linesFirst {
		id, reds, greens, blues := d.parseLine(line)
		for _, red := range reds {
			if red > maxReds {
				continue outer
			}
		}
		for _, green := range greens {
			if green > maxGreens {
				continue outer
			}
		}
		for _, blue := range blues {
			if blue > maxBlues {
				continue outer
			}
		}
		total += id
	}

	return total, nil
}

// Second implements aoc.Solution.
func (d *day2) Second() (any, error) {
	total := 0

	for _, line := range d.linesSecond {
		_, reds, greens, blues := d.parseLine(line)
		maxRed := slices.Max(reds)
		maxGreen := slices.Max(greens)
		maxBlue := slices.Max(blues)

		total += maxRed * maxGreen * maxBlue
	}

	return total, nil
}

// String implements aoc.Solution.
func (d *day2) String() string {
	return fmt.Sprintf("***AOC %d***\n___DAY %d___\n", d.year, d.day)
}

func Day2(y int, d int) aoc.Solution {
	day := &day2{
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
