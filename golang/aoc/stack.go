package aoc

import "fmt"

type Stack[T any] struct {
	items []T
}

func NewStack[T any]() *Stack[T] {
	return &Stack[T]{}
}

func (s *Stack[T]) Push(item T) {
	s.items = append(s.items, item)
}

func (s *Stack[T]) Pop() T {
	n := len(s.items)
	item := s.items[n-1]

	s.items = s.items[:n-1]
	return item
}

func (s *Stack[T]) Peek() T {
	n := len(s.items)
	return s.items[n-1]
}

func (s *Stack[T]) Empty() bool {
	return len(s.items) == 0
}

func (s *Stack[T]) Size() int {
	return len(s.items)
}

func (s *Stack[T]) Print(prefix string) {
	fmt.Printf("%s: ", prefix)
	for i := len(s.items) - 1; i >= 0; i-- {
		fmt.Printf("%v ", s.items[i])
	}
	fmt.Println()
}
