package main

import (
	"fmt"
	"time"
)

func main() {
	fmt.Printf("@%3.1f", getInternetTime())
}

func getInternetTime() float64 {
	var h, m, s = time.Now().UTC().Add(time.Hour).Clock()
	return (float64(h)*3600 + float64(m)*60 + float64(s)) / 86.4
}
