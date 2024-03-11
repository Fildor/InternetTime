package main

import (
	"fmt"
	"time"
)

func main() {
	fmt.Printf("@%3.1f", getInternetTimeV2())
}

func getInternetTimeV2() float64 {
	var utcTime = time.Now().UTC()
	hour, min, sec := utcTime.Add(time.Hour).Clock()
	return (float64(hour%24)*3600.0 +
		float64(min*60.0) +
		float64(sec)) / 86.4
}
