package days

import (
	"fmt"
	"strconv"
	"strings"

	"aoc/aoc"
)

type day3 struct {
	year        int
	day         int
	linesFirst  []string
	linesSecond []string
}

func (d *day3) isSymbol(ch byte) bool {
	if ch >= '0' && ch <= '9' {
		return false
	}

	return ch != '.'
}

func (d *day3) isDigit(ch byte) bool {
	return ch >= '0' && ch <= '9'
}

func (d *day3) isAdjacent(x, y int, lines []string) bool {
	width := len(lines)
	height := len(lines[0])

	if x-1 >= 0 && d.isSymbol(lines[y][x-1]) {
		return true
	}

	if x+1 < width && d.isSymbol(lines[y][x+1]) {
		return true
	}

	if y-1 >= 0 && d.isSymbol(lines[y-1][x]) {
		return true
	}

	if y+1 < height && d.isSymbol(lines[y+1][x]) {
		return true
	}

	if x-1 >= 0 && y-1 >= 0 && d.isSymbol(lines[y-1][x-1]) {
		return true
	}

	if x+1 < width && y-1 >= 0 && d.isSymbol(lines[y-1][x+1]) {
		return true
	}

	if x-1 >= 0 && y+1 < height && d.isSymbol(lines[y+1][x-1]) {
		return true
	}

	if x+1 < width && y+1 < height && d.isSymbol(lines[y+1][x+1]) {
		return true
	}

	return false
}

// First implements aoc.Solution.
func (d *day3) First() ([]string, error) {
	var total int

	for y, line := range d.linesFirst {
		var (
			num        int
			isAdjacent bool
		)
		for x, ch := range line {
			n, err := strconv.Atoi(string(ch))
			if err != nil {
				if isAdjacent {
					total += num
					isAdjacent = false
					num = 0
				} else {
					num = 0
				}
				continue
			}

			num = num*10 + n

			if d.isAdjacent(x, y, d.linesFirst) {
				isAdjacent = true
			}
		}
		if isAdjacent {
			total += num
		}
	}

	return aoc.Output(total), nil
}

func (d *day3) findStartAndEnd(x int, width int, line string) (int, int) {
	var (
		start = x
		end   = x
	)

	for start-1 >= 0 && d.isDigit(line[start-1]) {
		start--
	}

	for end+1 < width && d.isDigit(line[end+1]) {
		end++
	}

	return start, end
}

func (d *day3) findAdjacents(x int, y int, lines []string, used [][]bool) (int, int) {
	var numStr strings.Builder

	width := len(lines)
	height := len(lines[0])
	num1, num2 := 0, 0

	// left side
	if x-1 >= 0 && d.isDigit(lines[y][x-1]) {
		start, end := d.findStartAndEnd(x-1, width, lines[y])
		for x2 := start; x2 <= end; x2++ {
			if used[y][x2] {
				break
			}
			numStr.WriteByte(lines[y][x2])
			used[y][x2] = true
		}

		num1, _ = strconv.Atoi(numStr.String())
		numStr.Reset()
	}

	// right side
	if x+1 < width && d.isDigit(lines[y][x+1]) {
		start, end := d.findStartAndEnd(x+1, width, lines[y])
		for x2 := start; x2 <= end; x2++ {
			if used[y][x2] {
				break
			}
			numStr.WriteByte(lines[y][x2])
			used[y][x2] = true
		}
		if num1 == 0 {
			num1, _ = strconv.Atoi(numStr.String())
		} else if num2 == 0 {
			num2, _ = strconv.Atoi(numStr.String())
		}
		numStr.Reset()
	}

	// top side
	if y-1 >= 0 && d.isDigit(lines[y-1][x]) {
		start, end := d.findStartAndEnd(x, width, lines[y-1])
		for x2 := start; x2 <= end; x2++ {
			if used[y-1][x2] {
				break
			}
			numStr.WriteByte(lines[y-1][x2])
			used[y-1][x2] = true
		}
		if num1 == 0 {
			num1, _ = strconv.Atoi(numStr.String())
		} else if num2 == 0 {
			num2, _ = strconv.Atoi(numStr.String())
		}
		numStr.Reset()
	}

	// bottom side
	if y+1 < height && d.isDigit(lines[y+1][x]) {
		start, end := d.findStartAndEnd(x, width, lines[y+1])
		for x2 := start; x2 <= end; x2++ {
			if used[y+1][x2] {
				break
			}
			numStr.WriteByte(lines[y+1][x2])
			used[y+1][x2] = true
		}
		if num1 == 0 {
			num1, _ = strconv.Atoi(numStr.String())
		} else if num2 == 0 {
			num2, _ = strconv.Atoi(numStr.String())
		}
		numStr.Reset()
	}

	// top left corner
	if x-1 >= 0 && y-1 >= 0 && d.isDigit(lines[y-1][x-1]) && !used[y-1][x-1] {
		start, end := d.findStartAndEnd(x-1, width, lines[y-1])
		for x2 := start; x2 <= end; x2++ {
			if used[y-1][x2] {
				break
			}
			numStr.WriteByte(lines[y-1][x2])
			used[y-1][x2] = true
		}
		if num1 == 0 {
			num1, _ = strconv.Atoi(numStr.String())
		} else if num2 == 0 {
			num2, _ = strconv.Atoi(numStr.String())
		}
		numStr.Reset()
	}

	// top right corner
	if x+1 < width && y-1 >= 0 && d.isDigit(lines[y-1][x+1]) && !used[y-1][x+1] {
		start, end := d.findStartAndEnd(x+1, width, lines[y-1])
		for x2 := start; x2 <= end; x2++ {
			if used[y-1][x2] {
				break
			}
			numStr.WriteByte(lines[y-1][x2])
			used[y-1][x2] = true
		}
		if num1 == 0 {
			num1, _ = strconv.Atoi(numStr.String())
		} else if num2 == 0 {
			num2, _ = strconv.Atoi(numStr.String())
		}
		numStr.Reset()
	}

	// bottom left corner
	if x-1 >= 0 && y+1 >= 0 && d.isDigit(lines[y+1][x-1]) && !used[y+1][x-1] {
		start, end := d.findStartAndEnd(x-1, width, lines[y+1])
		for x2 := start; x2 <= end; x2++ {
			if used[y+1][x2] {
				break
			}
			numStr.WriteByte(lines[y+1][x2])
			used[y+1][x2] = true
		}
		if num1 == 0 {
			num1, _ = strconv.Atoi(numStr.String())
		} else if num2 == 0 {
			num2, _ = strconv.Atoi(numStr.String())
		}
		numStr.Reset()
	}

	// bottom right corner
	if x+1 < width && y+1 < height && d.isDigit(lines[y+1][x+1]) && !used[y+1][x+1] {
		start, end := d.findStartAndEnd(x+1, width, lines[y+1])
		for x2 := start; x2 <= end; x2++ {
			if used[y+1][x2] {
				break
			}
			numStr.WriteByte(lines[y+1][x2])
			used[y+1][x2] = true
		}
		if num1 == 0 {
			num1, _ = strconv.Atoi(numStr.String())
		} else if num2 == 0 {
			num2, _ = strconv.Atoi(numStr.String())
		}
		numStr.Reset()
	}

	return num1, num2
}

// Second implements aoc.Solution.
func (d *day3) Second() ([]string, error) {
	var (
		total  int
		width  = len(d.linesSecond)
		height = len(d.linesSecond[0])
		used   = make([][]bool, height)
	)

	for i := 0; i < height; i++ {
		used[i] = make([]bool, width)
	}

	for y, line := range d.linesSecond {
		for x, ch := range line {
			if ch != '*' {
				continue
			}
			num1, num2 := d.findAdjacents(x, y, d.linesSecond, used)
			if num1 == 0 || num2 == 0 {
				continue
			}

			fmt.Println(num1, num2)

			total += num1 * num2
		}
	}

	return aoc.Output(total), nil
}

// String implements aoc.Solution.
func (d *day3) String() string {
	return fmt.Sprintf("***AOC %d***\n___DAY %d___\n", d.year, d.day)
}

func Day3(y int, d int) aoc.Solution {
	day := &day3{
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
