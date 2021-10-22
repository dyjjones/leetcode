package main

import (
	"fmt"
	"leetcode/lib"
)

func main() {
	fmt.Println("Run `go test` for checking solutions.")

	problems := lib.ListProblems()
	if len(problems) == 0 {
		fmt.Println("No solutions available.")
	} else {
		fmt.Println("Problems solved in Go:")
		for _, num := range problems {
			fmt.Println(num)
		}
	}
}
