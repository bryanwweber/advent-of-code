package main

import (
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
	"unicode"
)

func main() {
	filePath := "data.txt"
	content, err := os.ReadFile(filePath)
	if err != nil {
		log.Fatal(err)
	}

	lines := strings.Split(string(content), "\n")
	calibration_value := 0
	for _, line := range lines {
		if line == "" {
			continue
		}

		digits := extractDigits(line)

		var first_digit, last_digit string
		if len(digits) < 2 {
			first_digit = digits[0]
			last_digit = digits[0]
		} else {
			first_digit = digits[0]
			last_digit = digits[len(digits)-1]
		}
		digit, _ := strconv.Atoi(first_digit + last_digit)
		calibration_value += digit
	}
	fmt.Println(calibration_value)
}

func extractDigits(line string) []string {
	digits := make([]string, 0)
	for _, char := range line {
		if unicode.IsDigit(char) {
			digits = append(digits, string(char))
		}
	}
	return digits
}
