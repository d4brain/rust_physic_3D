# Rust Physik-Labor 3D

Eine eigenstaendige Desktop-App mit Bevy 0.18.0 und Rapier 0.33.0.
Keine Bilder, Modelle oder sonstigen externen Assets erforderlich.

## Start unter Windows

1. ZIP vollstaendig entpacken, zum Beispiel nach D:\Rust.
2. Den entpackten Ordner physik_3d oeffnen.
3. STARTEN.bat doppelklicken.

Alternativ PowerShell in diesem Ordner oeffnen und ausfuehren:

    cargo run --release

Rust und die Visual Studio C++ Build Tools muessen installiert sein.
Falls Rust zu alt ist: `rustup update stable`.
Beim ersten Start werden die Bibliotheken aus dem Internet heruntergeladen.
Der erste Build kann mehrere Minuten dauern und mehrere GB belegen.

## Bedienung

- Linke Maustaste: die vier grossen Buttons bedienen.
- 1: Wuerfel erzeugen.
- 2: Kugel erzeugen.
- Leertaste: Pause / Weiter.
- R: Anfangsszene neu aufbauen und Simulation fortsetzen.
- Pfeil links/rechts: Kamera um die Szene drehen.
- Pfeil oben/unten: Kamerahoehe aendern.
- Ziffernblock + / -: zoomen; alternativ Equal/Minus-Tasten.
- Fenster schliessen: beenden.

Die Szene startet mit 30 Wuerfeln und sechs fallenden Kugeln.
Schwerkraft, Kollisionen, Reibung, Rotation und Rueckprall werden mit Rapier berechnet.
Der Boden und die niedrigen Randwaende sind statisch.
Maximal 250 dynamische Objekte. Aus der Szene gefallene Objekte werden entfernt.
Waehrend Pause kannst du Objekte hinzufuegen; sie fallen erst bei Weiter.
Der Zustand wird nicht gespeichert.

## EXE erstellen

    cargo build --release

Danach liegt die Anwendung unter target\release\physik_3d.exe.
Das ist ein neues Projekt. Keine Dateien deines hello_welt-Projekts ersetzen.
Alle Abhaengigkeiten sind bereits in Cargo.toml eingetragen.

## Dateien

- src/main.rs: Simulation, 3D-Szene, Kamera, Buttons.
- Cargo.toml: Projektdefinition und Abhaengigkeiten.
- STARTEN.bat: Start per Doppelklick.

## Pruefstand

Die direkten Bibliotheksversionen sind auf eine laut Rapier-Dokumentation
kompatible Kombination festgelegt. In der Erstellungsumgebung stand kein
Rust-Compiler zur Verfuegung: Build und grafischer Lauf sind noch nicht getestet.
Cargo.lock wird beim ersten erfolgreichen Cargo-Aufruf erzeugt.
