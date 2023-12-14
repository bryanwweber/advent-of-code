package main

import (
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func main() {
	inputFile := "2023/05/input.txt"
	// inputFile := "2023/05/test-input.txt"
	content, err := os.ReadFile(inputFile)
	if err != nil {
		log.Fatal("Could not read input file")
	}
	lines := strings.Split(string(content), "\n")
	var initialSeeds []int
	var seedSoil, soilFertilizer, fertilizerWater, waterLight, lightTemperature, temperatureHumidity, humidityLocation []int
	for ii, line := range lines {
		if line == "" {
			continue
		}
		thisMapType := line[0:5]
		switch thisMapType {
		case "seeds":
			initialSeeds = getInitialSeeds(line)
		case "seed-":
			seedSoil = append(seedSoil, ii+1)
		case "soil-":
			seedSoil = append(seedSoil, ii-1)
			soilFertilizer = append(soilFertilizer, ii+1)
		case "ferti":
			soilFertilizer = append(soilFertilizer, ii-1)
			fertilizerWater = append(fertilizerWater, ii+1)
		case "water":
			fertilizerWater = append(fertilizerWater, ii-1)
			waterLight = append(waterLight, ii+1)
		case "light":
			waterLight = append(waterLight, ii-1)
			lightTemperature = append(lightTemperature, ii+1)
		case "tempe":
			lightTemperature = append(lightTemperature, ii-1)
			temperatureHumidity = append(temperatureHumidity, ii+1)
		case "humid":
			temperatureHumidity = append(temperatureHumidity, ii-1)
			humidityLocation = []int{ii + 1, len(lines) - 1}
		default:
			continue
		}
	}
	fmt.Println("seedToSoilMap")
	seedToSoilMap := mapRanges(lines, seedSoil[0], seedSoil[1])
	fmt.Println("soilToFertilizerMap")
	soilToFertilizerMap := mapRanges(lines, soilFertilizer[0], soilFertilizer[1])
	fertilizerToWaterMap := mapRanges(lines, fertilizerWater[0], fertilizerWater[1])
	waterToLightMap := mapRanges(lines, waterLight[0], waterLight[1])
	lightToTemperatureMap := mapRanges(lines, lightTemperature[0], lightTemperature[1])
	temperatureToHumidityMap := mapRanges(lines, temperatureHumidity[0], temperatureHumidity[1])
	humidityToLocatioNMap := mapRanges(lines, humidityLocation[0], humidityLocation[1])
	smallestLocation := int(^uint(0) >> 1)
	for ii, seedNumber := range initialSeeds {
		fmt.Printf("Working on seed index %d\n", ii)
		soilNumber := getValueIfExists(seedToSoilMap, seedNumber)
		fertilizerNumber := getValueIfExists(soilToFertilizerMap, soilNumber)
		waterNumber := getValueIfExists(fertilizerToWaterMap, fertilizerNumber)
		lightNumber := getValueIfExists(waterToLightMap, waterNumber)
		temperatureNumber := getValueIfExists(lightToTemperatureMap, lightNumber)
		humidityNumber := getValueIfExists(temperatureToHumidityMap, temperatureNumber)
		locationNumber := getValueIfExists(humidityToLocatioNMap, humidityNumber)
		if locationNumber < smallestLocation {
			smallestLocation = locationNumber
		}
	}
	fmt.Println(smallestLocation)
}

func getValueIfExists(mappings []map[string]int, key int) int {
	for _, mapping := range mappings {
		if key >= mapping["sourceStart"] && key <= mapping["sourceEnd"] {
			position := key - mapping["sourceStart"]
			value := mapping["destinationStart"] + position
			return value
		}
	}
	return key
}

func mapRanges(lines []string, startLine int, endLine int) []map[string]int {
	returnMap := make([]map[string]int, 0)
	for ii := startLine; ii < endLine; ii++ {
		thisMap := make(map[string]int)
		rangeValues := mapAtoi(strings.Split(lines[ii], " "))
		destinationStart := rangeValues[0]
		sourceStart := rangeValues[1]
		length := rangeValues[2]
		thisMap["destinationStart"] = destinationStart
		thisMap["sourceStart"] = sourceStart
		thisMap["sourceEnd"] = sourceStart + length
		returnMap = append(returnMap, thisMap)
	}
	return returnMap
}

func getInitialSeeds(line string) []int {
	match := strings.Split(line, ":")
	numbers := strings.Split(match[1], " ")
	return mapAtoi(numbers)
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
