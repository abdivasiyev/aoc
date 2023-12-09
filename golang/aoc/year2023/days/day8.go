package days

import (
	"fmt"
	"strings"

	"aoc/aoc"
)

type day8 struct {
	year        int
	day         int
	linesFirst  []string
	linesSecond []string
}

type networkNode struct {
	value string
	left  string
	right string
}

// First implements aoc.Solution.
func (d *day8) First() ([]string, error) {
	instructions := []byte(d.linesFirst[0])
	network := make(map[string]networkNode)

	for i := 1; i < len(d.linesFirst); i++ {
		args := strings.Split(d.linesFirst[i], "=")

		currentNode := strings.TrimSpace(args[0])

		args[1] = strings.TrimSpace(args[1])
		args[1] = strings.TrimLeft(args[1], "(")
		args[1] = strings.TrimRight(args[1], ")")

		leftAndRight := strings.Split(args[1], ",")
		network[currentNode] = networkNode{
			value: currentNode,
			left:  leftAndRight[0],
			right: strings.TrimSpace(leftAndRight[1]),
		}
	}

	currentInstruction := 0
	steps := 0
	node := network["AAA"]

	for {
		currentInstruction %= len(instructions)

		if node.value == "ZZZ" {
			break
		}

		switch instructions[currentInstruction] {
		case 'L':
			node = network[node.left]
		case 'R':
			node = network[node.right]
		}

		steps++
		currentInstruction++
	}

	return aoc.Output(steps), nil
}

// Second implements aoc.Solution.
func (d *day8) Second() ([]string, error) {
	var nodes []networkNode
	instructions := []byte(d.linesSecond[0])
	network := make(map[string]networkNode)

	for i := 1; i < len(d.linesSecond); i++ {
		args := strings.Split(d.linesSecond[i], "=")

		currentNode := strings.TrimSpace(args[0])

		args[1] = strings.TrimSpace(args[1])
		args[1] = strings.TrimLeft(args[1], "(")
		args[1] = strings.TrimRight(args[1], ")")

		leftAndRight := strings.Split(args[1], ",")
		network[currentNode] = networkNode{
			value: currentNode,
			left:  leftAndRight[0],
			right: strings.TrimSpace(leftAndRight[1]),
		}

		if currentNode[len(currentNode)-1] == 'A' {
			nodes = append(nodes, networkNode{
				value: currentNode,
				left:  leftAndRight[0],
				right: strings.TrimSpace(leftAndRight[1]),
			})
		}
	}

	var (
		allSteps int64 = 1
	)

	for _, node := range nodes {
		var (
			steps              int64
			currentInstruction int
			currNode           = node
		)
		for last := currNode.value[len(currNode.value)-1]; last != 'Z'; last = currNode.value[len(currNode.value)-1] {
			currentInstruction %= len(instructions)
			switch instructions[currentInstruction] {
			case 'L':
				currNode = network[currNode.left]
			case 'R':
				currNode = network[currNode.right]
			}
			currentInstruction++
			steps++
		}

		fmt.Println(node, steps)

		allSteps = aoc.LCM(allSteps, steps)
	}

	return aoc.Output(allSteps), nil
}

// Name implements aoc.Solution.
func (d *day8) Name() (int, int) {
	return d.year, d.day
}

func Day8(y int, d int) aoc.Solution {
	day := &day8{
		year:        y,
		day:         d,
		linesFirst:  nil,
		linesSecond: nil,
	}

	var err error
	day.linesFirst, err = aoc.ReadInput(fmt.Sprintf("./input/%d/%d/1/input.txt", y, d))
	if err != nil {
		panic(err)
	}
	day.linesSecond, err = aoc.ReadInput(fmt.Sprintf("./input/%d/%d/2/input.txt", y, d))
	if err != nil {
		panic(err)
	}

	return day
}
