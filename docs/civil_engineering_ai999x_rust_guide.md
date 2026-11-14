# civil-engineering: the Civil Engineering program, user guide

**civil-engineering-ai999x 1.6.1**, Rust build: command line, desktop app and browser. **Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X · support@ai999x.com

Civil Engineering covers pipe and open-channel hydraulics, hydrology, groundwater flow, soil mechanics and foundations, water networks and surveying. This guide is about `civil-engineering`, the model as a self-contained program for Windows and Linux: 88 capabilities in 7 topics, answered on your own computer with no network, no Python and nothing else to install. The same compiled model runs in the AI999X desktop app and in the browser on the model's page (https://www.ai999x.com/models/civil-engineering-ai999x/); the Python package (`pip install civil-engineering-ai999x`) has its own guide. Every command output printed here is what the program printed.

## Contents

- 1. Get it: desktop app, command line, browser, Python
- 2. Quick start
- 3. Writing inputs
- 4. Outcomes and exit codes
- 5. Scripts and agents: batch and JSON
- 6. Figures
- 7. In a web page or Node.js: WebAssembly
- 8. Every capability

## 1. Get it

Version 1.5.0 of the AI999X desktop app and of the command-line bundle. Checksums for every file: https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS; the newest version is always under https://downloads.ai999x.com/desktop/latest/. Full instructions: https://www.ai999x.com/desktop-cli/

| what | system | download |
|---|---|---|
| Desktop app: Civil Engineering | Windows x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-civil-engineering-1.5.0-windows-x86_64.zip |
| Desktop app: Civil Engineering | Linux x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-civil-engineering-1.5.0-linux-x86_64.tar.gz |
| Desktop app: all models | Windows x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-1.5.0-windows-x86_64.zip |
| Desktop app: all models | Linux x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-1.5.0-linux-x86_64.tar.gz |
| Command line | Windows x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-windows-x86_64.zip |
| Command line | Linux x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-linux-x86_64.tar.gz |

### Desktop app

Civil Engineering in its own app: search, questions in words, input forms, live figures and explanations. Windows: Download ai999x-civil-engineering-1.5.0-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x-civil-engineering.exe. If Windows SmartScreen asks, choose More info → Run anyway. Linux:

```shell
curl -LO https://downloads.ai999x.com/desktop/1.5.0/ai999x-civil-engineering-1.5.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-civil-engineering-1.5.0-linux-x86_64.tar.gz
./ai999x-civil-engineering-1.5.0-linux-x86_64/ai999x-civil-engineering
```

It opens on this model: type a question or pick a capability, fill in the form and press Compute. The AI999X desktop app with every model works the same way (choose the model first). Windows: Download ai999x-1.5.0-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x.exe. If Windows SmartScreen asks, choose More info → Run anyway. Linux:

```shell
curl -LO https://downloads.ai999x.com/desktop/1.5.0/ai999x-1.5.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-1.5.0-linux-x86_64.tar.gz
./ai999x-1.5.0-linux-x86_64/ai999x
```

### Command line: Linux

The bundle holds one program per model; `civil-engineering` is this one. The checksum line should print OK.

```shell
curl -LO https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
mkdir -p ~/.local/share && tar xzf ai999x-cli-1.5.0-linux-x86_64.tar.gz -C ~/.local/share
echo 'export PATH="$HOME/.local/share/ai999x-cli-1.5.0-linux-x86_64:$PATH"' >> ~/.bashrc && source ~/.bashrc
civil-engineering --version
```

### Command line: Windows (PowerShell)

The fourth line should print True.

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

### In your browser

Open https://www.ai999x.com/models/civil-engineering-ai999x/, choose a capability and press Compute in your browser: the same program runs on your device as WebAssembly; nothing is sent anywhere.

### Python

```shell
pip install civil-engineering-ai999x
```

## 2. Quick start

### 2.1 Check the install

Prints the model, its version and that this is the Rust build.

```shell
$ civil-engineering --version
civil-engineering-ai999x 1.6.0 (Rust build, ax-engine 1.6.0)
```

### 2.2 Ask in words

Write the question as you would say it: the program reads the quantities and their units, picks the capability and answers.

```shell
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

### 2.3 Find a capability

Search the capabilities by what you want to know; the first column is the id to pass to info and solve.

```shell
$ civil-engineering search "manning discharge in a rectangular channel"
manning_rectangular_discharge        Manning discharge in a rectangular channel  (open-channel hydraulics)
manning_trapezoidal_discharge        Manning discharge in a trapezoidal channel  (open-channel hydraulics)
normal_depth_rectangular             Normal depth in a rectangular channel  (open-channel hydraulics)
gradually_varied_flow_length         Length of a gradually varied flow profile (rectangular channel)  (open-channel hydraulics)
critical_depth_rectangular           Critical depth in a rectangular channel  (open-channel hydraulics)
hydraulic_jump_rectangular           Hydraulic jump in a rectangular channel  (open-channel hydraulics)
rectangular_weir_discharge           Sharp-crested rectangular weir discharge  (open-channel hydraulics)
channel_froude_number                Froude number of flow in a rectangular channel  (open-channel hydraulics)
… 12 more
```

### 2.4 List the capabilities

Every capability the program answers: its id (what you pass to solve) and its name.

```shell
$ civil-engineering list
alternate_depth_rectangular              Specific energy and alternate depth in a rectangular channel
area_from_offsets                        Area between a survey line and a boundary from offsets
broad_crested_weir_discharge             Broad-crested weir discharge
channel_froude_number                    Froude number of flow in a rectangular channel
chezy_from_manning                       Chezy coefficient from Manning's n
chezy_velocity                           Chezy velocity
circular_channel_section                 Part-full circular conduit: area, wetted perimeter, hydraulic radius
coefficient_of_consolidation             Coefficient of consolidation from a consolidation time
… 80 more
```

### 2.5 What a capability needs

The equation or method, every input and output with its SI unit. Read this before solve: it tells you the input names.

```shell
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

### 2.6 Solve

Inputs as name=value, each with any unit of the right kind. The answer lists every output, the method used, each input converted to SI, and the conditions the answer assumes.

```shell
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

### 2.7 Machine-readable answer

--json prints the same answer as one line of JSON (values in SI): for scripts and other programs.

```shell
$ civil-engineering solve manning_rectangular_discharge "b=3 m" "y=1.2 m" S=0.001 n=0.013 --json
{"outcome":"SOLVED","message":"","value":6.68290020277,"unit":"m^3/s","solved_for":"Q","si":[["b",3.0],["y",1.2],["S",0.001],["n",0.013]],"values":[["Q",6.68290020277,"m^3/s"],["V",1.85636116744,"m/s"],["A",3.6,"m^2"],["R",0.666666666667,"m"]],"target":"Q","phrase":"discharge (m^3/s)","assumptions":["b, y > 0","0 < S < 1","0.005 <= n <= 0.2"],"name":"Manning discharge in a rectangular channel","equation":"A = b y, R = A/(b + 2y), Q = A R^(2/3) S^(1/2)/n"}
```

### 2.8 When an input is missing

No number is guessed: the answer names what is missing, shows a complete call, and the program exits with status 1.

```shell
$ civil-engineering solve manning_rectangular_discharge "b=3 m" "y=1.2 m" S=0.001
NEEDS_INPUT: manning_rectangular_discharge needs n (Manning roughness coefficient). Example: solve('manning_rectangular_discharge', b='3 m', y='1.2 m', S=0.001, n=0.013).
```

### 2.9 Many questions at once

One JSON request per line in, one JSON answer per line out, in order: for scripts, notebooks and agents (the program starts once). In PowerShell: Get-Content cases.jsonl | civil-engineering batch

`cases.jsonl`:

```json
{"key": "manning_rectangular_discharge", "inputs": {"b": "3 m", "y": "1.2 m", "S": 0.001, "n": 0.013}}
{"key": "theis_drawdown", "inputs": {"Q": "0.02893518519 m^3/s", "T": "0.005787037037 m^2/s", "S": 0.0002, "r": "100 m", "t": "86400 s"}}
```

```shell
$ civil-engineering batch < cases.jsonl
{"outcome":"SOLVED","message":"","value":6.68290020277,"unit":"m^3/s","solved_for":"Q","si":[["b",3.0],["y",1.2],["S",0.001],["n",0.013]],"values":[["Q",6.68290020277,"m^3/s"],["V",1.85636116744,"m/s"],["A",3.6,"m^2"],["R",0.666666666667,"m"]],"target":"Q","phrase":"discharge (m^3/s)","assumptions":["b, y > 0","0 < S < 1","0.005 <= n <= 0.2"],"name":"Manning discharge in a rectangular channel","equation":"A = b y, R = A/(b + 2y), Q = A R^(2/3) S^(1/2)/n"}
{"outcome":"SOLVED","message":"","value":2.51923946839,"unit":"m","solved_for":"s","si":[["Q",0.02893518519],["T",0.005787037037],["S",0.0002],["r",100.0],["t",86400.0]],"values":[["s",2.51923946839,"m"],["u",0.00100000000001,""],["W",6.33153936413,""]],"target":"s","phrase":"drawdown (m)","assumptions":["confined, homogeneous, infinite aquifer","fully penetrating well, constant rate","u <= 50"],"name":"Theis drawdown around a pumped well","equation":"u = r^2 S/(4 T t), W(u) = E1(u) (series for u <= 1, continued fraction above), s = Q W/(4 pi T)"}
```

## 3. Writing inputs

Inputs are written `name=value`, one per argument; `civil-engineering info <capability>` lists the names. Quote an argument that holds a space or brackets (bash, zsh and PowerShell all accept double quotes). The answer always states each input as it was read, converted to SI, so a misread unit is visible at once.

| kind | example | how it is read |
|---|---|---|
| a number with a unit | `"R=0.128 nm"` | any unit of the same kind; converted to SI |
| a number in SI | `R=1.28e-10` | no unit means the SI unit shown by info |
| a choice | `structure=FCC` | one of the values info lists |
| a list or vector | `"v=[1, 2, 3]" or "v=1, 2, 3"` | brackets keep it a list |
| a matrix | `"A=[[1, 2], [3, 4]]"` | one list per row |
| true / false | `inclusive=true` | for yes/no options |

An equation can be solved for any of its quantities: give the others and add `--target <symbol>`. Without `--target`, the program solves for the quantity the capability is named after.

## 4. Outcomes and exit codes

The program never guesses a number: when it cannot answer, it says why and exits with status 1, so a script can tell an answer from a refusal without reading the text.

| outcome | meaning | exit status |
|---|---|---|
| SOLVED | the answer: every output with its unit, the method, the inputs in SI, the conditions assumed | 0 |
| NEEDS_INPUT | an input is missing; the message names it and shows a complete call | 1 |
| INVALID_INPUT | a value or unit of the wrong kind, an unknown input name, or a value outside the conditions | 1 |
| AMBIGUOUS | the inputs allow two different answers; say which one is meant | 1 |
| OUT_OF_SCOPE | the model has no capability for this, or the result would not be meaningful | 1 |
| (usage) | a command the program does not know: it prints its usage | 2 |

## 5. Scripts and agents: batch and JSON

`civil-engineering solve ... --json` prints one answer as one line of JSON. `civil-engineering batch` reads one request per line on standard input and writes one JSON answer per line, in order, starting the program once for any number of requests. A request is `{"key": "<capability>", "inputs": {"name": "value", ...}, "target": null}`. In PowerShell, pipe the file in: `Get-Content cases.jsonl | civil-engineering batch`. `civil-engineering catalog` prints every capability with its inputs as JSON, to build forms or tool definitions.

| answer field | meaning |
|---|---|
| outcome | one of the outcomes above |
| message | why there is no answer (empty when SOLVED) |
| value, unit | the main result in SI units |
| values | every output: [symbol, value, unit] (procedures) |
| target | the quantity solved for |
| si | each numeric input converted to SI: [symbol, value] |
| assumptions | the conditions the answer relies on |
| name, equation | the capability and its equation or method |

```python
import json, subprocess

reqs = [{"key": "<capability>", "inputs": {"name": "value"}}]
out = subprocess.run(["civil-engineering", "batch"], input="\n".join(map(json.dumps, reqs)),
                     capture_output=True, text=True, check=True).stdout
answers = [json.loads(line) for line in out.splitlines()]
```

## 6. Figures

`civil-engineering figures <capability> name=value ... --out DIR` draws the live figures for those inputs and writes each as an SVG file into DIR (an existing folder); without `--out` the SVG text comes back as JSON. The desktop app and the model page draw the same figures as soon as you press Compute.

## 7. In a web page or Node.js: WebAssembly

The same model is a WebAssembly module, `civil_engineering_ai999x.wasm` (https://www.ai999x.com/compute/rust/civil_engineering_ai999x.wasm), with a small loader, `ax-wasm.js` (https://www.ai999x.com/compute/rust/ax-wasm.js). It runs in a browser, a Web Worker or Node.js, with no server.

```html
<script src="ax-wasm.js"></script>
<script>
  AXWasm.load("civil_engineering_ai999x.wasm").then(function (m) {
    var a = m.solve({key: "<capability>", inputs: {name: "value"}});
    console.log(a.outcome, a.values || a.value);
    console.log(m.catalog().capabilities.length + " capabilities");
  });
</script>
```

## 8. Every capability

By topic. For each: what it does, what it needs and gives, when it applies, and a command with what the program printed for it. `civil-engineering info <id>` shows the same inputs and outputs in the terminal.

### Foundations (9)

<a id="coulomb_active_thrust"></a>

#### Coulomb active thrust on a rough wall with sloping backfill, `coulomb_active_thrust`

Coulomb active earth pressure coefficient and thrust per metre on a rough vertical wall (wall friction delta) retaining a cohesionless backfill sloping at beta, with the horizontal and vertical components of the thrust.

$$
K_a = \frac{\cos^2\phi}{\cos\delta\left[1+\sqrt{\frac{\sin(\phi+\delta)\sin(\phi-\beta)}{\cos\delta\cos\beta}}\right]^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `phi` | in | friction angle of the backfill (degrees) | - |
| `delta` | in | wall friction angle (degrees) | - |
| `beta` | in | backfill slope angle above horizontal (degrees) | - |
| `gam` | in | unit weight of the backfill | N/m^3 |
| `H` | in | wall height | m |
| `Ka` | out | Coulomb active coefficient | - |
| `Pa` | out | active thrust per metre, inclined at delta (N/m) | N/m |
| `Pa_h` | out | horizontal component (N/m) | N/m |
| `Pa_v` | out | vertical component (N/m) | N/m |

**Applies when:** vertical wall back; cohesionless dry backfill; 0 <= delta <= phi; 0 <= beta < phi.

```shell
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

**What the worked example means:** This calculation determines the Coulomb active earth pressure coefficient and the resulting thrust on a rough retaining wall. Using the backfill friction angle of 30, wall friction angle of 20, and slope angle of 10, the method computes a coefficient of 0.340022. This coefficient, combined with the unit weight 18000 N/m^3 and height 6 m, yields a total thrust per metre of 110167 N/m N/m. The thrust acts at an angle equal to the wall friction, so it is resolved into a horizontal push of 103523 N/m N/m and a downward vertical force of 37679.4 N/m N/m.

<a id="coulomb_passive_thrust"></a>

#### Coulomb passive thrust on a rough wall: `coulomb_passive_thrust`

Coulomb (plane-wedge) passive earth pressure coefficient and thrust per metre on a rough vertical wall with wall friction delta and a soil surface rising at beta (plane wedges overestimate the passive resistance when delta is large; keep delta below about phi/2 in design).

$$
K_p = \frac{\cos^2\phi}{\cos\delta\left[1-\sqrt{\frac{\sin(\phi+\delta)\sin(\phi+\beta)}{\cos\delta\cos\beta}}\right]^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `phi` | in | friction angle of the backfill (degrees) | - |
| `delta` | in | wall friction angle (degrees) | - |
| `beta` | in | backfill slope angle above horizontal (degrees) | - |
| `gam` | in | unit weight of the backfill | N/m^3 |
| `H` | in | wall height | m |
| `Kp` | out | Coulomb passive coefficient | - |
| `Pp` | out | passive thrust per metre, inclined at delta (N/m) | N/m |
| `Pp_h` | out | horizontal component (N/m) | N/m |
| `Pp_v` | out | vertical component (N/m) | N/m |

**Applies when:** vertical wall back; cohesionless dry soil; plane wedge (phi + delta + beta < 90).

```shell
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

**What the worked example means:** This calculation determines the passive earth resistance on a rough vertical wall retaining cohesionless dry soil with a sloping backface. Using the plane wedge method, it computes the passive coefficient Kp as 4.9765, which scales the unit weight 18000 N/m^3 and height 2 m to find the total thrust Pp of 179154 N/m N/m. The resulting force acts at an angle delta relative to the normal, yielding a horizontal component of 173049 N/m N/m and a vertical component of 46368.5 N/m N/m. This value is critical for assessing the stability of anchored walls and determining required embedment depths for sheet piles.

<a id="rankine_active_thrust"></a>

#### Rankine active earth pressure and thrust on a wall, `rankine_active_thrust`

Rankine active pressure coefficient, active pressure at the base, total active thrust per metre of a smooth vertical wall retaining a horizontal backfill (with optional cohesion and surcharge; the tension zone is ignored), the tension-crack depth and the height of the resultant above the base. Dry backfill.

$$
K_a = \frac{1-\sin\phi}{1+\sin\phi},\quad \sigma_a = K_a(\gamma z + q) - 2c\sqrt{K_a}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `phi` | in | friction angle of the backfill (degrees) | - |
| `gam` | in | unit weight of the backfill | N/m^3 |
| `H` | in | wall height | m |
| `c` | in | cohesion (default 0) (optional) | Pa |
| `q` | in | uniform surcharge (default 0) (optional) | Pa |
| `Ka` | out | active earth pressure coefficient | - |
| `sigma_base` | out | active pressure at the base (Pa) | Pa |
| `Pa` | out | active thrust per metre of wall (N/m) | N/m |
| `z_crack` | out | tension-crack depth (m) | m |
| `ybar` | out | height of the resultant above the base (m) | m |

**Applies when:** smooth vertical wall, horizontal backfill; dry backfill; tension zone ignored.

```shell
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

**What the worked example means:** The active thrust is 108000 N/m per metre, with a base pressure of 36000 Pa.

<a id="rankine_passive_thrust"></a>

#### Rankine passive earth pressure and thrust: `rankine_passive_thrust`

Rankine passive pressure coefficient, passive pressure at the base, total passive thrust per metre on a smooth vertical wall with a horizontal soil surface (optional cohesion and surcharge) and the height of the resultant.

$$
K_p = \frac{1+\sin\phi}{1-\sin\phi},\quad \sigma_p = K_p(\gamma z+q) + 2c\sqrt{K_p}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `phi` | in | friction angle of the soil (degrees) | - |
| `gam` | in | unit weight of the soil | N/m^3 |
| `H` | in | height of the wall in contact | m |
| `c` | in | cohesion (default 0) (optional) | Pa |
| `q` | in | uniform surcharge (default 0) (optional) | Pa |
| `Kp` | out | passive earth pressure coefficient | - |
| `sigma_base` | out | passive pressure at the base (Pa) | Pa |
| `Pp` | out | passive thrust per metre of wall (N/m) | N/m |
| `ybar` | out | height of the resultant above the base (m) | m |

**Applies when:** smooth vertical wall, horizontal soil surface; dry soil.

```shell
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

**What the worked example means:** This calculation determines the resistance provided by the soil when a wall moves towards it. Using a friction angle of 32 degrees, the passive coefficient is found to be 3.25459. For a wall height of 2 m metres with unit weight 18000 N/m^3, the pressure at the base reaches 117165 Pa. The total horizontal force resisting movement is calculated as 117165 N/m newtons per metre of wall length, acting at a height of 0.666667 m metres above the base.

<a id="terzaghi_bearing_capacity"></a>

#### Terzaghi bearing capacity with stated factors, `terzaghi_bearing_capacity`

Ultimate, allowable and net bearing capacity of a strip, square or circular footing by Terzaghi's equation with the bearing capacity factors Nc, Nq and Ngamma stated (from a table): qu = s_c c Nc + q Nq + s_g gamma B Ngamma, s_c = 1.3 and s_g = 0.4 (square) or 0.3 (circular, B = diameter).

$$
q_u = s_c\,c N_c + \gamma D_f N_q + s_\gamma\,\gamma B N_\gamma
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `c` | in | cohesion | Pa |
| `gam` | in | unit weight of the soil | N/m^3 |
| `B` | in | footing width (diameter of a circular footing) | m |
| `Df` | in | embedment depth | m |
| `Nc` | in | bearing capacity factor Nc (stated) | - |
| `Nq` | in | bearing capacity factor Nq (stated) | - |
| `Ngamma` | in | bearing capacity factor Ngamma (stated) | - |
| `shape` | in | footing shape: strip, square or circular (default strip) (optional) | - |
| `FS` | in | factor of safety (default 3) (optional) | - |
| `qu` | out | ultimate bearing capacity (Pa) | Pa |
| `qa` | out | allowable bearing capacity qu/FS (Pa) | Pa |
| `q_net` | out | net ultimate bearing capacity qu - gamma Df (Pa) | Pa |

**Applies when:** c, Df >= 0; factors as stated (Terzaghi's table for the friction angle); general shear failure, vertical central load.

```shell
$ civil-engineering solve terzaghi_bearing_capacity "c=20000 Pa" "gam=18000 N/m^3" "B=2 m" "Df=1.5 m" Nc=37.16 Nq=22.46 Ngamma=19.13 shape=square
qu = 1848052 Pa
qa = 616017.3333 Pa
q_net = 1821052 Pa
  by: Terzaghi bearing capacity with stated factors  (qu = s_c c Nc + gamma Df Nq + s_g gamma B Ngamma (strip 1, 0.5; square 1.3, 0.4; circular 1.3, 0.3))
  c = 20000 Pa
  gam = 18000 N/m^3
  B = 2 m
  Df = 1.5 m
  Nc = 37.16
  Nq = 22.46
  Ngamma = 19.13
  shape = square
  assumes: c, Df >= 0; factors as stated (Terzaghi's table for the friction angle); general shear failure, vertical central load
```

**What the worked example means:** Terzaghi's equation calculates the maximum pressure a shallow foundation can withstand before shear failure occurs. Using the provided cohesion, unit weight, dimensions, and stated bearing capacity factors, the model determines the ultimate bearing capacity as 1.84805e+06 Pa. Dividing this by the factor of safety yields the allowable bearing capacity of 616017 Pa, while subtracting the overburden pressure gives the net ultimate bearing capacity of 1.82105e+06 Pa. These values indicate the load limits for safe design under general shear failure assumptions.

<a id="strip_footing_bearing_capacity"></a>

#### Ultimate bearing capacity of a strip footing, `strip_footing_bearing_capacity`

Ultimate and allowable bearing capacity of a strip footing from the general bearing capacity equation with Prandtl-Reissner factors Nc, Nq and Vesic's (1973) Ngamma (no shape, depth or inclination factors; drained parameters, water table well below the footing).

$$
q_u = cN_c + \gamma D_f N_q + \tfrac12\gamma B N_\gamma
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `c` | in | cohesion | Pa |
| `phi` | in | friction angle (degrees) | - |
| `gam` | in | unit weight of the soil | N/m^3 |
| `B` | in | footing width | m |
| `Df` | in | embedment depth | m |
| `FS` | in | factor of safety for the allowable pressure (default 3) (optional) | - |
| `qu` | out | ultimate bearing capacity (Pa) | Pa |
| `qa` | out | allowable bearing pressure qu/FS (Pa) | Pa |
| `Nc` | out | bearing capacity factor Nc | - |
| `Nq` | out | bearing capacity factor Nq | - |
| `Ngamma` | out | bearing capacity factor Ngamma (Vesic) | - |

**Applies when:** 20 <= phi <= 45 degrees; general shear failure; strip footing, vertical central load.

```shell
$ civil-engineering solve strip_footing_bearing_capacity "c=10000 Pa" phi=30 "gam=18000 N/m^3" "B=1.5 m" "Df=1 m"
qu = 935050.0425 Pa
qa = 311683.3475 Pa
Nc = 30.13962779
Nq = 18.40112222
Ngamma = 22.40248627
  by: Ultimate bearing capacity of a strip footing  (Nq = e^(pi tan phi) tan^2(45 + phi/2), Nc = (Nq - 1) cot phi, Ngamma = 2(Nq + 1) tan phi; qu = c Nc + gam Df Nq + B gam Ngamma/2)
  c = 10000 Pa
  phi = 30
  gam = 18000 N/m^3
  B = 1.5 m
  Df = 1 m
  assumes: 20 <= phi <= 45 degrees; general shear failure; strip footing, vertical central load
```

**What the worked example means:** This calculation determines the ultimate bearing capacity of a strip footing using the general shear failure theory. Based on the soil cohesion, friction angle, unit weight, footing width, and embedment depth, the computed ultimate load intensity is 935050 Pa. Applying a factor of safety yields an allowable bearing pressure of 311683 Pa, which represents the maximum safe stress that can be applied to the foundation without causing shear failure.

<a id="stress_below_rectangular_load"></a>

#### Vertical stress below a uniformly loaded rectangle (Newmark), `stress_below_rectangular_load`

Increase in vertical stress at depth z below the corner (or the centre, by superposition of four corners) of a flexible rectangular area B x L under a uniform pressure q (Boussinesq integrated by Newmark).

$$
\Delta\sigma_z = q\,I(m,n),\quad m = \frac{B}{z},\ n = \frac{L}{z}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `q` | in | uniform applied pressure | Pa |
| `B` | in | width of the loaded area | m |
| `L` | in | length of the loaded area | m |
| `z` | in | depth below the surface | m |
| `position` | in | point below the 'corner' (default) or the 'centre' (optional) | - |
| `dsigma` | out | vertical stress increase (Pa) | Pa |
| `I` | out | influence factor dsigma/q | - |

**Applies when:** elastic, homogeneous, isotropic half-space; flexible uniform load.

```shell
$ civil-engineering solve stress_below_rectangular_load "q=200000 Pa" "B=2 m" "L=3 m" "z=4 m"
dsigma = 21414.5858 Pa
I = 0.107072929
  by: Vertical stress below a uniformly loaded rectangle (Newmark)  (I = (1/4 pi)[2mn sqrt(s)/(s + m^2 n^2) (s + 1)/s + atan2(2mn sqrt(s), s - m^2 n^2)], s = m^2 + n^2 + 1, m = B/z, n = L/z)
  q = 200000 Pa
  B = 2 m
  L = 3 m
  z = 4 m
  assumes: elastic, homogeneous, isotropic half-space; flexible uniform load
```

**What the worked example means:** This calculation determines the additional vertical stress within the soil mass beneath a rectangular foundation. By applying Newmark's integration of the Boussinesq solution, the method computes an influence factor based on the geometry and depth. For a uniform pressure of 200000 Pa over an area defined by width 2 m and length 3 m, evaluated at depth 4 m, the resulting influence factor is 0.107073. This yields a total stress increase of 21414.6 Pa, which represents the load transferred to that specific subsurface point.

<a id="stress_below_strip_load"></a>

#### Vertical stress below a uniformly loaded strip, `stress_below_strip_load`

Increase in vertical stress at depth z and horizontal offset x from the centreline of an infinitely long strip of width B under a uniform pressure q (elastic half-space).

$$
\Delta\sigma_z = \frac{q}{\pi}\left[\alpha + \sin\alpha\cos(\alpha+2\delta)\right]
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `q` | in | uniform applied pressure | Pa |
| `B` | in | strip width | m |
| `z` | in | depth below the surface | m |
| `x` | in | horizontal offset from the strip centreline (default 0) (optional) | m |
| `dsigma` | out | vertical stress increase (Pa) | Pa |
| `I` | out | influence factor dsigma/q | - |

**Applies when:** elastic, homogeneous, isotropic half-space; flexible uniform load, plane strain.

```shell
$ civil-engineering solve stress_below_strip_load "q=100000 Pa" "B=2 m" "z=3 m"
dsigma = 39581.86964 Pa
I = 0.3958186964
  by: Vertical stress below a uniformly loaded strip  (dsigma = (q/pi)[atan(u/z) + u z/(u^2 + z^2)] from u = x - B/2 to x + B/2)
  q = 100000 Pa
  B = 2 m
  z = 3 m
  assumes: elastic, homogeneous, isotropic half-space; flexible uniform load, plane strain
```

**What the worked example means:** This calculation determines the additional vertical stress at a specific depth and horizontal location beneath a long strip foundation. Using the applied pressure, strip width, depth, and offset, it computes the stress increase as 39581.9 Pa and the corresponding influence factor as 0.395819. The result indicates how much the soil stress rises above its initial state due to the load.

<a id="stress_below_circular_load"></a>

#### Vertical stress below the centre of a uniformly loaded circle, `stress_below_circular_load`

Increase in vertical stress at depth z below the centre of a flexible circular area of radius a carrying a uniform pressure q (Boussinesq, elastic half-space).

$$
\Delta\sigma_z = q\left[1-\left(\frac{1}{1+(a/z)^2}\right)^{3/2}\right]
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `q` | in | uniform applied pressure | Pa |
| `a` | in | radius of the loaded area | m |
| `z` | in | depth below the surface | m |
| `dsigma` | out | vertical stress increase (Pa) | Pa |
| `I` | out | influence factor dsigma/q | - |

**Applies when:** elastic, homogeneous, isotropic half-space; flexible uniform load; below the centre.

```shell
$ civil-engineering solve stress_below_circular_load "q=150000 Pa" "a=2 m" "z=3 m"
dsigma = 63594.77713 Pa
I = 0.4239651808
  by: Vertical stress below the centre of a uniformly loaded circle  (dsigma = q [1 - (1/(1 + (a/z)^2))^(3/2)])
  q = 150000 Pa
  a = 2 m
  z = 3 m
  assumes: elastic, homogeneous, isotropic half-space; flexible uniform load; below the centre
```

**What the worked example means:** This calculation determines the increase in vertical stress at a specific depth beneath the center of a flexible circular area carrying a uniform pressure. By applying the influence factor derived from elastic half-space theory, the result shows that for an applied pressure of 150000 Pa over a circle with radius 2 m at a depth of 3 m, the induced vertical stress is 63594.8 Pa. The dimensionless influence factor for this configuration is 0.423965, which represents the ratio of the stress increase to the surface pressure.

### Groundwater (8)

<a id="darcy_flow_layered_soil"></a>

#### Darcy flow across soil layers in series; equivalent conductivities, `darcy_flow_layered_soil`

Flow rate, specific discharge and the heads at the layer interfaces for one-dimensional Darcy flow across one or more soil layers in series (flow perpendicular to the layering), with the equivalent vertical (series) and horizontal (parallel) conductivities. Heads are measured from the downstream face.

$$
K_v = \frac{\sum H_i}{\sum H_i/K_i},\quad K_h = \frac{\sum K_iH_i}{\sum H_i},\quad Q = K_v\frac{\Delta h}{\sum H_i}A
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `H` | in | layer thicknesses in m, in flow order | - |
| `K` | in | hydraulic conductivity of each layer in m/s | - |
| `A` | in | flow cross-section area | m^2 |
| `dh` | in | head difference across all layers | m |
| `Q` | out | flow rate (m^3/s) | m^3/s |
| `q` | out | specific discharge (Darcy velocity, m/s) | m/s |
| `Kv` | out | equivalent conductivity across the layers (m/s) | m/s |
| `Kh` | out | equivalent conductivity along the layers (m/s) | m/s |
| `heads` | out | head at each interior interface above the outlet (m) | m |

**Applies when:** steady, saturated, laminar flow; flow perpendicular to the layers.

```shell
$ civil-engineering solve darcy_flow_layered_soil "H=[2]" "K=[1e-05]" "A=10 m^2" "dh=0.5 m"
Q = 2.5e-05 m^3/s
q = 2.5e-06 m/s
Kv = 1e-05 m/s
Kh = 1e-05 m/s
heads = [0.0] m
  by: Darcy flow across soil layers in series; equivalent conductivities  (Kv = sum H / sum(H_i/K_i), Kh = sum(K_i H_i)/sum H, q = Kv dh/sum H, Q = q A)
  A = 10 m^2
  dh = 0.5 m
  assumes: steady, saturated, laminar flow; flow perpendicular to the layers
```

**What the worked example means:** This calculation determines the steady-state groundwater flow through multiple horizontal soil layers stacked vertically. By treating the layers as resistors in series for vertical flow, it computes an equivalent vertical hydraulic conductivity, 1e-05 m/s, and an equivalent horizontal conductivity, 1e-05 m/s. The resulting specific discharge is 2.5e-06 m/s, which scales with the cross-sectional area 10 m^2 to produce the total volumetric flow rate 2.5e-05 m^3/s. These values help engineers assess seepage potential and design drainage systems by summarizing complex stratigraphy into single effective parameters.

<a id="well_field_drawdown"></a>

#### Drawdown from several wells by superposition (Theis), `well_field_drawdown`

Total drawdown at a point from several wells pumping (positive rate) or injecting (negative rate) at constant rates in a confined aquifer, by superposing Theis solutions; with each well's contribution. Coordinates in m, rates in m^3/day.

$$
s = \sum_i \frac{Q_i}{4\pi T}W\!\left(\frac{r_i^2S}{4Tt}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `wx` | in | well x coordinates in m | - |
| `wy` | in | well y coordinates in m | - |
| `Qs` | in | pumping rates in m^3/day (negative for injection) | - |
| `T` | in | transmissivity | m^2/s |
| `S` | in | storativity | - |
| `t` | in | time since pumping began (all wells together) | s |
| `x0` | in | x coordinate of the point | m |
| `y0` | in | y coordinate of the point | m |
| `s` | out | total drawdown (m) | m |
| `parts` | out | drawdown from each well (m) | m |

**Applies when:** confined, homogeneous, infinite aquifer; all wells started together; point at least 5 cm from any well.

```shell
$ civil-engineering solve well_field_drawdown "wx=[0, 200]" "wy=[0, 0]" "Qs=[2000, 1500]" "T=0.00462962963 m^2/s" S=0.0003 "t=432000 s" "x0=100 m" "y0=50 m"
s = 4.935878027 m
parts = [2.82050172954,2.11537629715] m
  by: Drawdown from several wells by superposition (Theis)  (s = sum Q_i W(u_i)/(4 pi T), u_i = r_i^2 S/(4 T t))
  T = 0.00462962963 m^2/s
  S = 0.0003
  t = 432000 s
  x0 = 100 m
  y0 = 50 m
  assumes: confined, homogeneous, infinite aquifer; all wells started together; point at least 5 cm from any well
```

**What the worked example means:** This calculation determines the total water level decline at a specific point caused by multiple wells operating simultaneously in a confined aquifer. By applying the principle of superposition to Theis solutions, the model sums the drawdown contribution from each well based on its pumping rate and distance to the observation point. The resulting total drawdown is 4.93588 m.

<a id="falling_head_permeability"></a>

#### Hydraulic conductivity from a falling-head test, `falling_head_permeability`

Hydraulic conductivity of a soil sample from a falling-head permeameter: standpipe diameter d, sample diameter D and length L, the standpipe head falling from h1 to h2 in time t.

$$
K = \frac{aL}{At}\ln\frac{h_1}{h_2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `d` | in | standpipe diameter | m |
| `D` | in | sample diameter | m |
| `L` | in | sample length | m |
| `h1` | in | initial head | m |
| `h2` | in | final head | m |
| `t` | in | elapsed time | s |
| `K` | out | hydraulic conductivity (m/s) | m/s |
| `K_cm_s` | out | hydraulic conductivity (cm/s) | - |

**Applies when:** h2 < h1; saturated sample, laminar flow.

```shell
$ civil-engineering solve falling_head_permeability "d=0.01 m" "D=0.075 m" "L=0.15 m" "h1=1 m" "h2=0.6 m" "t=600 s"
K = 2.270336106e-06 m/s
K_cm_s = 0.0002270336106
  by: Hydraulic conductivity from a falling-head test  (K = (a L/(A t)) ln(h1/h2), a and A the standpipe and sample areas)
  d = 0.01 m
  D = 0.075 m
  L = 0.15 m
  h1 = 1 m
  h2 = 0.6 m
  t = 600 s
  assumes: h2 < h1; saturated sample, laminar flow
```

**What the worked example means:** This calculation determines the hydraulic conductivity of a soil sample using data from a falling-head permeameter test. Measuring the rate at which water level drops in a standpipe gives a value of 2.27034e-06 m/s meters per second. This result indicates how easily water flows through the soil under saturated conditions.

<a id="thiem_unconfined_conductivity"></a>

#### Hydraulic conductivity from steady pumping (unconfined aquifer), `thiem_unconfined_conductivity`

Hydraulic conductivity of an unconfined aquifer from the steady saturated thicknesses h1 < h2 in two observation wells at distances r1 < r2 from a well pumped at Q (Thiem-Dupuit equation).

$$
K = \frac{Q\ln(r_2/r_1)}{\pi(h_2^2-h_1^2)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Q` | in | pumping rate | m^3/s |
| `r1` | in | distance of the nearer observation well | m |
| `r2` | in | distance of the farther observation well | m |
| `h1` | in | saturated thickness at r1 | m |
| `h2` | in | saturated thickness at r2 | m |
| `K` | out | hydraulic conductivity (m/s) | m/s |
| `K_m_d` | out | hydraulic conductivity (m/day) | - |

**Applies when:** steady state; unconfined aquifer, Dupuit assumptions; r1 < r2, h1 < h2.

```shell
$ civil-engineering solve thiem_unconfined_conductivity "Q=0.01736111111 m^3/s" "r1=20 m" "r2=80 m" "h1=18.2 m" "h2=19.6 m"
K = 0.0001447648968 m/s
K_m_d = 12.50768708
  by: Hydraulic conductivity from steady pumping (unconfined aquifer)  (K = Q ln(r2/r1)/(pi (h2^2 - h1^2)))
  Q = 0.01736111111 m^3/s
  r1 = 20 m
  r2 = 80 m
  h1 = 18.2 m
  h2 = 19.6 m
  assumes: steady state; unconfined aquifer, Dupuit assumptions; r1 < r2, h1 < h2
```

**What the worked example means:** This calculation determines the hydraulic conductivity of an unconfined aquifer using the Thiem-Dupuit equation. It relies on the steady pumping rate of 0.0173611 m^3/s cubic meters per second and the saturated thicknesses measured at two observation wells. The nearer well at a distance of 20 m meters has a saturated thickness of 18.2 m meters, while the farther well at 80 m meters shows a thickness of 19.6 m meters. The resulting conductivity value of 0.000144765 m/s meters per second indicates how easily water flows through the porous medium under these specific field conditions.

<a id="theis_drawdown"></a>

#### Theis drawdown around a pumped well: `theis_drawdown`

Transient drawdown at distance r and time t from a well pumped at a constant rate in a confined aquifer (Theis), with u and the well function W(u) = E1(u).

$$
s = \frac{Q}{4\pi T}W(u),\quad u = \frac{r^2S}{4Tt},\quad W(u) = \int_u^\infty \frac{e^{-x}}{x}\,dx
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Q` | in | pumping rate | m^3/s |
| `T` | in | transmissivity | m^2/s |
| `S` | in | storativity (storage coefficient) | - |
| `r` | in | distance from the pumped well | m |
| `t` | in | time since pumping began | s |
| `s` | out | drawdown (m) | m |
| `u` | out | u = r^2 S/(4 T t) | - |
| `W` | out | well function W(u) | - |

**Applies when:** confined, homogeneous, infinite aquifer; fully penetrating well, constant rate; u <= 50.

```shell
$ civil-engineering solve theis_drawdown "Q=0.02893518519 m^3/s" "T=0.005787037037 m^2/s" S=0.0002 "r=100 m" "t=86400 s"
s = 2.519239468 m
u = 0.001
W = 6.331539364
  by: Theis drawdown around a pumped well  (u = r^2 S/(4 T t), W(u) = E1(u) (series for u <= 1, continued fraction above), s = Q W/(4 pi T))
  Q = 0.02893518519 m^3/s
  T = 0.005787037037 m^2/s
  S = 0.0002
  r = 100 m
  t = 86400 s
  assumes: confined, homogeneous, infinite aquifer; fully penetrating well, constant rate; u <= 50
```

**What the worked example means:** This calculation determines the transient drawdown in a confined aquifer at a specific distance and time using the Theis method. Given a pumping rate of 0.0289352 m^3/s, transmissivity of 0.00578704 m^2/s, storativity of 0.0002, observation distance of 100 m, and elapsed time of 86400 s, the model computes an intermediate parameter u equal to 0.001. This leads to a well function value W of 6.33154. The final result is a predicted drawdown s of 2.51924 m, which represents the decline in hydraulic head due to the sustained extraction of water.

<a id="cooper_jacob_analysis"></a>

#### Transmissivity and storativity by the Cooper-Jacob straight line, `cooper_jacob_analysis`

Transmissivity and storativity of a confined aquifer from a pumping test by the Cooper-Jacob method: a least-squares straight line through drawdown against log10(time) (late-time data, u < 0.05, chosen by the user). Times are in minutes, drawdowns in m.

$$
T = \frac{2.303\,Q}{4\pi\,\Delta s},\quad S = \frac{2.25\,T\,t_0}{r^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `times` | in | times since pumping began, in minutes | - |
| `drawdowns` | in | drawdowns at those times, in m | - |
| `Q` | in | pumping rate | m^3/s |
| `r` | in | distance of the observation well | m |
| `T` | out | transmissivity (m^2/s) | m^2/s |
| `S` | out | storativity | - |
| `ds_per_cycle` | out | drawdown per log cycle of time (m) | m |
| `t0_min` | out | zero-drawdown intercept time (min) | - |
| `T_m2_d` | out | transmissivity (m^2/day) | - |

**Applies when:** late-time data on the straight line (u < 0.05); confined aquifer.

```shell
$ civil-engineering solve cooper_jacob_analysis "times=[10, 20, 50, 100, 200, 500]" "drawdowns=[0.62, 0.79, 1.02, 1.19, 1.36, 1.59]" "Q=0.02314814815 m^3/s" "r=50 m"
T = 0.007434341841 m^2/s
S = 0.0003296492203
ds_per_cycle = 0.5706342049 m
t0_min = 0.8211372737
T_m2_d = 642.3271351
  by: Transmissivity and storativity by the Cooper-Jacob straight line  (fit s = a + b log10 t; T = 2.303 Q/(4 pi b), S = 2.25 T t0/r^2 with t0 = 10^(-a/b))
  Q = 0.02314814815 m^3/s
  r = 50 m
  assumes: late-time data on the straight line (u < 0.05); confined aquifer
```

**What the worked example means:** The Cooper-Jacob method estimates aquifer properties by fitting a straight line to late-time drawdown data plotted against the logarithm of time. Using the pumping rate of 0.0231481 m^3/s and observation distance of 50 m, the calculated transmissivity is 0.00743434 m^2/s and the storativity is 0.000329649. The slope of this line, representing a drawdown change of 0.570634 m per log cycle, directly determines the transmissivity, while the intercept time of 0.821137 minutes helps establish the storativity.

<a id="thiem_confined_transmissivity"></a>

#### Transmissivity from steady pumping (Thiem, confined aquifer), `thiem_confined_transmissivity`

Transmissivity (and conductivity, if the aquifer thickness is given) of a confined aquifer from the steady drawdowns in two observation wells at distances r1 < r2 from a well pumped at Q (Thiem equation).

$$
T = \frac{Q\ln(r_2/r_1)}{2\pi(s_1-s_2)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Q` | in | pumping rate | m^3/s |
| `r1` | in | distance of the nearer observation well | m |
| `r2` | in | distance of the farther observation well | m |
| `s1` | in | drawdown at r1 | m |
| `s2` | in | drawdown at r2 | m |
| `b` | in | aquifer thickness (optional) (optional) | m |
| `T` | out | transmissivity (m^2/s) | m^2/s |
| `T_m2_d` | out | transmissivity (m^2/day) | - |
| `K` | out | hydraulic conductivity T/b (m/s) | m/s |

**Applies when:** steady state; confined aquifer, fully penetrating well; r1 < r2, s1 > s2.

```shell
$ civil-engineering solve thiem_confined_transmissivity "Q=0.02893518519 m^3/s" "r1=15 m" "r2=60 m" "s1=2.8 m" "s2=1.1 m"
T = 0.003755371735 m^2/s
T_m2_d = 324.4641179
  by: Transmissivity from steady pumping (Thiem, confined aquifer)  (T = Q ln(r2/r1)/(2 pi (s1 - s2)))
  Q = 0.02893518519 m^3/s
  r1 = 15 m
  r2 = 60 m
  s1 = 2.8 m
  s2 = 1.1 m
  assumes: steady state; confined aquifer, fully penetrating well; r1 < r2, s1 > s2
```

In the worked example, Q = 0.02893518519 m^3/s, r1 = 15 m, r2 = 60 m, s1 = 2.8 m, s2 = 1.1 m go through the equation above and give T = 0.003755371735 m^2/s, T_m2_d = 324.4641179. It applies only when steady state; confined aquifer, fully penetrating well; r1 < r2, s1 > s2: outside that it says what is wrong instead of returning a number.

<a id="dupuit_flow_with_recharge"></a>

#### Unconfined flow between two water bodies with recharge (Dupuit), `dupuit_flow_with_recharge`

Steady unconfined flow per unit width between two water bodies (heads h1 at x = 0 and h2 at x = L above the base) with uniform recharge W: the flows at both ends (positive toward x = L), the groundwater divide and the highest water table (Dupuit-Forchheimer).

$$
h^2 = h_1^2 - \frac{(h_1^2-h_2^2)x}{L} + \frac{W}{K}(L-x)x,\quad q(x) = \frac{K(h_1^2-h_2^2)}{2L} - W\left(\frac L2 - x\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `K` | in | hydraulic conductivity | m/s |
| `h1` | in | head at x = 0 (above the impermeable base) | m |
| `h2` | in | head at x = L | m |
| `L` | in | distance between the water bodies | m |
| `W` | in | recharge rate | m/s |
| `q0` | out | flow per unit width at x = 0 (m^2/s, positive toward x = L) | m^2/s |
| `qL` | out | flow per unit width at x = L (m^2/s) | m^2/s |
| `x_divide` | out | position of the groundwater divide (m); outside 0..L means no divide | m |
| `h_max` | out | highest water table between the water bodies (m) | m |

**Applies when:** steady, unconfined, homogeneous aquifer on a horizontal base; Dupuit assumptions; W > 0.

```shell
$ civil-engineering solve dupuit_flow_with_recharge "K=0.0001157407407 m/s" "h1=12 m" "h2=9 m" "L=600 m" "W=9.506426344e-09 m/s"
q0 = 3.224460984e-06 m^2/s
qL = 8.92831679e-06 m^2/s
x_divide = -339.1874998 m
h_max = 12 m
  by: Unconfined flow between two water bodies with recharge (Dupuit)  (h^2 = h1^2 - (h1^2 - h2^2) x/L + (W/K)(L - x) x; q = K(h1^2 - h2^2)/(2L) - W(L/2 - x))
  K = 0.0001157407407 m/s
  h1 = 12 m
  h2 = 9 m
  L = 600 m
  W = 9.506426344e-09 m/s
  assumes: steady, unconfined, homogeneous aquifer on a horizontal base; Dupuit assumptions; W > 0
```

**What the worked example means:** This calculation determines the steady-state unconfined flow profile between two water bodies with uniform recharge. Using hydraulic conductivity 0.000115741 m/s, boundary heads 12 m and 9 m, distance 600 m, and recharge rate 9.50643e-09 m/s, it computes the flow per unit width at both ends, 3.22446e-06 m^2/s and 8.92832e-06 m^2/s. The position of the groundwater divide is found at -339.188 m, where the highest water table reaches 12 m. These values help assess regional drainage patterns and aquifer behavior under Dupuit assumptions.

### Hydrology (17)

<a id="thiessen_polygon_rainfall"></a>

#### Areal rainfall by Thiessen polygons: `thiessen_polygon_rainfall`

Mean areal rainfall over a catchment by the Thiessen (Voronoi) polygon method: each gauge represents the part of the catchment nearer to it than to any other gauge; the areas, weights and the arithmetic mean are also given. Coordinates in km (areas in km^2), rainfall in mm; the boundary is a simple polygon listed vertex by vertex.

$$
\bar P = \sum_i \frac{A_i}{A} P_i
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `gx` | in | gauge x coordinates in km | - |
| `gy` | in | gauge y coordinates in km | - |
| `rain` | in | rainfall at each gauge in mm | - |
| `bx` | in | catchment boundary x coordinates in km | - |
| `by` | in | catchment boundary y coordinates in km | - |
| `mean` | out | Thiessen mean rainfall (mm) | - |
| `areas` | out | area represented by each gauge (km^2) | - |
| `weights` | out | Thiessen weight of each gauge | - |
| `arithmetic_mean` | out | arithmetic mean (mm) | - |
| `catchment_area` | out | catchment area (km^2) | - |

**Applies when:** distinct gauge locations; simple (non-self-intersecting) boundary polygon.

```shell
$ civil-engineering solve thiessen_polygon_rainfall "gx=[2, 8, 5]" "gy=[2, 3, 9]" "rain=[45, 60, 38]" "bx=[0, 10, 10, 0]" "by=[0, 0, 10, 10]"
mean = 47.23995192
areas = [29.6650641026,32.5608974359,37.7740384615]
weights = [0.296650641026,0.325608974359,0.377740384615]
arithmetic_mean = 47.66666667
catchment_area = 100
  by: Areal rainfall by Thiessen polygons  (clip the catchment polygon by the perpendicular-bisector half-planes of every pair of gauges; weights = cell area/catchment area)
  assumes: distinct gauge locations; simple (non-self-intersecting) boundary polygon
```

**What the worked example means:** This calculation determines the average rainfall over the specified catchment using the Thiessen polygon method. Dividing the total catchment area of 100 km^2 into zones closer to each gauge than any other assigns weights based on these areas. The resulting weighted mean rainfall is 47.24 mm, which differs from the simple arithmetic mean of 47.6667 mm because it accounts for uneven spatial distribution of the gauges within the boundary defined by coordinates [0, 10, 10, 0], [0, 0, 10, 10].

<a id="unit_hydrograph_convolution"></a>

#### Direct runoff hydrograph by unit-hydrograph convolution, `unit_hydrograph_convolution`

The direct runoff (and total) hydrograph of a storm: the convolution of the excess-rainfall depths of successive periods with the unit hydrograph of the same period, plus a constant base flow.

$$
Q_n = \sum_{m=1}^{\min(n,M)} P_m\,U_{n-m+1}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `excess` | in | excess rainfall in cm per period (one per period) | - |
| `uh` | in | unit-hydrograph ordinates in m^3/s per cm of excess, one per period | - |
| `baseflow` | in | constant base flow (default 0) (optional) | m^3/s |
| `DRO` | out | direct runoff ordinates (m^3/s) | m^3/s |
| `total` | out | total flow ordinates (m^3/s) | m^3/s |
| `peak` | out | peak total flow (m^3/s) | m^3/s |
| `peak_period` | out | period of the peak (1-based) | - |

**Applies when:** excess depths and the unit hydrograph share one period; linear, time-invariant catchment.

```shell
$ civil-engineering solve unit_hydrograph_convolution "excess=[1.5, 2.5, 1]" "uh=[0, 10, 30, 22, 12, 5, 0]"
DRO = [0.0,15.0,70.0,118.0,103.0,59.5,24.5,5.0,0.0] m^3/s
total = [0.0,15.0,70.0,118.0,103.0,59.5,24.5,5.0,0.0] m^3/s
peak = 118 m^3/s
peak_period = 4
  by: Direct runoff hydrograph by unit-hydrograph convolution  (Q_k = sum_i P_i U_(k-i+1) (discrete convolution))
  assumes: excess depths and the unit hydrograph share one period; linear, time-invariant catchment
```

**What the worked example means:** This calculation determines the direct runoff and total streamflow hydrograph by convolving the excess rainfall depths with the unit hydrograph ordinates. The resulting direct runoff ordinates are added to the constant base flow to yield the total flow. For this specific storm event, the peak total flow reached 118 m^3/s m^3/s during period 4, reflecting the catchment's linear response to the input rainfall pattern.

<a id="green_ampt_infiltration"></a>

#### Green-Ampt cumulative infiltration under ponding, `green_ampt_infiltration`

Cumulative infiltration and infiltration rate at time t for a soil ponded from t = 0, by the Green-Ampt model (implicit equation solved to full precision).

$$
F - \psi\Delta\theta\,\ln\!\left(1+\frac{F}{\psi\Delta\theta}\right) = Kt,\quad f = K\left(1+\frac{\psi\Delta\theta}{F}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `K` | in | saturated hydraulic conductivity | m/s |
| `psi` | in | wetting-front suction head | m |
| `dtheta` | in | moisture deficit (porosity minus initial water content) | - |
| `t` | in | time since ponding began | s |
| `F` | out | cumulative infiltration (m) | m |
| `f` | out | infiltration rate (m/s) | m/s |
| `F_mm` | out | cumulative infiltration (mm) | - |
| `f_mm_h` | out | infiltration rate (mm/h) | - |

**Applies when:** ponded surface from t = 0; homogeneous soil, sharp wetting front.

```shell
$ civil-engineering solve green_ampt_infiltration "K=3.027777778e-06 m/s" "psi=0.1101 m" dtheta=0.247 "t=3600 s"
F = 0.03209669252 m
f = 5.593135822e-06 m/s
F_mm = 32.09669252
f_mm_h = 20.13528896
  by: Green-Ampt cumulative infiltration under ponding  (F - psi dtheta ln(1 + F/(psi dtheta)) = K t (bisection); f = K(1 + psi dtheta/F))
  K = 3.027777778e-06 m/s
  psi = 0.1101 m
  dtheta = 0.247
  t = 3600 s
  assumes: ponded surface from t = 0; homogeneous soil, sharp wetting front
```

**What the worked example means:** The calculation determines the total water absorbed into the soil, which is 0.0320967 m, and the instantaneous infiltration rate, which is 5.59314e-06 m/s. These values are derived from the Green-Ampt model using a saturated hydraulic conductivity of 3.02778e-06 m/s, a wetting-front suction head of 0.1101 m, a moisture deficit of 0.247, and a time since ponding of 3600 s. This result helps engineers estimate runoff potential and design drainage systems for soils that are fully covered by water.

<a id="gumbel_design_flood"></a>

#### Gumbel (EV1) design flood: `gumbel_design_flood`

The T-year flood from a series of annual maximum discharges by the Gumbel (extreme value type I) distribution fitted by the method of moments (Chow's frequency factor for a long record; sample standard deviation with n - 1).

$$
Q_T = \bar Q + K_T s,\quad K_T = -\frac{\sqrt6}{\pi}\left[0.5772 + \ln\ln\frac{T}{T-1}\right]
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `peaks` | in | annual maximum discharges in m^3/s | - |
| `T` | in | return period (years) | - |
| `Q_T` | out | T-year flood (m^3/s) | m^3/s |
| `K_T` | out | frequency factor | - |
| `mean` | out | mean annual maximum (m^3/s) | m^3/s |
| `std` | out | sample standard deviation (m^3/s) | m^3/s |

**Applies when:** at least five annual maxima; T > 1 year; method of moments (no sample-size correction table).

```shell
$ civil-engineering solve gumbel_design_flood "peaks=[420, 380, 510, 290, 640, 455, 370, 520, 600, 330]" T=100
Q_T = 812.63712 m^3/s
K_T = 3.13666843
mean = 451.5 m^3/s
std = 115.1339799 m^3/s
  by: Gumbel (EV1) design flood  (K_T = -(sqrt 6/pi)(0.5772 + ln ln(T/(T - 1))), Q_T = mean + K_T s)
  T = 100
  assumes: at least five annual maxima; T > 1 year; method of moments (no sample-size correction table)
```

**What the worked example means:** This calculation determines the design flood magnitude for a specified return period using the Gumbel extreme value distribution. It begins by computing the arithmetic mean and sample standard deviation of the annual maximum discharge records, yielding values of 451.5 m^3/s and 115.134 m^3/s respectively. These statistics are then combined with a frequency factor of 3.13667, which depends on the return period 100, to produce the final estimate of 812.637 m^3/s. This result represents the discharge expected to be exceeded on average once every 100 years, serving as a critical parameter for hydraulic structure sizing.

<a id="horton_infiltration"></a>

#### Horton infiltration rate and cumulative infiltration, `horton_infiltration`

Infiltration capacity at time t and cumulative infiltration from the start of ponding by Horton's equation.

$$
f = f_c + (f_0-f_c)e^{-kt},\quad F = f_c t + \frac{f_0-f_c}{k}\left(1-e^{-kt}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `f0` | in | initial infiltration capacity | m/s |
| `fc` | in | final (constant) infiltration capacity | m/s |
| `k` | in | Horton decay constant | - |
| `t` | in | time since ponding began | s |
| `f` | out | infiltration capacity at t (m/s) | m/s |
| `F` | out | cumulative infiltration (m) | m |
| `f_mm_h` | out | infiltration capacity (mm/h) | - |
| `F_mm` | out | cumulative infiltration (mm) | - |

**Applies when:** 0 <= fc < f0; ponded surface from t = 0.

```shell
$ civil-engineering solve horton_infiltration "f0=2.083333333e-05 m/s" "fc=2.777777778e-06 m/s" k=0.001111111111 "t=3600 s"
f = 3.108476814e-06 m/s
F = 0.02595237087 m
f_mm_h = 11.19051653
F_mm = 25.95237087
  by: Horton infiltration rate and cumulative infiltration  (f = fc + (f0 - fc) e^(-k t), F = fc t + (f0 - fc)(1 - e^(-k t))/k)
  f0 = 2.083333333e-05 m/s
  fc = 2.777777778e-06 m/s
  k = 0.001111111111
  t = 3600 s
  assumes: 0 <= fc < f0; ponded surface from t = 0
```

**What the worked example means:** This calculation determines the soil's ability to absorb water over time using Horton's equation. Given an initial capacity of 2.08333e-05 m/s, a final constant capacity of 2.77778e-06 m/s, and a decay rate defined by 0.00111111, the model computes the current capacity as 3.10848e-06 m/s and the total water absorbed since ponding began at time 3600 s as 0.0259524 m. These values help estimate how much rainfall will run off versus soak into the ground.

<a id="kling_gupta_efficiency"></a>

#### Kling-Gupta efficiency: `kling_gupta_efficiency`

Kling-Gupta efficiency of simulated against observed values (Gupta et al. 2009) with its components: the correlation r, the variability ratio alpha (population standard deviations) and the bias ratio beta.

$$
KGE = 1-\sqrt{(r-1)^2+(\alpha-1)^2+(\beta-1)^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `obs` | in | observed values (any unit) | - |
| `sim` | in | simulated values, same unit and order | - |
| `KGE` | out | Kling-Gupta efficiency | - |
| `r` | out | Pearson correlation | - |
| `alpha` | out | ratio of standard deviations sim/obs | - |
| `beta` | out | ratio of means sim/obs | - |

**Applies when:** paired series of equal length; both series vary.

```shell
$ civil-engineering solve kling_gupta_efficiency "obs=[12, 30, 55, 41, 22, 15]" "sim=[10, 34, 50, 44, 20, 17]"
KGE = 0.9627391636
r = 0.9768699556
alpha = 0.970787486
beta = 1
  by: Kling-Gupta efficiency  (KGE = 1 - sqrt((r - 1)^2 + (alpha - 1)^2 + (beta - 1)^2))
  assumes: paired series of equal length; both series vary
```

**What the worked example means:** This calculation determines the Kling-Gupta efficiency, which measures how well simulated values match observed data by combining three distinct components. The result of 0.962739 reflects the overall performance based on the Pearson correlation coefficient of 0.97687, the ratio of standard deviations known as alpha at 0.970787, and the ratio of means known as beta at 1. A value closer to one indicates that the simulation accurately captures the timing, magnitude, and variability of the observed series.

<a id="log_pearson_iii_design_flood"></a>

#### Log-Pearson type III design flood: `log_pearson_iii_design_flood`

The T-year flood by the log-Pearson type III distribution fitted by moments of the base-10 logarithms (station skew only, no regional skew weighting or outlier tests), frequency factor from the exact Pearson III quantile.

$$
\log Q_T = \bar y + K_T(G, T)\,s_y
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `peaks` | in | annual maximum discharges in m^3/s | - |
| `T` | in | return period (years) | - |
| `Q_T` | out | T-year flood (m^3/s) | m^3/s |
| `K_T` | out | Pearson III frequency factor | - |
| `mean_log` | out | mean of log10 Q | - |
| `std_log` | out | standard deviation of log10 Q | - |
| `skew` | out | skew coefficient of log10 Q (unbiased) | - |

**Applies when:** at least ten annual maxima; station skew only; no outlier screening.

```shell
$ civil-engineering solve log_pearson_iii_design_flood "peaks=[420, 380, 510, 290, 640, 455, 370, 520, 600, 330, 880, 410]" T=100
Q_T = 1083.770605 m^3/s
K_T = 2.753907684
mean_log = 2.664610768
std_log = 0.1344731349
skew = 0.598210844
  by: Log-Pearson type III design flood  (moments of log10 Q; K_T = Pearson III quantile (gamma inverse with shape 4/G^2); Q_T = 10^(m + K s))
  T = 100
  assumes: at least ten annual maxima; station skew only; no outlier screening
```

**What the worked example means:** This calculation determines the design flood for a return period of 100 years by fitting the Log-Pearson Type III distribution to annual maximum discharges. The process begins by computing the mean, standard deviation, and skew coefficient of the base-10 logarithms of the input peaks, resulting in values of 2.66461, 0.134473, and 0.598211 respectively. These statistics are used to derive the frequency factor 2.75391, which scales the variability around the mean log discharge. The final output, 1083.77 m^3/s, represents the estimated peak flow rate that has a probability of being equaled or exceeded once every 100 years on average.

<a id="muskingum_routing"></a>

#### Muskingum flood routing: `muskingum_routing`

Routes an inflow hydrograph through a river reach by the Muskingum method (storage S = K[X I + (1 - X) O]), giving the outflow ordinates, routing coefficients, peak outflow and attenuation. The first outflow equals the first inflow unless stated.

$$
O_{j+1} = C_0 I_{j+1} + C_1 I_j + C_2 O_j,\quad C_0 = \frac{\Delta t-2KX}{2K(1-X)+\Delta t}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `inflow` | in | inflow ordinates in m^3/s at equal intervals | - |
| `K` | in | storage (travel-time) constant | s |
| `X` | in | weighting factor (0 to 0.5) | - |
| `dt` | in | routing interval | s |
| `O1` | in | initial outflow (default: the first inflow) (optional) | m^3/s |
| `outflow` | out | outflow ordinates (m^3/s) | m^3/s |
| `C0` | out | coefficient C0 | - |
| `C1` | out | coefficient C1 | - |
| `C2` | out | coefficient C2 | - |
| `peak_outflow` | out | peak outflow (m^3/s) | m^3/s |
| `attenuation` | out | peak attenuation (m^3/s) | m^3/s |

**Applies when:** 0 <= X <= 0.5; 2 K X <= dt <= 2 K (1 - X) (no negative coefficient).

```shell
$ civil-engineering solve muskingum_routing "inflow=[10, 30, 68, 50, 40, 31, 23, 15, 10]" "K=43200 s" X=0.2 "dt=21600 s"
outflow = [10.0,10.9523809524,21.8321995465,42.959723572,45.8360456806,42.6284048803,36.7101168421,29.8005373935,22.5145672061] m^3/s
C0 = 0.04761904762
C1 = 0.4285714286
C2 = 0.5238095238
peak_outflow = 45.83604568 m^3/s
attenuation = 22.16395432 m^3/s
  by: Muskingum flood routing  (O_j+1 = C0 I_j+1 + C1 I_j + C2 O_j, C0 = (dt - 2KX)/D, C1 = (dt + 2KX)/D, C2 = (2K(1 - X) - dt)/D, D = 2K(1 - X) + dt)
  K = 43200 s
  X = 0.2
  dt = 21600 s
  assumes: 0 <= X <= 0.5; 2 K X <= dt <= 2 K (1 - X) (no negative coefficient)
```

**What the worked example means:** This calculation routes the inflow hydrograph through the river reach using the Muskingum method. It computes the storage constant and weighting factor to determine the routing coefficients C0, C1, and C2. The resulting outflow ordinates show how the flood wave is modified by the reach's storage capacity. The final peak outflow is 45.836 m^3/s, which indicates that the reach reduced the original peak flow by 22.164 m^3/s. This helps in understanding the downstream impact of the flood event.

<a id="nash_sutcliffe_efficiency"></a>

#### Nash-Sutcliffe efficiency, RMSE and percent bias, `nash_sutcliffe_efficiency`

Goodness of fit of simulated to observed flows: Nash-Sutcliffe efficiency, root-mean-square error and percent bias (positive when the model underestimates, as hydroeval defines it).

$$
NSE = 1 - \frac{\sum (O_i-S_i)^2}{\sum (O_i-\bar O)^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `obs` | in | observed values (any unit) | - |
| `sim` | in | simulated values, same unit and order | - |
| `NSE` | out | Nash-Sutcliffe efficiency | - |
| `RMSE` | out | root-mean-square error (unit of the data) | - |
| `PBIAS` | out | percent bias 100 sum(obs - sim)/sum(obs) | - |

**Applies when:** paired series of equal length; observed values not all equal.

```shell
$ civil-engineering solve nash_sutcliffe_efficiency "obs=[12, 30, 55, 41, 22, 15]" "sim=[10, 34, 50, 44, 20, 17]"
NSE = 0.9542379136
RMSE = 3.214550254
PBIAS = 0
  by: Nash-Sutcliffe efficiency, RMSE and percent bias  (NSE = 1 - sum(obs - sim)^2/sum(obs - mean obs)^2)
  assumes: paired series of equal length; observed values not all equal
```

In the worked example, obs = [12, 30, 55, 41, 22, 15], sim = [10, 34, 50, 44, 20, 17] go through the equation above and give NSE = 0.9542379136, RMSE = 3.214550254, PBIAS = 0. It applies only when paired series of equal length; observed values not all equal: outside that it says what is wrong instead of returning a number.

<a id="rational_method_peak_runoff"></a>

#### Peak runoff by the rational method: `rational_method_peak_runoff`

The peak runoff Q = C i A of a small catchment from the runoff coefficient, the rainfall intensity for a duration equal to the time of concentration, and the catchment area.

$$
Q = C\,i\,A
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `C` | in | runoff coefficient (0 to 1) | - |
| `i` | in | rainfall intensity | m/s |
| `A` | in | catchment area | m^2 |
| `Q` | out | peak runoff (m^3/s) | m^3/s |
| `Q_Ls` | out | peak runoff (L/s) | - |

**Applies when:** 0 < C <= 1; A <= 800 ha (small catchments); intensity for a duration equal to the time of concentration.

```shell
$ civil-engineering solve rational_method_peak_runoff C=0.6 "i=1.388888889e-05 m/s" "A=200000 m^2"
Q = 1.666666667 m^3/s
Q_Ls = 1666.666667
  by: Peak runoff by the rational method  (Q = C i A (SI); = 0.278 C i[mm/h] A[km^2])
  C = 0.6
  i = 1.388888889e-05 m/s
  A = 200000 m^2
  assumes: 0 < C <= 1; A <= 800 ha (small catchments); intensity for a duration equal to the time of concentration
```

**What the worked example means:** This calculation determines the peak runoff rate using the rational method. It combines the runoff coefficient, rainfall intensity, and catchment area to estimate the maximum flow. The resulting peak discharge is 1.66667 m^3/s, which equals 1666.67 when expressed in liters per second.

<a id="phi_index"></a>

#### Phi-index of a storm: `phi_index`

The constant loss rate (phi-index) that, subtracted from every interval of a rainfall hyetograph, leaves exactly the observed runoff depth; with the excess rainfall of each interval.

$$
\sum_i \max(P_i - \phi\,\Delta t,\,0) = R
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `rain` | in | rainfall depth in each interval in mm | - |
| `dt` | in | duration of one interval | s |
| `R` | in | observed direct runoff depth | m |
| `phi` | out | phi-index (mm/h) | - |
| `excess` | out | excess rainfall in each interval (mm) | - |
| `contributing` | out | number of intervals producing runoff | - |

**Applies when:** 0 < runoff < total rainfall; equal intervals.

```shell
$ civil-engineering solve phi_index "rain=[4, 9, 18, 12, 3, 1]" "dt=3600 s" "R=0.02 m"
phi = 6.333333333
excess = [0.0,2.66666666667,11.6666666667,5.66666666667,0.0,0.0]
contributing = 3
  by: Phi-index of a storm  (the k largest intervals contribute: phi dt = (sum of the k largest - R)/k, with k found by consistency)
  dt = 3600 s
  R = 0.02 m
  assumes: 0 < runoff < total rainfall; equal intervals
```

**What the worked example means:** This calculation finds the constant loss rate, known as the phi-index, that reduces the given rainfall hyetograph to match the observed direct runoff depth. By applying a uniform subtraction of 6.33333 to each interval, exactly 3 intervals generate excess rainfall totaling 0.02 m. This result indicates the average infiltration capacity during the storm, allowing engineers to separate effective rainfall from losses for hydrograph analysis.

<a id="design_return_period_for_risk"></a>

#### Return period for an acceptable risk over a design life, `design_return_period_for_risk`

The design return period whose event has only the stated probability R of being exceeded during an n-year life.

$$
T = \frac{1}{1-(1-R)^{1/n}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `R` | in | acceptable risk of exceedance (fraction, or % in a question) | - |
| `n` | in | design life (whole years) | - |
| `T` | out | required return period (years) | - |
| `p_annual` | out | annual exceedance probability | - |

**Applies when:** 0 < R < 1; independent years.

```shell
$ civil-engineering solve design_return_period_for_risk R=0.1 n=50
T = 475.0612547
p_annual = 0.002104991704
  by: Return period for an acceptable risk over a design life  (T = 1/(1 - (1 - R)^(1/n)))
  R = 0.1
  assumes: 0 < R < 1; independent years
```

**What the worked example means:** This calculation determines the design return period necessary to ensure that the probability of an event being exceeded during a 50-year life remains at or below the acceptable risk of 0.1. The resulting return period is 475.061 years, which implies that any single year has an annual exceedance probability of 0.00210499. This approach allows engineers to balance safety requirements with economic constraints by linking the structural lifespan directly to the statistical likelihood of extreme events.

<a id="weibull_plotting_positions"></a>

#### Return periods by the Weibull plotting position, `weibull_plotting_positions`

Ranks annual maxima from largest to smallest and gives each its exceedance probability m/(n + 1) and return period (n + 1)/m (Weibull plotting position).

$$
P = \frac{m}{n+1},\quad T = \frac{n+1}{m}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `peaks` | in | annual maxima (any unit; the ranking keeps it) | - |
| `ranked` | out | values ranked from largest to smallest | - |
| `T` | out | return period of each (years) | - |
| `P` | out | exceedance probability of each | - |

**Applies when:** annual series; ties keep distinct ranks.

```shell
$ civil-engineering solve weibull_plotting_positions "peaks=[420, 380, 510, 290, 640]"
ranked = [640.0,510.0,420.0,380.0,290.0]
T = [6.0,3.0,2.0,1.5,1.2]
P = [0.166666666667,0.333333333333,0.5,0.666666666667,0.833333333333]
  by: Return periods by the Weibull plotting position  (rank m from the largest; P = m/(n + 1), T = 1/P)
  assumes: annual series; ties keep distinct ranks
```

**What the worked example means:** This calculation ranks the input annual maxima from largest to smallest. For each value, it determines the rank m relative to the total number of observations n. The exceedance probability is calculated as the rank divided by one plus the total count, resulting in values like [0.166667, 0.333333, 0.5, 0.666667, 0.833333]. Consequently, the return period is derived as the reciprocal of this probability, or equivalently, one plus the total count divided by the rank, producing results such as [6, 3, 2, 1.5, 1.2] years. This method provides a non-parametric estimate of how frequently extreme events are expected to occur based on historical data.

<a id="flood_exceedance_risk"></a>

#### Risk of exceedance over a design life: `flood_exceedance_risk`

Probability that an event of return period T is equalled or exceeded at least once in n years (independent years), with the annual exceedance probability.

$$
R = 1 - \left(1 - \frac{1}{T}\right)^n
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T` | in | return period (years) | - |
| `n` | in | design life or period (whole years) | - |
| `risk` | out | probability of at least one exceedance in n years | - |
| `p_annual` | out | annual exceedance probability 1/T | - |
| `p_none` | out | probability of no exceedance | - |

**Applies when:** T > 1 year; independent years.

```shell
$ civil-engineering solve flood_exceedance_risk T=100 n=50
risk = 0.3949939329
p_annual = 0.01
p_none = 0.6050060671
  by: Risk of exceedance over a design life  (risk = 1 - (1 - 1/T)^n)
  T = 100
  assumes: T > 1 year; independent years
```

**What the worked example means:** This calculation determines the probability that an event with a specified return period will be equaled or exceeded at least once during the project's design life. Using a return period of 100 and a duration of 50 years, the annual exceedance probability is found to be 0.01. The resulting total risk of at least one occurrence is 0.394994, while the probability of no occurrences is 0.605006. This metric helps engineers understand long-term exposure rather than relying solely on single-year probabilities.

<a id="scs_curve_number_runoff"></a>

#### SCS curve-number runoff depth: `scs_curve_number_runoff`

Direct runoff depth from a storm rainfall depth by the SCS (NRCS) curve-number method, with the potential retention S and the initial abstraction Ia = lambda S (lambda = 0.2 unless stated).

$$
Q = \frac{(P-I_a)^2}{P-I_a+S},\quad S = \frac{25400}{CN}-254,\quad I_a = \lambda S
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `P` | in | storm rainfall depth | m |
| `CN` | in | curve number (30 to 100) | - |
| `lam` | in | initial-abstraction ratio (default 0.2) (optional) | - |
| `Q` | out | runoff depth (mm) | - |
| `S` | out | potential maximum retention (mm) | - |
| `Ia` | out | initial abstraction (mm) | - |
| `C` | out | runoff ratio Q/P | - |

**Applies when:** 30 <= CN <= 100; rainfall depth in mm; 0 <= lambda <= 0.3.

```shell
$ civil-engineering solve scs_curve_number_runoff "P=0.1 m" CN=80
Q = 50.53905836
S = 63.5
Ia = 12.7
C = 0.5053905836
  by: SCS curve-number runoff depth  (S = 25400/CN - 254 (mm), Ia = lambda S, Q = (P - Ia)^2/(P - Ia + S) for P > Ia, else 0)
  P = 0.1 m
  CN = 80
  assumes: 30 <= CN <= 100; rainfall depth in mm; 0 <= lambda <= 0.3
```

**What the worked example means:** The computed runoff depth is 50.5391 for a rainfall depth of 0.1 m, resulting in a runoff ratio of 0.505391.

<a id="kirpich_time_of_concentration"></a>

#### Time of concentration by the Kirpich formula, `kirpich_time_of_concentration`

The time of concentration tc = 0.0078 L^0.77 S^-0.385 minutes (L in feet; equivalent to 0.0195 L^0.77 S^-0.385 with L in metres) of a small rural catchment from the flow length and the average slope.

$$
t_c = 0.0078\,L^{0.77}S^{-0.385}\ \text{(min, L in ft)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `L` | in | length of the longest flow path | m |
| `S` | in | average slope (m/m) | - |
| `tc` | out | time of concentration (s) | s |
| `tc_min` | out | time of concentration (min) | - |
| `tc_h` | out | time of concentration (h) | - |

**Applies when:** 0 < S <= 1; L <= 100 km; small rural catchments (Kirpich's data: 0.4 to 45 ha).

```shell
$ civil-engineering solve kirpich_time_of_concentration "L=1500 m" S=0.02
tc = 1469.765514 s
tc_min = 24.4960919
tc_h = 0.4082681983
  by: Time of concentration by the Kirpich formula  (tc = 0.0078 L_ft^0.77 S^-0.385 min, L_ft = L/0.3048)
  L = 1500 m
  S = 0.02
  assumes: 0 < S <= 1; L <= 100 km; small rural catchments (Kirpich's data: 0.4 to 45 ha)
```

**What the worked example means:** This calculation determines the time of concentration for water to travel from the most remote point in a catchment to its outlet using the Kirpich formula. By inputting a longest flow path length of 1500 m and an average slope of 0.02, the model computes a result of 24.4961 minutes. This value is essential for estimating rainfall intensity in hydrological analyses, particularly when applying the rational method to small rural watersheds where channel flow dominates.

<a id="unit_hydrograph_derivation"></a>

#### Unit hydrograph from a storm by least-squares deconvolution, `unit_hydrograph_derivation`

The unit hydrograph that best reproduces an observed direct-runoff hydrograph from its excess-rainfall depths (least-squares solution of the convolution equations), with the RMS misfit.

$$
\min_u \lVert A u - Q\rVert_2,\quad A_{kj} = P_{k-j+1}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `dro` | in | observed direct runoff ordinates in m^3/s (base flow removed) | - |
| `excess` | in | excess rainfall in cm per period | - |
| `uh` | out | unit-hydrograph ordinates (m^3/s per cm) | m^3/s |
| `rms_residual` | out | RMS misfit of the reconstructed runoff (m^3/s) | m^3/s |
| `peak` | out | unit-hydrograph peak (m^3/s per cm) | m^3/s |

**Applies when:** len(dro) >= len(excess) + 1; the same period for both.

```shell
$ civil-engineering solve unit_hydrograph_derivation "dro=[0, 15, 61, 77, 52, 29, 11, 3]" "excess=[1.5, 2]"
uh = [-0.502834815065,11.0475725314,25.6537253747,17.3404996047,11.3869004489,4.27012446017,1.55034025912] m^3/s
rms_residual = 0.4011393048 m^3/s
peak = 25.65372537 m^3/s
  by: Unit hydrograph from a storm by least-squares deconvolution  (least squares on the convolution system: (A^T A) u = A^T Q)
  assumes: len(dro) >= len(excess) + 1; the same period for both
```

**What the worked example means:** This calculation determines the unit hydrograph that best matches the observed direct runoff by solving a least-squares problem. The resulting ordinates, such as the peak flow of 25.6537 m^3/s m^3/s per cm, represent the watershed's response to one centimeter of excess rainfall. The quality of this fit is quantified by the root mean square residual of 0.401139 m^3/s m^3/s, indicating how closely the reconstructed hydrograph aligns with the measured data.

### Open-channel hydraulics (23)

<a id="broad_crested_weir_discharge"></a>

#### Broad-crested weir discharge: `broad_crested_weir_discharge`

Discharge over a broad-crested weir for a stated discharge coefficient (critical depth 2H/3 on the crest, approach velocity head included in H).

$$
Q = C_d\,b\sqrt{g}\left(\tfrac{2}{3}H\right)^{3/2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Cd` | in | discharge coefficient (stated, e.g. 0.95) | - |
| `b` | in | crest width | m |
| `H` | in | upstream head over the crest | m |
| `Q` | out | discharge (m^3/s) | m^3/s |
| `yc` | out | critical depth on the crest 2H/3 (m) | m |

**Applies when:** Cd stated; critical flow on the crest; H includes the approach velocity head.

```shell
$ civil-engineering solve broad_crested_weir_discharge Cd=0.95 "b=5 m" "H=0.6 m"
Q = 3.763726876 m^3/s
yc = 0.4 m
  by: Broad-crested weir discharge  (Q = Cd b sqrt(g) (2H/3)^(3/2))
  Cd = 0.95
  b = 5 m
  H = 0.6 m
  assumes: Cd stated; critical flow on the crest; H includes the approach velocity head
```

**What the worked example means:** This calculation determines the flow rate over a broad-crested weir using the given discharge coefficient, crest width, and upstream head. The result indicates that the total discharge is 3.76373 m^3/s, while the water depth at the critical point on the crest is 0.4 m. This method assumes that the flow reaches critical conditions on the wide crest and that the measured head includes the velocity of approach.

<a id="chezy_from_manning"></a>

#### Chezy coefficient from Manning's n: `chezy_from_manning`

The Chezy coefficient equivalent to a Manning roughness n at hydraulic radius R (SI units).

$$
C = \frac{R^{1/6}}{n}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `n` | in | Manning roughness coefficient | - |
| `R` | in | hydraulic radius | m |
| `C` | out | Chezy coefficient (m^(1/2)/s) | m^0.5/s |

**Applies when:** 0.005 <= n <= 0.2; R > 0.

```shell
$ civil-engineering solve chezy_from_manning n=0.013 "R=0.75 m"
C = 73.32186869 m^0.5/s
  by: Chezy coefficient from Manning's n  (C = R^(1/6)/n)
  n = 0.013
  R = 0.75 m
  assumes: 0.005 <= n <= 0.2; R > 0
```

**What the worked example means:** This calculation determines the equivalent Chezy coefficient using Manning's roughness coefficient and the hydraulic radius. The result, 73.3219 m^0.5/s, represents the flow resistance parameter in SI units. It allows engineers to convert between Manning's and Chezy's formulations for uniform steady flow conditions.

<a id="chezy_velocity"></a>

#### Chezy velocity: `chezy_velocity`

Mean velocity of uniform flow from the Chezy equation for a Chezy coefficient C (m^(1/2)/s), hydraulic radius R and slope S.

$$
V = C\sqrt{R S}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `C` | in | Chezy coefficient (m^(1/2)/s) | m^0.5/s |
| `R` | in | hydraulic radius | m |
| `S` | in | channel (energy) slope, m/m | - |
| `V` | out | mean velocity (m/s) | m/s |

**Applies when:** C, R > 0; 0 < S < 1.

```shell
$ civil-engineering solve chezy_velocity "C=55 m^0.5/s" "R=1.1 m" S=0.0004
V = 1.153689733 m/s
  by: Chezy velocity  (V = C sqrt(R S))
  C = 55 m^0.5/s
  R = 1.1 m
  S = 0.0004
  assumes: C, R > 0; 0 < S < 1
```

**What the worked example means:** This calculation determines the average speed of water flowing through an open channel. It uses the Chezy equation, which combines the resistance factor known as the Chezy coefficient 55 m^0.5/s with the geometric property called hydraulic radius 1.1 m and the slope of the energy grade line 0.0004. The resulting value 1.15369 m/s represents how fast the flow moves on average across the channel cross-section.

<a id="critical_depth_circular"></a>

#### Critical depth in a circular conduit: `critical_depth_circular`

Critical depth, area, velocity and minimum specific energy for a discharge Q in a circular pipe or culvert flowing part full (refused when the critical depth would exceed 0.95 D).

$$
\frac{Q^2\,T}{g\,A^3} = 1,\quad A = \frac{D^2}{8}(\theta-\sin\theta),\ T = D\sin\frac{\theta}{2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Q` | in | discharge | m^3/s |
| `D` | in | pipe diameter | m |
| `yc` | out | critical depth (m) | m |
| `Ac` | out | critical flow area (m^2) | m^2 |
| `Vc` | out | critical velocity (m/s) | m/s |
| `Emin` | out | minimum specific energy (m) | m |

**Applies when:** Q, D > 0; critical depth below 0.95 D.

```shell
$ civil-engineering solve critical_depth_circular "Q=0.5 m^3/s" "D=1 m"
yc = 0.3988412681 m
Ac = 0.2922347611 m^2
Vc = 1.710953201 m/s
Emin = 0.548044166 m
  by: Critical depth in a circular conduit  (bisection on Q^2 T(y) = g A(y)^3 with circular-segment geometry)
  Q = 0.5 m^3/s
  D = 1 m
  assumes: Q, D > 0; critical depth below 0.95 D
```

**What the worked example means:** This calculation determines the critical flow conditions for a circular conduit carrying a discharge of 0.5 m^3/s with a pipe diameter of 1 m. The method iteratively solves the specific energy equation using circular segment geometry to find the depth where the Froude number equals one. The resulting critical depth is 0.398841 m, which corresponds to a flow area of 0.292235 m^2, a critical velocity of 1.71095 m/s, and a minimum specific energy of 0.548044 m. These values are essential for assessing whether the pipe operates under subcritical or supercritical flow regimes.

<a id="critical_depth_rectangular"></a>

#### Critical depth in a rectangular channel: `critical_depth_rectangular`

Critical depth, critical velocity and minimum specific energy for a discharge Q in a rectangular channel.

$$
y_c = \left(\frac{q^2}{g}\right)^{1/3},\quad E_{min} = \tfrac{3}{2}y_c
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Q` | in | discharge | m^3/s |
| `b` | in | channel width | m |
| `yc` | out | critical depth (m) | m |
| `Vc` | out | critical velocity (m/s) | m/s |
| `Emin` | out | minimum specific energy 1.5 yc (m) | m |
| `q` | out | discharge per unit width (m^2/s) | m^2/s |

**Applies when:** Q, b > 0.

```shell
$ civil-engineering solve critical_depth_rectangular "Q=8 m^3/s" "b=3 m"
yc = 0.8983029359 m
Vc = 2.96856056 m/s
Emin = 1.347454404 m
q = 2.666666667 m^2/s
  by: Critical depth in a rectangular channel  (yc = (q^2/g)^(1/3), q = Q/b)
  Q = 8 m^3/s
  b = 3 m
  assumes: Q, b > 0
```

**What the worked example means:** This calculation determines the critical flow conditions for a rectangular channel using a discharge of 8 m^3/s and a width of 3 m. The resulting critical depth is 0.898303 m, which represents the depth where the Froude number equals one. At this state, the specific energy reaches its minimum value of 1.34745 m, and the average velocity becomes 2.96856 m/s. These values define the boundary between subcritical and supercritical flow regimes.

<a id="critical_depth_trapezoidal"></a>

#### Critical depth in a trapezoidal channel: `critical_depth_trapezoidal`

Critical depth, area, velocity and minimum specific energy for a discharge Q in a trapezoidal (or triangular, b = 0) channel, from Q^2 T = g A^3.

$$
\frac{Q^2\,T}{g\,A^3} = 1,\quad A = (b+zy)y,\ T = b+2zy
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Q` | in | discharge | m^3/s |
| `b` | in | bottom width | m |
| `z` | in | side slope, horizontal per unit vertical | - |
| `yc` | out | critical depth (m) | m |
| `Ac` | out | critical flow area (m^2) | m^2 |
| `Vc` | out | critical velocity (m/s) | m/s |
| `Emin` | out | minimum specific energy (m) | m |

**Applies when:** Q > 0; b >= 0; z > 0.

```shell
$ civil-engineering solve critical_depth_trapezoidal "Q=15 m^3/s" "b=3 m" z=2
yc = 1.071080027 m
Ac = 5.507664928 m^2
Vc = 2.723477226 m/s
Emin = 1.449129374 m
  by: Critical depth in a trapezoidal channel  (bisection on Q^2 T(y) = g A(y)^3)
  Q = 15 m^3/s
  b = 3 m
  z = 2
  assumes: Q > 0; b >= 0; z > 0
```

**What the worked example means:** This calculation determines the critical depth, cross-sectional area, velocity, and minimum specific energy for a trapezoidal or triangular channel carrying a discharge of 15 m^3/s with a bottom width of 3 m and side slope of 2. The result indicates that at this depth of 1.07108 m, the Froude number equals one, meaning the flow transitions between subcritical and supercritical states. The corresponding critical area is 5.50766 m^2 square meters, and the mean velocity is 2.72348 m/s meters per second.

<a id="orifice_discharge"></a>

#### Discharge through a small orifice under a head, `orifice_discharge`

Discharge through a small sharp-edged orifice of diameter d under a constant head h for a stated discharge coefficient (Torricelli's velocity times Cd times the orifice area).

$$
Q = C_d\,\frac{\pi d^2}{4}\sqrt{2gh}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Cd` | in | discharge coefficient (stated, e.g. 0.61) | - |
| `d` | in | orifice diameter | m |
| `h` | in | head above the orifice centre | m |
| `Q` | out | discharge (m^3/s) | m^3/s |
| `V_ideal` | out | ideal jet velocity sqrt(2 g h) (m/s) | m/s |
| `area` | out | orifice area (m^2) | m^2 |

**Applies when:** small orifice (d much smaller than h); free jet; Cd stated.

```shell
$ civil-engineering solve orifice_discharge Cd=0.61 "d=0.05 m" "h=3 m"
Q = 0.009189033904 m^3/s
V_ideal = 7.672027112 m/s
area = 0.001963495408 m^2
  by: Discharge through a small orifice under a head  (Q = Cd (pi d^2/4) sqrt(2 g h))
  Cd = 0.61
  d = 0.05 m
  h = 3 m
  assumes: small orifice (d much smaller than h); free jet; Cd stated
```

**What the worked example means:** This calculation determines the volumetric flow rate through a small sharp-edged orifice. Using the stated discharge coefficient, the ideal jet velocity derived from the head, and the geometric area of the opening, the result indicates that water flows at a rate of 0.00918903 m^3/s. This value represents the actual volume of fluid passing through the hole per second, accounting for energy losses and contraction effects inherent in real-world flow conditions.

<a id="channel_froude_number"></a>

#### Froude number of flow in a rectangular channel, `channel_froude_number`

Froude number V/sqrt(g y) of flow at depth y in a rectangular channel, and whether the flow is subcritical, critical or supercritical.

$$
Fr = \frac{V}{\sqrt{g\,y}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Q` | in | discharge | m^3/s |
| `b` | in | channel width | m |
| `y` | in | flow depth | m |
| `Fr` | out | Froude number | - |
| `V` | out | mean velocity (m/s) | m/s |
| `regime` | out | flow regime: subcritical, critical or supercritical | - |

**Applies when:** Q, b, y > 0.

```shell
$ civil-engineering solve channel_froude_number "Q=12 m^3/s" "b=4 m" "y=0.5 m"
Fr = 2.709141846
V = 6 m/s
regime = supercritical
  by: Froude number of flow in a rectangular channel  (V = Q/(b y), Fr = V/sqrt(g y))
  Q = 12 m^3/s
  b = 4 m
  y = 0.5 m
  assumes: Q, b, y > 0
```

**What the worked example means:** This calculation determines the mean velocity and Froude number for a rectangular channel based on discharge, width, and depth. The computed mean velocity is 6 m/s meters per second, while the resulting Froude number is 2.70914. This dimensionless value classifies the flow as supercritical, which helps predict how disturbances propagate through the water.

<a id="hydraulic_jump_rectangular"></a>

#### Hydraulic jump in a rectangular channel: `hydraulic_jump_rectangular`

Sequent (conjugate) depth, energy loss and dissipated power of a hydraulic jump in a horizontal rectangular channel (Belanger's equation from the momentum balance).

$$
\frac{y_2}{y_1} = \tfrac12\left(\sqrt{1+8Fr_1^2}-1\right),\quad \Delta E = \frac{(y_2-y_1)^3}{4y_1y_2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Q` | in | discharge | m^3/s |
| `b` | in | channel width | m |
| `y1` | in | upstream (supercritical) depth | m |
| `y2` | out | sequent depth after the jump (m) | m |
| `Fr1` | out | upstream Froude number | - |
| `Fr2` | out | downstream Froude number | - |
| `dE` | out | head lost in the jump (m) | m |
| `power_loss` | out | power dissipated gamma Q dE (W) | W |
| `ratio` | out | depth ratio y2/y1 | - |

**Applies when:** horizontal, frictionless bed; Fr1 > 1.

```shell
$ civil-engineering solve hydraulic_jump_rectangular "Q=10 m^3/s" "b=2 m" "y1=0.4 m"
y2 = 3.375206273 m
Fr1 = 6.310234717
Fr2 = 0.2574453318
dE = 4.876755035 m
power_loss = 478409.669 W
ratio = 8.438015684
  by: Hydraulic jump in a rectangular channel  (y2 = (y1/2)(sqrt(1 + 8 Fr1^2) - 1), dE = (y2 - y1)^3/(4 y1 y2))
  Q = 10 m^3/s
  b = 2 m
  y1 = 0.4 m
  assumes: horizontal, frictionless bed; Fr1 > 1
```

**What the worked example means:** This calculation determines the hydraulic jump characteristics for a discharge of 10 m^3/s m^3/s in a channel of width 2 m m with an upstream depth of 0.4 m m. The upstream Froude number is computed as 6.31023, confirming supercritical flow, while the downstream Froude number is 0.257445. The sequent depth after the jump is 3.37521 m m, yielding a depth ratio of 8.43802. Consequently, the head lost across the jump is 4.87676 m m, which corresponds to a total power dissipation of 478410 W W, essential information for designing stilling basins to prevent bed erosion.

<a id="gradually_varied_flow_length"></a>

#### Length of a gradually varied flow profile (rectangular channel), `gradually_varied_flow_length`

Distance along a prismatic rectangular channel between two sections of a gradually varied flow profile, by integrating dx/dy = (1 - Fr^2)/(S0 - Sf) exactly (not the direct-step approximation), with the profile type (M1, M2, M3, S1, S2, S3). Positive L: the section at depth y2 lies downstream of the one at depth y1.

$$
\frac{dx}{dy} = \frac{1-Fr^2}{S_0-S_f},\quad S_f = \frac{n^2V^2}{R^{4/3}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Q` | in | discharge | m^3/s |
| `b` | in | channel width | m |
| `S0` | in | bed slope, m/m (positive) | - |
| `n` | in | Manning roughness coefficient | - |
| `y1` | in | depth at the first section | m |
| `y2` | in | depth at the second section | m |
| `L` | out | distance from section 1 to section 2, positive downstream (m) | m |
| `yn` | out | normal depth (m) | m |
| `yc` | out | critical depth (m) | m |
| `profile` | out | profile type (M1 ... S3) | - |

**Applies when:** S0 > 0 (mild or steep slope); prismatic rectangular channel; both depths in one zone, 2% from yn and yc.

```shell
$ civil-engineering solve gradually_varied_flow_length "Q=20 m^3/s" "b=5 m" S0=0.0004 n=0.015 "y1=4 m" "y2=3 m"
L = -4594.586187 m
yn = 2.563823865 m
yc = 1.177109844 m
profile = M1
  by: Length of a gradually varied flow profile (rectangular channel)  (x2 - x1 = integral of (1 - Fr^2)/(S0 - Sf) dy from y1 to y2, Sf from Manning (tanh-sinh quadrature))
  Q = 20 m^3/s
  b = 5 m
  S0 = 0.0004
  n = 0.015
  y1 = 4 m
  y2 = 3 m
  assumes: S0 > 0 (mild or steep slope); prismatic rectangular channel; both depths in one zone, 2% from yn and yc
```

**What the worked example means:** This calculation determines the longitudinal distance along a prismatic rectangular channel between two specified flow depths in a gradually varied flow profile. By integrating the governing differential equation using Manning's friction slope, it finds that the section at depth 3 m lies -4594.59 m downstream from the section at depth 4 m. The result depends on the classification of the profile relative to normal depth 2.56382 m and critical depth 1.17711 m.

<a id="manning_rectangular_discharge"></a>

#### Manning discharge in a rectangular channel, `manning_rectangular_discharge`

Uniform-flow discharge and velocity in a rectangular channel from Manning's equation.

$$
Q = \frac{1}{n}A R^{2/3} S^{1/2},\quad A = by,\ R = \frac{by}{b+2y}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `b` | in | bottom width | m |
| `y` | in | flow depth | m |
| `S` | in | bed slope, m/m | - |
| `n` | in | Manning roughness coefficient | - |
| `Q` | out | discharge (m^3/s) | m^3/s |
| `V` | out | mean velocity (m/s) | m/s |
| `A` | out | flow area (m^2) | m^2 |
| `R` | out | hydraulic radius (m) | m |

**Applies when:** b, y > 0; 0 < S < 1; 0.005 <= n <= 0.2.

```shell
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

**What the worked example means:** This calculation determines the uniform flow discharge and mean velocity in a rectangular channel. Using the bottom width of 3 m, flow depth of 1.2 m, bed slope of 0.001, and Manning roughness coefficient of 0.013, it computes the flow area as 3.6 m^2 and hydraulic radius as 0.666667 m. The resulting discharge of 6.6829 m^3/s indicates how much water passes per second, while the velocity of 1.85636 m/s shows how fast the water moves.

<a id="manning_trapezoidal_discharge"></a>

#### Manning discharge in a trapezoidal channel, `manning_trapezoidal_discharge`

Uniform-flow discharge and velocity in a trapezoidal channel (side slopes z H:1V) from Manning's equation.

$$
Q = \frac{1}{n}A R^{2/3} S^{1/2},\quad A=(b+zy)y,\ P=b+2y\sqrt{1+z^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `b` | in | bottom width | m |
| `y` | in | flow depth | m |
| `z` | in | side slope, horizontal per unit vertical | - |
| `S` | in | bed slope, m/m | - |
| `n` | in | Manning roughness coefficient | - |
| `Q` | out | discharge (m^3/s) | m^3/s |
| `V` | out | mean velocity (m/s) | m/s |
| `A` | out | flow area (m^2) | m^2 |
| `R` | out | hydraulic radius (m) | m |

**Applies when:** y > 0; b >= 0; z >= 0; 0 < S < 1; 0.005 <= n <= 0.2.

```shell
$ civil-engineering solve manning_trapezoidal_discharge "b=4 m" "y=1.5 m" z=2 S=0.0008 n=0.025
Q = 11.72490667 m^3/s
V = 1.116657778 m/s
A = 10.5 m^2
R = 0.9805565963 m
  by: Manning discharge in a trapezoidal channel  (A = (b + z y) y, R = A/(b + 2 y sqrt(1 + z^2)), Q = A R^(2/3) S^(1/2)/n)
  b = 4 m
  y = 1.5 m
  z = 2
  S = 0.0008
  n = 0.025
  assumes: y > 0; b >= 0; z >= 0; 0 < S < 1; 0.005 <= n <= 0.2
```

**What the worked example means:** This calculation determines the uniform-flow discharge and velocity for a trapezoidal channel using Manning's equation. It uses the bottom width, flow depth, side slope, bed slope, and roughness coefficient to compute the geometric properties. The resulting flow area is 10.5 m^2, hydraulic radius is 0.980557 m, and discharge is 11.7249 m^3/s, which represents the volume of water moving through the channel per second.

<a id="manning_partially_full_pipe"></a>

#### Manning flow in a part-full circular pipe: `manning_partially_full_pipe`

Gravity (open-channel) discharge and velocity in a circular sewer or culvert flowing at depth y, from Manning's equation, with the full-bore capacity and the ratio Q/Q_full (n taken as constant with depth).

$$
Q = \frac{1}{n}A R^{2/3} S^{1/2},\quad Q_{full} = \frac{1}{n}\frac{\pi D^2}{4}\left(\frac{D}{4}\right)^{2/3}S^{1/2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `D` | in | pipe diameter | m |
| `y` | in | flow depth | m |
| `S` | in | pipe slope, m/m | - |
| `n` | in | Manning roughness coefficient | - |
| `Q` | out | discharge (m^3/s) | m^3/s |
| `V` | out | mean velocity (m/s) | m/s |
| `A` | out | flow area (m^2) | m^2 |
| `R` | out | hydraulic radius (m) | m |
| `Q_full` | out | full-bore capacity (m^3/s) | m^3/s |
| `ratio` | out | Q/Q_full | - |

**Applies when:** 0 < y <= D; 0 < S < 1; n constant with depth.

```shell
$ civil-engineering solve manning_partially_full_pipe "D=0.6 m" "y=0.3 m" S=0.005 n=0.013
Q = 0.217085863 m^3/s
V = 1.535568363 m/s
A = 0.1413716694 m^2
R = 0.15 m
Q_full = 0.434171726 m^3/s
ratio = 0.5
  by: Manning flow in a part-full circular pipe  (circular-segment geometry, Q = A R^(2/3) S^(1/2)/n; Q_full with R = D/4)
  D = 0.6 m
  y = 0.3 m
  S = 0.005
  n = 0.013
  assumes: 0 < y <= D; 0 < S < 1; n constant with depth
```

**What the worked example means:** This calculation determines the gravity flow capacity in a circular sewer or culvert. Using Manning's equation with slope 0.005 and roughness 0.013, it computes the wetted area 0.141372 m^2 and hydraulic radius 0.15 m. The resulting discharge 0.217086 m^3/s and velocity 1.53557 m/s are compared to the full-bore capacity 0.434172 m^3/s to assess how close the pipe is to surcharging.

<a id="manning_velocity"></a>

#### Manning velocity from hydraulic radius: `manning_velocity`

Mean velocity of uniform flow from Manning's equation (SI) for a hydraulic radius R, slope S and roughness n.

$$
V = \frac{1}{n}R^{2/3}S^{1/2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `R` | in | hydraulic radius | m |
| `S` | in | channel (energy) slope, m/m | - |
| `n` | in | Manning roughness coefficient | - |
| `V` | out | mean velocity (m/s) | m/s |

**Applies when:** R > 0; 0 < S < 1; 0.005 <= n <= 0.2.

```shell
$ civil-engineering solve manning_velocity "R=0.8 m" S=0.0005 n=0.015
V = 1.284656645 m/s
  by: Manning velocity from hydraulic radius  (V = R^(2/3) S^(1/2) / n)
  R = 0.8 m
  S = 0.0005
  n = 0.015
  assumes: R > 0; 0 < S < 1; 0.005 <= n <= 0.2
```

**What the worked example means:** This calculation determines the average flow speed in a channel using Manning's equation. It combines the hydraulic radius 0.8 m, which represents the channel's efficiency in transporting water, with the energy slope 0.0005 that drives the flow. The roughness coefficient 0.015 accounts for frictional resistance along the boundary. The resulting mean velocity 1.28466 m/s indicates how fast water moves through the cross-section under uniform flow conditions.

<a id="manning_from_chezy"></a>

#### Manning's n from a Chezy coefficient: `manning_from_chezy`

The Manning roughness n equivalent to a Chezy coefficient C at hydraulic radius R (SI units).

$$
n = \frac{R^{1/6}}{C}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `C` | in | Chezy coefficient (m^(1/2)/s) | m^0.5/s |
| `R` | in | hydraulic radius | m |
| `n` | out | Manning roughness coefficient | - |

**Applies when:** C, R > 0.

```shell
$ civil-engineering solve manning_from_chezy "C=60 m^0.5/s" "R=1.2 m"
n = 0.01718088868
  by: Manning's n from a Chezy coefficient  (n = R^(1/6)/C)
  C = 60 m^0.5/s
  R = 1.2 m
  assumes: C, R > 0
```

**What the worked example means:** This calculation determines the Manning roughness coefficient, which quantifies resistance to flow in open channels. It uses the formula n equals R raised to the power of one-sixth divided by C. By inputting a Chezy coefficient of 60 m^0.5/s and a hydraulic radius of 1.2 m, the resulting Manning roughness value is 0.0171809. This conversion allows engineers to apply Manning's equation when historical data or specific measurements are provided in terms of the Chezy coefficient.

<a id="normal_depth_rectangular"></a>

#### Normal depth in a rectangular channel: `normal_depth_rectangular`

The uniform-flow (normal) depth that carries a discharge Q in a rectangular channel (Manning), with the velocity and Froude number at that depth.

$$
Q = \frac{1}{n}\,b y\left(\frac{by}{b+2y}\right)^{2/3}S^{1/2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Q` | in | discharge | m^3/s |
| `b` | in | bottom width | m |
| `S` | in | bed slope, m/m | - |
| `n` | in | Manning roughness coefficient | - |
| `yn` | out | normal depth (m) | m |
| `V` | out | velocity at normal depth (m/s) | m/s |
| `Fr` | out | Froude number at normal depth | - |

**Applies when:** Q, b > 0; 0 < S < 1.

```shell
$ civil-engineering solve normal_depth_rectangular "Q=10 m^3/s" "b=4 m" S=0.0005 n=0.015
yn = 1.754416688 m
V = 1.424975046 m/s
Fr = 0.3434838961
  by: Normal depth in a rectangular channel  (bisection on Manning's Q(y) = Q (Q increases with y))
  Q = 10 m^3/s
  b = 4 m
  S = 0.0005
  n = 0.015
  assumes: Q, b > 0; 0 < S < 1
```

**What the worked example means:** This calculation determines the uniform flow depth required to carry the specified discharge. Using Manning's equation, the model solves for the depth where the channel's conveyance matches the input flow rate. For the given bottom width of 4 m, bed slope of 0.0005, and roughness coefficient of 0.015, the resulting normal depth is 1.75442 m. At this depth, the mean velocity is 1.42498 m/s and the Froude number is 0.343484, indicating the flow regime characteristics.

<a id="normal_depth_trapezoidal"></a>

#### Normal depth in a trapezoidal channel: `normal_depth_trapezoidal`

The uniform-flow (normal) depth that carries a discharge Q in a trapezoidal channel (Manning), with the velocity, area and Froude number (on the hydraulic depth A/T).

$$
Q = \frac{1}{n}A R^{2/3}S^{1/2},\quad A=(b+zy)y,\ P=b+2y\sqrt{1+z^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Q` | in | discharge | m^3/s |
| `b` | in | bottom width | m |
| `z` | in | side slope, horizontal per unit vertical | - |
| `S` | in | bed slope, m/m | - |
| `n` | in | Manning roughness coefficient | - |
| `yn` | out | normal depth (m) | m |
| `V` | out | velocity at normal depth (m/s) | m/s |
| `A` | out | flow area (m^2) | m^2 |
| `Fr` | out | Froude number on the hydraulic depth | - |

**Applies when:** Q > 0; b >= 0; z >= 0; 0 < S < 1.

```shell
$ civil-engineering solve normal_depth_trapezoidal "Q=20 m^3/s" "b=5 m" z=2 S=0.0004 n=0.025
yn = 2.16558405 m
V = 0.9897350221 m/s
A = 20.20742881 m^2
Fr = 0.2598312543
  by: Normal depth in a trapezoidal channel  (bisection on Manning's Q(y) = Q)
  Q = 20 m^3/s
  b = 5 m
  z = 2
  S = 0.0004
  n = 0.025
  assumes: Q > 0; b >= 0; z >= 0; 0 < S < 1
```

**What the worked example means:** This calculation determines the uniform-flow water depth in a trapezoidal channel required to carry the discharge of 20 m^3/s m^3/s. Using Manning's equation with a roughness coefficient of 0.025, bottom width of 5 m m, side slope of 2, and bed slope of 0.0004, the method iterates to find the depth where the computed flow matches the target discharge. The resulting normal depth is 2.16558 m m, which produces a cross-sectional flow area of 20.2074 m^2 m^2. At this depth, the mean velocity is 0.989735 m/s m/s, and the Froude number based on hydraulic depth is 0.259831, indicating the flow regime under these specific geometric and frictional conditions.

<a id="circular_channel_section"></a>

#### Part-full circular conduit: area, wetted perimeter, hydraulic radius, `circular_channel_section`

Flow area, wetted perimeter, hydraulic radius, top width and hydraulic depth of a circular pipe or culvert flowing part full at depth y, with the central angle of the wetted arc.

$$
\theta = 2\cos^{-1}\!\left(1-\frac{2y}{D}\right),\quad A = \frac{D^2}{8}(\theta-\sin\theta),\quad P = \frac{D\theta}{2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `D` | in | pipe diameter | m |
| `y` | in | flow depth | m |
| `A` | out | flow area (m^2) | m^2 |
| `P` | out | wetted perimeter (m) | m |
| `R` | out | hydraulic radius A/P (m) | m |
| `T` | out | top width (m) | m |
| `Dh` | out | hydraulic depth A/T (m) | m |
| `theta` | out | central angle of the wetted arc (degrees) | - |

**Applies when:** 0 < y < D.

```shell
$ civil-engineering solve circular_channel_section "D=1 m" "y=0.6 m"
A = 0.4920283568 m^2
P = 1.772154248 m
R = 0.2776442047 m
T = 0.9797958971 m
Dh = 0.5021743388 m
theta = 203.0739181
  by: Part-full circular conduit: area, wetted perimeter, hydraulic radius  (theta = 2 acos(1 - 2y/D), A = D^2 (theta - sin theta)/8, P = D theta/2, T = D sin(theta/2))
  D = 1 m
  y = 0.6 m
  assumes: 0 < y < D
```

**What the worked example means:** This calculation determines the hydraulic properties of a circular conduit flowing part-full. Given a diameter of 1 m and a flow depth of 0.6 m, it computes the central angle of the wetted arc as 203.074 degrees. This geometry results in a flow area of 0.492028 m^2, a wetted perimeter of 1.77215 m, and a hydraulic radius of 0.277644 m. The top width of the water surface is found to be 0.979796 m, which gives a hydraulic depth of 0.502174 m. These values are essential for analyzing open channel flow capacity and velocity in pipes.

<a id="rectangular_channel_section"></a>

#### Rectangular channel: area, wetted perimeter, hydraulic radius, `rectangular_channel_section`

Flow area, wetted perimeter, hydraulic radius, top width and hydraulic depth of a rectangular channel flowing at depth y.

$$
A = b\,y,\quad P = b + 2y,\quad R = \frac{A}{P}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `b` | in | bottom width | m |
| `y` | in | flow depth | m |
| `A` | out | flow area (m^2) | m^2 |
| `P` | out | wetted perimeter (m) | m |
| `R` | out | hydraulic radius A/P (m) | m |
| `T` | out | top width (m) | m |
| `Dh` | out | hydraulic depth A/T (m) | m |

**Applies when:** b > 0; y > 0.

```shell
$ civil-engineering solve rectangular_channel_section "b=3 m" "y=1.2 m"
A = 3.6 m^2
P = 5.4 m
R = 0.6666666667 m
T = 3 m
Dh = 1.2 m
  by: Rectangular channel: area, wetted perimeter, hydraulic radius  (A = b y, P = b + 2y, R = A/P, T = b, D = A/T)
  b = 3 m
  y = 1.2 m
  assumes: b > 0; y > 0
```

**What the worked example means:** This calculation determines the geometric properties of water flowing in a rectangular channel. Using a bottom width of 3 m and a flow depth of 1.2 m, it computes the cross-sectional flow area as 3.6 m^2, which represents the total space occupied by the water. The wetted perimeter, calculated as 5.4 m, measures the length of the channel boundary in contact with the water. From these values, the hydraulic radius is derived as 0.666667 m, indicating the efficiency of the channel shape for conveying flow. Additionally, the top width is found to be 3 m, equal to the bottom width, and the hydraulic depth is 1.2 m, which is used in Froude number calculations.

<a id="rectangular_weir_discharge"></a>

#### Sharp-crested rectangular weir discharge: `rectangular_weir_discharge`

Discharge over a sharp-crested rectangular weir for a stated discharge coefficient Cd and head H, with Francis' allowance of 0.1 H per end contraction (0, 1 or 2 contractions).

$$
Q = \tfrac{2}{3}\,C_d\,L_{e}\sqrt{2g}\,H^{3/2},\quad L_e = b - 0.1\,nH
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Cd` | in | discharge coefficient (stated, e.g. 0.62) | - |
| `b` | in | crest length | m |
| `H` | in | head over the crest | m |
| `contractions` | in | number of end contractions (0, 1 or 2; default 0) (optional) | - |
| `Q` | out | discharge (m^3/s) | m^3/s |
| `L_eff` | out | effective crest length (m) | m |

**Applies when:** Cd stated; free (unsubmerged) nappe; b - 0.1 n H > 0.

```shell
$ civil-engineering solve rectangular_weir_discharge Cd=0.62 "b=2 m" "H=0.35 m"
Q = 0.7581968821 m^3/s
L_eff = 2 m
  by: Sharp-crested rectangular weir discharge  (Q = (2/3) Cd L_eff sqrt(2g) H^(3/2), L_eff = b - 0.1 n H)
  Cd = 0.62
  b = 2 m
  H = 0.35 m
  assumes: Cd stated; free (unsubmerged) nappe; b - 0.1 n H > 0
```

**What the worked example means:** This calculation determines the flow rate over a sharp-crested rectangular weir using the standard Francis formula. It adjusts the physical crest length based on the number of end contractions to find the effective width, then applies the discharge coefficient and head to compute the total volume of water passing per second. The resulting discharge of 0.758197 m^3/s reflects the capacity under free-flow conditions, while the effective length of 2 m accounts for the narrowing caused by side contractions.

<a id="alternate_depth_rectangular"></a>

#### Specific energy and alternate depth in a rectangular channel, `alternate_depth_rectangular`

The specific energy of flow at depth y in a rectangular channel and the alternate depth with the same specific energy on the other side of critical depth.

$$
E = y + \frac{q^2}{2 g y^2},\quad y_2^3 - E\,y_2^2 + \frac{q^2}{2g} = 0
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Q` | in | discharge | m^3/s |
| `b` | in | channel width | m |
| `y` | in | flow depth | m |
| `E` | out | specific energy y + V^2/2g (m) | m |
| `y_alt` | out | alternate depth (m) | m |
| `yc` | out | critical depth (m) | m |
| `Fr` | out | Froude number at depth y | - |
| `regime` | out | flow regime at depth y | - |

**Applies when:** Q, b, y > 0; y is not the critical depth.

```shell
$ civil-engineering solve alternate_depth_rectangular "Q=6 m^3/s" "b=2 m" "y=0.4 m"
E = 3.266972477 m
y_alt = 3.222807838 m
yc = 0.9716827674 m
Fr = 3.78614083
regime = supercritical
  by: Specific energy and alternate depth in a rectangular channel  (E = y + q^2/(2 g y^2); the other positive root of y^3 - E y^2 + q^2/(2g) = 0 by bisection)
  Q = 6 m^3/s
  b = 2 m
  y = 0.4 m
  assumes: Q, b, y > 0; y is not the critical depth
```

**What the worked example means:** This calculation determines the specific energy of flow in a rectangular channel using the given discharge, width, and depth. The result shows that the total head relative to the bed is 3.26697 m, which combines potential and kinetic energy components. It also identifies the alternate depth 3.22281 m, representing the other possible flow condition sharing this same energy but lying on the opposite side of critical depth. The Froude number 3.78614 indicates whether the current regime is subcritical or supercritical.

<a id="trapezoidal_channel_section"></a>

#### Trapezoidal channel: area, wetted perimeter, hydraulic radius, `trapezoidal_channel_section`

Flow area, wetted perimeter, hydraulic radius, top width and hydraulic depth of a trapezoidal channel with bottom width b and side slopes z (horizontal : vertical); b = 0 is a triangular channel.

$$
A = (b + z y)\,y,\quad P = b + 2y\sqrt{1+z^2},\quad T = b + 2zy
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `b` | in | bottom width | m |
| `y` | in | flow depth | m |
| `z` | in | side slope, horizontal per unit vertical | - |
| `A` | out | flow area (m^2) | m^2 |
| `P` | out | wetted perimeter (m) | m |
| `R` | out | hydraulic radius A/P (m) | m |
| `T` | out | top width (m) | m |
| `Dh` | out | hydraulic depth A/T (m) | m |

**Applies when:** y > 0; b >= 0; z >= 0.

```shell
$ civil-engineering solve trapezoidal_channel_section "b=4 m" "y=1.5 m" z=2
A = 10.5 m^2
P = 10.70820393 m
R = 0.9805565963 m
T = 10 m
Dh = 1.05 m
  by: Trapezoidal channel: area, wetted perimeter, hydraulic radius  (A = (b + z y) y, P = b + 2 y sqrt(1 + z^2), T = b + 2 z y)
  b = 4 m
  y = 1.5 m
  z = 2
  assumes: y > 0; b >= 0; z >= 0
```

**What the worked example means:** This calculation determines the geometric properties of flow in a trapezoidal channel using the bottom width 4 m, flow depth 1.5 m, and side slope 2. The resulting flow area 10.5 m^2 represents the cross-sectional area available for water, while the wetted perimeter 10.7082 m indicates the length of the channel boundary in contact with the fluid. The hydraulic radius 0.980557 m, defined as the ratio of area to perimeter, characterizes the efficiency of the channel section for conveying flow.

<a id="v_notch_weir_discharge"></a>

#### V-notch (triangular) weir discharge: `v_notch_weir_discharge`

Discharge over a sharp-crested triangular (V-notch) weir of notch angle theta for a stated discharge coefficient.

$$
Q = \tfrac{8}{15}\,C_d\sqrt{2g}\,\tan\frac{\theta}{2}\,H^{5/2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Cd` | in | discharge coefficient (stated, e.g. 0.58) | - |
| `theta` | in | notch angle (degrees) | - |
| `H` | in | head above the vertex | m |
| `Q` | out | discharge (m^3/s) | m^3/s |

**Applies when:** Cd stated; 0 < theta < 180 degrees; free nappe.

```shell
$ civil-engineering solve v_notch_weir_discharge Cd=0.58 theta=90 "H=0.25 m"
Q = 0.04281798687 m^3/s
  by: V-notch (triangular) weir discharge  (Q = (8/15) Cd sqrt(2g) tan(theta/2) H^(5/2))
  Cd = 0.58
  theta = 90
  H = 0.25 m
  assumes: Cd stated; 0 < theta < 180 degrees; free nappe
```

**What the worked example means:** This calculation determines the flow rate passing over a sharp-crested triangular weir using the standard discharge equation. It takes the stated discharge coefficient, the notch angle, and the vertical head above the vertex as inputs to produce the volumetric discharge. The resulting value of 0.042818 m^3/s represents the total water volume moving through the opening per second under free-flow conditions.

### Pipe flow and water networks (5)

<a id="parallel_pipes_hazen_williams"></a>

#### Flow division among parallel pipes (Hazen-Williams), `parallel_pipes_hazen_williams`

How a total flow Q divides among two or more pipes in parallel between the same two junctions, and the common head loss, from the Hazen-Williams formula.

$$
Q = \sum_i \left(\frac{h_f}{k_i}\right)^{0.54},\quad k_i = \frac{10.67L_i}{C_i^{1.852}D_i^{4.871}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Q` | in | total flow | m^3/s |
| `L` | in | pipe lengths in m (one per pipe) | - |
| `D` | in | pipe diameters in mm (one per pipe) | - |
| `C` | in | Hazen-Williams C of each pipe | - |
| `hf` | out | head loss between the junctions (m) | m |
| `flows` | out | flow in each pipe (m^3/s) | m^3/s |

**Applies when:** the same head loss across every pipe; 40 <= C <= 160; lists of equal length.

```shell
$ civil-engineering solve parallel_pipes_hazen_williams "Q=0.3 m^3/s" "L=[800, 1200]" "D=[300, 400]" "C=[120, 110]"
hf = 7.944067369 m
flows = [0.116758581689,0.183241418311] m^3/s
  by: Flow division among parallel pipes (Hazen-Williams)  (hf = (Q / sum k_i^(-1/1.852))^1.852 with k_i = 10.6668 L_i/(C_i^1.852 D_i^4.871); Q_i = (hf/k_i)^(1/1.852))
  Q = 0.3 m^3/s
  assumes: the same head loss across every pipe; 40 <= C <= 160; lists of equal length
```

**What the worked example means:** This calculation determines how the total flow 0.3 m^3/s splits between multiple pipes connected in parallel. It computes the shared head loss 7.94407 m across all branches and the individual flow rate for each pipe using the Hazen-Williams equation. The result shows that higher resistance in a specific branch leads to lower flow through that path, while the pressure drop remains identical for every parallel route.

<a id="hazen_williams_head_loss"></a>

#### Hazen-Williams head loss in a pipe: `hazen_williams_head_loss`

Friction head loss, velocity and friction slope for water flowing at Q through a pipe, from the Hazen-Williams formula (SI form used by EPANET).

$$
h_f = \frac{10.67\,L\,Q^{1.852}}{C^{1.852}D^{4.871}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Q` | in | discharge | m^3/s |
| `D` | in | pipe diameter | m |
| `L` | in | pipe length | m |
| `C` | in | Hazen-Williams roughness coefficient | - |
| `hf` | out | friction head loss (m) | m |
| `V` | out | mean velocity (m/s) | m/s |
| `Sf` | out | friction slope hf/L (m/m) | - |

**Applies when:** water near 15 C, turbulent flow; 40 <= C <= 160.

```shell
$ civil-engineering solve hazen_williams_head_loss "Q=0.05 m^3/s" "D=0.3 m" "L=1000 m" C=120
hf = 2.064555214 m
V = 0.7073553026 m/s
Sf = 0.002064555214
  by: Hazen-Williams head loss in a pipe  (hf = 10.6668 L Q^1.852/(C^1.852 D^4.871))
  Q = 0.05 m^3/s
  D = 0.3 m
  L = 1000 m
  C = 120
  assumes: water near 15 C, turbulent flow; 40 <= C <= 160
```

**What the worked example means:** This calculation determines the friction head loss in a pipe using the Hazen-Williams formula. Given a discharge of 0.05 m^3/s flowing through a pipe with diameter 0.3 m and length 1000 m, and assuming a roughness coefficient of 120, the resulting head loss is 2.06456 m. The mean velocity of the water is calculated as 0.707355 m/s, and the friction slope, which represents the head loss per unit length of the pipe, is 0.00206456. These values help engineers assess pressure drops and pump requirements in water distribution systems.

<a id="hazen_williams_flow"></a>

#### Pipe flow from a head loss (Hazen-Williams), `hazen_williams_flow`

The discharge a pipe carries under a given friction head loss (for example between two reservoirs), from the Hazen-Williams formula.

$$
Q = \left(\frac{h_f\,C^{1.852}D^{4.871}}{10.67\,L}\right)^{0.54}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `hf` | in | friction head loss | m |
| `D` | in | pipe diameter | m |
| `L` | in | pipe length | m |
| `C` | in | Hazen-Williams roughness coefficient | - |
| `Q` | out | discharge (m^3/s) | m^3/s |
| `V` | out | mean velocity (m/s) | m/s |

**Applies when:** 40 <= C <= 160; hf > 0.

```shell
$ civil-engineering solve hazen_williams_flow "hf=5 m" "D=0.3 m" "L=1500 m" C=110
Q = 0.05936365982 m^3/s
V = 0.8398239911 m/s
  by: Pipe flow from a head loss (Hazen-Williams)  (Q = (hf C^1.852 D^4.871/(10.6668 L))^(1/1.852))
  hf = 5 m
  D = 0.3 m
  L = 1500 m
  C = 110
  assumes: 40 <= C <= 160; hf > 0
```

**What the worked example means:** This calculation determines the flow rate and average speed of water moving through a pipe when the energy lost to friction is known. Using the Hazen-Williams equation, which relates head loss to pipe dimensions and roughness, the model computes that the discharge equals 0.0593637 m^3/s and the mean velocity equals 0.839824 m/s. These values indicate how much water passes through the pipe per second and how fast it moves, based on the specific diameter, length, and material condition provided.

<a id="hardy_cross_network"></a>

#### Pipe network flows by the Hardy Cross method (Hazen-Williams), `hardy_cross_network`

Steady flows in every pipe of a looped water network and the head loss from the source to every node, by Hardy Cross loop corrections (finished with simultaneous loop corrections to full precision), Hazen-Williams friction. Nodes are numbered 1..N; node 1 is the source and supplies the total demand.

$$
\Delta Q = -\frac{\sum k\,Q|Q|^{0.852}}{1.852\sum k|Q|^{0.852}},\quad k = \frac{10.67L}{C^{1.852}D^{4.871}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `pipes` | in | pipes as node pairs 1-2, 2-3, ... (the flow sign follows this direction) | - |
| `L` | in | pipe lengths in m (in pipe order) | - |
| `D` | in | pipe diameters in mm (in pipe order) | - |
| `C` | in | Hazen-Williams C of each pipe (in pipe order) | - |
| `demands` | in | demand at nodes 1..N in m^3/s (node 1, the source, takes 0) | - |
| `flows` | out | flow in each pipe, positive from its first to its second node (m^3/s) | m^3/s |
| `head_loss` | out | head loss from node 1 to each node (m) | m |
| `loops` | out | independent loops | - |
| `iterations` | out | Hardy Cross sweeps before the Newton finish | - |

**Applies when:** one source (node 1); connected network; 40 <= C <= 160; demands >= 0.

```shell
$ civil-engineering solve hardy_cross_network "pipes=1-2, 2-3, 3-4, 4-1, 2-4" "L=[600, 500, 600, 500, 780]" "D=[300, 250, 200, 250, 200]" "C=[120, 120, 120, 120, 120]" "demands=[0, 0.04, 0.05, 0.03]"
flows = [0.0714399845267,0.0331343839504,-0.0168656160496,-0.0485600154733,-0.00169439942369] m^3/s
head_loss = [0.0,2.39874834362,3.56973630784,2.37675423164] m
loops = 2
iterations = 14
  by: Pipe network flows by the Hardy Cross method (Hazen-Williams)  (loop corrections dQ = -sum(k Q|Q|^0.852)/(1.852 sum(k|Q|^0.852)) until converged, then Newton on all loops)
  assumes: one source (node 1); connected network; 40 <= C <= 160; demands >= 0
```

**What the worked example means:** This calculation determines the flow distribution and pressure drops throughout a looped water network. By applying the Hardy Cross method with Hazen-Williams friction, the model iteratively adjusts initial flow guesses to satisfy both mass conservation at nodes and energy conservation around loops. The resulting pipe flows are [0.07144, 0.0331344, -0.0168656, -0.04856, -0.0016944] m^3/s, where positive values indicate movement from the first to the second node of each pair. The cumulative head loss from the source node to every other node is [0, 2.39875, 3.56974, 2.37675] m. These results rely on the input parameters including pipe lengths [600, 500, 600, 500, 780], diameters [300, 250, 200, 250, 200], roughness coefficients [120, 120, 120, 120, 120], and nodal demands [0, 0.04, 0.05, 0.03]. The solution converged after 14 iterations across 2 independent loops.

<a id="three_reservoir_problem"></a>

#### Three-reservoir problem (Hazen-Williams): `three_reservoir_problem`

Head at the junction of three pipes from three reservoirs and the flow in each pipe (positive toward the junction), with Hazen-Williams friction and no demand at the junction.

$$
\sum_{i=1}^{3}\operatorname{sgn}(z_i-h)\left(\frac{|z_i-h|}{k_i}\right)^{0.54} = 0
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `z` | in | reservoir water levels in m (three) | - |
| `L` | in | pipe lengths in m (three) | - |
| `D` | in | pipe diameters in mm (three) | - |
| `C` | in | Hazen-Williams C of each pipe (three) | - |
| `h_junction` | out | piezometric head at the junction (m) | m |
| `flows` | out | flow in each pipe toward the junction (m^3/s) | m^3/s |

**Applies when:** no demand at the junction; 40 <= C <= 160.

```shell
$ civil-engineering solve three_reservoir_problem "z=[120, 90, 40]" "L=[2000, 1500, 2500]" "D=[400, 300, 350]" "C=[120, 120, 120]"
h_junction = 91.94679 m
flows = [0.299838366058,-0.0389148570849,-0.260923508973] m^3/s
  by: Three-reservoir problem (Hazen-Williams)  (bisection on the junction head h: sum of (z_i - h)/|z_i - h| (|z_i - h|/k_i)^(1/1.852) = 0)
  assumes: no demand at the junction; 40 <= C <= 160
```

**What the worked example means:** This calculation determines the piezometric head at the central junction of three pipes connecting to reservoirs. Using the Hazen-Williams equation, the method balances the sum of signed flow rates to zero, ensuring conservation of mass with no external demand at the junction. The result shows that the equilibrium head is 91.9468 m, which dictates the flow distribution in each pipe, represented as [0.299838, -0.0389149, -0.260924] m^3/s. Positive values indicate flow entering the junction from a reservoir.

### Soil mechanics (20)

<a id="degree_of_consolidation"></a>

#### Average degree of consolidation after a time (Terzaghi), `degree_of_consolidation`

Time factor and average degree of one-dimensional consolidation of a clay layer after time t (Terzaghi, uniform initial excess pore pressure), and the settlement reached if the final consolidation settlement is given. Hdr is the drainage path: half the thickness for double drainage, the full thickness for single drainage.

$$
U = 1 - \sum_{m=0}^{\infty}\frac{2}{M^2}e^{-M^2T_v},\quad M = \frac{\pi}{2}(2m+1),\quad T_v = \frac{c_v t}{H_{dr}^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `cv` | in | coefficient of consolidation | m^2/s |
| `t` | in | time since loading | s |
| `Hdr` | in | drainage path length | m |
| `Sc` | in | final consolidation settlement (optional) (optional) | m |
| `Tv` | out | time factor cv t/Hdr^2 | - |
| `U` | out | average degree of consolidation | - |
| `U_percent` | out | average degree of consolidation (%) | - |
| `St` | out | settlement at time t (m) | m |

**Applies when:** 1e-4 <= Tv <= 5; uniform initial excess pore pressure; Terzaghi 1-D theory.

```shell
$ civil-engineering solve degree_of_consolidation "cv=7.922021954e-08 m^2/s" "t=31557600 s" "Hdr=3 m"
Tv = 0.2777777778
U = 0.5913702173
U_percent = 59.13702173
  by: Average degree of consolidation after a time (Terzaghi)  (Tv = cv t/Hdr^2; U = 1 - sum 2/M^2 exp(-M^2 Tv), M = pi(2m + 1)/2)
  cv = 7.922021954e-08 m^2/s
  t = 31557600 s
  Hdr = 3 m
  assumes: 1e-4 <= Tv <= 5; uniform initial excess pore pressure; Terzaghi 1-D theory
```

In the worked example, cv = 7.922021954e-08 m^2/s, t = 31557600 s, Hdr = 3 m go through the equation above and give Tv = 0.2777777778, U = 0.5913702173, U_percent = 59.13702173. It applies only when 1e-4 <= Tv <= 5; uniform initial excess pore pressure; Terzaghi 1-D theory: outside that it says what is wrong instead of returning a number.

<a id="coefficient_of_consolidation"></a>

#### Coefficient of consolidation from a consolidation time, `coefficient_of_consolidation`

Coefficient of consolidation from the time at which a degree of consolidation is reached (for example t50 by Casagrande's method or t90 by Taylor's), using the exact time factor rather than the rounded 0.197 or 0.848.

$$
c_v = \frac{T_v(U)\,H_{dr}^2}{t}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Hdr` | in | drainage path length (half the specimen height for double drainage) | m |
| `t` | in | time to reach U | s |
| `U` | in | degree of consolidation reached (fraction, or % in a question) | - |
| `cv` | out | coefficient of consolidation (m^2/s) | m^2/s |
| `Tv` | out | time factor for U | - |
| `cv_m2_yr` | out | coefficient of consolidation (m^2/yr) | - |

**Applies when:** 1% <= U <= 99%; Terzaghi 1-D theory.

```shell
$ civil-engineering solve coefficient_of_consolidation "Hdr=0.01 m" "t=750 s" U=0.5
cv = 2.623076527e-08 m^2/s
Tv = 0.1967307395
cv_m2_yr = 0.8277799981
  by: Coefficient of consolidation from a consolidation time  (Tv(U) exactly (bisection), cv = Tv Hdr^2/t)
  Hdr = 0.01 m
  t = 750 s
  U = 0.5
  assumes: 1% <= U <= 99%; Terzaghi 1-D theory
```

**What the worked example means:** This calculation determines the coefficient of consolidation, which represents the rate at which soil expels water under load. Using the exact time factor corresponding to the degree of consolidation rather than rounded approximations ensures precision. The result, 2.62308e-08 m^2/s, indicates how quickly settlement progresses given the measured time 750 s and drainage distance 0.01 m. This value is essential for predicting long-term ground movement in engineering projects.

<a id="direct_shear_strength_parameters"></a>

#### Cohesion and friction angle from direct shear tests, `direct_shear_strength_parameters`

Mohr-Coulomb cohesion and friction angle from direct shear tests by a least-squares line of peak shear stress against normal stress. Stresses in kPa.

$$
\tau_f = c + \sigma\tan\phi
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `sigma` | in | normal stresses in kPa | - |
| `tau` | in | shear stresses at failure in kPa | - |
| `phi` | out | friction angle (degrees) | - |
| `c` | out | cohesion intercept (kPa) | - |
| `tan_phi` | out | slope tan(phi) | - |

**Applies when:** at least two tests; shear strength rises with normal stress.

```shell
$ civil-engineering solve direct_shear_strength_parameters "sigma=[50, 100, 200]" "tau=[45, 73, 131]"
phi = 29.8681357
c = 16
tan_phi = 0.5742857143
  by: Cohesion and friction angle from direct shear tests  (least squares tau = c + sigma tan phi)
  assumes: at least two tests; shear strength rises with normal stress
```

**What the worked example means:** This calculation determines the Mohr-Coulomb strength parameters by fitting a straight line to direct shear test data. The slope of this line yields the tangent of the friction angle, which corresponds to an angle of 29.8681 degrees, while the intercept with the vertical axis provides the cohesion value of 16 kPa. These values characterize the soil's resistance to shearing under various normal loads.

<a id="triaxial_strength_parameters"></a>

#### Cohesion and friction angle from triaxial tests, `triaxial_strength_parameters`

Mohr-Coulomb cohesion and friction angle from two or more triaxial tests at failure, by a least-squares line through the stress points (p, q) = ((s1 + s3)/2, (s1 - s3)/2): sin phi = slope, c = intercept/cos phi. Stresses in kPa (effective or total, consistently).

$$
\sin\phi = \tan\alpha,\quad c = \frac{a}{\cos\phi}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `s3` | in | confining (minor principal) stresses at failure in kPa | - |
| `s1` | in | major principal stresses at failure in kPa | - |
| `phi` | out | friction angle (degrees) | - |
| `c` | out | cohesion (kPa) | - |
| `alpha` | out | slope angle of the Kf line (degrees) | - |

**Applies when:** at least two tests; s1 > s3; a non-negative intercept.

```shell
$ civil-engineering solve triaxial_strength_parameters "s3=[100, 200, 300]" "s1=[330, 590, 860]"
phi = 26.87779551
c = 19.43781638
alpha = 24.32720568
  by: Cohesion and friction angle from triaxial tests  (least squares q = a + p tan(alpha); phi = asin(tan alpha), c = a/cos phi)
  assumes: at least two tests; s1 > s3; a non-negative intercept
```

**What the worked example means:** This calculation determines the Mohr-Coulomb strength parameters from triaxial test data. By performing a least-squares regression on the transformed stress points, it finds the slope angle of the failure envelope in p-q space. The resulting friction angle is 26.8778 and the cohesion intercept is 19.4378, representing the soil's shear strength characteristics under the tested conditions.

<a id="thick_clay_layer_settlement"></a>

#### Consolidation settlement of a thick clay layer integrated over depth, `thick_clay_layer_settlement`

Primary consolidation settlement of a thick normally consolidated clay layer under a uniform added stress, with the initial effective stress rising linearly with depth (integrated exactly over the thickness), and the usual single-layer estimate at mid-depth for comparison.

$$
S = \frac{C_c}{1+e_0}\int_0^H \log_{10}\frac{\sigma'_0(z)+\Delta\sigma}{\sigma'_0(z)}\,dz
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `H` | in | clay layer thickness | m |
| `s_top` | in | initial effective vertical stress at the top of the clay | Pa |
| `gsub` | in | submerged unit weight of the clay | N/m^3 |
| `ds` | in | uniform added vertical stress | Pa |
| `Cc` | in | compression index | - |
| `e0` | in | initial void ratio | - |
| `S` | out | settlement integrated over depth (m) | m |
| `S_one_layer` | out | single-layer estimate at mid-depth (m) | m |

**Applies when:** normally consolidated clay; uniform added stress with depth; s0' = s_top + gsub z.

```shell
$ civil-engineering solve thick_clay_layer_settlement "H=10 m" "s_top=40000 Pa" "gsub=8000 N/m^3" "ds=100000 Pa" Cc=0.45 e0=1.2
S = 0.7531290745 m
S_one_layer = 0.7203733325 m
  by: Consolidation settlement of a thick clay layer integrated over depth  (S = Cc/(1+e0) integral_0^H log10((s0'(z) + ds)/s0'(z)) dz in closed form)
  H = 10 m
  s_top = 40000 Pa
  gsub = 8000 N/m^3
  ds = 100000 Pa
  Cc = 0.45
  e0 = 1.2
  assumes: normally consolidated clay; uniform added stress with depth; s0' = s_top + gsub z
```

**What the worked example means:** This calculation determines the primary consolidation settlement for a thick, normally consolidated clay layer by integrating the strain over its entire depth. The initial effective stress increases linearly from 40000 Pa at the top due to the submerged unit weight of 8000 N/m^3. Under a uniform added stress of 100000 Pa, the total settlement computed through exact integration is 0.753129 m. For comparison, the simplified method using mid-depth properties yields an estimate of 0.720373 m, highlighting the difference between precise integration and approximate methods in deep deposits.

<a id="grain_size_coefficients"></a>

#### Effective sizes and gradation coefficients from a sieve analysis, `grain_size_coefficients`

D10, D30 and D60 by log-linear interpolation of the grading curve, and the coefficients of uniformity Cu and curvature Cc. Sieve sizes in mm, percent passing in %.

$$
C_u = \frac{D_{60}}{D_{10}},\quad C_c = \frac{D_{30}^2}{D_{10}D_{60}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `sizes` | in | sieve sizes in mm | - |
| `passing` | in | percent passing each sieve, in % | - |
| `D10` | out | D10 (mm) | - |
| `D30` | out | D30 (mm) | - |
| `D60` | out | D60 (mm) | - |
| `Cu` | out | coefficient of uniformity D60/D10 | - |
| `Cc` | out | coefficient of curvature D30^2/(D10 D60) | - |

**Applies when:** percent passing rises strictly with size; the curve spans 10% to 60%.

```shell
$ civil-engineering solve grain_size_coefficients "sizes=[4.75, 2, 0.85, 0.425, 0.25, 0.15, 0.075]" "passing=[95, 82, 64, 43, 27, 14, 6]"
D10 = 0.1060660172
D30 = 0.2761526286
D60 = 0.7448689636
Cu = 7.022691937
Cc = 0.9652553618
  by: Effective sizes and gradation coefficients from a sieve analysis  (log-linear interpolation of size against percent passing)
  assumes: percent passing rises strictly with size; the curve spans 10% to 60%
```

**What the worked example means:** This calculation determines the particle diameters at which ten, thirty and sixty percent of the soil passes, denoted as 0.106066, 0.276153 and 0.744869. These values are derived from the sieve analysis data using log-linear interpolation. The resulting coefficient of uniformity 7.02269 indicates the spread of particle sizes, while the coefficient of curvature 0.965255 describes the shape of the gradation curve.

<a id="excess_pore_pressure_isochrone"></a>

#### Excess pore pressure at a depth during consolidation, `excess_pore_pressure_isochrone`

Excess pore-water pressure and local degree of consolidation at depth z and time t in a doubly drained clay layer of thickness 2 Hdr under a uniform initial excess pressure u0 (Terzaghi); z is measured from the top drainage face (for a singly drained layer of thickness Hdr, use 0 <= z <= Hdr).

$$
u(z,t) = \sum_{m=0}^{\infty}\frac{2u_0}{M}\sin\frac{Mz}{H_{dr}}\,e^{-M^2T_v}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `u0` | in | initial excess pore pressure | Pa |
| `cv` | in | coefficient of consolidation | m^2/s |
| `t` | in | time since loading | s |
| `Hdr` | in | drainage path length | m |
| `z` | in | depth below the top drainage face | m |
| `u` | out | excess pore pressure (Pa) | Pa |
| `Uz` | out | local degree of consolidation 1 - u/u0 | - |
| `Tv` | out | time factor | - |

**Applies when:** 0.001 <= Tv <= 3; uniform initial excess pore pressure; double drainage (or symmetric half).

```shell
$ civil-engineering solve excess_pore_pressure_isochrone "u0=100000 Pa" "cv=6.337617563e-08 m^2/s" "t=31557600 s" "Hdr=3 m" "z=1.5 m"
u = 52247.33366 Pa
Uz = 0.4775266634
Tv = 0.2222222222
  by: Excess pore pressure at a depth during consolidation  (u = sum 2u0/M sin(M z/Hdr) exp(-M^2 Tv), M = pi(2m + 1)/2)
  u0 = 100000 Pa
  cv = 6.337617563e-08 m^2/s
  t = 31557600 s
  Hdr = 3 m
  z = 1.5 m
  assumes: 0.001 <= Tv <= 3; uniform initial excess pore pressure; double drainage (or symmetric half)
```

**What the worked example means:** This calculation determines the remaining excess pore water pressure at a specific depth within a clay layer after a given time. Using the initial pressure, drainage path length, and coefficient of consolidation, it computes the time factor as 0.222222. The resulting pressure of 52247.3 Pa reflects how much of the original load is still carried by the water rather than the soil skeleton, corresponding to a local consolidation ratio of 0.477527.

<a id="infinite_slope_factor_of_safety"></a>

#### Factor of safety of an infinite slope: `infinite_slope_factor_of_safety`

The factor of safety against translational sliding of an infinite slope on a plane at depth z, with seepage parallel to the slope and the water table at a height m z above the slip plane (m = 0 dry, m = 1 at the surface).

$$
FS = \frac{c' + (\gamma - m\gamma_w)\,z\cos^2\beta\,\tan\phi'}{\gamma z\sin\beta\cos\beta}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `c` | in | effective cohesion | Pa |
| `phi` | in | effective friction angle (degrees) | - |
| `gam` | in | unit weight of the soil (saturated below the water table) | N/m^3 |
| `z` | in | depth of the slip plane (vertical) | m |
| `beta` | in | slope angle (degrees) | - |
| `m` | in | water-table height above the slip plane as a fraction of z (default 0, dry) (optional) | - |
| `FS` | out | factor of safety | - |
| `tau` | out | driving shear stress on the slip plane (Pa) | Pa |
| `sigma_eff` | out | effective normal stress on the slip plane (Pa) | Pa |

**Applies when:** c >= 0; 0 <= phi < 60 degrees; 1 <= beta <= 80 degrees; 0 <= m <= 1; seepage parallel to the slope.

```shell
$ civil-engineering solve infinite_slope_factor_of_safety "c=5000 Pa" phi=30 "gam=18000 N/m^3" "z=3 m" beta=25
FS = 1.479873738
tau = 20683.19996 Pa
sigma_eff = 44355.26546 Pa
  by: Factor of safety of an infinite slope  (FS = [c' + (gamma - m gamma_w) z cos^2(beta) tan(phi')]/(gamma z sin(beta) cos(beta)))
  c = 5000 Pa
  phi = 30
  gam = 18000 N/m^3
  z = 3 m
  beta = 25
  assumes: c >= 0; 0 <= phi < 60 degrees; 1 <= beta <= 80 degrees; 0 <= m <= 1; seepage parallel to the slope
```

**What the worked example means:** The factor of safety for the infinite slope is 1.47987, indicating the ratio of resisting to driving forces on the slip plane.

<a id="liquid_limit_flow_curve"></a>

#### Liquid limit from Casagrande cup tests: `liquid_limit_flow_curve`

Liquid limit (water content at 25 blows) and flow index from Casagrande cup tests by a least-squares line of water content against log10(blows), with the plasticity index when the plastic limit is given. Water contents in percent.

$$
LL = w(N=25),\quad PI = LL - PL
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `blows` | in | number of blows of each test | - |
| `w` | in | water content of each test in % | - |
| `PL` | in | plastic limit in % (optional) (optional) | - |
| `LL` | out | liquid limit (%) | - |
| `flow_index` | out | flow index (% per log cycle of blows) | - |
| `PI` | out | plasticity index LL - PL (%) | - |

**Applies when:** water content falls with blows; water contents in %.

```shell
$ civil-engineering solve liquid_limit_flow_curve "blows=[15, 22, 30, 38]" "w=[42.1, 40.3, 38.7, 37.6]" PL=21
LL = 39.62917155
flow_index = 11.21786009
PI = 18.62917155
  by: Liquid limit from Casagrande cup tests  (least squares w = a + b log10 N; LL = a + b log10 25; flow index = -b)
  PL = 21
  assumes: water content falls with blows; water contents in %
```

**What the worked example means:** This calculation determines the liquid limit by fitting a least-squares line to water content data against the logarithm of blow counts. The result, 39.6292 percent, represents the water content at twenty-five blows. The slope of this line yields the flow index, 11.2179 percent per log cycle, indicating how rapidly water content changes with impact energy. If a plastic limit was provided, the plasticity index is derived as the difference between these two limits.

<a id="mohr_coulomb_failure_stress"></a>

#### Major principal stress at failure (Mohr-Coulomb), `mohr_coulomb_failure_stress`

Major principal stress at failure for a given minor principal stress (triaxial test), with the normal and shear stresses on the failure plane and its angle to the major principal plane (45 + phi/2).

$$
\sigma_1 = \sigma_3\tan^2\!\left(45^\circ+\frac{\phi}{2}\right) + 2c\tan\!\left(45^\circ+\frac{\phi}{2}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `s3` | in | minor principal (confining) stress | Pa |
| `c` | in | cohesion | Pa |
| `phi` | in | friction angle (degrees) | - |
| `s1` | out | major principal stress at failure (Pa) | Pa |
| `sigma_f` | out | normal stress on the failure plane (Pa) | Pa |
| `tau_f` | out | shear stress on the failure plane (Pa) | Pa |
| `theta` | out | failure plane angle from the major principal plane (degrees) | - |

**Applies when:** 0 < phi < 60 degrees; s3, c >= 0.

```shell
$ civil-engineering solve mohr_coulomb_failure_stress "s3=100000 Pa" "c=15000 Pa" phi=28
s1 = 326911.004 Pa
sigma_f = 160191.3702 Pa
tau_f = 100175.2624 Pa
theta = 59
  by: Major principal stress at failure (Mohr-Coulomb)  (s1 = s3 tan^2(45 + phi/2) + 2c tan(45 + phi/2); tangent point at sigma = p - r sin phi, tau = r cos phi)
  s3 = 100000 Pa
  c = 15000 Pa
  phi = 28
  assumes: 0 < phi < 60 degrees; s3, c >= 0
```

**What the worked example means:** This calculation determines the major principal stress required to cause soil failure under a given confining pressure. Using the Mohr-Coulomb criterion, it computes that the major principal stress reaches 326911 Pa. The normal stress on the failure plane is 160191 Pa and the shear stress is 100175 Pa. The angle of the failure plane relative to the major principal plane is 59 degrees.

<a id="proctor_compaction_optimum"></a>

#### Optimum water content and maximum dry density from a compaction test, `proctor_compaction_optimum`

Optimum water content and maximum dry density from Proctor compaction points by a least-squares parabola, with the zero-air-voids dry density at the optimum when Gs is given. Water contents in %, dry densities in Mg/m^3.

$$
w_{opt} = -\frac{c_1}{2c_2},\quad \rho_{zav} = \frac{G_s\rho_w}{1+wG_s}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `w` | in | water contents in % | - |
| `rho_d` | in | dry densities in Mg/m^3 | - |
| `Gs` | in | specific gravity of the solids (optional) (optional) | - |
| `w_opt` | out | optimum water content (%) | - |
| `rho_dmax` | out | maximum dry density (Mg/m^3) | - |
| `rho_zav` | out | zero-air-voids dry density at the optimum (Mg/m^3) | - |

**Applies when:** at least three points around the peak; a parabola opening downward.

```shell
$ civil-engineering solve proctor_compaction_optimum "w=[8, 10, 12, 14, 16]" "rho_d=[1.72, 1.8, 1.84, 1.82, 1.76]" Gs=2.68
w_opt = 12.41176471
rho_dmax = 1.83760084
rho_zav = 2.011052845
  by: Optimum water content and maximum dry density from a compaction test  (least-squares parabola rho_d = c0 + c1 w + c2 w^2; w_opt = -c1/(2 c2); zero air voids rho_w Gs/(1 + w Gs))
  Gs = 2.68
  assumes: at least three points around the peak; a parabola opening downward
```

**What the worked example means:** This calculation determines the optimum moisture content and peak dry density by fitting a least-squares parabola to your compaction test data. The resulting optimum water content is 12.4118 and the corresponding maximum dry density is 1.8376. If specific gravity was provided, the theoretical zero-air-voids dry density at this optimum is calculated as 2.01105, allowing you to assess the degree of saturation achieved.

<a id="plasticity_liquidity_index"></a>

#### Plasticity, liquidity and consistency indices, `plasticity_liquidity_index`

The plasticity index PI = LL - PL, the Casagrande A-line value 0.73 (LL - 20) and, with the natural water content, the liquidity index (w - PL)/PI and consistency index (LL - w)/PI. Limits and water content in percent.

$$
PI = LL - PL,\quad LI = \frac{w - PL}{PI},\quad CI = \frac{LL - w}{PI}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `LL` | in | liquid limit in % | - |
| `PL` | in | plastic limit in % | - |
| `w` | in | natural water content in % (optional) (optional) | - |
| `PI` | out | plasticity index (%) | - |
| `A_line_PI` | out | plasticity index on the A-line at this LL (%) | - |
| `LI` | out | liquidity index | - |
| `CI` | out | consistency index | - |

**Applies when:** 5 <= LL <= 1000 %; 0 <= PL < LL; inputs in percent.

```shell
$ civil-engineering solve plasticity_liquidity_index LL=45 PL=22 w=30
PI = 23
A_line_PI = 18.25
LI = 0.347826087
CI = 0.652173913
  by: Plasticity, liquidity and consistency indices  (PI = LL - PL; LI = (w - PL)/PI; CI = (LL - w)/PI; A-line PI = 0.73 (LL - 20))
  LL = 45
  PL = 22
  w = 30
  assumes: 5 <= LL <= 1000 %; 0 <= PL < LL; inputs in percent
```

**What the worked example means:** This calculation determines the consistency and classification of fine-grained soils using Atterberg limits. The plasticity index, computed as the difference between the liquid limit and plastic limit, equals 23. This value is compared against the Casagrande A-line reference for this specific liquid limit, which yields a theoretical plasticity index of 18.25. Additionally, the liquidity index is calculated as 0.347826 and the consistency index as 0.652174, indicating how close the natural water content is to the plastic or liquid boundaries.

<a id="consolidation_settlement_nc"></a>

#### Primary consolidation settlement of a normally consolidated clay, `consolidation_settlement_nc`

Primary consolidation settlement of a normally consolidated clay layer of thickness H from the compression index Cc, the initial void ratio and the initial and added effective stresses at mid-depth.

$$
S_c = \frac{C_c H}{1+e_0}\log_{10}\frac{\sigma'_0+\Delta\sigma}{\sigma'_0}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Cc` | in | compression index | - |
| `e0` | in | initial void ratio | - |
| `H` | in | clay layer thickness | m |
| `s0` | in | initial effective vertical stress at mid-depth | Pa |
| `ds` | in | added vertical stress at mid-depth | Pa |
| `Sc` | out | consolidation settlement (m) | m |
| `de` | out | change in void ratio | - |
| `e_final` | out | final void ratio | - |

**Applies when:** normally consolidated clay; stresses at mid-depth of the layer.

```shell
$ civil-engineering solve consolidation_settlement_nc Cc=0.32 e0=0.92 "H=4 m" "s0=80000 Pa" "ds=60000 Pa"
Sc = 0.1620253658 m
de = 0.07777217558
e_final = 0.8422278244
  by: Primary consolidation settlement of a normally consolidated clay  (Sc = Cc H/(1 + e0) log10((s0 + ds)/s0))
  Cc = 0.32
  e0 = 0.92
  H = 4 m
  s0 = 80000 Pa
  ds = 60000 Pa
  assumes: normally consolidated clay; stresses at mid-depth of the layer
```

**What the worked example means:** This calculation determines the vertical settlement of a normally consolidated clay layer, resulting in 0.162025 m. It uses the compression index 0.32, initial void ratio 0.92, layer thickness 4 m, and stress conditions 80000 Pa and 60000 Pa at mid-depth. The outcome reflects how much the soil compresses under the new load, yielding a change in void ratio of 0.0777722 and a final void ratio of 0.842228.

<a id="consolidation_settlement_oc"></a>

#### Primary consolidation settlement of an overconsolidated clay, `consolidation_settlement_oc`

Primary consolidation settlement of an overconsolidated clay: recompression with the swelling index Cs up to the preconsolidation stress, then virgin compression with Cc.

$$
S_c = \frac{H}{1+e_0}\left[C_s\log\frac{\sigma'_p}{\sigma'_0} + C_c\log\frac{\sigma'_0+\Delta\sigma}{\sigma'_p}\right]
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Cc` | in | compression index | - |
| `Cs` | in | swelling (recompression) index | - |
| `e0` | in | initial void ratio | - |
| `H` | in | clay layer thickness | m |
| `s0` | in | initial effective vertical stress at mid-depth | Pa |
| `ds` | in | added vertical stress at mid-depth | Pa |
| `sp` | in | preconsolidation stress | Pa |
| `Sc` | out | consolidation settlement (m) | m |
| `de` | out | change in void ratio | - |
| `OCR` | out | overconsolidation ratio sp/s0 | - |
| `case` | out | whether the load passes the preconsolidation stress | - |

**Applies when:** Cs < Cc; sp >= s0; stresses at mid-depth.

```shell
$ civil-engineering solve consolidation_settlement_oc Cc=0.4 Cs=0.06 e0=1.1 "H=5 m" "s0=90000 Pa" "ds=150000 Pa" "sp=160000 Pa"
Sc = 0.2034027429 m
de = 0.08542915202
OCR = 1.777777778
case = passes the preconsolidation stress
  by: Primary consolidation settlement of an overconsolidated clay  (sigma1 <= sp: Sc = Cs H/(1+e0) log(s1/s0); otherwise Sc = H/(1+e0)[Cs log(sp/s0) + Cc log(s1/sp)])
  Cc = 0.4
  Cs = 0.06
  e0 = 1.1
  H = 5 m
  s0 = 90000 Pa
  ds = 150000 Pa
  sp = 160000 Pa
  assumes: Cs < Cc; sp >= s0; stresses at mid-depth
```

**What the worked example means:** This calculation determines the vertical settlement of an overconsolidated clay layer by applying stress increments to the mid-depth. The result, 0.203403 m, represents the total compression derived from recompression up to the preconsolidation stress using the swelling index and virgin compression beyond that point using the compression index. The associated change in void ratio is 0.0854292, while the case classification passes the preconsolidation stress indicates whether the final effective stress exceeds the historical maximum.

<a id="relative_density_from_unit_weights"></a>

#### Relative density of a granular soil: `relative_density_from_unit_weights`

Relative density (density index) of a sand from its in-place dry unit weight and the minimum and maximum dry unit weights of the laboratory tests.

$$
D_r = \frac{\gamma_{d,max}}{\gamma_d}\,\frac{\gamma_d-\gamma_{d,min}}{\gamma_{d,max}-\gamma_{d,min}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `gd` | in | in-place dry unit weight | N/m^3 |
| `gd_min` | in | minimum dry unit weight | N/m^3 |
| `gd_max` | in | maximum dry unit weight | N/m^3 |
| `Dr` | out | relative density (fraction) | - |
| `Dr_percent` | out | relative density (%) | - |

**Applies when:** gd_min <= gd <= gd_max.

```shell
$ civil-engineering solve relative_density_from_unit_weights "gd=16200 N/m^3" "gd_min=14500 N/m^3" "gd_max=17800 N/m^3"
Dr = 0.5660306771
Dr_percent = 56.60306771
  by: Relative density of a granular soil  (Dr = (gd_max/gd)(gd - gd_min)/(gd_max - gd_min) = (e_max - e)/(e_max - e_min))
  gd = 16200 N/m^3
  gd_min = 14500 N/m^3
  gd_max = 17800 N/m^3
  assumes: gd_min <= gd <= gd_max
```

**What the worked example means:** This calculation determines the relative density of a granular soil by comparing its current state to laboratory-defined extremes. It uses the in-place dry unit weight of 16200 N/m^3, the minimum dry unit weight of 14500 N/m^3, and the maximum dry unit weight of 17800 N/m^3. The resulting relative density is 0.566031, which indicates how tightly packed the soil particles are compared to their loosest and densest possible arrangements.

<a id="secondary_compression_settlement"></a>

#### Secondary compression (creep) settlement of a clay layer, `secondary_compression_settlement`

The secondary compression settlement S_s = C_alpha H/(1 + e_p) log10(t2/t1) of a clay layer of thickness H after the end of primary consolidation t1, up to a later time t2.

$$
S_s = \frac{C_\alpha H}{1 + e_p}\log_{10}\frac{t_2}{t_1}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Ca` | in | secondary compression index C_alpha | - |
| `H` | in | layer thickness (at the end of primary consolidation) | m |
| `ep` | in | void ratio at the end of primary consolidation | - |
| `t1` | in | time at the end of primary consolidation | s |
| `t2` | in | later time | s |
| `Ss` | out | secondary compression settlement (m) | m |
| `Ss_mm` | out | settlement (mm) | - |
| `C_alpha_mod` | out | modified secondary compression index C_alpha/(1 + e_p) | - |

**Applies when:** 0 < C_alpha <= 0.2; t2 > t1 (t1 = end of primary consolidation).

```shell
$ civil-engineering solve secondary_compression_settlement Ca=0.02 "H=5 m" ep=1.1 "t1=31557600 s" "t2=631152000 s"
Ss = 0.06195380932 m
Ss_mm = 61.95380932
C_alpha_mod = 0.009523809524
  by: Secondary compression (creep) settlement of a clay layer  (S_s = C_alpha H/(1 + e_p) log10(t2/t1))
  Ca = 0.02
  H = 5 m
  ep = 1.1
  t1 = 31557600 s
  t2 = 631152000 s
  assumes: 0 < C_alpha <= 0.2; t2 > t1 (t1 = end of primary consolidation)
```

**What the worked example means:** This calculation determines the long-term settlement due to creep after primary consolidation ends. Using the secondary compression index 0.02, layer thickness 5 m, and void ratio 1.1, the modified index is 0.00952381. The total secondary settlement over time from 3.15576e+07 s to 6.31152e+08 s results in 0.0619538 m meters.

<a id="soil_phase_relations"></a>

#### Soil phase relations from void ratio and water content, `soil_phase_relations`

Porosity, degree of saturation and the dry, bulk, saturated and submerged unit weights of a soil from its specific gravity of solids Gs, void ratio e and water content w (unit weight of water 9.81 kN/m^3).

$$
S e = w G_s,\quad \gamma = \frac{G_s(1+w)}{1+e}\gamma_w,\quad \gamma_{sat} = \frac{G_s+e}{1+e}\gamma_w
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Gs` | in | specific gravity of the solids | - |
| `e` | in | void ratio | - |
| `w` | in | water content (fraction, or % in a question) | - |
| `n` | out | porosity e/(1 + e) | - |
| `S` | out | degree of saturation w Gs/e | - |
| `gamma_d` | out | dry unit weight (N/m^3) | N/m^3 |
| `gamma` | out | bulk (moist) unit weight (N/m^3) | N/m^3 |
| `gamma_sat` | out | saturated unit weight (N/m^3) | N/m^3 |
| `gamma_sub` | out | submerged (buoyant) unit weight (N/m^3) | N/m^3 |

**Applies when:** w Gs <= e (not oversaturated); 2.0 <= Gs <= 3.0.

```shell
$ civil-engineering solve soil_phase_relations Gs=2.7 e=0.65 w=0.18
n = 0.3939393939
S = 0.7476923077
gamma_d = 16052.72727 N/m^3
gamma = 18942.21818 N/m^3
gamma_sat = 19917.27273 N/m^3
gamma_sub = 10107.27273 N/m^3
  by: Soil phase relations from void ratio and water content  (n = e/(1 + e), S = w Gs/e, gamma_d = Gs gamma_w/(1 + e), gamma = Gs gamma_w (1 + w)/(1 + e), gamma_sat = (Gs + e) gamma_w/(1 + e))
  Gs = 2.7
  e = 0.65
  w = 0.18
  assumes: w Gs <= e (not oversaturated); 2.0 <= Gs <= 3.0
```

**What the worked example means:** This calculation determines key phase relationships for the soil sample using the given specific gravity, void ratio, and water content. The resulting porosity of 0.393939 indicates the volume fraction of voids, while the degree of saturation of 0.747692 shows how much of that void space is filled with water. The computed dry unit weight of 16052.7 N/m^3 and saturated unit weight of 19917.3 N/m^3 provide essential density parameters for geotechnical analysis.

<a id="soil_phase_from_lab_masses"></a>

#### Soil properties from a weighed sample: `soil_phase_from_lab_masses`

Water content, bulk and dry densities and unit weights, void ratio, porosity and degree of saturation of a soil sample from its wet mass, oven-dry mass, volume and the specific gravity of the solids.

$$
w = \frac{M-M_d}{M_d},\quad e = \frac{G_s\rho_w}{\rho_d}-1,\quad S = \frac{wG_s}{e}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `M` | in | wet (moist) mass of the sample | kg |
| `Md` | in | oven-dry mass of the sample | kg |
| `V` | in | volume of the sample | m^3 |
| `Gs` | in | specific gravity of the solids | - |
| `w` | out | water content | - |
| `rho` | out | bulk density (kg/m^3) | kg/m^3 |
| `rho_d` | out | dry density (kg/m^3) | kg/m^3 |
| `e` | out | void ratio | - |
| `n` | out | porosity | - |
| `S` | out | degree of saturation | - |
| `gamma` | out | bulk unit weight (N/m^3) | N/m^3 |
| `gamma_d` | out | dry unit weight (N/m^3) | N/m^3 |

**Applies when:** M >= Md; consistent data (0 < e, S <= 1).

```shell
$ civil-engineering solve soil_phase_from_lab_masses "M=1.85 kg" "Md=1.6 kg" "V=0.001 m^3" Gs=2.7
w = 0.15625
rho = 1850 kg/m^3
rho_d = 1600 kg/m^3
e = 0.6875
n = 0.4074074074
S = 0.6136363636
gamma = 18148.5 N/m^3
gamma_d = 15696 N/m^3
  by: Soil properties from a weighed sample  (w = (M - Md)/Md, rho_d = Md/V, e = Gs rho_w/rho_d - 1, S = w Gs/e)
  M = 1.85 kg
  Md = 1.6 kg
  V = 0.001 m^3
  Gs = 2.7
  assumes: M >= Md; consistent data (0 < e, S <= 1)
```

**What the worked example means:** This calculation determines key soil properties from a weighed sample. The water content, representing the mass of water relative to solids, is computed as 0.15625. Using the oven-dry mass and volume, the dry density is found to be 1600 kg/m^3, which leads to a void ratio of 0.6875. These values characterize the physical state of the soil, indicating how much space between particles is filled with air or water.

<a id="consolidation_time"></a>

#### Time to reach a degree of consolidation: `consolidation_time`

Time needed for a clay layer to reach an average degree of consolidation U (Terzaghi), with the time factor.

$$
t = \frac{T_v(U)\,H_{dr}^2}{c_v}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `cv` | in | coefficient of consolidation | m^2/s |
| `Hdr` | in | drainage path length | m |
| `U` | in | average degree of consolidation (fraction, or % in a question) | - |
| `Tv` | out | time factor | - |
| `t` | out | time (s) | s |
| `t_days` | out | time (days) | - |

**Applies when:** 1% <= U <= 99%; Terzaghi 1-D theory.

```shell
$ civil-engineering solve consolidation_time "cv=9.506426344e-08 m^2/s" "Hdr=2.5 m" U=0.9
Tv = 0.848085408
t = 55757375.15 s
t_days = 645.3399902
  by: Time to reach a degree of consolidation  (Tv from U(Tv) = U (bisection on the Fourier series), t = Tv Hdr^2/cv)
  cv = 9.506426344e-08 m^2/s
  Hdr = 2.5 m
  U = 0.9
  assumes: 1% <= U <= 99%; Terzaghi 1-D theory
```

**What the worked example means:** This calculation determines the time required for a clay layer to achieve an average degree of consolidation of 0.9. It uses the coefficient of consolidation 9.50643e-08 m^2/s and the drainage path length 2.5 m to compute the time factor 0.848085, which then yields the total time 5.57574e+07 s. The result indicates how long engineers must wait for settlement to stabilize before proceeding with construction.

<a id="effective_stress_at_depth"></a>

#### Total, pore-water and effective vertical stress at a depth, `effective_stress_at_depth`

Total vertical stress, hydrostatic pore-water pressure and effective vertical stress at depth z in a layered soil with a water table at depth zw (no capillary rise, hydrostatic water). Each layer's unit weight is the one that applies there (moist above the water table, saturated below), in kN/m^3.

$$
\sigma'_v = \sum \gamma_i h_i - \gamma_w (z - z_w)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `H` | in | layer thicknesses in m, from the surface down | - |
| `gam` | in | unit weight of each layer in kN/m^3 | - |
| `zw` | in | depth of the water table | m |
| `z` | in | depth of the point | m |
| `sigma` | out | total vertical stress (Pa) | Pa |
| `u` | out | pore-water pressure (Pa) | Pa |
| `sigma_eff` | out | effective vertical stress (Pa) | Pa |
| `sigma_eff_kPa` | out | effective vertical stress (kPa) | - |

**Applies when:** hydrostatic pore water; no capillary rise; 0 < z <= total thickness.

```shell
$ civil-engineering solve effective_stress_at_depth "H=[3, 5]" "gam=[17.5, 19.5]" "zw=3 m" "z=8 m"
sigma = 150000 Pa
u = 49050 Pa
sigma_eff = 100950 Pa
sigma_eff_kPa = 100.95
  by: Total, pore-water and effective vertical stress at a depth  (sigma = sum gamma_i h_i above z, u = gamma_w (z - zw) below the water table, sigma' = sigma - u)
  zw = 3 m
  z = 8 m
  assumes: hydrostatic pore water; no capillary rise; 0 < z <= total thickness
```

**What the worked example means:** This calculation determines the total, pore-water, and effective vertical stresses at a specified depth within a layered soil profile. The total vertical stress, reported as 150000 Pa, represents the cumulative weight of all soil layers above the point of interest. If the depth lies below the water table located at 3 m, the hydrostatic pore-water pressure, shown as 49050 Pa, is calculated based on the height of the water column. The effective vertical stress, given as 100950 Pa or 100.95, is derived by subtracting the pore-water pressure from the total vertical stress. This value is critical because it reflects the actual stress carried by the soil skeleton, governing its strength and stiffness.

### Surveying (6)

<a id="area_from_offsets"></a>

#### Area between a survey line and a boundary from offsets, `area_from_offsets`

Area between a chain line and an irregular boundary from offsets at a regular interval, by the trapezoidal rule and, for an odd number of offsets, Simpson's one-third rule. Offsets in m.

$$
A = \frac{d}{3}\left(y_1 + y_n + 4\sum y_{even} + 2\sum y_{odd}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `offsets` | in | offsets in m at the regular interval | - |
| `d` | in | interval between offsets | m |
| `A_trapezoidal` | out | area by the trapezoidal rule (m^2) | m^2 |
| `A_simpson` | out | area by Simpson's rule (m^2) | m^2 |

**Applies when:** regular interval; Simpson's rule only for an odd number of offsets.

```shell
$ civil-engineering solve area_from_offsets "offsets=[0, 2.5, 3.5, 5, 4.6, 3.2, 0]" "d=10 m"
A_trapezoidal = 188 m^2
A_simpson = 196.6666667 m^2
  by: Area between a survey line and a boundary from offsets  (trapezoidal: d (y1/2 + y2 + ... + yn/2); Simpson: (d/3)(y1 + 4y2 + 2y3 + ... + yn))
  d = 10 m
  assumes: regular interval; Simpson's rule only for an odd number of offsets
```

**What the worked example means:** This calculation determines the area enclosed by a straight chain line and an irregular boundary using perpendicular offsets measured at regular intervals. The result 188 m^2 represents the area computed by approximating the boundary as straight segments, while 196.667 m^2 uses parabolic arcs for a more accurate estimate when there is an odd number of offsets. These values quantify the land surface in square meters based on the interval 10 m and the specific offset measurements provided.

<a id="traverse_bowditch_adjustment"></a>

#### Closed traverse: misclosure and Bowditch (compass rule) adjustment, `traverse_bowditch_adjustment`

Latitudes and departures of a closed loop traverse from whole-circle bearings (azimuths clockwise from north, degrees) and distances (m), the misclosure and relative precision (1 in X), the coordinates adjusted by the Bowditch (compass) rule, and the enclosed area. The last leg returns to the starting station.

$$
\delta E_i = -e_E\frac{d_i}{\sum d},\quad \delta N_i = -e_N\frac{d_i}{\sum d}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `az` | in | azimuths (whole-circle bearings) of the legs in degrees | - |
| `d` | in | leg lengths in m | - |
| `E0` | in | easting of the starting station (default 0) (optional) | m |
| `N0` | in | northing of the starting station (default 0) (optional) | m |
| `misclosure_E` | out | misclosure in departure (m) | m |
| `misclosure_N` | out | misclosure in latitude (m) | m |
| `linear_misclosure` | out | linear misclosure (m) | m |
| `precision` | out | relative precision X in 1:X (0 if it closes exactly) | - |
| `E` | out | adjusted eastings of the stations (m) | m |
| `N` | out | adjusted northings of the stations (m) | m |
| `area` | out | enclosed area (m^2) | m^2 |

**Applies when:** closed loop traverse; azimuths 0 to 360 degrees.

```shell
$ civil-engineering solve traverse_bowditch_adjustment "az=[45, 135.01, 225, 315]" "d=[100, 100.02, 99.98, 100]"
misclosure_E = 0.01593938435 m
misclosure_N = -0.0123427325 m
linear_misclosure = 0.02015953915 m
precision = 19841.72341
E = [0.0,70.7066932726,141.415182997,70.7146629647] m
N = [0.0,70.7137638018,-0.020312884737,-70.7137638018] m
area = 9999.999767 m^2
  by: Closed traverse: misclosure and Bowditch (compass rule) adjustment  (dep = d sin(az), lat = d cos(az); corrections -e d_i/sum d (compass rule); area by coordinates)
  assumes: closed loop traverse; azimuths 0 to 360 degrees
```

**What the worked example means:** This calculation determines the closure error of a loop survey by summing latitudes and departures derived from azimuths and distances. The resulting linear misclosure of 0.0201595 m indicates how far the final station falls short of the starting point, while the relative precision of 1 in 19841.7 assesses the quality of the field measurements against standard tolerances. Using the Bowditch rule, errors are distributed proportionally to leg lengths to produce adjusted coordinates, which are then used to compute the enclosed area of 10000 m^2. This process ensures that the survey network is geometrically consistent before establishing boundaries or calculating land area.

<a id="earthwork_volume_end_areas"></a>

#### Earthwork volume from cross-section areas: `earthwork_volume_end_areas`

Volume of cut or fill between equally spaced cross-sections by the average-end-area method, and by the prismoidal (Simpson) rule when the number of sections is odd. Areas in m^2.

$$
V = d\left(\frac{A_1+A_n}{2}+\sum_{i=2}^{n-1}A_i\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `areas` | in | cross-section areas in m^2, in order | - |
| `d` | in | spacing between sections | m |
| `V_end_area` | out | volume by average end areas (m^3) | m^3 |
| `V_prismoidal` | out | volume by the prismoidal rule (m^3) | m^3 |

**Applies when:** equally spaced sections; prismoidal rule only for an odd number of sections.

```shell
$ civil-engineering solve earthwork_volume_end_areas "areas=[12.5, 18.2, 25.6, 20.1, 15.8]" "d=20 m"
V_end_area = 1561 m^3
V_prismoidal = 1551.333333 m^3
  by: Earthwork volume from cross-section areas  (V = d (A1/2 + A2 + ... + An/2); prismoidal V = (d/3)(A1 + 4A2 + 2A3 + ... + An))
  d = 20 m
  assumes: equally spaced sections; prismoidal rule only for an odd number of sections
```

**What the worked example means:** This calculation determines the total volume of cut or fill between cross-sections spaced at equal intervals. Using the average end area method, the result is 1561 m^3. This value represents the estimated quantity of material required for construction projects such as roads or dams, assuming linear ground variation between sections.

<a id="horizontal_circular_curve"></a>

#### Elements of a simple horizontal circular curve, `horizontal_circular_curve`

Tangent length, curve length, long chord, external distance, middle ordinate and the chainages of the tangent points of a simple circular curve of radius R joining tangents that deflect by Delta at the intersection point PI.

$$
T = R\tan\frac{\Delta}{2},\quad L = R\Delta,\quad LC = 2R\sin\frac{\Delta}{2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `R` | in | curve radius | m |
| `Delta` | in | deflection (intersection) angle (degrees) | - |
| `PI` | in | chainage of the intersection point (default 0) (optional) | m |
| `T` | out | tangent length (m) | m |
| `L` | out | curve length (m) | m |
| `LC` | out | long chord (m) | m |
| `E` | out | external distance (m) | m |
| `M` | out | middle ordinate (m) | m |
| `PC` | out | chainage of the start of the curve (m) | m |
| `PT` | out | chainage of the end of the curve (m) | m |

**Applies when:** 0 < Delta < 180 degrees; chainage measured along the tangents then the curve.

```shell
$ civil-engineering solve horizontal_circular_curve "R=300 m" Delta=40 "PI=1250 m"
T = 109.1910703 m
L = 209.4395102 m
LC = 205.212086 m
E = 19.25333174 m
M = 18.09221376 m
PC = 1140.80893 m
PT = 1350.24844 m
  by: Elements of a simple horizontal circular curve  (T = R tan(D/2), L = R D, LC = 2R sin(D/2), E = R(sec(D/2) - 1), M = R(1 - cos(D/2)), PC = PI - T, PT = PC + L)
  R = 300 m
  Delta = 40
  PI = 1250 m
  assumes: 0 < Delta < 180 degrees; chainage measured along the tangents then the curve
```

**What the worked example means:** This calculation determines the geometric properties of a simple circular curve with radius 300 m deflecting by angle 40 at intersection point 1250 m. The tangent length is 109.191 m, the arc length is 209.44 m, and the long chord is 205.212 m. The external distance is 19.2533 m and the middle ordinate is 18.0922 m. The chainage of the point of curvature is 1140.81 m and the point of tangency is 1350.25 m. These values define the physical layout required for construction staking.

<a id="vertical_parabolic_curve"></a>

#### Elevations on a vertical parabolic curve: `vertical_parabolic_curve`

Elevation at a station on an equal-tangent parabolic vertical curve, with the highest (crest) or lowest (sag) point on the curve and the rate of change of grade. Grades are fractions or percentages; stations and lengths in m.

$$
y = y_{BVC} + g_1x + \frac{g_2-g_1}{2L}x^2,\quad x_t = -\frac{g_1L}{g_2-g_1}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `g1` | in | entering grade (fraction, or %) | - |
| `g2` | in | leaving grade (fraction, or %) | - |
| `L` | in | curve length (horizontal) | m |
| `PVI` | in | station of the intersection point | m |
| `elev_PVI` | in | elevation of the intersection point | m |
| `x` | in | station where the elevation is wanted | m |
| `elev` | out | elevation at the station (m) | m |
| `x_turn` | out | station of the highest/lowest point on the curve (m) | m |
| `elev_turn` | out | elevation of that point (m) | m |
| `r` | out | rate of change of grade (% per m) | - |
| `BVC` | out | station of the beginning of the curve (m) | m |
| `elev_BVC` | out | elevation of the BVC (m) | m |

**Applies when:** equal-tangent parabola; BVC <= station <= EVC.

```shell
$ civil-engineering solve vertical_parabolic_curve g1=0.03 g2=-0.02 "L=200 m" "PVI=1500 m" "elev_PVI=120 m" "x=1450 m"
elev = 118.1875 m
x_turn = 1520 m
elev_turn = 118.8 m
r = -0.025
BVC = 1400 m
elev_BVC = 117 m
  by: Elevations on a vertical parabolic curve  (y = y_BVC + g1 x + (g2 - g1) x^2/(2L), x from the BVC; turning point at x = -g1 L/(g2 - g1))
  g1 = 0.03
  g2 = -0.02
  L = 200 m
  PVI = 1500 m
  elev_PVI = 120 m
  x = 1450 m
  assumes: equal-tangent parabola; BVC <= station <= EVC
```

**What the worked example means:** This calculation determines the vertical alignment along an equal-tangent parabolic curve. Using the entering grade of 0.03 and leaving grade of -0.02 over a length of 200 m, the elevation at the requested station 1450 m is found to be 118.188 m. The highest or lowest point on this curve occurs at station 1520 m, which helps in assessing drainage and sight distance requirements.

<a id="levelling_rise_and_fall"></a>

#### Reduced levels by the rise-and-fall method, `levelling_rise_and_fall`

Reduced levels of every station of a levelling run from staff readings (m) and their sight types (BS, IS, FS in reading order; a BS after an FS is the new set-up on the same change point), with the total rise and fall and the arithmetic check sum BS - sum FS (= last RL - first RL).

$$
\sum \text{BS} - \sum \text{FS} = \sum\text{rise} - \sum\text{fall} = RL_{last} - RL_{first}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `readings` | in | staff readings in m, in reading order | - |
| `sights` | in | sight type of each reading: BS, IS or FS | - |
| `RL0` | in | reduced level of the first station (benchmark) | m |
| `RL` | out | reduced level of each station (m) | m |
| `rise` | out | sum of rises (m) | m |
| `fall` | out | sum of falls (m) | m |
| `last_RL` | out | reduced level of the last station (m) | m |
| `check` | out | sum BS - sum FS (m) | m |

**Applies when:** first reading BS, last FS; a BS after the first follows an FS (change point).

```shell
$ civil-engineering solve levelling_rise_and_fall "readings=[1.525, 2.105, 1.64, 0.985, 2.32]" "sights=BS, IS, FS, BS, FS" "RL0=100 m"
RL = [100.0,99.42,99.885,98.55] m
rise = 0.465 m
fall = 1.915 m
last_RL = 98.55 m
check = -1.45 m
  by: Reduced levels by the rise-and-fall method  (rise or fall = previous reading - current reading within a set-up; RL = previous RL + rise - fall)
  RL0 = 100 m
  sights = BS, IS, FS, BS, FS
  assumes: first reading BS, last FS; a BS after the first follows an FS (change point)
```

**What the worked example means:** This calculation determines the elevation of each survey station using the rise-and-fall method. By comparing consecutive staff readings within instrument setups, it computes individual rises or falls to update the reduced level from the starting benchmark of 100 m. The process yields a sum of rises equal to 0.465 m and a sum of falls equal to 1.915 m. The arithmetic check confirms that the difference between the sum of back sights and fore sights matches the change in elevation, resulting in a check value of -1.45 m meters.

Questions and reports: support@ai999x.com. AI999X, The Future Is Self-Improving.
