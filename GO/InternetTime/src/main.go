package main

import (
	"fmt"
	"time"
)

func main() {
	fmt.Printf("@%3.1f", getInternetTime())
}

func getInternetTime() float64 {
	var utcTime = time.Now().UTC()
	var result float64 = 0.0
	result += float64((utcTime.Hour()+1)%24) * 3600.0
	result += float64(utcTime.Minute()) * 60.0
	result += float64(utcTime.Second())
	result /= 86.4
	return result
}
