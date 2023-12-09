package days

import (
	"fmt"
	"math"
	"strings"

	"aoc/aoc"
)

type day5 struct {
	year        int
	day         int
	linesFirst  []string
	linesSecond []string
}

func (d *day5) Parse() error {
	return nil
}

type singleMap struct {
	start int
	end   int
	delta int
}

type mapping struct {
	m []singleMap
}

func (m *mapping) add(dest int, src int, size int) {
	m.m = append(m.m, singleMap{
		start: src,
		end:   src + size,
		delta: dest - src,
	})
}

func (m *mapping) apply(val int) int {
	for _, sm := range m.m {
		if sm.start <= val && val <= sm.end {
			return val + sm.delta
		}
	}

	return val
}

func (d *day5) parse(lines []string) ([]int, []mapping) {
	var maps []mapping
	seeds := aoc.Collect(strings.Split(lines[0], ": ")[1], " ")
	currMap := mapping{}

	for i := 1; i < len(lines); i++ {
		if strings.Contains(lines[i], ":") {
			maps = append(maps, currMap)
			currMap = mapping{}
			continue
		}
		nums := aoc.Collect(lines[i], " ")
		currMap.add(nums[0], nums[1], nums[2])
	}

	if len(currMap.m) > 0 {
		maps = append(maps, currMap)
	}

	return seeds, maps
}

// First implements aoc.Solution.
func (d *day5) First() ([]string, error) {
	seeds, maps := d.parse(d.linesFirst)
	minLoc := math.MaxInt

	for _, seed := range seeds {
		cur := seed
		for _, m := range maps {
			cur = m.apply(cur)
		}
		minLoc = min(minLoc, cur)
	}

	return aoc.Output(minLoc), nil
}

// Second implements aoc.Solution.
func (d *day5) Second() ([]string, error) {
	seeds, maps := d.parse(d.linesSecond)
	minLoc := math.MaxInt

	// 23926672
	// 11611182

	for i := 0; i <= len(seeds)-2; i += 2 {
		for seed := seeds[i]; seed < seeds[i]+seeds[i+1]; seed++ {
			cur := seed
			for _, m := range maps {
				cur = m.apply(cur)
			}
			minLoc = min(minLoc, cur)
		}
	}

	return aoc.Output(minLoc), nil
}

// Name implements aoc.Solution.
func (d *day5) Name() (int, int) {
	return d.year, d.day
}
func Day5(y int, d int) aoc.Solution {
	day := &day5{
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
