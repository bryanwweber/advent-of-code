package main

import (
	"fmt"
	"log"
	"os"
	"regexp"
	"strconv"
	"strings"
)

func main() {
	inputFile := "2023/04/input.txt"
	// inputFile := "2023/04/test-input.txt"
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
	wonCards := make(map[int][]int)
	for _, line := range lines {
		if line == "" {
			continue
		}
		matches := cardRegex.FindStringSubmatch(line)
		cardNumber, _ := strconv.Atoi(matches[1])
		winningCard := matches[2]
		numberCard := matches[3]
		winningNumbers := mapAtoi(strings.Split(winningCard, " "))
		ourNumbers := mapAtoi(strings.Split(numberCard, " "))
		intersection := intersect(winningNumbers, ourNumbers)
		nWinningNumbers := len(intersection)
		cards := make([]int, 0)
		if nWinningNumbers > 0 {
			for ii := cardNumber + 1; ii <= cardNumber+nWinningNumbers; ii++ {
				cards = append(cards, ii)
			}
		}
		wonCards[cardNumber] = cards
	}
	totalCards := make(map[int]int)
	nCards := len(lines) - 1
	for cardNumber := 1; cardNumber <= nCards; cardNumber++ {
		addedCards := wonCards[cardNumber]
		totalCards[cardNumber]++
		for _, wonCard := range addedCards {
			totalCards[wonCard] += totalCards[cardNumber]
		}
	}
	fmt.Println(totalCards)
	sum := 0
	for _, n := range totalCards {
		sum += n
	}
	fmt.Println(sum)
}

func mapAtoi(numbers []string) []int {
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
