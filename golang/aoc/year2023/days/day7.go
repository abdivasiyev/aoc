package days

import (
	"fmt"
	"math"
	"slices"
	"sort"
	"strconv"
	"strings"

	"aoc/aoc"
)

type day7 struct {
	year        int
	day         int
	linesFirst  []string
	linesSecond []string
}

func (d *day7) Parse() error {
	return nil
}

type hand struct {
	cards    string
	handType int
	bid      int
}

func (h *hand) setWithJokerType() {
	m := make([]int, 85)
	cardTypes := []byte{'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'}
	for i := range h.cards {
		m[h.cards[i]]++
	}

	maxElement := math.MinInt
	maxIndex := -1
	for i, cnt := range m {
		if cnt > maxElement && i != 'J' {
			maxElement = cnt
			maxIndex = i
		} else if maxIndex > -1 && cnt == maxElement {
			first := slices.Index(cardTypes, byte(maxIndex))
			second := slices.Index(cardTypes, byte(i))

			if second < first {
				maxIndex = i
			}
		}
	}

	m[maxIndex] += m['J']
	m['J'] = 0

	sort.Slice(m, func(i, j int) bool {
		return m[i] > m[j]
	})

	if m[0] == 5 {
		h.handType = 7
	} else if m[0] == 4 {
		h.handType = 6
	} else if m[0] == 3 && m[1] == 2 {
		h.handType = 5
	} else if m[0] == 3 && m[1] < 2 {
		h.handType = 4
	} else if m[0] == 2 && m[1] == 2 {
		h.handType = 3
	} else if m[0] == 2 && m[1] < 2 {
		h.handType = 2
	} else {
		h.handType = 1
	}
}

func (h *hand) setType() {
	m := make([]int, 85)
	for i := range h.cards {
		m[h.cards[i]]++
	}

	sort.Slice(m, func(i, j int) bool {
		return m[i] > m[j]
	})

	if m[0] == 5 {
		h.handType = 7
	} else if m[0] == 4 {
		h.handType = 6
	} else if m[0] == 3 && m[1] == 2 {
		h.handType = 5
	} else if m[0] == 3 && m[1] < 2 {
		h.handType = 4
	} else if m[0] == 2 && m[1] == 2 {
		h.handType = 3
	} else if m[0] == 2 && m[1] < 2 {
		h.handType = 2
	} else {
		h.handType = 1
	}
}

// First implements aoc.Solution.
func (d *day7) First() ([]string, error) {
	var (
		cardTypes = []byte{'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'}
		hands     []hand
		total     int
	)

	//fmt.Println(cardTypes)

	for _, line := range d.linesFirst {
		parsed := strings.Split(line, " ")
		if len(parsed) != 2 {
			continue
		}
		cards, bid := parsed[0], parsed[1]
		b, _ := strconv.Atoi(bid)

		h := hand{
			cards: cards,
			bid:   b,
		}

		h.setType()

		hands = append(hands, h)
	}

	sort.Slice(hands, func(i, j int) bool {
		if hands[i].handType != hands[j].handType {
			return hands[i].handType < hands[j].handType
		}

		for k := 0; k < len(hands[i].cards); k++ {
			if hands[i].cards[k] == hands[j].cards[k] {
				continue
			}

			first := slices.Index(cardTypes, hands[i].cards[k])
			second := slices.Index(cardTypes, hands[j].cards[k])
			return first > second
		}

		return false
	})

	for i, h := range hands {
		total += (i + 1) * h.bid
	}

	return aoc.Output(total), nil
}

// Second implements aoc.Solution.
func (d *day7) Second() ([]string, error) {
	var (
		cardTypes = []byte{'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'}
		hands     []hand
		total     int
	)

	//fmt.Println(cardTypes)

	for _, line := range d.linesSecond {
		parsed := strings.Split(line, " ")
		if len(parsed) != 2 {
			continue
		}
		cards, bid := parsed[0], parsed[1]
		b, _ := strconv.Atoi(bid)

		h := hand{
			cards: cards,
			bid:   b,
		}

		h.setWithJokerType()

		hands = append(hands, h)
	}

	sort.Slice(hands, func(i, j int) bool {
		if hands[i].handType != hands[j].handType {
			return hands[i].handType < hands[j].handType
		}

		for k := 0; k < len(hands[i].cards); k++ {
			if hands[i].cards[k] == hands[j].cards[k] {
				continue
			}

			first := slices.Index(cardTypes, hands[i].cards[k])
			second := slices.Index(cardTypes, hands[j].cards[k])
			return first > second
		}

		return false
	})

	for i, h := range hands {
		fmt.Println(h.cards, h.handType, i+1)
		total += (i + 1) * h.bid
	}

	return aoc.Output(total), nil
}

// Name implements aoc.Solution.
func (d *day7) Name() (int, int) {
	return d.year, d.day
}
func Day7(y int, d int) aoc.Solution {
	day := &day7{
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
