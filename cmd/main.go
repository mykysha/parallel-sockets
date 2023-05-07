package main

import (
	"fmt"
	"net"
	"os"
	"strconv"
	"time"
)

// Define the f(x) and g(x) functions
func f(x int) int {
	// Implement the f(x) function logic here
	return x + 1
}

func g(x int) int {
	// Implement the g(x) function logic here
	return x * 2
}

// Function to handle a helper computation
func handleHelper(conn net.Conn, x int, computeFunc func(int) int) {
	defer conn.Close()

	// Perform the computation and send the result
	result := computeFunc(x)
	conn.Write([]byte(strconv.Itoa(result)))
}

// Main function
func main() {
	// Create a TCP listener for the helpers
	listener, err := net.Listen("tcp", "127.0.0.1:0")
	if err != nil {
		fmt.Println("Error creating listener:", err)
		os.Exit(1)
	}
	defer listener.Close()

	// Accept connections from the helpers in separate Goroutines
	go func() {
		conn, _ := listener.Accept()
		handleHelper(conn, 5, f)
	}()

	go func() {
		conn, _ := listener.Accept()
		handleHelper(conn, 5, g)
	}()

	// Create connections to the helpers
	conn1, _ := net.Dial("tcp", listener.Addr().String())
	conn2, _ := net.Dial("tcp", listener.Addr().String())

	// Set up channels for receiving results
	resultChan1 := make(chan int)
	resultChan2 := make(chan int)

	// Read results from the helpers in separate Goroutines
	go func() {
		buf := make([]byte, 1024)
		n, _ := conn1.Read(buf)
		result, _ := strconv.Atoi(string(buf[:n]))
		resultChan1 <- result
	}()

	go func() {
		buf := make([]byte, 1024)
		n, _ := conn2.Read(buf)
		result, _ := strconv.Atoi(string(buf[:n]))
		resultChan2 <- result
	}()

	// Timeout loop for handling user input
	timeout := time.NewTicker(10 * time.Second)

	for {
		select {
		case result1 := <-resultChan1:
			if result1 != 0 {
				fmt.Println("Result:", result1)
				os.Exit(0)
			}
		case result2 := <-resultChan2:
			if result2 != 0 {
				fmt.Println("Result:", result2)
				os.Exit(0)
			}
		case <-timeout.C:
			fmt.Println("1) continue the calculation, 2) stop, or 3) continue without asking more")

			var choice int

			fmt.Scan(&choice)

			switch choice {
			case 1:
				timeout = time.NewTicker(10 * time.Second)
			case 2:
				os.Exit(0)
			case 3:
				timeout.Stop()
			}
		}
	}
}
