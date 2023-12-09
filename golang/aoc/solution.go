package aoc

type Solution interface {
	Name() (year int, day int)
	Parse() error
	First() ([]string, error)
	Second() ([]string, error)
}
