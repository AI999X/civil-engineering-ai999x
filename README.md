# civil-engineering-ai999x — offline, verified civil engineering solver with step-by-step answers

[![tests](https://github.com/AI999X/civil-engineering-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/civil-engineering-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/civil-engineering-ai999x)](https://pypi.org/project/civil-engineering-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/civil-engineering-ai999x)

**[Install](https://pypi.org/project/civil-engineering-ai999x/)** · **[Capability guide](https://huggingface.co/AI999X/civil-engineering-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/civil-engineering-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.0.0

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
| Package | `civil-engineering-ai999x` 1.0.0 (import `civil_engineering_ai999x`) |
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

Command line: `civil-engineering-ai999x "<question>"`, `civil-engineering-ai999x --capabilities`. The outputs above were produced by running this exact code on civil-engineering-ai999x 1.0.0.

## How every answer is verified

1. **Reading** — the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning** — only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing** — exact symbolic or high-precision numeric evaluation.
4. **Checking** — dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

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
@software{civil_engineering_ai999x_1_0_0,
  title  = {civil-engineering-ai999x: an offline, verified civil engineering solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.0.0},
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
