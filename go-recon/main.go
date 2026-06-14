package main

import (
	"flag"
	"fmt"
	"net"
	"sort"
	"sync"
	"time"
)

type Port struct {
	Num     int
	Service string
}

func ScanPort(host string, port int) *Port {
	addr := fmt.Sprintf("%s:%d", host, port)
	conn, err := net.DialTimeout("tcp", addr, 1*time.Second)
	if err != nil {
		return nil
	}
	defer conn.Close()
	return &Port{Num: port, Service: GuessService(port)}
}

func GuessService(port int) string {
	services := map[int]string{
		21:    "FTP",
		22:    "SSH",
		23:    "Telnet",
		25:    "SMTP",
		53:    "DNS",
		80:    "HTTP",
		110:   "POP3",
		143:   "IMAP",
		443:   "HTTPS",
		445:   "SMB",
		3306:  "MySQL",
		3389:  "RDP",
		5432:  "PostgreSQL",
		5900:  "VNC",
		8080:  "HTTP-Alt",
		27017: "MongoDB",
	}
	if s, ok := services[port]; ok {
		return s
	}
	return "Unknown"
}

func ScanRange(host string, start, end, threads int) []Port {
	var results []Port
	var mu sync.Mutex
	var wg sync.WaitGroup
	sem := make(chan struct{}, threads)

	for port := start; port <= end; port++ {
		wg.Add(1)
		go func(p int) {
			defer wg.Done()
			sem <- struct{}{}
			defer func() { <-sem }()

			if res := ScanPort(host, p); res != nil {
				mu.Lock()
				results = append(results, *res)
				mu.Unlock()
			}
		}(port)
	}
	wg.Wait()
	return results
}

func main() {
	host := flag.String("h", "", "Target host")
	start := flag.Int("s", 1, "Start port")
	end := flag.Int("e", 1000, "End port")
	threads := flag.Int("t", 50, "Threads")
	flag.Parse()

	if *host == "" {
		fmt.Println("Usage: recon -h <host> -s <start> -e <end> -t <threads>")
		return
	}

	fmt.Println("🔍 Port Scanner Started")
	fmt.Printf("Target: %s\nRange: %d-%d\nThreads: %d\n\n", *host, *start, *end, *threads)

	results := ScanRange(*host, *start, *end, *threads)

	sort.Slice(results, func(i, j int) bool {
		return results[i].Num < results[j].Num
	})

	fmt.Printf("\n✅ Found %d open ports:\n", len(results))
	fmt.Println("Port\tService")
	fmt.Println("════\t═══════")
	for _, p := range results {
		fmt.Printf("%d\t%s\n", p.Num, p.Service)
	}
}