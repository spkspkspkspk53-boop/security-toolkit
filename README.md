# security-toolkit

Advanced Security Tools Suite - IDS/IPS, Credential Testing, Recon, Packet Crafting

## Tools

1. **IDS/IPS** (Go) - Real-time network monitoring, anomaly detection, IP blocking
2. **Credential Tester** (Rust) - Multi-protocol brute force (SSH, HTTP)
3. **Port Scanner** (Go) - Fast port scanning, service detection, OS fingerprinting
4. **Packet Crafter** (Rust) - Custom packet builder (SYN, ACK)

## Build & Run

```bash
# Build all
cd go-idsips && go build
cd ../rust-cred && cargo build --release
cd ../go-recon && go build
cd ../rust-packet && cargo build --release

# Usage
./go-idsips/idsips -i eth0
./rust-cred/target/release/cred-tester <host> <port> <proto> <wordlist>
./go-recon/recon -h <host> -s 1 -e 1000 -t 50
./rust-packet/target/release/packet-crafter <src> <dst> <sport> <dport>
