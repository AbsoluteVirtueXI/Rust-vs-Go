package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

const BinaryName = "go-sed"

func main() {
	old, new, filename, err := ExtractArgs()
	if err != nil {
		fmt.Printf("%v\n", err)
		return
	}

	_, err = sed(old, new, filename)
	if err != nil {
		fmt.Printf("Error: %v\n", err)
	}

}

func ExtractArgs() (string, string, string, error) {
	if len(os.Args) != 4 {
		return "", "", "", fmt.Errorf("usage: %s word1 word2 filename", BinaryName)
	}
	return os.Args[1], os.Args[2], os.Args[3], nil
}

func sed(old, new, filename string) (uint, error) {

	// Open filename
	f, err := os.Open(filename)
	if err != nil {
		return 0, fmt.Errorf("%v", err)
	}
	defer f.Close()

	var count uint

	scanner := bufio.NewScanner(f)
	for scanner.Scan() {
		_, line, nb := ProcessLine(old, new, scanner.Text())
		count += nb
		fmt.Printf("%v\n", line)
	}

	return count, nil
}

func ProcessLine(old, new, line string) (bool, string, uint) {
	var found bool
	var count uint

	if strings.Contains(line, old) {
		count = uint(strings.Count(line, old))
		line = strings.Replace(line, old, new, -1)
		found = true
	}

	return found, line, count
}
