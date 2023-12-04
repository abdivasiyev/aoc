package aoc

import "fmt"

type Solution interface {
	fmt.Stringer
	First() ([]string, error)
	Second() ([]string, error)
}
