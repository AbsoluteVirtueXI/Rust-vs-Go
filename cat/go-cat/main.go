package main

import (
	"fmt"
	"os"
)

const BinaryName = "go-cat"

func main() {

	filename, err := ExtractFilename()
	if err != nil {
		fmt.Printf("%v\n", err)
		return
	}

	fileContent, err := ReadFile(filename)
	if err != nil {
		fmt.Printf("Error: %s\n", err)
		return
	}

	fmt.Printf("%s", fileContent)
}

func ExtractFilename() (string, error) {
	if len(os.Args) != 2 {
		return "", fmt.Errorf("usage %s filename", BinaryName)
	}
	return os.Args[1], nil
}

func ReadFile(filename string) (string, error) {
	data, err := os.ReadFile(filename)

	if err != nil {
		return "", err
	}

	if len(data) == 0 {
		return "", fmt.Errorf("file %v is empty", filename)
	}

	return string(data), nil
}
