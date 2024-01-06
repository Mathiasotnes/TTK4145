// Use `go run foo.go` to run your program

package main

import (
	"fmt"
    "runtime"
    // "time"
)

var i = 0

func incrementing() {
    for j := 0; j < 1000000; j++ {
        i++
    }
}

func decrementing() {
    for j := 0; j < 1000000; j++ {
        i--
    }
}

func incrementingChan(incChan, doneChan chan int, increments int) {
    for j := 0; j < increments; j++ {
        incChan <- 1
    }
    doneChan <- 1
}

func decrementingChan(decChan, doneChan chan int, decrements int) {
    for j := 0; j < decrements; j++ {
        decChan <- 1
    }
    doneChan <- 1
}

func server(incChan, decChan, doneChan chan int) {
    completed := 0

    for {
        select {
        case <-incChan:
            i++
        case <-decChan:
            i--
        case <-doneChan:
            completed++
            if completed == 2 {
                fmt.Println("Server quitting")
                fmt.Println("The magic number is:", i)
                return
            }
        }
    }
}

func main() {
    // What does GOMAXPROCS do? What happens if you set it to 1?
    runtime.GOMAXPROCS(2)    
	
    // go incrementing()
    // go decrementing()
	
    // We have no direct way to wait for the completion of a goroutine (without additional synchronization of some sort)
    // We will do it properly with channels soon. For now: Sleep.
    // time.Sleep(500*time.Millisecond)
    // Println("The magic number is:", i)

    incChan := make(chan int)
    decChan := make(chan int)
    doneChan := make(chan int)

    go incrementingChan(incChan, doneChan, 1000000)
    go decrementingChan(decChan, doneChan, 999900)

    server(incChan, decChan, doneChan)
}
