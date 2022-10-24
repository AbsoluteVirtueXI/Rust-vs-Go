package main

import (
	"crypto/rand"
	"fmt"
	"go-guessing-game/input"
	"math"
	"math/big"
)

const min int8 = math.MinInt8
const max int8 = math.MaxInt8

func main() {
	var err, r = rand.Int(rand.Reader, big.NewInt(int64(max)))
	fmt.Println(math.MinInt8)
	fmt.Println(math.MaxInt8)
	fmt.Println(err, r)
	input.WhatIsMyKeyBoard()
}
