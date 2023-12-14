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
	inputFile := "2023/03/input.txt"
	// inputFile := "2023/03/test-input.txt"
	content, err := os.ReadFile(inputFile)
	if err != nil {
		log.Fatal("Could not read input file")
	}
	lines := strings.Split(string(content), "\n")

	re, err := regexp.Compile(`\d{1,3}`)
	if err != nil {
		log.Fatal("Something wrong with the regex")
	}
	symbolRegex, err := regexp.Compile(`[^\d.]`)
	if err != nil {
		log.Fatal("Something wrong with the regex")
	}
	partMappings := make(map[int]map[int][]int)
	symbolAdjacencyMatrix := make(map[int]map[int]string)
	for row, line := range lines {
		matches := re.FindAllStringIndex(line, -1)
		for _, match := range matches {
			thisPartNumber, _ := strconv.Atoi(line[match[0]:match[1]])
			for ii := match[0] - 1; ii <= match[1]; ii++ {
				if ii < 0 || ii > len(line) {
					continue
				}
				partMappings = addToPartMappings(partMappings, row, ii, thisPartNumber)
				if row > 0 {
					partMappings = addToPartMappings(partMappings, row-1, ii, thisPartNumber)
				}
				if row < len(lines) {
					partMappings = addToPartMappings(partMappings, row+1, ii, thisPartNumber)
				}
			}
		}
		symbols := symbolRegex.FindAllStringIndex(line, -1)
		for _, symbol := range symbols {
			symbolAdjacencyMatrix = addToSymbols(symbolAdjacencyMatrix, row, symbol[0], string(line[symbol[0]]))
		}
	}
	// Doing this with a slice rather than summing directly makes debugging easier
	allPartNumbers := make([]int, 0)
	gearRatios := make([]int, 0)
	for row, valueMap := range symbolAdjacencyMatrix {
		for col, symbol := range valueMap {
			if symbol != "*" {
				continue
			}
			partNumbers, exists := partMappings[row][col]
			if !exists {
				continue
			}
			if len(partNumbers) != 2 {
				continue
			}
			gearRatios = append(gearRatios, partNumbers[0]*partNumbers[1])
			allPartNumbers = append(allPartNumbers, partNumbers...)
		}
	}
	fmt.Println(allPartNumbers)
	sum := 0
	for _, pn := range gearRatios {
		sum += pn
	}
	fmt.Println(sum)
}

func addToSymbols(symbolMappings map[int]map[int]string, row int, ii int, thisValue string) map[int]map[int]string {
	valueMap, rowExists := symbolMappings[row]
	if rowExists {
		valueMap[ii] = thisValue
	} else {
		valueMap = map[int]string{ii: thisValue}
		symbolMappings[row] = valueMap
	}
	return symbolMappings
}

func addToPartMappings(partMappings map[int]map[int][]int, row int, ii int, thisPartNumber int) map[int]map[int][]int {
	valueMap, rowExists := partMappings[row]
	if rowExists {
		value, colExists := valueMap[ii]
		if colExists {
			value = append(value, thisPartNumber)
		} else {
			value = []int{thisPartNumber}
		}
		valueMap[ii] = value
	} else {
		valueMap = map[int][]int{ii: {thisPartNumber}}
		partMappings[row] = valueMap
	}
	return partMappings
}
