package aoc

import "fmt"

type Solution interface {
	fmt.Stringer
	First() (any, error)
	Second() (any, error)
}
