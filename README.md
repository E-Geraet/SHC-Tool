# SHC-Tool

**Support Help CLI** - A versatile command-line tool for system administration and network diagnostics.

## Overview

SHC-Tool is a CLI tool developed in Rust that helps system administrators with everyday tasks. It provides functions for network diagnostics, log analysis with AI support, and basic security scans.

## Features

- **IP Information**: Detailed display of network interfaces, IP addresses and gateway information
- **Ping Functionality**: Simple network connectivity testing
- **Log Analysis**: Collection and AI-assisted analysis of system logs with Ollama
- **Network Scanning**: Port scanning for security analysis
- **System Tests**: Integrated self-tests for all components

## Installation

### Prerequisites

- Rust 1.70+ (Edition 2021)
- Optional: Ollama for AI-assisted log analysis

### Build from Source

```bash
git clone <repository-url>
cd SHC
cargo build --release
```

You'll find the executable in `target/release/SHC`.

## Usage

### Basic Syntax

```bash
shc-tool <COMMAND> [OPTIONS]
```

### Available Commands

#### 1. Show IP Information

```bash
shc-tool ip
```

Shows detailed information about all network interfaces:
- MAC addresses
- IPv4/IPv6 addresses
- Gateway information
- DNS servers
- Interface status and properties

#### 2. Ping Test

```bash
shc-tool ping <TARGET>
```

**Examples:**
```bash
shc-tool ping google.com
shc-tool ping 8.8.8.8
```

#### 3. Log Analysis

```bash
# Simple log overview
shc-tool logs

# Show specific log file
shc-tool logs --file /var/log/syslog

# AI-assisted analysis
shc-tool logs --analyze --query "Show me all recent errors"

# Advanced options
shc-tool logs --analyze --query "What's wrong with the SSH service?" --lines 500 --model gemma3:4b
```

**Log Analysis Options:**
- `--zip`: Create ZIP archive of log files (planned)
- `--analyze`: Enable AI-assisted analysis
- `--query <QUESTION>`: Specific question for AI analysis
- `--lines <NUMBER>`: Number of lines to analyze (default: 200)
- `--model <MODEL>`: Ollama model (default: gemma3:4b)
- `--file <PATH>`: Analyze specific log file

#### 4. Network Scan

```bash
shc-tool scan <TARGET>
```

**Examples:**
```bash
shc-tool scan 192.168.1.1
shc-tool scan 10.0.0.0/24
```

Scans commonly used ports and shows:
- Open ports and associated services
- Security warnings for critical services
- Scan summary

#### 5. System Tests

```bash
# Basic functionality test
shc-tool test

# Complete test of all components
shc-tool test --all
```

## Ollama Integration

For AI-assisted log analysis you need Ollama:

### Ollama Setup

1. **Installation**: Follow the [Ollama documentation](https://ollama.ai/)

2. **Start service**:
   ```bash
   ollama serve
   ```

3. **Install model**:
   ```bash
   ollama pull gemma3:4b
   # or another model of your choice
   ```

### Supported Models

- `gemma2:2b` (default, fast)
- `gemma3:4b` (balanced)
- All other models supported by Ollama

## Supported Log Files

The tool automatically searches for the following log files (Linux):
- `/var/log/syslog`
- `/var/log/auth.log`
- `/var/log/kern.log`
- `/var/log/messages`
- `/var/log/dmesg`

You can also specify specific log files with `--file`.

## Configuration

### Ollama Configuration

By default, the tool uses:
- **Server**: `http://localhost:11434`
- **Model**: `gemma3:4b`

These can be adjusted via command line parameters.

### Scan Configuration

The network scan checks the following default ports:
- 21 (FTP), 22 (SSH), 23 (Telnet)
- 25 (SMTP), 53 (DNS), 80 (HTTP), 443 (HTTPS)
- 110 (POP3), 143 (IMAP), 993 (IMAPS), 995 (POP3S)
- 135 (RPC), 139 (NetBIOS), 445 (SMB)
- 1433 (SQL Server), 3389 (RDP), 5432 (PostgreSQL)
- 5900 (VNC), 8080 (HTTP-Alt), 8443 (HTTPS-Alt)

## Security Notes

- The tool only performs passive scans
- Only use it on networks you're authorized for
- Ollama integration processes log data locally
- No data is sent to external services (except local Ollama)

## Troubleshooting

### Common Issues

**Ollama not available:**
```bash
# Check if Ollama is running
ollama serve

# Install a model
ollama pull gemma3:4b
```

**No permission for log files:**
```bash
# Run with sudo (Linux)
sudo shc-tool logs

# Or specify a readable file
shc-tool logs --file ./my-log.txt
```

**Ping command not found:**
- Make sure `ping` is installed
- On Windows, the Windows ping command is used automatically

## Example Outputs

### IP Command
```
=== Network Interface Information ===

Standard Network Interface:
  Name: eth0
  Index: 2
  Type: Ethernet
  Status: UP, RUNNING, MULTICAST
  MAC Address: 00:1b:44:11:3a:b7
  IPv4 Addresses:
    - 192.168.1.100 (Netmask: 255.255.255.0)
  Default Gateway:
    MAC Address: 00:1b:44:11:3a:01
    IPv4: [192.168.1.1]
```

### Log Analysis
```bash
$ shc-tool logs --analyze --query "Are there SSH connection problems?"

Analyzing log data with gemma3:4b ... please wait.
Analysis Result:
Based on the log data, multiple failed SSH connection attempts from IP 203.0.113.42 
were detected. This could indicate a brute-force attack.
Recommendation: Check SSH configuration and consider implementing fail2ban.
```

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Create a pull request

## License

[Insert license here]

## Version

**Current Version**: 0.1.1

### Changelog

- **0.1.1**: First working version
  - IP information
  - Ping functionality
  - Log analysis with Ollama
  - Network scanning
  - System tests

## Planned Features

- [ ] ZIP export for log files
- [ ] Extended scan options // Ollama live chat for follow-up questions
- [ ] Configuration file support
- [ ] Windows Event Log support
- [ ] Monitoring dashboard
- [ ] Plugin system

## Support

For problems or questions:
1. Check the troubleshooting section
2. Run `shc-tool test --all`
3. Create an issue with the output of the test command
