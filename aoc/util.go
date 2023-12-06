package aoc

import (
	"strconv"
	"strings"
)

func Reverse(s string) string {
	r := []rune(s)
	for i, j := 0, len(r)-1; i < len(r)/2; i, j = i+1, j-1 {
		r[i], r[j] = r[j], r[i]
	}
	return string(r)
}

func Collect(line string, sep string) []int {
	numsStr := strings.Split(line, sep)
	nums := make([]int, len(numsStr))

	for i, numStr := range numsStr {
		nums[i], _ = strconv.Atoi(numStr)
	}

	return nums
}
