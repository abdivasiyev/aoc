package aoc

type Solution interface {
	Name() (year int, day int)
	First() ([]string, error)
	Second() ([]string, error)
}
