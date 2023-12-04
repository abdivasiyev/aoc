package aoc

import (
	"bufio"
	"os"
)

func ReadInput(path string) ([]string, error) {
	var result []string

	f, err := os.Open(path)
	if err != nil {
		return nil, err
	}
	defer f.Close()

	scanner := bufio.NewScanner(f)
	for scanner.Scan() {
		result = append(result, scanner.Text())
	}

	return result, nil
}
