package problems

func twoSum(nums []int, target int) []int {
	numIndices := make(map[int][]int)
	for i, n := range nums {
		numIndices[n] = append(numIndices[n], i)
	}

	for m, indices := range numIndices {
		n := target - m

		keyInMap := false
		for k := range numIndices {
			if k == n {
				keyInMap = true
			}
		}
		if keyInMap {
			if m == n {
				if 2 <= len(indices) {
					return indices[:2]
				}
			} else {
				return []int{indices[0], numIndices[n][0]}
			}
		}
	}

	return nil
}
