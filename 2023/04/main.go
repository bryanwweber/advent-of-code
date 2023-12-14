package main

import (
	"fmt"
	"log"
	"math"
	"os"
	"regexp"
	"strconv"
	"strings"
)

func main() {
	// inputFile := "2023/04/input.txt"
	inputFile := "2023/04/test-input.txt"
	content, err := os.ReadFile(inputFile)
	if err != nil {
		log.Fatal("Could not read input file")
	}
	lines := strings.Split(string(content), "\n")
	// Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
	cardRegex, err := regexp.Compile(`^Card\s+(\d+):\s([\d\s]+)\s\|\s([\d\s]+)$`)
	if err != nil {
		log.Fatal("Something wrong with the regex")
	}
	scores := make([]int, 0)
	nCards := make(map[int]int)
	for _, line := range lines {
		if line == "" {
			continue
		}
		matches := cardRegex.FindStringSubmatch(line)
		cardNumber, _ := strconv.Atoi(matches[1])
		winningCard := matches[2]
		numberCard := matches[3]
		winningNumbers := MapAtoi(strings.Split(winningCard, " "))
		ourNumbers := MapAtoi(strings.Split(numberCard, " "))
		intersection := intersect(winningNumbers, ourNumbers)
		scores = append(scores, int(math.Pow(2, float64(len(intersection)-1))))
	}
	sum := 0
	for _, score := range scores {
		sum += score
	}
	fmt.Println(sum)
}

func MapAtoi(numbers []string) []int {
	var result []int
	for _, number := range numbers {
		if number == "" {
			continue
		}
		num, _ := strconv.Atoi(number)
		result = append(result, num)
	}
	return result
}
func intersect(a []int, b []int) []int {
	m := make(map[int]bool)
	for _, item := range a {
		m[item] = true
	}

	var result []int
	for _, item := range b {
		if _, ok := m[item]; ok {
			result = append(result, item)
		}
	}
	return result
}
