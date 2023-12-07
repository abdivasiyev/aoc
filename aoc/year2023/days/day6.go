package days

import (
	"fmt"
	"math"
	"strconv"
	"strings"

	"aoc/aoc"
)

type day6 struct {
	year        int
	day         int
	linesFirst  []string
	linesSecond []string
}

func (d *day6) parse(lines []string) ([]int, []int) {
	if len(lines) < 2 {
		return nil, nil
	}

	times := aoc.Collect(strings.TrimSpace(strings.TrimLeft(lines[0], "Time:")), " ")
	distances := aoc.Collect(strings.TrimSpace(strings.TrimLeft(lines[1], "Distance:")), " ")

	return times, distances
}

// First implements aoc.Solution.
func (d *day6) First() ([]string, error) {
	times, distances := d.parse(d.linesFirst)
	multiplied := 1

	// time - t
	// distance - l
	// hold_time - x
	// movement_time - t-x
	// (t-x)*x > l
	// -x^2 + tx - l > 0
	// x1 = (-t+sqrt(t^2-4*(-1)*(-l)) / 2*(-1)
	// x2 = (-t-sqrt(t^2-4*(-1)*(-l)) / 2*(-1)

	for i := 0; i < len(times); i++ {
		t := float64(times[i])
		l := float64(distances[i])

		x1 := math.Ceil((-t + math.Sqrt(t*t-4*(-1)*(-l))) / (2 * -1))
		x2 := math.Floor((-t - math.Sqrt(t*t-4*(-1)*(-l))) / (2 * -1))

		if (t-x1)*x1 == l {
			x1++
		}

		if (t-x2)*x2 == l {
			x2--
		}

		multiplied *= int((x2 - x1) + 1)
	}

	return aoc.Output(multiplied), nil
}

// Second implements aoc.Solution.
func (d *day6) Second() ([]string, error) {
	times, distances := d.parse(d.linesSecond)
	multiplied := 1

	var (
		timeStr     strings.Builder
		distanceStr strings.Builder
	)

	for _, t := range times {
		timeStr.WriteString(fmt.Sprintf("%d", t))
	}

	for _, dst := range distances {
		distanceStr.WriteString(fmt.Sprintf("%d", dst))
	}

	time, _ := strconv.Atoi(timeStr.String())
	distance, _ := strconv.Atoi(distanceStr.String())

	t := float64(time)
	l := float64(distance)

	x1 := math.Ceil((-t + math.Sqrt(t*t-4*(-1)*(-l))) / (2 * -1))
	x2 := math.Floor((-t - math.Sqrt(t*t-4*(-1)*(-l))) / (2 * -1))

	if (t-x1)*x1 == l {
		x1++
	}

	if (t-x2)*x2 == l {
		x2--
	}

	multiplied *= int((x2 - x1) + 1)

	return aoc.Output(multiplied), nil
}

// String implements aoc.Solution.
func (d *day6) String() string {
	return fmt.Sprintf("***AOC %d***\n___DAY %d___\n", d.year, d.day)
}

func Day6(y int, d int) aoc.Solution {
	day := &day6{
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
