package aoc

import (
	"fmt"
	"strings"
	"time"
)

type Mode int

const (
	All Mode = iota
	Last
)

type Runner struct {
	solutions []Solution
	mode      Mode
}

func New(mode Mode) *Runner {
	return &Runner{
		mode: mode,
	}
}

func (r *Runner) Add(s ...Solution) {
	r.solutions = append(r.solutions, s...)
}

// Padding function
func printWithPadding(s []string) string {
	var result strings.Builder

	padding := len(fmt.Sprintf("%d", len(s)))
	for i, e := range s {
		result.WriteString(fmt.Sprintf("%*d: %+v\n", padding, i, e))
	}

	return result.String()
}

func (r *Runner) Run() {
	switch r.mode {
	case All:
		for _, solution := range r.solutions {
			r.run(solution)
		}
	case Last:
		solution := r.solutions[len(r.solutions)-1]
		r.run(solution)
	default:
		panic("unknown mode")
	}

}

func (r *Runner) run(solution Solution) {
	year, day := solution.Name()
	fmt.Printf("Year: %d, Day: %d\n", year, day)
	// Starting timer before Parse
	startParse := time.Now()
	err := solution.Parse()
	if err != nil {
		panic(err)
	}
	// Calculate elapsed time and print
	elapsedParse := time.Since(startParse)
	fmt.Printf("Parse: %v μs\n\n", elapsedParse.Microseconds())

	// Starting timer before First
	startFirst := time.Now()
	firstRes, err := solution.First()
	if err != nil {
		panic(err)
	}
	// Calculate elapsed time and print
	elapsedFirst := time.Since(startFirst)
	fmt.Printf("Part 1 [%v μs]: %v\n", elapsedFirst.Microseconds(), printWithPadding(firstRes))

	// Starting timer before Second
	startSecond := time.Now()
	secondRes, err := solution.Second()
	if err != nil {
		panic(err)
	}
	// Calculate elapsed time and print
	elapsedSecond := time.Since(startSecond)
	fmt.Printf("Part 2 [%v μs]: %v\n", elapsedSecond.Microseconds(), printWithPadding(secondRes))
}
