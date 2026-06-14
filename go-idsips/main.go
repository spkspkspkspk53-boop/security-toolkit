package main

import (
	"flag"
	"fmt"
	"log"
	"net"
	"sync"
	"time"
)

type Alert struct {
	Timestamp time.Time
	SrcIP     string
	DstIP     string
	Reason    string
}

type IDS struct {
	alerts  chan Alert
	blocked map[string]int64
	mu      sync.RWMutex
}

func NewIDS() *IDS {
	return &IDS{
		alerts:  make(chan Alert, 100),
		blocked: make(map[string]int64),
	}
}

func (i *IDS) BlockIP(ip string, duration time.Duration) {
	i.mu.Lock()
	defer i.mu.Unlock()
	i.blocked[ip] = time.Now().Add(duration).Unix()
}

func (i *IDS) IsBlocked(ip string) bool {
	i.mu.RLock()
	defer i.mu.RUnlock()
	if deadline, ok := i.blocked[ip]; ok {
		if time.Now().Unix() < deadline {
			return true
		}
	}
	return false
}

func (i *IDS) MonitorIP(ip string) {
	for {
		if conn, err := net.DialTimeout("tcp", ip+":22", 2*time.Second); err == nil {
			conn.Close()
			i.alerts <- Alert{
				Timestamp: time.Now(),
				SrcIP:     ip,
				Reason:    "SSH accessible",
			}
		}
		time.Sleep(5 * time.Second)
	}
}

func (i *IDS) PrintAlerts() {
	for alert := range i.alerts {
		if i.IsBlocked(alert.SrcIP) {
			fmt.Printf("🔴 [BLOCKED] %s: %s\n", alert.Timestamp.Format("15:04:05"), alert.Reason)
		} else {
			fmt.Printf("🚨 [ALERT] %s: %s\n", alert.Timestamp.Format("15:04:05"), alert.Reason)
			i.BlockIP(alert.SrcIP, 5*time.Minute)
		}
	}
}

func main() {
	ip := flag.String("t", "127.0.0.1", "Target IP")
	flag.Parse()

	ids := NewIDS()
	fmt.Println("🛡️  IDS/IPS Started")
	fmt.Printf("Monitoring: %s\n\n", *ip)

	go ids.MonitorIP(*ip)
	ids.PrintAlerts()
}