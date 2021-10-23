package lib

import (
	"os"
	"path/filepath"
	"regexp"
	"sort"
	"strconv"
)

func ListProblems() []int {
	problems, _ := unsortedProblems("problems")
	sort.Slice(problems, func(i, j int) bool {
		return problems[i] < problems[j]
	})
	return problems
}

func unsortedProblems(path string) ([]int, error) {
	var problems []int
	err := filepath.Walk(path, func(p string, info os.FileInfo, err error) error {
		regexMatch, err := regexp.MatchString(`p\d+.go`, info.Name())
		if !info.IsDir() && regexMatch {
			fname := info.Name()[1 : len(info.Name())-3]
			prob_num, _ := strconv.Atoi(fname)

			problems = append(problems, prob_num)
		}
		return err
	})
	return problems, err
}

// Find takes a slice and looks for an element in it. If found it will
// return it's key, otherwise it will return -1 and a bool of false.
func FindString(slice []string, val string) (int, bool) {
	for i, item := range slice {
		if item == val {
			return i, true
		}
	}
	return -1, false
}

// Find takes a slice and looks for an element in it. If found it will
// return it's key, otherwise it will return -1 and a bool of false.
func FindInt(slice []int, val int) (int, bool) {
	for i, item := range slice {
		if item == val {
			return i, true
		}
	}
	return -1, false
}
