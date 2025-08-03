# three-body-rs

Ein einfaches Rust-Projekt zur Simulation des **Drei-Körper-Problems** – einem klassischen Problem der Himmelsmechanik.

## 🔭 Was ist das Drei-Körper-Problem?

Das Drei-Körper-Problem beschreibt das Verhalten von drei Körpern, die sich gegenseitig durch Gravitation beeinflussen.
Anders als das Zwei-Körper-Problem, welches einen exakten analytischen Lösungsweg besitzt,
ist das Drei-Körper-Problem **nicht exakt lösbar** – kleine Änderungen in den Anfangsbedingungen führen zu chaotischem Verhalten.

In diesem Projekt wird das Problem numerisch gelöst, d.h. die Bewegungen werden in kleinen Zeitschritten iterativ berechnet.

## 📖 Inspiration: *Die drei Sonnen*

Der chinesische Sci-Fi-Roman [*Die drei Sonnen* (原名: 三体, *The Three-Body Problem*)](https://en.wikipedia.org/wiki/The_Three-Body_Problem_(novel)) von **Liu Cixin** nutzt das Drei-Körper-Problem als zentrales Motiv. Im Roman lebt eine außerirdische Zivilisation auf dem Planeten **Trisolaris**, der sich in einem chaotischen Sternensystem mit drei Sonnen befindet. Die Bewohner erleben daher extreme, unvorhersehbare Klimazyklen – ein direkter Bezug zur unlösbaren Natur des physikalischen Drei-Körper-Problems.

## 🧮 Mathematischer Ansatz

Das Problem wurde zunächst auf das Zwei-Körper-Problem runtergebrochen.
Das Projekt simuliert die Bewegungen von drei Himmelskörpern auf Basis der klassischen Newtonschen Mechanik:

- Jeder Körper erfährt eine Gravitationskraft von den beiden anderen Körpern:  
  $$
  \vec{F}_{ij} = G \cdot \frac{m_i \cdot m_j}{|\vec{r}_i - \vec{r}_j|^2} \cdot \hat{r}_{ij}
  $$

- Die Beschleunigungen werden aus der Kraft berechnet, und über numerische Integration (mittels Euler Verfahren) werden Geschwindigkeit
  und Position fortlaufend aktualisiert.

Dieses Projekt verwendet eine einfache Integrationsmethode, um den chaotischen Verlauf zu visualisieren und das Verhalten in einer 2D-Umgebung zu demonstrieren.
Je mehr Frames generiert werden desto genauer ist der Ansatz.

## 🚀 Projektstruktur

- `src/main.rs`: Hauptlogik der Simulation
- `body.rs`: Definition der physikalischen Körper und ihrer Eigenschaften
- `calculate.rs`: Einfache Vektor-Mathematik für Positionen, Geschwindigkeiten, Kräfte
- `api.rs`: Einfache nebensächliche Bevy implementierungen
- `camera.rs`: Kamera Logik

## 🧮 Mathematischer Ansatz

Weitere möglichkeiten um dieses Simulation zu erweitern wären adaptive Frame Anpassungen um Resourcen zu sparen,
oder Wahrscheinlichkeitsinseln aufzustellen um eine genauere Vorhersage zu treffen.

## ▶️ Ausführen

```bash
git clone https://github.com/Cajex/three-body-rs
cd three-body-rs
cargo run