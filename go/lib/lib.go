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
