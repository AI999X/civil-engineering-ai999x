# civil-engineering-ai999x: offline, verified civil engineering solver with step-by-step answers

[![tests](https://github.com/AI999X/civil-engineering-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/civil-engineering-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/civil-engineering-ai999x)](https://pypi.org/project/civil-engineering-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/civil-engineering-ai999x)

**[Desktop app & command line](https://www.ai999x.com/desktop-cli/)** · **[Compute in your browser](https://www.ai999x.com/models/civil-engineering-ai999x/#try)** · **[Install (Python)](https://pypi.org/project/civil-engineering-ai999x/)** · **[Guides](https://huggingface.co/AI999X/civil-engineering-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/civil-engineering-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.6.0

Field: **Engineering** · Tasks: Question Answering

civil-engineering-ai999x is an offline, deterministic civil engineering solver written in Rust: a desktop app, a command-line program, a browser module and a Python package. It answers civil engineering questions with verified results from 88 certified capabilities, gives 566 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**civil-engineering-ai999x** reads a question in plain English (or a typed call), picks a certified civil engineering capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions, or a refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green) ![Rust](https://img.shields.io/badge/Rust-Windows%20%7C%20Linux%20%7C%20WebAssembly-orange)

**Run it your way.** civil-engineering-ai999x comes as the AI999X desktop app and as the command-line program `civil-engineering` for Windows and Linux, runs in your browser as WebAssembly, and installs as a Python package. The desktop app, the command line and the browser run the same compiled Rust program on your own device: offline, no Python needed.

## Get it

| Way to run it | For | Get it |
|---|---|---|
| **AI999X Civil Engineering desktop app** 1.5.0 | Windows, Linux. This model in its own app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-civil-engineering-1.5.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-civil-engineering-1.5.0-linux-x86_64.tar.gz) |
| **AI999X desktop app** 1.5.0 | Windows, Linux. Every AI999X model in one app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-1.5.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-1.5.0-linux-x86_64.tar.gz) |
| **Command line** (`civil-engineering`) | Windows, Linux. One program per model, offline, no Python | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-linux-x86_64.tar.gz) |
| **In your browser** | any device. Nothing to install | [Compute on the model page](https://www.ai999x.com/models/civil-engineering-ai999x/#try) |
| **Python** | Python 3.10+ | `pip install civil-engineering-ai999x` |

Checksums: [SHA256SUMS](https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS) · every version: [https://downloads.ai999x.com/desktop/latest/](https://downloads.ai999x.com/desktop/latest/) · install guide: [desktop & command line](https://www.ai999x.com/desktop-cli/)

## Quick start

### Desktop app

Download **AI999X Civil Engineering** for your system (above), unpack it and start it; type a question in words or pick a capability, fill in the form and press Compute. Figures and an explanation of the answer appear beside it. The AI999X desktop app has the same, with Civil Engineering among all AI999X models.

Linux, from a terminal:

```bash
curl -LO https://downloads.ai999x.com/desktop/1.5.0/ai999x-civil-engineering-1.5.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-civil-engineering-1.5.0-linux-x86_64.tar.gz
./ai999x-civil-engineering-1.5.0-linux-x86_64/ai999x-civil-engineering
```

### Command line

Linux:

```bash
curl -LO https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
mkdir -p ~/.local/share && tar xzf ai999x-cli-1.5.0-linux-x86_64.tar.gz -C ~/.local/share
echo 'export PATH="$HOME/.local/share/ai999x-cli-1.5.0-linux-x86_64:$PATH"' >> ~/.bashrc && source ~/.bashrc
civil-engineering --version
```

Windows (PowerShell):

```powershell
$F = "ai999x-cli-1.5.0-windows-x86_64.zip"
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.5.0/$F" -OutFile $F
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS" -OutFile SHA256SUMS
(Get-FileHash $F -Algorithm SHA256).Hash.ToLower() -eq ((Select-String -SimpleMatch $F SHA256SUMS).Line -split ' ')[0]
Expand-Archive $F -DestinationPath "$env:LOCALAPPDATA\AI999X" -Force
$D = "$env:LOCALAPPDATA\AI999X\ai999x-cli-1.5.0-windows-x86_64"; $env:Path += ";$D"
[Environment]::SetEnvironmentVariable("Path", [Environment]::GetEnvironmentVariable("Path", "User") + ";$D", "User")
civil-engineering --version
```

Every command below was run as shown; the text under it is what the program printed.

**Ask in words**: Write the question as you would say it: the program reads the quantities and their units, picks the capability and answers.

```console
$ civil-engineering ask "Manning discharge in a rectangular channel with b = 3 m, y = 1.2 m, S = 0.001 and n = 0.013."
Read as: Manning discharge in a rectangular channel
Q = 6.682900203 m^3/s
V = 1.856361167 m/s
A = 3.6 m^2
R = 0.6666666667 m
  by: Manning discharge in a rectangular channel  (A = b y, R = A/(b + 2y), Q = A R^(2/3) S^(1/2)/n)
  b = 3 m
  y = 1.2 m
  S = 0.001
  n = 0.013
  assumes: b, y > 0; 0 < S < 1; 0.005 <= n <= 0.2
```

**What a capability needs**: The equation or method, every input and output with its SI unit. Read this before solve: it tells you the input names.

```console
$ civil-engineering info manning_rectangular_discharge
Manning discharge in a rectangular channel
  A = b y, R = A/(b + 2y), Q = A R^(2/3) S^(1/2)/n
  in   b        bottom width [m]
  in   y        flow depth [m]
  in   S        bed slope, m/m []
  in   n        Manning roughness coefficient []
  out  Q        discharge (m^3/s) [m^3/s]
  out  V        mean velocity (m/s) [m/s]
  out  A        flow area (m^2) [m^2]
  out  R        hydraulic radius (m) [m]
```

**Solve**: Inputs as name=value, each with any unit of the right kind. The answer lists every output, the method used, each input converted to SI, and the conditions the answer assumes.

```console
$ civil-engineering solve manning_rectangular_discharge "b=3 m" "y=1.2 m" S=0.001 n=0.013
Q = 6.682900203 m^3/s
V = 1.856361167 m/s
A = 3.6 m^2
R = 0.6666666667 m
  by: Manning discharge in a rectangular channel  (A = b y, R = A/(b + 2y), Q = A R^(2/3) S^(1/2)/n)
  b = 3 m
  y = 1.2 m
  S = 0.001
  n = 0.013
  assumes: b, y > 0; 0 < S < 1; 0.005 <= n <= 0.2
```

**Many questions at once**: One JSON request per line in, one JSON answer per line out, in order: for scripts, notebooks and agents (the program starts once). In PowerShell: Get-Content cases.jsonl | civil-engineering batch

`cases.jsonl`:

```json
{"key": "manning_rectangular_discharge", "inputs": {"b": "3 m", "y": "1.2 m", "S": 0.001, "n": 0.013}}
{"key": "theis_drawdown", "inputs": {"Q": "0.02893518519 m^3/s", "T": "0.005787037037 m^2/s", "S": 0.0002, "r": "100 m", "t": "86400 s"}}
```

```console
$ civil-engineering batch < cases.jsonl
{"outcome":"SOLVED","message":"","value":6.68290020277,"unit":"m^3/s","solved_for":"Q","si":[["b",3.0],["y",1.2],["S",0.001],["n",0.013]],"values":[["Q",6.68290020277,"m^3/s"],["V",1.85636116744,"m/s"],["A",3.6,"m^2"],["R",0.666666666667,"m"]],"target":"Q","phrase":"discharge (m^3/s)","assumptions":["b, y > 0","0 < S < 1","0.005 <= n <= 0.2"],"name":"Manning discharge in a rectangular channel","equation":"A = b y, R = A/(b + 2y), Q = A R^(2/3) S^(1/2)/n"}
{"outcome":"SOLVED","message":"","value":2.51923946839,"unit":"m","solved_for":"s","si":[["Q",0.02893518519],["T",0.005787037037],["S",0.0002],["r",100.0],["t",86400.0]],"values":[["s",2.51923946839,"m"],["u",0.00100000000001,""],["W",6.33153936413,""]],"target":"s","phrase":"drawdown (m)","assumptions":["confined, homogeneous, infinite aquifer","fully penetrating well, constant rate","u <= 50"],"name":"Theis drawdown around a pumped well","equation":"u = r^2 S/(4 T t), W(u) = E1(u) (series for u <= 1, continued fraction above), s = Q W/(4 pi T)"}
```

### In your browser

Open [Civil Engineering on ai999x.com](https://www.ai999x.com/models/civil-engineering-ai999x/#try), choose a capability and press **Compute**: the model runs on your own device as WebAssembly; nothing is sent anywhere. Change any input (any unit of the right kind) and the answer, its explanation and the figures follow.

In your own page: `civil_engineering_ai999x.wasm` and the loader `ax-wasm.js` (https://www.ai999x.com/compute/rust/ax-wasm.js).

### Python

```bash
pip install civil-engineering-ai999x
```

## Worked examples in every form

The same worked example from the command line, from Python and in the browser. Each command-line output is what the program printed.

### Coulomb active thrust on a rough wall with sloping backfill

Coulomb active earth pressure coefficient and thrust per metre on a rough vertical wall (wall friction delta) retaining a cohesionless backfill sloping at beta, with the horizontal and vertical components of the thrust.

Command line:

```console
$ civil-engineering solve coulomb_active_thrust phi=30 delta=20 beta=10 "gam=18000 N/m^3" "H=6 m"
Ka = 0.3400223954
Pa = 110167.2561 N/m
Pa_h = 103523.3576 N/m
Pa_v = 37679.42072 N/m
  by: Coulomb active thrust on a rough wall with sloping backfill  (Ka = cos^2 phi/(cos delta [1 + sqrt(sin(phi + delta) sin(phi - beta)/(cos delta cos beta))]^2), Pa = Ka gam H^2/2)
  phi = 30
  delta = 20
  beta = 10
  gam = 18000 N/m^3
  H = 6 m
  assumes: vertical wall back; cohesionless dry backfill; 0 <= delta <= phi; 0 <= beta < phi
```

Python:

```python
import civil_engineering_ai999x as m

a = m.solve("coulomb_active_thrust", phi="30", delta="20", beta="10", gam="18000 N/m^3", H="6 m")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/civil-engineering-ai999x/#cap-coulomb_active_thrust) and press **Compute in your browser**.

_What this means:_ This calculation determines the Coulomb active earth pressure coefficient and the resulting thrust on a rough retaining wall. Using the backfill friction angle of 30, wall friction angle of 20, and slope angle of 10, the method computes a coefficient of 0.340022. This coefficient, combined with the unit weight 18000 N/m^3 and height 6 m, yields a total thrust per metre of 110167 N/m N/m. The thrust acts at an angle equal to the wall friction, so it is resolved into a horizontal push of 103523 N/m N/m and a downward vertical force of 37679.4 N/m N/m.

### Coulomb passive thrust on a rough wall

Coulomb (plane-wedge) passive earth pressure coefficient and thrust per metre on a rough vertical wall with wall friction delta and a soil surface rising at beta (plane wedges overestimate the passive resistance when delta is large; keep delta below about phi/2 in design).

Command line:

```console
$ civil-engineering solve coulomb_passive_thrust phi=30 delta=15 beta=0 "gam=18000 N/m^3" "H=2 m"
Kp = 4.976500223
Pp = 179154.008 N/m
Pp_h = 173049.4832 N/m
Pp_v = 46368.46928 N/m
  by: Coulomb passive thrust on a rough wall  (Kp = cos^2 phi/(cos delta [1 - sqrt(sin(phi + delta) sin(phi + beta)/(cos delta cos beta))]^2), Pp = Kp gam H^2/2)
  phi = 30
  delta = 15
  beta = 0
  gam = 18000 N/m^3
  H = 2 m
  assumes: vertical wall back; cohesionless dry soil; plane wedge (phi + delta + beta < 90)
```

Python:

```python
import civil_engineering_ai999x as m

a = m.solve("coulomb_passive_thrust", phi="30", delta="15", beta="0", gam="18000 N/m^3", H="2 m")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/civil-engineering-ai999x/#cap-coulomb_passive_thrust) and press **Compute in your browser**.

_What this means:_ This calculation determines the passive earth resistance on a rough vertical wall retaining cohesionless dry soil with a sloping backface. Using the plane wedge method, it computes the passive coefficient Kp as 4.9765, which scales the unit weight 18000 N/m^3 and height 2 m to find the total thrust Pp of 179154 N/m N/m. The resulting force acts at an angle delta relative to the normal, yielding a horizontal component of 173049 N/m N/m and a vertical component of 46368.5 N/m N/m. This value is critical for assessing the stability of anchored walls and determining required embedment depths for sheet piles.

### Rankine active earth pressure and thrust on a wall

Rankine active pressure coefficient, active pressure at the base, total active thrust per metre of a smooth vertical wall retaining a horizontal backfill (with optional cohesion and surcharge; the tension zone is ignored), the tension-crack depth and the height of the resultant above the base. Dry backfill.

Command line:

```console
$ civil-engineering solve rankine_active_thrust phi=30 "gam=18000 N/m^3" "H=6 m"
Ka = 0.3333333333
sigma_base = 36000 Pa
Pa = 108000 N/m
z_crack = 0 m
ybar = 2 m
  by: Rankine active earth pressure and thrust on a wall  (Ka = (1 - sin phi)/(1 + sin phi); sigma_a = Ka(gam z + q) - 2c sqrt(Ka); Pa = area of the positive pressure diagram)
  phi = 30
  gam = 18000 N/m^3
  H = 6 m
  assumes: smooth vertical wall, horizontal backfill; dry backfill; tension zone ignored
```

Python:

```python
import civil_engineering_ai999x as m

a = m.solve("rankine_active_thrust", phi="30", gam="18000 N/m^3", H="6 m")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/civil-engineering-ai999x/#cap-rankine_active_thrust) and press **Compute in your browser**.

_What this means:_ The active thrust is 108000 N/m per metre, with a base pressure of 36000 Pa.

### Rankine passive earth pressure and thrust

Rankine passive pressure coefficient, passive pressure at the base, total passive thrust per metre on a smooth vertical wall with a horizontal soil surface (optional cohesion and surcharge) and the height of the resultant.

Command line:

```console
$ civil-engineering solve rankine_passive_thrust phi=32 "gam=18000 N/m^3" "H=2 m"
Kp = 3.254588303
sigma_base = 117165.1789 Pa
Pp = 117165.1789 N/m
ybar = 0.6666666667 m
  by: Rankine passive earth pressure and thrust  (Kp = (1 + sin phi)/(1 - sin phi); sigma_p = Kp(gam z + q) + 2c sqrt(Kp); Pp = area of the pressure diagram)
  phi = 32
  gam = 18000 N/m^3
  H = 2 m
  assumes: smooth vertical wall, horizontal soil surface; dry soil
```

Python:

```python
import civil_engineering_ai999x as m

a = m.solve("rankine_passive_thrust", phi="32", gam="18000 N/m^3", H="2 m")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/civil-engineering-ai999x/#cap-rankine_passive_thrust) and press **Compute in your browser**.

_What this means:_ This calculation determines the resistance provided by the soil when a wall moves towards it. Using a friction angle of 32 degrees, the passive coefficient is found to be 3.25459. For a wall height of 2 m metres with unit weight 18000 N/m^3, the pressure at the base reaches 117165 Pa. The total horizontal force resisting movement is calculated as 117165 N/m newtons per metre of wall length, acting at a height of 0.666667 m metres above the base.

## Documentation

- **Rust user guide**: the command line, every capability with a command and what it printed: [PDF](https://huggingface.co/AI999X/civil-engineering-ai999x/blob/main/docs/civil_engineering_ai999x_rust_guide.pdf) · [Markdown](https://huggingface.co/AI999X/civil-engineering-ai999x/blob/main/docs/civil_engineering_ai999x_rust_guide.md)
- **Python guide**: every capability with a Python example: [PDF](https://huggingface.co/AI999X/civil-engineering-ai999x/blob/main/docs/civil_engineering_ai999x_capabilities.pdf) · [Markdown](https://huggingface.co/AI999X/civil-engineering-ai999x/blob/main/docs/civil_engineering_ai999x_capabilities.md)
- **Desktop app & command line**: downloads, install, ask and search, figures, offline use, checksums: https://www.ai999x.com/desktop-cli/
- **Model page**: compute in your browser, every capability explained: https://www.ai999x.com/models/civil-engineering-ai999x/

## Highlights

- **Verified, not generated.** Every answer is computed by a certified civil engineering capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes (SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR), never a silent guess. Every one of its 566 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Native and in the browser.** Every capability also computes natively in Rust: the desktop app and the `civil-engineering` command line for Windows and Linux, and WebAssembly on the model page, no Python, no network.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10, 3.11, 3.12, 3.13, 3.14).
- **Coverage:** open-channel hydraulics, pipe networks, hydrology, groundwater flow, soil mechanics and foundations, and surveying.

## Model overview

| Property | Value |
|---|---|
| Package | `civil-engineering-ai999x` 1.6.0 (import `civil_engineering_ai999x`) |
| Type | Deterministic neuro-symbolic civil engineering engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 88 certified procedures |
| Knowledge pack | version 1.0.1 |
| Golden answers | 566 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python 3.10, 3.11, 3.12, 3.13, 3.14) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Python quick start

```bash
pip install civil-engineering-ai999x
```

```python
import civil_engineering_ai999x as civil_engineering

civil_engineering.ask("Manning discharge in a rectangular channel with b = 3 m, y = 1.2 m, S = 0.001 and n = 0.013.").show()
# -> SOLVED — Q = 6.6829, V = 1.85636, A = 3.6, R = 0.666667  (manning_rectangular_discharge)
civil_engineering.ask("Theis drawdown with pumping rate = 2500 m^3/d, transmissivity = 500 m^2/d, storativity = 0.0002, distance = 100 m and time = 24 h.").show()
# -> SOLVED — s = 2.51924, u = 0.001, W = 6.33154  (theis_drawdown)
civil_engineering.solve("consolidation_settlement_nc", Cc=0.32, e0=0.92, H="4 m", s0="80 kPa", ds="60 kPa").show()
# -> SOLVED — Sc = 0.162025, de = 0.0777722, e_final = 0.842228  (consolidation_settlement_nc)
civil_engineering.ask("Hardy cross network with pipes = [1-2, 2-3, 3-4, 4-1, 2-4], L = [600, 500, 600, 500, 780], D = [300, 250, 200, 250, 200], C = [120, 120, 120, 120, 120] and demands = [0, 0.04, 0.05, 0.03].").show()
# -> SOLVED — flows = [0.0714399845267, 0.0331343839504, -0.0168656160496, -0.0485600154733, -0.00169439942369], head_loss = [0.0, 2.39874834362, 3.56973630784, 2.37675423164], loops = 2, iterations = 14  (hardy_cross_network)
civil_engineering.ask("Horizontal circular curve with radius = 300 m, deflection angle = 40 degrees and PI chainage = 1250 m.").show()
# -> SOLVED — T = 109.191, L = 209.44, LC = 205.212, E = 19.2533  (horizontal_circular_curve)
civil_engineering.ask("Who created you?").message
# -> civil-engineering-ai999x was created and engineered by Ibnul Jaif Farabi at AI999X.
```

The Python package also installs a Python command, `civil-engineering-ai999x "<question>"`; for the native program use `civil-engineering` (above). The outputs above were produced by running this exact code on civil-engineering-ai999x 1.6.0.

## How every answer is verified

1. **Reading**: the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning**: only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing**: exact symbolic or high-precision numeric evaluation.
4. **Checking**: dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Python worked examples, with their steps

Each question was put to `civil-engineering-ai999x`; the answer is the one it returned, with the steps it took.

**Manning discharge in a rectangular channel** · _Open-channel hydraulics_

> Manning discharge in a rectangular channel with b = 3 m, y = 1.2 m, S = 0.001 and n = 0.013.

→ **Q = 6.682900203 m^3/s, V = 1.856361167 m/s, A = 3.6 m^2, R = 0.6666666667 m**

<details><summary>Steps</summary>

- Manning discharge in a rectangular channel: b = 3 m, y = 1.2 m, S = 0.001, n = 0.013 → Q = 6.682900203 m^3/s, V = 1.856361167 m/s, A = 3.6 m^2, R = 0.6666666667 m

</details>

**Theis drawdown around a pumped well** · _Groundwater_

> Theis drawdown with pumping rate = 2500 m^3/d, transmissivity = 500 m^2/d, storativity = 0.0002, distance = 100 m and time = 24 h.

→ **s = 2.519239468 m, u = 0.001, W = 6.331539364**

<details><summary>Steps</summary>

- Theis drawdown around a pumped well: Q = 0.02893518519 m^3/s, T = 0.005787037037 m^2/s, S = 0.0002, r = 100 m, t = 86400 s → s = 2.519239468 m, u = 0.001 m, W = 6.331539364 m

</details>

**Primary consolidation settlement of a normally consolidated clay** · _Soil mechanics_

`m.solve("consolidation_settlement_nc", Cc=0.32, e0=0.92, H="4 m", s0="80 kPa", ds="60 kPa")`

→ **Sc = 0.1620253658 m, de = 0.07777217558, e_final = 0.8422278244**

<details><summary>Steps</summary>

- Primary consolidation settlement of a normally consolidated clay: Cc = 0.32, e0 = 0.92, H = 4 m, s0 = 80 kPa, ds = 60 kPa → Sc = 0.1620253658 m, de = 0.07777217558 m, e_final = 0.8422278244 m

</details>

**Pipe network flows by the Hardy Cross method (Hazen-Williams)** · _Pipe flow and water networks_

> Hardy cross network with pipes = [1-2, 2-3, 3-4, 4-1, 2-4], L = [600, 500, 600, 500, 780], D = [300, 250, 200, 250, 200], C = [120, 120, 120, 120, 120] and demands = [0, 0.04, 0.05, 0.03].

→ **flows = [0.07143998453, 0.03313438395, -0.01686561605, -0.04856001547, -0.001694399424] m^3/s, head_loss = [0, 2.398748344, 3.569736308, 2.376754232] m, loops = 2, iterations = 14**

<details><summary>Steps</summary>

- Pipe network flows by the Hardy Cross method (Hazen-Williams): pipes = 1-2, 2-3, 3-4, 4-1, 2-4, L = [600, 500, 600, 500, 780], D = [300, 250, 200, 250, 200], C = [120, 120, 120, 120, 120], demands = [0, 0.04, 0.05, 0.03] → flows = [0.07143998453, 0.03313438395, -0.01686561605, -0.04856001547, -0.001694399424] m^3/s, head_loss = [0, 2.398748344, 3.569736308, 2.376754232] m, loops = 2 m^3/s, iterations = 14 m^3/s

</details>

**Elements of a simple horizontal circular curve** · _Surveying_

> Horizontal circular curve with radius = 300 m, deflection angle = 40 degrees and PI chainage = 1250 m.

→ **T = 109.1910703 m, L = 209.4395102 m, LC = 205.212086 m, E = 19.25333174 m, M = 18.09221376 m, PC = 1140.80893 m, PT = 1350.24844 m**

<details><summary>Steps</summary>

- Elements of a simple horizontal circular curve: R = 300 m, Delta = 40, PI = 1250 m → T = 109.1910703 m, L = 209.4395102 m, LC = 205.212086 m, E = 19.25333174 m, M = 18.09221376 m, PC = 1140.80893 m, PT = 1350.24844 m

</details>

**Coulomb active thrust on a rough wall with sloping backfill** · _Foundations_

> Coulomb active thrust with friction angle = 30 degrees, wall friction angle = 20 degrees, backfill slope = 10 degrees, unit weight = 18 kN/m^3 and height = 6 m.

→ **Ka = 0.3400223954, Pa = 110167.2561 N/m, Pa_h = 103523.3576 N/m, Pa_v = 37679.42072 N/m**

<details><summary>Steps</summary>

- Coulomb active thrust on a rough wall with sloping backfill: phi = 30, delta = 20, beta = 10, gam = 18000 N/m^3, H = 6 m → Ka = 0.3400223954, Pa = 110167.2561 N/m, Pa_h = 103523.3576 N/m, Pa_v = 37679.42072 N/m

</details>

## Figures

Every point was computed by `civil-engineering-ai999x`.

**Drawdown cone around a pumped well**

![Drawdown cone around a pumped well, computed by civil-engineering-ai999x](https://huggingface.co/AI999X/civil-engineering-ai999x/resolve/main/figures/theis-drawdown-profile.svg)

_Drawdown cone around a well pumping Q = 2500 m³/d from a confined aquifer (T = 500 m²/d, S = 0.0002) after 1, 10 and 100 days, drawdown computed by the package at 41 distances from 5 m to 1 km (shown on both sides of the well): 4.90277 m at r = 5 m after 1 d; 5.81894 m at r = 5 m after 10 d; 6.73511 m at r = 5 m after 100 d. Marked: r = 100 m, t = 1 d, s = 2.51924 m._

**Specific-energy diagram and alternate depths**

![Specific-energy diagram and alternate depths, computed by civil-engineering-ai999x](https://huggingface.co/AI999X/civil-engineering-ai999x/resolve/main/figures/alternate-depth-rectangular-diagram.svg)

_Specific-energy diagram of a rectangular channel carrying Q = 6 m³/s in b = 2 m (E computed by the package at 81 depths). Depth y = 0.4 m (supercritical, Fr = 3.78614) has E = 3.26697 m and the alternate depth 3.22281 m; marked: critical depth yc = 0.971683 m at the minimum energy E = 1.45752 m._

**Active earth pressure on a retaining wall**

![Active earth pressure on a retaining wall, computed by civil-engineering-ai999x](https://huggingface.co/AI999X/civil-engineering-ai999x/resolve/main/figures/rankine-active-thrust-diagram.svg)

_Rankine active earth pressure on a 6 m wall retaining soil with φ = 30° and γ = 18 kN/m³ (Ka = 0.333333), drawn as the pressure diagram against the wall: the pressure at each of 40 depths is the base pressure the package returns for a wall of that height. Without surcharge it grows to 36 kPa at the base, resultant Pa = 108 kN/m acting 2 m above the base (arrow); with a 20 kPa surcharge (dashed) the base pressure is 42.6667 kPa and Pa = 148 kN/m._

**Flood routing: inflow and outflow hydrographs**

![Flood routing: inflow and outflow hydrographs, computed by civil-engineering-ai999x](https://huggingface.co/AI999X/civil-engineering-ai999x/resolve/main/figures/muskingum-routing-hydrograph.svg)

_A flood wave routed through a river reach by the Muskingum method (K = 12 h, X = 0.2, Δt = 6 h; C0 = 0.047619, C1 = 0.428571, C2 = 0.52381): inflow per interval as bars, the outflow returned by the package as stems, lower and later; peak inflow 68 m³/s, marked: peak outflow t = 24 h, Q = 45.836 m³/s (attenuation 22.164 m³/s)._

**Flows in a looped pipe network**

![Flows in a looped pipe network, computed by civil-engineering-ai999x](https://huggingface.co/AI999X/civil-engineering-ai999x/resolve/main/figures/hardy-cross-network-network.svg)

_A looped pipe network (5 pipes, 2 loops; lengths [600, 500, 600, 500, 780] m, diameters [300, 250, 200, 250, 200] mm, Hazen–Williams C = 120) balanced by the Hardy Cross method in 14 iterations: arrows show the direction and size of each pipe flow returned by the package (L/s, a negative value flows against the stated pipe direction: 1→2: 71.44, 2→3: 33.1344, 3→4: -16.8656, 4→1: -48.56, 2→4: -1.6944); boxes give each node's demand. The node positions are a layout, not to scale._

## Capabilities

88 certified procedures. A few of them:

- **Procedures:** Area between a survey line and a boundary from offsets; Areal rainfall by Thiessen polygons; Average degree of consolidation after a time (Terzaghi); Broad-crested weir discharge; Chezy coefficient from Manning's n; Chezy velocity; Closed traverse: misclosure and Bowditch (compass rule) adjustment; Coefficient of consolidation from a consolidation time; Cohesion and friction angle from direct shear tests; Cohesion and friction angle from triaxial tests; Consolidation settlement of a thick clay layer integrated over depth; Coulomb active thrust on a rough wall with sloping backfill; Coulomb passive thrust on a rough wall; Critical depth in a circular conduit; Critical depth in a rectangular channel; Critical depth in a trapezoidal channel; Darcy flow across soil layers in series; equivalent conductivities; Direct runoff hydrograph by unit-hydrograph convolution; Discharge through a small orifice under a head; Drawdown from several wells by superposition (Theis); Earthwork volume from cross-section areas; Effective sizes and gradation coefficients from a sieve analysis; Elements of a simple horizontal circular curve; Elevations on a vertical parabolic curve …

`civil_engineering_ai999x.templates()` lists every question form `ask()` reads; the capability guide (PDF, every example executed) is included in this repository.

## Out of scope

- Open-ended explanation, essay writing or tutoring dialogue: it is a solver, not a chatbot.
- Questions outside its certified civil engineering capabilities, and any question whose stated numbers it cannot all use: it refuses with the reason instead.
- High-stakes engineering, medical or legal decisions without independent review.

## Limitations

- It answers what its certified capabilities cover (open-channel hydraulics, pipe networks, hydrology, groundwater flow, soil mechanics and foundations, and surveying); anything else is refused with a reason.
- `ask()` reads documented question forms and common phrasings; unusual wording may be refused; use `solve()` for guaranteed results.
- Results are as correct as the certified relations and their stated conditions; a certified relation is a checked model, not a claim about every physical situation.

## Citation

```bibtex
@software{civil_engineering_ai999x_1_6_0,
  title  = {civil-engineering-ai999x: an offline, verified civil engineering solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.6.0},
  url    = {https://huggingface.co/AI999X/civil-engineering-ai999x}
}
```

## Contact

Questions, bug reports and requests: open a discussion on https://huggingface.co/AI999X/civil-engineering-ai999x/discussions or visit https://ai999x.com.

## Support the project

civil-engineering-ai999x is free and open source. If it helps you, you can support further development on [Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi).

Created and engineered by **Ibnul Jaif Farabi** · **[AI999X](https://huggingface.co/AI999X)** · [ai999x.com](https://ai999x.com)

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/civil-engineering-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.1`; the [GitHub release](https://github.com/AI999X/civil-engineering-ai999x/releases/tag/v1.6.1) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `civil-engineering` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/civil-engineering`) and its data (`rust/models/civil_engineering/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p civil-engineering-ai999x
./target/release/civil-engineering --help
cargo test -p civil-engineering-ai999x
```
