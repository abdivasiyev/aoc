package aoc

import "fmt"

func Output(args ...any) []string {
	result := make([]string, len(args))

	for i := range args {
		result[i] = fmt.Sprint(args[i])
	}

	return result
}
