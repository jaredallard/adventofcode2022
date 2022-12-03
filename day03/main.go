package main

import (
	"bufio"
	"fmt"
	"os"
)

// pointMap contains a map of runes to their
// point (priotity) value
var pointMap = map[rune]uint{
	'a': 1,
	'b': 2,
	'c': 3,
	'd': 4,
	'e': 5,
	'f': 6,
	'g': 7,
	'h': 8,
	'i': 9,
	'j': 10,
	'k': 11,
	'l': 12,
	'm': 13,
	'n': 14,
	'o': 15,
	'p': 16,
	'q': 17,
	'r': 18,
	's': 19,
	't': 20,
	'u': 21,
	'v': 22,
	'w': 23,
	'x': 24,
	'y': 25,
	'z': 26,
	'A': 27,
	'B': 28,
	'C': 29,
	'D': 30,
	'E': 31,
	'F': 32,
	'G': 33,
	'H': 34,
	'I': 35,
	'J': 36,
	'K': 37,
	'L': 38,
	'M': 39,
	'N': 40,
	'O': 41,
	'P': 42,
	'Q': 43,
	'R': 44,
	'S': 45,
	'T': 46,
	'U': 47,
	'V': 48,
	'W': 49,
	'X': 50,
	'Y': 51,
	'Z': 52,
}

func main() {
	f, err := os.Open("input.txt")
	if err != nil {
		panic(err)
	}
	defer f.Close()

	var p1Points uint
	var p2Points uint
	scanner := bufio.NewScanner(f)

	lastThreeLines := make([]string, 0, 3)
	for scanner.Scan() {
		l := scanner.Text()

		// logic for problem 2
		lastThreeLines = append(lastThreeLines, l)
		if len(lastThreeLines) == 3 {
			l1Map := make(map[rune]struct{})
			l2Map := make(map[rune]struct{})
			l3Map := make(map[rune]struct{})

			for _, r := range lastThreeLines[0] {
				l1Map[r] = struct{}{}
			}

			for _, r := range lastThreeLines[1] {
				l2Map[r] = struct{}{}
			}

			for _, r := range lastThreeLines[2] {
				l3Map[r] = struct{}{}
			}

			// find the item that is in all three maps
			// and add it to the points
			for r := range l1Map {
				if _, ok := l2Map[r]; ok {
					if _, ok := l3Map[r]; ok {
						p2Points += pointMap[r]
						break
					}
				}
			}

			lastThreeLines = make([]string, 0, 3)
		}

		// split the string into a slice of runes (characters)
		runes := []rune(l)

		// split the runes into two slices (one of each half)
		half := len(runes) / 2
		firstHalf := runes[:half]
		secondHalf := runes[half:]

		// create a map of each rune that occurs in each half
		firstHalfMap := make(map[rune]struct{})
		secondHalfMap := make(map[rune]struct{})

		for _, r := range firstHalf {
			firstHalfMap[r] = struct{}{}
		}
		for _, r := range secondHalf {
			secondHalfMap[r] = struct{}{}
		}

		// find which runes occur in both halves
		duplicateItems := make(map[rune]struct{})
		for r := range firstHalfMap {
			if _, ok := secondHalfMap[r]; ok {
				duplicateItems[r] = struct{}{}
			}
		}

		// if there is more than one rune or if there are no runes
		// then something is wrong :(
		if len(duplicateItems) != 1 {
			panic("something is wrong")
		}

		var duplicateItem rune
		for r := range duplicateItems {
			duplicateItem = r
			break
		}

		//fmt.Printf("Found item in both: %s (priority %d)\n", string(duplicateItem), pointMap[duplicateItem])
		p1Points += pointMap[duplicateItem]
	}
	if err := scanner.Err(); err != nil {
		panic(err)
	}

	fmt.Println("Problem 1: ", p1Points)
	fmt.Println("Problem 2: ", p2Points)
}
