package main

import (
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func main() {
	filePath := "2023/02/input.txt"
	// filePath := "2023/02/test-input.txt"
	content, err := os.ReadFile(filePath)
	if err != nil {
		log.Fatal(err)
	}

	maxRed := 12
	maxGreen := 13
	maxBlue := 14
	goodGame := make([]int, 0)
	lines := strings.Split(string(content), "\n")
	for _, line := range lines {
		if line == "" {
			continue
		}
		game, allBatches, found := strings.Cut(line, ":")
		if !found {
			log.Fatalf("Couldn't split %s", line)
		}
		_, gameIndex, found := strings.Cut(game, " ")
		if !found {
			log.Fatalf("Couldn't split game %s", game)
		}

		batches := strings.Split(allBatches, ";")
		thisGameGood := true
		for _, batch := range batches {
			batch = strings.TrimSpace(batch)
			allCubes := strings.Split(batch, ",")
			for _, cube := range allCubes {
				nCube, color, found := strings.Cut(strings.TrimSpace(cube), " ")
				if !found {
					log.Fatalf("Couldn't split cube %s", cube)
				}
				nCubeInt, _ := strconv.Atoi(nCube)
				if color == "red" && nCubeInt > maxRed {
					thisGameGood = false
					break
				} else if color == "green" && nCubeInt > maxGreen {
					thisGameGood = false
					break
				} else if color == "blue" && nCubeInt > maxBlue {
					thisGameGood = false
					break
				}
			}
			if !thisGameGood {
				break
			}
		}
		if thisGameGood {
			gameIndexInt, _ := strconv.Atoi(gameIndex)
			goodGame = append(goodGame, gameIndexInt)
		}
	}

	sum := 0
	for _, num := range goodGame {
		sum += num
	}
	fmt.Println(sum)
}
