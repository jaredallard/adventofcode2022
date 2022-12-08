package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type node struct {
	name      string
	size      int64
	directory bool
	children  map[string]*node
	parent    *node
}

func (n *node) Size() int64 {
	if !n.directory {
		return n.size
	}

	var size int64
	for _, d := range n.children {
		size += d.Size()
	}
	return size
}

func NewNode(name string, size int64, directory bool, parent *node) *node {
	return &node{name, size, directory, make(map[string]*node), parent}
}

func hErr(err error) {
	if err != nil {
		panic(err)
	}
}

func main() {
	input, err := os.Open("input.txt")
	hErr(err)
	defer input.Close()

	scanner := bufio.NewScanner(input)

	var currentDirectory *node
	nodes := make([]*node, 0)

	for scanner.Scan() {
		line := scanner.Text()
		parts := strings.Split(line, " ")

		// command
		if strings.HasPrefix(line, "$") {
			command := parts[1]
			args := make([]string, 0)
			if len(parts) > 2 {
				args = parts[2:]
			}

			if command == "cd" {
				lsPath := args[0]
				if lsPath == ".." {
					currentDirectory = currentDirectory.parent
				} else if lsPath == "/" {
					currentDirectory = NewNode("/", 0, true, nil)
					nodes = append(nodes, currentDirectory)
				} else {
					currentDirectory = currentDirectory.children[lsPath]
				}
			}

			continue
		}

		// output (file or dir)
		if parts[0] == "dir" {
			dirName := parts[1]
			currentDirectory.children[dirName] = NewNode(dirName, 0, true, currentDirectory)
			nodes = append(nodes, currentDirectory.children[dirName])
		} else {
			fileName := parts[1]
			size, err := strconv.Atoi(parts[0])
			hErr(err)

			currentDirectory.children[fileName] = NewNode(fileName, int64(size), false, currentDirectory)
		}
	}

	var problem1 int64
	for _, n := range nodes {
		size := n.Size()
		if size <= 100000 {
			problem1 += size
		}
	}

	root := nodes[0]
	total := int64(70000000)
	toFree := int64(30000000)
	used := root.Size()
	target := total - toFree
	smallest := int64(0)

	for _, n := range nodes {
		newUsed := (used - n.Size())
		if newUsed <= target && n.Size() < smallest || smallest == 0 {
			smallest = n.Size()
		}
	}
	problem2 := smallest

	fmt.Println("Problem 1:", problem1)
	fmt.Println("Problem 2:", problem2)
}
