# SHC-Tool

**Support Help CLI** - Ein vielseitiges Kommandozeilen-Tool für Systemadministration und Netzwerkdiagnose.

## 📋 Übersicht

SHC-Tool ist ein in Rust entwickeltes CLI-Tool, das Systemadministratoren bei alltäglichen Aufgaben unterstützt. Es bietet Funktionen für Netzwerkdiagnose, Log-Analyse mit KI-Unterstützung und grundlegende Sicherheitsscans.

## ✨ Features

- **🔍 IP-Informationen**: Detaillierte Anzeige von Netzwerk-Interfaces, IP-Adressen und Gateway-Informationen
- **🏓 Ping-Funktionalität**: Einfache Netzwerk-Konnektivitätsprüfung
- **📋 Log-Analyse**: Sammlung und KI-gestützte Analyse von System-Logs mit Ollama
- **🔍 Netzwerk-Scan**: Port-Scanning für Sicherheitsanalysen
- **🧪 System-Tests**: Integrierte Selbsttests für alle Komponenten

## 🚀 Installation

### Voraussetzungen

- Rust 1.70+ (Edition 2021)
- Optional: Ollama für KI-gestützte Log-Analyse

### Build von Source

```bash
git clone <repository-url>
cd SHC
cargo build --release
```

Die ausführbare Datei findest du dann in `target/release/SHC`.

## 🛠️ Verwendung

### Grundlegende Syntax

```bash
shc-tool <COMMAND> [OPTIONS]
```

### Verfügbare Befehle

#### 1. IP-Informationen anzeigen

```bash
shc-tool ip
```

Zeigt detaillierte Informationen über alle Netzwerk-Interfaces an:
- MAC-Adressen
- IPv4/IPv6-Adressen
- Gateway-Informationen
- DNS-Server
- Interface-Status und -Eigenschaften

#### 2. Ping-Test

```bash
shc-tool ping <TARGET>
```

**Beispiele:**
```bash
shc-tool ping google.com
shc-tool ping 8.8.8.8
```

#### 3. Log-Analyse

```bash
# Einfache Log-Übersicht
shc-tool logs

# Spezifische Log-Datei anzeigen
shc-tool logs --file /var/log/syslog

# KI-gestützte Analyse
shc-tool logs --analyze --query "Zeige mir alle Fehler der letzten Zeit"

# Erweiterte Optionen
shc-tool logs --analyze --query "Was ist mit dem SSH-Service?" --lines 500 --model gemma3:4b
```

**Log-Analyse Optionen:**
- `--zip`: Erstellt ZIP-Archiv der Log-Dateien (geplant)
- `--analyze`: Aktiviert KI-gestützte Analyse
- `--query <FRAGE>`: Spezifische Frage für die KI-Analyse
- `--lines <ANZAHL>`: Anzahl der zu analysierenden Zeilen (Standard: 200)
- `--model <MODELL>`: Ollama-Modell (Standard: gemma3:4b)
- `--file <PFAD>`: Spezifische Log-Datei analysieren

#### 4. Netzwerk-Scan

```bash
shc-tool scan <TARGET>
```

**Beispiele:**
```bash
shc-tool scan 192.168.1.1
shc-tool scan 10.0.0.0/24
```

Scannt häufig verwendete Ports und zeigt:
- Offene Ports und zugehörige Services
- Sicherheitswarnungen für kritische Services
- Scan-Zusammenfassung

#### 5. System-Tests

```bash
# Basis-Funktionalitätstest
shc-tool test

# Vollständiger Test aller Komponenten
shc-tool test --all
```

## 🤖 Ollama-Integration

Für die KI-gestützte Log-Analyse benötigst du Ollama:

### Ollama Setup

1. **Installation**: Folge der [Ollama-Dokumentation](https://ollama.ai/)

2. **Service starten**:
   ```bash
   ollama serve
   ```

3. **Modell installieren**:
   ```bash
   ollama pull gemma3:4b
   # oder ein anderes Modell deiner Wahl
   ```

### Unterstützte Modelle

- `gemma2:2b` (Standard, schnell)
- `gemma3:4b` (ausgewogen)
- Alle anderen von Ollama unterstützten Modelle

## 📁 Unterstützte Log-Dateien

Das Tool sucht automatisch nach folgenden Log-Dateien (Linux):
- `/var/log/syslog`
- `/var/log/auth.log`
- `/var/log/kern.log`
- `/var/log/messages`
- `/var/log/dmesg`

Du kannst auch spezifische Log-Dateien mit `--file` angeben.

## 🔧 Konfiguration

### Ollama-Konfiguration

Standardmäßig verwendet das Tool:
- **Server**: `http://localhost:11434`
- **Modell**: `gemma3:4b`

Diese können über Kommandozeilenparameter angepasst werden.

### Scan-Konfiguration

Der Netzwerk-Scan überprüft folgende Standard-Ports:
- 21 (FTP), 22 (SSH), 23 (Telnet)
- 25 (SMTP), 53 (DNS), 80 (HTTP), 443 (HTTPS)
- 110 (POP3), 143 (IMAP), 993 (IMAPS), 995 (POP3S)
- 135 (RPC), 139 (NetBIOS), 445 (SMB)
- 1433 (SQL Server), 3389 (RDP), 5432 (PostgreSQL)
- 5900 (VNC), 8080 (HTTP-Alt), 8443 (HTTPS-Alt)

## 🛡️ Sicherheitshinweise

- Das Tool führt nur passive Scans durch
- Verwende es nur in Netzwerken, für die du autorisiert bist
- Die Ollama-Integration verarbeitet Log-Daten lokal
- Keine Daten werden an externe Services gesendet (außer an lokales Ollama)

## 🐛 Troubleshooting

### Häufige Probleme

**Ollama nicht verfügbar:**
```bash
# Prüfe ob Ollama läuft
ollama serve

# Installiere ein Modell
ollama pull gemma3:4b
```

**Keine Berechtigung für Log-Dateien:**
```bash
# Führe mit sudo aus (Linux)
sudo shc-tool logs

# Oder spezifiziere eine lesbare Datei
shc-tool logs --file ./my-log.txt
```

**Ping-Befehl nicht gefunden:**
- Stelle sicher, dass `ping` installiert ist
- Unter Windows wird automatisch der Windows-ping-Befehl verwendet

## 📊 Beispiel-Ausgaben

### IP-Kommando
```
=== Netzwerk Interface Informationen ===

🔹 Standard Network Interface:
  Name: eth0
  Index: 2
  Typ: Ethernet
  Status: UP, RUNNING, MULTICAST
  MAC-Adresse: 00:1b:44:11:3a:b7
  IPv4-Adressen:
    - 192.168.1.100 (Netzmaske: 255.255.255.0)
  🚪 Standard-Gateway:
    MAC-Adresse: 00:1b:44:11:3a:01
    IPv4: [192.168.1.1]
```

### Log-Analyse
```bash
$ shc-tool logs --analyze --query "Gibt es SSH-Verbindungsprobleme?"

🤖 Analysiere Log-Daten mit gemma3:4b ... bitte warten.
✅ Analyse-Ergebnis:
Basierend auf den Log-Daten wurden mehrere fehlgeschlagene SSH-Verbindungsversuche 
von der IP 203.0.113.42 erkannt. Diese könnten auf einen Brute-Force-Angriff hindeuten.
Empfehlung: Überprüfe die SSH-Konfiguration und erwäge die Implementierung von fail2ban.
```

## 🤝 Beitragen

Beiträge sind willkommen! Bitte:

1. Fork das Repository
2. Erstelle einen Feature-Branch
3. Committe deine Änderungen
4. Erstelle einen Pull Request

## 📝 Lizenz

[Lizenz hier einfügen]

## 🔄 Version

**Aktuelle Version**: 0.1.1

### Changelog

- **0.1.1**: Erste funktionsfähige Version
  - IP-Informationen
  - Ping-Funktionalität
  - Log-Analyse mit Ollama
  - Netzwerk-Scanning
  - System-Tests

## 🎯 Geplante Features

- [ ] ZIP-Export für Log-Dateien
- [ ] Erweiterte Scan-Optionen // Ollama live Chat für Nachfragen
- [ ] Konfigurationsdatei-Support
- [ ] Windows-Event-Log-Support
- [ ] Monitoring-Dashboard
- [ ] Plugin-System

## 📞 Support

Bei Problemen oder Fragen:
1. Überprüfe die Troubleshooting-Sektion
2. Führe `shc-tool test --all` aus
3. Erstelle ein Issue mit der Ausgabe des Test-Befehls
