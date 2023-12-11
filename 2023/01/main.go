package main

import (
	"fmt"
	"log"
	"os"
	"regexp"
	"strconv"
	"strings"
	"unicode"
)

func main() {
	filePath := "2023/01/data.txt"
	// filePath := "2023/01/part-2-test.txt"
	content, err := os.ReadFile(filePath)
	if err != nil {
		log.Fatal(err)
	}

	lines := strings.Split(string(content), "\n")
	calibration_value := 0
	m := make(map[string]string)
	m["one"] = "1"
	m["two"] = "2"
	m["three"] = "3"
	m["four"] = "4"
	m["five"] = "5"
	m["six"] = "6"
	m["seven"] = "7"
	m["seve"] = "7"
	m["eight"] = "8"
	m["eigh"] = "8"
	m["nine"] = "9"
	// Puzzle rules are apparently that eighthree is 83 and sevenine is 79
	// twone is 21, threeight is 38, fiveight is 58, nineight is 98
	regexPattern := `one|two|three|four|five|six|seven|eight|nine|\d`
	re, err := regexp.Compile(regexPattern)
	if err != nil {
		log.Fatal(err)
	}
	var maybe_first_digit, maybe_last_digit, first_digit, last_digit string
	allDigits := make([]int, 0)
	allMatches := make([][]string, 0)
	for _, line := range lines {
		if line == "" {
			continue
		}

		// digits := re.FindAllString(line, -1)
		digits := checkForDigits(line, re)
		allMatches = append(allMatches, digits)
		// fmt.Println(digits)
		// break

		if len(digits) < 2 {
			maybe_first_digit = digits[0]
			maybe_last_digit = digits[0]
		} else {
			maybe_first_digit = digits[0]
			maybe_last_digit = digits[len(digits)-1]
		}
		if unicode.IsDigit([]rune(maybe_first_digit)[0]) {
			first_digit = maybe_first_digit
		} else {
			first_digit = m[maybe_first_digit]
		}
		if unicode.IsDigit([]rune(maybe_last_digit)[0]) {
			last_digit = maybe_last_digit
		} else {
			last_digit = m[maybe_last_digit]
		}
		digit, _ := strconv.Atoi(first_digit + last_digit)
		allDigits = append(allDigits, digit)
		calibration_value += digit
	}
	fmt.Println(calibration_value)
	// for ii := range allDigits {
	// 	fmt.Printf("%d\t%d: %s\n", ii+1, allDigits[ii], allMatches[ii])
	// }
}

func checkForDigits(line string, re *regexp.Regexp) []string {
	pos := 0
	digits := make([]string, 0)
	for {
		match := re.FindStringSubmatchIndex(line[pos:])
		if match == nil {
			break
		}
		start := match[0]
		end := match[1]
		digits = append(digits, line[pos+start:pos+end])

		pos = pos + start + 1
	}
	return digits
}
