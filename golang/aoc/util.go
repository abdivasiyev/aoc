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
	var nums []int
	numsStr := strings.Split(line, sep)

	for _, numStr := range numsStr {
		numStr = strings.TrimSpace(numStr)
		if numStr == "" {
			continue
		}
		n, _ := strconv.Atoi(numStr)
		nums = append(nums, n)
	}

	return nums
}

func GCD[T int | int64 | int32](a, b T) T {
	for b != 0 {
		t := b
		b = a % b
		a = t
	}
	return a
}

func LCM[T int | int64 | int32](a, b T, integers ...T) T {
	result := a * b / GCD(a, b)

	for i := 0; i < len(integers); i++ {
		result = LCM(result, integers[i])
	}

	return result
}

func Abs(n int) int {
	if n < 0 {
		return -n
	}

	return n
}

func CopyExcept[T any](slice []T, i int) []T {
	s := make([]T, 0, len(slice)-1)
	s = append(s, slice[:i]...)
	s = append(s, slice[i+1:]...)
	return s
}
