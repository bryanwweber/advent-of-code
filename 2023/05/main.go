package main

import (
	"fmt"
	"log"
	"os"
	"sort"
	"strconv"
	"strings"
)

func main() {
	// inputFile := "2023/05/input.txt"
	inputFile := "2023/05/test-input.txt"
	content, err := os.ReadFile(inputFile)
	if err != nil {
		log.Fatal("Could not read input file")
	}
	lines := strings.Split(string(content), "\n")
	allMaps := make(map[string][]RangeMap)
	var seedSoil, soilFertilizer, fertilizerWater, waterLight, lightTemperature, temperatureHumidity, humidityLocation []int
	for ii, line := range lines {
		if line == "" {
			continue
		}
		thisMapType := line[0:5]
		switch thisMapType {
		case "seeds":
			allMaps["initialSeeds"] = getInitialSeeds(line)
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
	fmt.Println(humidityLocation)
	mapOrdering := []string{
		"initialSeeds", "seedToSoil", "soilToFertilizer", "fertilizerToWater",
		"waterToLight", "lightToTemperature", "temperatureToHumidity",
		"humidityToLocation",
	}
	allMaps["seedToSoil"] = mapRanges(lines, seedSoil[0], seedSoil[1], "seed", "soil")
	allMaps["soilToFertilizer"] = mapRanges(lines, soilFertilizer[0], soilFertilizer[1], "soil", "fertilizer")
	allMaps["fertilizerToWater"] = mapRanges(lines, fertilizerWater[0], fertilizerWater[1], "fertilizer", "water")
	allMaps["waterToLight"] = mapRanges(lines, waterLight[0], waterLight[1], "water", "light")
	allMaps["lightToTemperature"] = mapRanges(lines, lightTemperature[0], lightTemperature[1], "light", "temperature")
	allMaps["temperatureToHumidity"] = mapRanges(lines, temperatureHumidity[0], temperatureHumidity[1], "temperature", "humidity")
	allMaps["humidityToLocation"] = mapRanges(lines, humidityLocation[0], humidityLocation[1], "humidity", "location")
	fmt.Println(allMaps["initialSeeds"])
	fmt.Println(allMaps["seedToSoil"])
	destination := allMaps["initialSeeds"][0].destination
	newSource := make([]Range, 0)
	for _, thisMap := range allMaps[mapOrdering[1]] {
		if thisMap.source.end <= destination.end {
			fmt.Printf("source: %v, dest: %v\n", thisMap.source, destination)
			newStart := thisMap.source.start
			if newStart < destination.start {
				newStart = destination.start
			}
			newEnd := destination.end
			newSource = append(newSource, Range{newStart, newEnd})
		}
	}
	destination = allMaps["initialSeeds"][1].destination
	for _, thisMap := range allMaps[mapOrdering[1]] {
		if thisMap.source.end <= destination.end {
			if thisMap.source.start < destination.start {
				continue
			}
			fmt.Printf("source: %v, dest: %v\n", thisMap.source, destination)
			newStart := thisMap.source.start
			if newStart < destination.start {
				newStart = destination.start
			}
			newEnd := destination.end
			newSource = append(newSource, Range{newStart, newEnd})
		}
	}
	fmt.Println(newSource)
	// for _, thisMap := range mapOrdering {
	// 	nextStart := mapSourceToDestination(allMaps[thisMap], source.start)
	// 	if nextStart == -1 {
	// 		log.Fatalf("Bad Mapping for %s: %d", thisMap, source.start)
	// 	}
	// 	nextEnd := mapSourceToDestination(allMaps[thisMap], source.end)
	// 	if nextEnd == -1 {
	// 		log.Fatalf("Bad Mapping for %s: %d", thisMap, source.start)
	// 	}
	// 	nextSource := Range{nextStart, nextEnd}
	// 	fmt.Printf("%v\t%v\n", source, nextSource)
	// 	source = nextSource
	// }
}

type Range struct {
	start int
	end   int
}

type RangeMap struct {
	source      Range
	destination Range
}

func somethingNamed(end int, r Range) []Range {
	if end <= r.end {
		return []Range{{r.start, end}}
	} else {
		newRanges := make([]Range, 0)
		newRanges = append(newRanges, r)
		newRanges = append(newRanges, Range{r.end, end})
		return newRanges
	}
}

func createMonotonicRangeFromStartAndEnd(start, end int, r []RangeMap) {
	sort.Slice(r, func(i, j int) bool { return r[i].source.start > r[j].source.start })
	for _, thisRange := range r {
		if start > thisRange.source.start {
			if end <= thisRange.source.end {
				fmt.Println("Fully contained")
			}
		}
	}
}

func mapSourceToDestination(r []RangeMap, source int) int {
	for _, thisRange := range r {
		if source > thisRange.source.start && source < thisRange.source.end {
			return source - thisRange.source.start + thisRange.destination.start
		}
	}
	return source
}

func ensureMonotonicSource(ranges []RangeMap) []RangeMap {
	returnMap := make([]RangeMap, 0)
	for ii, thisRange := range ranges {
		if ii == len(ranges)-1 {
			returnMap = append(returnMap, thisRange)
			continue
		}

		nextRange := ranges[ii+1]

		if thisRange.source.start > 0 && len(returnMap) == 0 {
			insertedMap := RangeMap{
				Range{0, thisRange.source.start},
				Range{0, nextRange.destination.start},
			}
			returnMap = append(returnMap, insertedMap)
		}
		returnMap = append(returnMap, thisRange)
		if thisRange.source.end != nextRange.source.start {
			insertedMap := RangeMap{
				Range{thisRange.source.end + 1, nextRange.source.start},
				Range{thisRange.destination.end, nextRange.destination.start},
			}
			returnMap = append(returnMap, insertedMap)
		}
	}
	return returnMap
}

func mapRanges(lines []string, startLine, endLine int, sourceName, destinationName string) []RangeMap {
	returnMap := make([]RangeMap, 0)
	for ii := startLine; ii < endLine; ii++ {
		rangeValues := mapAtoi(strings.Split(lines[ii], " "))
		destinationStart := rangeValues[0]
		sourceStart := rangeValues[1]
		length := rangeValues[2]
		thisRange := RangeMap{
			Range{sourceStart, sourceStart + length},
			Range{destinationStart, destinationStart + length},
		}
		returnMap = append(returnMap, thisRange)
	}
	sort.Slice(returnMap, func(i, j int) bool {
		return returnMap[i].source.start < returnMap[j].source.start
	})
	returnMap = ensureMonotonicSource(returnMap)
	return returnMap
}

func getInitialSeeds(line string) []RangeMap {
	match := strings.Split(line, ":")
	numbers := strings.Split(match[1], " ")
	seedValues := mapAtoi(numbers)
	seedRanges := make([]RangeMap, 0)
	for ii := 0; ii < len(seedValues); ii += 2 {
		start := seedValues[ii]
		end := seedValues[ii+1] + start
		seedRanges = append(seedRanges, RangeMap{Range{0, 0}, Range{start, end}})
	}
	sort.Slice(seedRanges, func(i, j int) bool {
		return seedRanges[i].source.start < seedRanges[j].source.start
	})
	return seedRanges
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
