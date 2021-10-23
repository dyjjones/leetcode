package problems

import (
	"reflect"
	"testing"
)

func TestP1(t *testing.T) {
	testCases := []struct {
		nums     []int
		target   int
		expected []int
	}{
		{[]int{2, 7, 11, 15}, 9, []int{0, 1}},
		{[]int{3, 2, 4}, 6, []int{1, 2}},
		{[]int{3, 3}, 6, []int{0, 1}},
	}

	badTestCases := []struct {
		nums     []int
		target   int
		expected []int
	}{
		{[]int{2, 7, 11, 15}, 10, nil},
		{[]int{3, 2, 4}, 8, nil},
		{[]int{3, 3}, 7, nil},
	}

	// bad_test_cases = [
	//     ([2,7,11,15], 10, None),
	//     ([3,2,4], 8, None),
	//     ([3,3], 7, None)
	// ]

	t.Run("Test valid test cases.", func(t *testing.T) {
		for _, testCase := range testCases {
			got := twoSum(testCase.nums, testCase.target)
			if !reflect.DeepEqual(got, testCase.expected) {
				t.Errorf("got %v, expected %v", got, testCase.expected)
			}
		}
	})

	t.Run("Test cases with no solution.", func(t *testing.T) {
		for _, testCase := range badTestCases {
			got := twoSum(testCase.nums, testCase.target)
			if !reflect.DeepEqual(got, testCase.expected) {
				t.Errorf("got %v, expected %v", got, testCase.expected)
			}
		}
	})

}
