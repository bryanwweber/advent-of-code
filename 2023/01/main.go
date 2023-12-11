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
	m["eight"] = "8"
	m["nine"] = "9"
	// The backticks here represent a raw string that doesn't need to escape
	// the slash at the end
	regexPattern := `one|two|three|four|five|six|seven|eight|nine|\d`
	re, err := regexp.Compile(regexPattern)
	if err != nil {
		log.Fatal(err)
	}
	var maybe_first_digit, maybe_last_digit, first_digit, last_digit string
	// The first value here is the index of the loop, similar to Python
	// enumerate. We don't need the index so we discard it
	for _, line := range lines {
		if line == "" {
			continue
		}

		digits := checkForDigits(line, re)

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
		calibration_value += digit
	}
	fmt.Println(calibration_value)
}

func checkForDigits(line string, re *regexp.Regexp) []string {
	// Puzzle rules are apparently that eighthree is 83 and sevenine is 79
	// twone is 21, threeight is 38, fiveight is 58, nineight is 98
	// We therefore advance the start of the next substring match to the start
	// of the previous match, plus one character.
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
