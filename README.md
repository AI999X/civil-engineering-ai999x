# civil-engineering-ai999x — offline, verified civil engineering solver with step-by-step answers

[![tests](https://github.com/AI999X/civil-engineering-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/civil-engineering-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/civil-engineering-ai999x)](https://pypi.org/project/civil-engineering-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/civil-engineering-ai999x)

**[Install](https://pypi.org/project/civil-engineering-ai999x/)** · **[Capability guide](https://huggingface.co/AI999X/civil-engineering-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/civil-engineering-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.0.1

Field: **Engineering** · Tasks: Question Answering, Sentence Similarity, Feature Extraction

civil-engineering-ai999x is an offline, deterministic Python solver that answers civil engineering questions with verified results from 88 certified capabilities, gives 566 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**civil-engineering-ai999x** is a compact, deterministic civil engineering engine for Python. It reads a question in plain English (or a typed call), picks a certified civil engineering capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions — or an honest refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green)

## Highlights

- **Verified, not generated.** Every answer is computed by a certified civil engineering capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes — SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR — never a silent guess. Every one of its 566 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10, 3.11, 3.12, 3.13, 3.14).
- **Coverage:** open-channel hydraulics, pipe networks, hydrology, groundwater flow, soil mechanics and foundations, and surveying.

## Model overview

| Property | Value |
|---|---|
| Package | `civil-engineering-ai999x` 1.0.1 (import `civil_engineering_ai999x`) |
| Type | Deterministic neuro-symbolic civil engineering engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 88 certified procedures |
| Knowledge pack | version 1.0.1 |
| Golden answers | 566 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python 3.10, 3.11, 3.12, 3.13, 3.14) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Quickstart

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

Command line: `civil-engineering-ai999x "<question>"`, `civil-engineering-ai999x --capabilities`. The outputs above were produced by running this exact code on civil-engineering-ai999x 1.0.1.

## Find the right capability, and draw it

`civil_engineering.search()` finds which certified capability a question is about — across **every** AI999X model, so a question for another model points you to it. It runs **[ai999x-encoder](https://huggingface.co/AI999X/ai999x-encoder)**, AI999X's own capability encoder (4.7M parameters, a 4.9 MB ONNX model, offline; the root `model.safetensors` of this repository): `pip install "civil-engineering-ai999x[search]"`. `ask(question, suggest=True)` adds the same pointers to a question it does not answer; an answer itself never changes.

```python
import civil_engineering_ai999x as civil_engineering

civil_engineering.search("Solve x^2 - 5x + 6 = 0", k=2)
# -> Roots of a quadratic equation (math-foundations-ai999x, 0.63); Roots of a cubic equation (math-foundations-ai999x, 0.59)
```

## How every answer is verified

1. **Reading** — the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning** — only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing** — exact symbolic or high-precision numeric evaluation.
4. **Checking** — dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Worked examples

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

![Drawdown cone around a pumped well — computed by civil-engineering-ai999x](https://huggingface.co/AI999X/civil-engineering-ai999x/resolve/main/figures/theis-drawdown-profile.svg)

_Drawdown cone around a well pumping Q = 2500 m³/d from a confined aquifer (T = 500 m²/d, S = 0.0002) after 1, 10 and 100 days, drawdown computed by the package at 41 distances from 5 m to 1 km (shown on both sides of the well): 4.90277 m at r = 5 m after 1 d; 5.81894 m at r = 5 m after 10 d; 6.73511 m at r = 5 m after 100 d. Marked: r = 100 m, t = 1 d, s = 2.51924 m._

**Specific-energy diagram and alternate depths**

![Specific-energy diagram and alternate depths — computed by civil-engineering-ai999x](https://huggingface.co/AI999X/civil-engineering-ai999x/resolve/main/figures/alternate-depth-rectangular-diagram.svg)

_Specific-energy diagram of a rectangular channel carrying Q = 6 m³/s in b = 2 m (E computed by the package at 81 depths). Depth y = 0.4 m (supercritical, Fr = 3.78614) has E = 3.26697 m and the alternate depth 3.22281 m; marked: critical depth yc = 0.971683 m at the minimum energy E = 1.45752 m._

**Active earth pressure on a retaining wall**

![Active earth pressure on a retaining wall — computed by civil-engineering-ai999x](https://huggingface.co/AI999X/civil-engineering-ai999x/resolve/main/figures/rankine-active-thrust-diagram.svg)

_Rankine active earth pressure on a 6 m wall retaining soil with φ = 30° and γ = 18 kN/m³ (Ka = 0.333333), drawn as the pressure diagram against the wall: the pressure at each of 40 depths is the base pressure the package returns for a wall of that height. Without surcharge it grows to 36 kPa at the base, resultant Pa = 108 kN/m acting 2 m above the base (arrow); with a 20 kPa surcharge (dashed) the base pressure is 42.6667 kPa and Pa = 148 kN/m._

**Flood routing: inflow and outflow hydrographs**

![Flood routing: inflow and outflow hydrographs — computed by civil-engineering-ai999x](https://huggingface.co/AI999X/civil-engineering-ai999x/resolve/main/figures/muskingum-routing-hydrograph.svg)

_A flood wave routed through a river reach by the Muskingum method (K = 12 h, X = 0.2, Δt = 6 h; C0 = 0.047619, C1 = 0.428571, C2 = 0.52381): inflow per interval as bars, the outflow returned by the package as stems — lower and later; peak inflow 68 m³/s, marked: peak outflow t = 24 h, Q = 45.836 m³/s (attenuation 22.164 m³/s)._

**Flows in a looped pipe network**

![Flows in a looped pipe network — computed by civil-engineering-ai999x](https://huggingface.co/AI999X/civil-engineering-ai999x/resolve/main/figures/hardy-cross-network-network.svg)

_A looped pipe network (5 pipes, 2 loops; lengths [600, 500, 600, 500, 780] m, diameters [300, 250, 200, 250, 200] mm, Hazen–Williams C = 120) balanced by the Hardy Cross method in 14 iterations: arrows show the direction and size of each pipe flow returned by the package (L/s, a negative value flows against the stated pipe direction: 1→2: 71.44, 2→3: 33.1344, 3→4: -16.8656, 4→1: -48.56, 2→4: -1.6944); boxes give each node's demand. The node positions are a layout, not to scale._

## Capabilities

88 certified procedures. A few of them:

- **Procedures:** Area between a survey line and a boundary from offsets; Areal rainfall by Thiessen polygons; Average degree of consolidation after a time (Terzaghi); Broad-crested weir discharge; Chezy coefficient from Manning's n; Chezy velocity; Closed traverse: misclosure and Bowditch (compass rule) adjustment; Coefficient of consolidation from a consolidation time; Cohesion and friction angle from direct shear tests; Cohesion and friction angle from triaxial tests; Consolidation settlement of a thick clay layer integrated over depth; Coulomb active thrust on a rough wall with sloping backfill; Coulomb passive thrust on a rough wall; Critical depth in a circular conduit; Critical depth in a rectangular channel; Critical depth in a trapezoidal channel; Darcy flow across soil layers in series; equivalent conductivities; Direct runoff hydrograph by unit-hydrograph convolution; Discharge through a small orifice under a head; Drawdown from several wells by superposition (Theis); Earthwork volume from cross-section areas; Effective sizes and gradation coefficients from a sieve analysis; Elements of a simple horizontal circular curve; Elevations on a vertical parabolic curve …

`civil_engineering_ai999x.templates()` lists every question form `ask()` reads; the capability guide (PDF, every example executed) is included in this repository.

## Out of scope

- Open-ended explanation, essay writing or tutoring dialogue: it is a solver, not a chatbot.
- Questions outside its certified civil engineering capabilities, and any question whose stated numbers it cannot all use — it refuses with the reason instead.
- High-stakes engineering, medical or legal decisions without independent review.

## Limitations

- It answers what its certified capabilities cover (open-channel hydraulics, pipe networks, hydrology, groundwater flow, soil mechanics and foundations, and surveying); anything else is refused with a reason.
- `ask()` reads documented question forms and common phrasings; unusual wording may be refused — use `solve()` for guaranteed results.
- Results are as correct as the certified relations and their stated conditions; a certified relation is a checked model, not a claim about every physical situation.

## Citation

```bibtex
@software{civil_engineering_ai999x_1_0_1,
  title  = {civil-engineering-ai999x: an offline, verified civil engineering solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.0.1},
  url    = {https://huggingface.co/AI999X/civil-engineering-ai999x}
}
```

## Contact

Questions, bug reports and requests: open a discussion on https://huggingface.co/AI999X/civil-engineering-ai999x/discussions or visit https://ai999x.com.

## Support the project

civil-engineering-ai999x is free and open source. If it helps you, you can support further development on [Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi).

Created and engineered by **Ibnul Jaif Farabi** · **[AI999X](https://huggingface.co/AI999X)** · [ai999x.com](https://ai999x.com)

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/civil-engineering-ai999x/) (`pip install .` builds it). Releases are tagged `v<version>`; the wheel is attached to each [GitHub release](https://github.com/AI999X/civil-engineering-ai999x/releases). The optional ONNX models (formula graphs and, where measured, a decision model) are on [Hugging Face](https://huggingface.co/AI999X/civil-engineering-ai999x/tree/main/onnx).
