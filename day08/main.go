package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func hErr(err error) {
	if err != nil {
		panic(err)
	}
}

// getTreeInfo returns if a tree is visible from the provided coords
// and it's score
func getTreeInfo(grid [][]int, x, y int) (bool, int) {
	aboveVis := true
	belowVis := true
	leftVis := true
	rightVis := true
	alwaysVis := false

	var up int
	var down int
	var left int
	var right int

	// always visible if we're on the edge
	if x == 0 || x == len(grid[y])-1 || y == 0 || y == len(grid)-1 {
		alwaysVis = true
	}
	ourHeight := grid[y][x]

	// check above
	for i := y - 1; i >= 0; i-- {
		up++
		if ourHeight <= grid[i][x] {
			aboveVis = false
			break
		}
	}

	// check below
	for i := y + 1; i < len(grid); i++ {
		down++
		if ourHeight <= grid[i][x] {
			belowVis = false
			break
		}
	}

	// check left
	for i := x - 1; i >= 0; i-- {
		left++
		if ourHeight <= grid[y][i] {
			leftVis = false
			break
		}
	}

	// check right
	for i := x + 1; i < len(grid[y]); i++ {
		right++
		if ourHeight <= grid[y][i] {
			rightVis = false
			break
		}
	}

	visible := alwaysVis || aboveVis || belowVis || leftVis || rightVis
	score := up * down * left * right
	return visible, score
}

func main() {
	f, err := os.Open("input.txt")
	hErr(err)
	defer f.Close()

	grid := make([][]int, 0)
	visible := make(map[string]bool)

	scanner := bufio.NewScanner(f)
	for scanner.Scan() {
		line := scanner.Text()
		row := make([]int, 0)
		for _, c := range strings.Split(line, "") {
			i, err := strconv.Atoi(c)
			hErr(err)
			row = append(row, i)
		}
		grid = append(grid, row)
	}

	var highestScore int
	for y := range grid {
		for x := range grid[y] {
			key := strconv.Itoa(x) + "," + strconv.Itoa(y)
			isVisible, score := getTreeInfo(grid, x, y)
			if isVisible {
				visible[key] = true
			}
			if score > highestScore {
				highestScore = score
			}
		}
	}

	fmt.Println("Problem 1:", len(visible))
	fmt.Println("Problem 2:", highestScore)
}
