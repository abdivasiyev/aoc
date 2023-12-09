package aoc

import (
	"bufio"
	"errors"
	"os"
)

func ReadInput(path string) ([]string, error) {
	var result []string

	f, err := os.Open(path)
	if err != nil {
		if errors.Is(err, os.ErrNotExist) {
			// create file
			f, err = os.Create(path)
			if err != nil {
				return nil, err
			}
		} else {
			return nil, err
		}
	}
	defer f.Close()

	scanner := bufio.NewScanner(f)
	for scanner.Scan() {
		line := scanner.Text()
		if line == "" {
			continue
		}
		result = append(result, line)
	}

	return result, nil
}
