# Civil Engineering (civil-engineering-ai999x)

[![tests](https://github.com/AI999X/civil-engineering-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/civil-engineering-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/civil-engineering-ai999x)](https://pypi.org/project/civil-engineering-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/civil-engineering-ai999x)

Civil Engineering covers pipe and open-channel hydraulics, hydrology, groundwater flow, soil mechanics and foundations, water networks and surveying: 88 calculations in 7 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Civil Engineering desktop app: Broad-crested weir discharge, with its worked example and live scene](https://www.ai999x.com/?ax_shot=civil-engineering-ai999x/hero.webp)](https://www.ai999x.com/models/civil-engineering-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-1.6.4-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-1.6.4-linux-x86_64.tar.gz) |
| **Command line** `civil-engineering` (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-linux-x86_64.tar.gz) |
| **Browser** (any device) | [Try it on the model page](https://www.ai999x.com/models/civil-engineering-ai999x/#try) |
| **Python** 3.10+ | `pip install civil-engineering-ai999x`: on Windows and Linux (x86-64) `pip` installs the same compiled engine; on other systems (macOS, ARM) it installs a pure-Python build that gives the same answers |

## Ask in words: 15 examples

Each question below was asked on this release; the comment under it is the line it printed.

```python
import civil_engineering_ai999x as civil_engineering

a = civil_engineering.ask(
    "Broad crested weir discharge with discharge coefficient = 0.95, "
    "crest width = 5 m and head = 0.6 m."
)
print(a.outcome, a.values)
# SOLVED {'Q': 3.76372687638, 'yc': 0.4}

a = civil_engineering.ask(
    "Area from offsets with offsets = [0, 2.5, 3.5, 5, 4.6, 3.2, 0] "
    "and interval = 10 m."
)
print(a.outcome, a.values)
# SOLVED {'A_trapezoidal': 188.0, 'A_simpson': 196.666666667}

a = civil_engineering.ask(
    "Chezy coefficient from manning with Manning n = 0.013 and "
    "hydraulic radius = 0.75 m."
)
print(a.outcome, a.values)
# SOLVED {'C': 73.3218686921}

a = civil_engineering.ask(
    "Falling head permeability with standpipe diameter = 10 mm, sample "
    "diameter = 75 mm, sample length = 150 mm, initial head = 100 cm, "
    "final head = 60 cm and time = 600 s."
)
print(a.outcome, a.values)
# SOLVED {'K': 2.27033610563e-06, 'K_cm_s': 0.000227033610563}

a = civil_engineering.ask(
    "Chezy velocity with Chezy C = 55 m^0.5/s, hydraulic radius = 1.1 "
    "m and slope = 0.0004."
)
print(a.outcome, a.values)
# SOLVED {'V': 1.15368973299}

a = civil_engineering.ask(
    "Flow from head loss by hazen williams with head loss = 5 m, "
    "diameter = 300 mm, length = 1500 m and Hazen-Williams C = 110."
)
print(a.outcome, a.values)
# SOLVED {'Q': 0.0593636598191, 'V': 0.839823991132}

a = civil_engineering.ask(
    "Direct shear strength parameters with normal stresses = [50, 100, "
    "200] and shear stresses = [45, 73, 131]."
)
print(a.outcome, a.values)
# SOLVED {'phi': 29.8681357032, 'c': 16.0, 'tan_phi': 0.574285714286}

a = civil_engineering.ask(
    "Earthwork volume from cross sections with areas = [12.5, 18.2, "
    "25.6, 20.1, 15.8] and spacing = 20 m."
)
print(a.outcome, a.values)
# SOLVED {'V_end_area': 1561.0, 'V_prismoidal': 1551.33333333}

a = civil_engineering.ask(
    "Thiem unconfined conductivity with pumping rate = 1500 m^3/d, "
    "first distance = 20 m, second distance = 80 m, first head = 18.2 "
    "m and second head = 19.6 m."
)
print(a.outcome, a.values)
# SOLVED {'K': 0.0001447648968, 'K_m_d': 12.5076870835}

a = civil_engineering.ask(
    "Terzaghi bearing capacity with cohesion = 20 kPa, unit weight = "
    "18 kN/m^3, width = 2 m, depth = 1.5 m, Nc = 37.16, Nq = 22.46, "
    "Ngamma = 19.13 and footing shape = square."
)
print(a.outcome, a.values)
# SOLVED {'qu': 1848052.0, 'qa': 616017.333333, 'q_net': 1821052.0}

a = civil_engineering.ask(
    "Theis drawdown with pumping rate = 2500 m^3/d, transmissivity = "
    "500 m^2/d, storativity = 0.0002, distance = 100 m and time = 24 "
    "h."
)
print(a.outcome, a.values)
# SOLVED {'s': 2.51923946796, 'u': 0.001, 'W': 6.33153936414}

a = civil_engineering.ask(
    "Thick clay layer settlement with thickness = 10 m, top effective "
    "stress = 40 kPa, submerged unit weight = 8 kN/m^3, stress "
    "increase = 100 kPa, compression index = 0.45 and initial void "
    "ratio = 1.2."
)
print(a.outcome, a.values)
# SOLVED {'S': 0.753129074489, 'S_one_layer': 0.720373332501}

a = civil_engineering.ask(
    "Stress below a rectangular load with pressure = 200 kPa, width = "
    "2 m, length = 3 m and depth = 4 m."
)
print(a.outcome, a.values)
# SOLVED {'dsigma': 21414.5857952, 'I': 0.107072928976}

a = civil_engineering.ask(
    "Thiem confined transmissivity with pumping rate = 2500 m^3/d, "
    "first distance = 15 m, second distance = 60 m, first drawdown = "
    "2.8 m and second drawdown = 1.1 m."
)
print(a.outcome, a.values)
# SOLVED {'T': 0.00375537173462, 'T_m2_d': 324.464117872}

a = civil_engineering.ask(
    "Excess pore pressure during consolidation with initial excess "
    "pore pressure = 100 kPa, coefficient of consolidation = 2 m^2/yr, "
    "time = 1 yr, drainage path = 3 m and depth = 1.5 m."
)
print(a.outcome, a.values)
# SOLVED {'u': 52247.3336591, 'Uz': 0.477526663409, 'Tv': 0.222222222222}
```

## Screenshots

[![The Civil Engineering desktop app: Broad-crested weir discharge, with its worked example and live scene](https://www.ai999x.com/?ax_shot=civil-engineering-ai999x/hero-dark.webp)](https://www.ai999x.com/models/civil-engineering-ai999x/)

[![Primary consolidation settlement of a normally consolidated clay: the live scene in the Civil Engineering desktop app](https://www.ai999x.com/?ax_shot=civil-engineering-ai999x/see-1.webp)](https://www.ai999x.com/models/civil-engineering-ai999x/)

*Primary consolidation settlement of a normally consolidated clay*

[![Unit hydrograph from a storm by least-squares deconvolution: the live scene in the Civil Engineering desktop app](https://www.ai999x.com/?ax_shot=civil-engineering-ai999x/see-2.webp)](https://www.ai999x.com/models/civil-engineering-ai999x/)

*Unit hydrograph from a storm by least-squares deconvolution*

[![Coulomb active thrust on a rough wall with sloping backfill: the live scene in the Civil Engineering desktop app](https://www.ai999x.com/?ax_shot=civil-engineering-ai999x/see-3.webp)](https://www.ai999x.com/models/civil-engineering-ai999x/)

*Coulomb active thrust on a rough wall with sloping backfill*

Every calculation, with its formula, a worked example and its figures: **[Civil Engineering on ai999x.com](https://www.ai999x.com/models/civil-engineering-ai999x/)**. Install guides for the app and the command line: [https://www.ai999x.com/desktop-cli/](https://www.ai999x.com/desktop-cli/).

Version 1.6.4 · License: Apache-2.0 · Author: Ibnul Jaif Farabi · Organization: AI999X · Contact: support@ai999x.com · [Support the project](https://buymeacoffee.com/ibnuljaiffarabi)

*The Future Is Self-Improving.*

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/civil-engineering-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.4`; the [GitHub release](https://github.com/AI999X/civil-engineering-ai999x/releases/tag/v1.6.4) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `civil-engineering` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/civil-engineering`) and its data (`rust/models/civil_engineering/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p civil-engineering-ai999x
./target/release/civil-engineering --help
cargo test -p civil-engineering-ai999x
```
