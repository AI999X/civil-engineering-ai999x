# Civil Engineering (civil-engineering-ai999x)

[![tests](https://github.com/AI999X/civil-engineering-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/civil-engineering-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/civil-engineering-ai999x)](https://pypi.org/project/civil-engineering-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/civil-engineering-ai999x)

Civil Engineering covers pipe and open-channel hydraulics, hydrology, groundwater flow, soil mechanics and foundations, water networks and surveying: 88 calculations in 7 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Civil Engineering desktop app: Manning discharge in a rectangular channel, with its worked example and live scene](https://www.ai999x.com/?ax_shot=civil-engineering-ai999x/hero.webp)](https://www.ai999x.com/models/civil-engineering-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-civil-engineering-1.6.5-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-civil-engineering-1.6.5-linux-x86_64.tar.gz) |
| **Command line** `civil-engineering` (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-linux-x86_64.tar.gz) |
| **Browser** (any device) | [Try it on the model page](https://www.ai999x.com/models/civil-engineering-ai999x/#capabilities) |
| **Python** 3.10+ | `pip install civil-engineering-ai999x`: on Windows and Linux (x86-64) `pip` installs the same compiled engine; on other systems (macOS, ARM) it installs a pure-Python build that gives the same answers |

## See it

From the Civil Engineering desktop app: every scene and chart is drawn from the computed answer.

[![Civil Engineering desktop app: a slider moves an input and the answer, the scene and the charts follow](https://www.ai999x.com/?ax_shot=civil-engineering-ai999x/clip-loop.webp)](https://www.ai999x.com/?ax_shot=civil-engineering-ai999x/clip.mp4)

*[Watch the full clip (MP4)](https://www.ai999x.com/?ax_shot=civil-engineering-ai999x/clip.mp4)*

| | |
|---|---|
| [![Manning discharge in a rectangular channel: the live scene in the Civil Engineering desktop app](https://www.ai999x.com/?ax_shot=civil-engineering-ai999x/scene-1.webp)](https://www.ai999x.com/models/civil-engineering-ai999x/#cap-manning_rectangular_discharge)<br>Manning discharge in a rectangular channel | [![Theis drawdown around a pumped well: the live scene in the Civil Engineering desktop app](https://www.ai999x.com/?ax_shot=civil-engineering-ai999x/scene-2.webp)](https://www.ai999x.com/models/civil-engineering-ai999x/#cap-theis_drawdown)<br>Theis drawdown around a pumped well |
| [![Primary consolidation settlement of a normally consolidated clay: the live scene in the Civil Engineering desktop app](https://www.ai999x.com/?ax_shot=civil-engineering-ai999x/scene-3.webp)](https://www.ai999x.com/models/civil-engineering-ai999x/#cap-consolidation_settlement_nc)<br>Primary consolidation settlement of a normally consolidated clay | [![Pipe network flows by the Hardy Cross method (Hazen-Williams): the live scene in the Civil Engineering desktop app](https://www.ai999x.com/?ax_shot=civil-engineering-ai999x/scene-4.webp)](https://www.ai999x.com/models/civil-engineering-ai999x/#cap-hardy_cross_network)<br>Pipe network flows by the Hardy Cross method (Hazen-Williams) |
| [![Elements of a simple horizontal circular curve: the live scene in the Civil Engineering desktop app](https://www.ai999x.com/?ax_shot=civil-engineering-ai999x/scene-5.webp)](https://www.ai999x.com/models/civil-engineering-ai999x/#cap-horizontal_circular_curve)<br>Elements of a simple horizontal circular curve | [![Coulomb active thrust on a rough wall with sloping backfill: the live scene in the Civil Engineering desktop app](https://www.ai999x.com/?ax_shot=civil-engineering-ai999x/scene-6.webp)](https://www.ai999x.com/models/civil-engineering-ai999x/#cap-coulomb_active_thrust)<br>Coulomb active thrust on a rough wall with sloping backfill |
| [![Return period for an acceptable risk over a design life: the live scene in the Civil Engineering desktop app](https://www.ai999x.com/?ax_shot=civil-engineering-ai999x/scene-7.webp)](https://www.ai999x.com/models/civil-engineering-ai999x/#cap-design_return_period_for_risk)<br>Return period for an acceptable risk over a design life | [![Direct runoff hydrograph by unit-hydrograph convolution: the live scene in the Civil Engineering desktop app](https://www.ai999x.com/?ax_shot=civil-engineering-ai999x/scene-8.webp)](https://www.ai999x.com/models/civil-engineering-ai999x/#cap-unit_hydrograph_convolution)<br>Direct runoff hydrograph by unit-hydrograph convolution |

Charts:

|   |   |   |   |
|---|---|---|---|
| [![Manning discharge in a rectangular channel: a chart in the Civil Engineering desktop app](https://www.ai999x.com/?ax_shot=civil-engineering-ai999x/chart-1.webp)](https://www.ai999x.com/models/civil-engineering-ai999x/#cap-manning_rectangular_discharge)<br>Manning discharge in a rectangular channel | [![Theis drawdown around a pumped well: a chart in the Civil Engineering desktop app](https://www.ai999x.com/?ax_shot=civil-engineering-ai999x/chart-2.webp)](https://www.ai999x.com/models/civil-engineering-ai999x/#cap-theis_drawdown)<br>Theis drawdown around a pumped well | [![Primary consolidation settlement of a normally consolidated clay: a chart in the Civil Engineering desktop app](https://www.ai999x.com/?ax_shot=civil-engineering-ai999x/chart-3.webp)](https://www.ai999x.com/models/civil-engineering-ai999x/#cap-consolidation_settlement_nc)<br>Primary consolidation settlement of a normally consolidated clay | [![Pipe network flows by the Hardy Cross method (Hazen-Williams): a chart in the Civil Engineering desktop app](https://www.ai999x.com/?ax_shot=civil-engineering-ai999x/chart-4.webp)](https://www.ai999x.com/models/civil-engineering-ai999x/#cap-hardy_cross_network)<br>Pipe network flows by the Hardy Cross method (Hazen-Williams) |

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

## Command line: 15 examples

Each command below was run with `civil-engineering` of this release; the lines under it are the answer it printed (the full answer goes on with the method, every input in SI units and the conditions it assumes).

```text
$ civil-engineering solve coulomb_active_thrust phi=30 delta=20 beta=10 "gam=18000 N/m^3" "H=6 m"
Ka = 0.3400223954
Pa = 110167.2561 N/m
Pa_h = 103523.3576 N/m
Pa_v = 37679.42072 N/m

$ civil-engineering solve darcy_flow_layered_soil "H=[2]" "K=[1e-05]" "A=10 m^2" "dh=0.5 m"
Q = 2.5e-05 m^3/s
q = 2.5e-06 m/s
Kv = 1e-05 m/s
Kh = 1e-05 m/s
heads = [0.0] m

$ civil-engineering solve thiessen_polygon_rainfall "gx=[2, 8, 5]" "gy=[2, 3, 9]" "rain=[45, 60, 38]" "bx=[0, 10, 10, 0]" "by=[0, 0, 10, 10]"
mean = 47.23995192
areas = [29.6650641026,32.5608974359,37.7740384615]
weights = [0.296650641026,0.325608974359,0.377740384615]
arithmetic_mean = 47.66666667
catchment_area = 100

$ civil-engineering solve broad_crested_weir_discharge Cd=0.95 "b=5 m" "H=0.6 m"
Q = 3.763726876 m^3/s
yc = 0.4 m

$ civil-engineering solve parallel_pipes_hazen_williams "Q=0.3 m^3/s" "L=[800, 1200]" "D=[300, 400]" "C=[120, 110]"
hf = 7.944067369 m
flows = [0.116758581689,0.183241418311] m^3/s

$ civil-engineering solve degree_of_consolidation "cv=7.922021954e-08 m^2/s" "t=31557600 s" "Hdr=3 m"
Tv = 0.2777777778
U = 0.5913702173
U_percent = 59.13702173

$ civil-engineering solve area_from_offsets "offsets=[0, 2.5, 3.5, 5, 4.6, 3.2, 0]" "d=10 m"
A_trapezoidal = 188 m^2
A_simpson = 196.6666667 m^2

$ civil-engineering solve coulomb_passive_thrust phi=30 delta=15 beta=0 "gam=18000 N/m^3" "H=2 m"
Kp = 4.976500223
Pp = 179154.008 N/m
Pp_h = 173049.4832 N/m
Pp_v = 46368.46928 N/m

$ civil-engineering solve well_field_drawdown "wx=[0, 200]" "wy=[0, 0]" "Qs=[2000, 1500]" "T=0.00462962963 m^2/s" S=0.0003 "t=432000 s" "x0=100 m" "y0=50 m"
s = 4.935878027 m
parts = [2.82050172954,2.11537629715] m

$ civil-engineering solve unit_hydrograph_convolution "excess=[1.5, 2.5, 1]" "uh=[0, 10, 30, 22, 12, 5, 0]"
DRO = [0.0,15.0,70.0,118.0,103.0,59.5,24.5,5.0,0.0] m^3/s
total = [0.0,15.0,70.0,118.0,103.0,59.5,24.5,5.0,0.0] m^3/s
peak = 118 m^3/s
peak_period = 4

$ civil-engineering solve chezy_from_manning n=0.013 "R=0.75 m"
C = 73.32186869 m^0.5/s

$ civil-engineering solve hazen_williams_head_loss "Q=0.05 m^3/s" "D=0.3 m" "L=1000 m" C=120
hf = 2.064555214 m
V = 0.7073553026 m/s
Sf = 0.002064555214

$ civil-engineering solve coefficient_of_consolidation "Hdr=0.01 m" "t=750 s" U=0.5
cv = 2.623076527e-08 m^2/s
Tv = 0.1967307395
cv_m2_yr = 0.8277799981

$ civil-engineering solve traverse_bowditch_adjustment "az=[45, 135.01, 225, 315]" "d=[100, 100.02, 99.98, 100]"
misclosure_E = 0.01593938435 m
misclosure_N = -0.0123427325 m
linear_misclosure = 0.02015953915 m
precision = 19841.72341
E = [0.0,70.7066932726,141.415182997,70.7146629647] m
N = [0.0,70.7137638018,-0.020312884737,-70.7137638018] m

$ civil-engineering solve rankine_active_thrust phi=30 "gam=18000 N/m^3" "H=6 m"
Ka = 0.3333333333
sigma_base = 36000 Pa
Pa = 108000 N/m
z_crack = 0 m
ybar = 2 m
```

## Rust library: 15 examples

The crate `civil-engineering-ai999x` (in `rust/` of the GitHub repository) answers a JSON request with `civil_engineering_ai999x::solve_json`. Each call below was compiled and run on this release; the comment under it is what it printed. The whole program is `rust/civil-engineering/examples/readme.rs`: `cd rust && cargo run --release --example readme -p civil-engineering-ai999x`.

```rust
use serde_json::{json, Value};

fn show(request: Value) {
    let answer: Value = serde_json::from_str(&civil_engineering_ai999x::solve_json(&request.to_string())).unwrap();
    println!("{}", answer["outcome"].as_str().unwrap_or(""));
    for v in answer["values"].as_array().into_iter().flatten() {
        let unit = v[2].as_str().unwrap_or("");
        let unit = if unit.is_empty() || unit == "1" { String::new() } else { format!(" {unit}") };
        println!("{} = {}{}", v[0].as_str().unwrap_or(""), v[1], unit);
    }
}

show(json!({"key": "coulomb_active_thrust", "inputs": {"phi": "30", "delta": "20", "beta": "10", "gam": "18000 N/m^3", "H": "6 m"}}));
// SOLVED
// Ka = 0.340022395359
// Pa = 110167.256096 N/m
// Pa_h = 103523.357606 N/m
// Pa_v = 37679.4207198 N/m

show(json!({"key": "darcy_flow_layered_soil", "inputs": {"H": [2], "K": [1e-05], "A": "10 m^2", "dh": "0.5 m"}}));
// SOLVED
// Q = 0.000025 m^3/s
// q = 2.5e-6 m/s
// Kv = 0.00001 m/s
// Kh = 0.00001 m/s
// heads = [0.0] m

show(json!({"key": "thiessen_polygon_rainfall", "inputs": {"gx": [2, 8, 5], "gy": [2, 3, 9], "rain": [45, 60, 38], "bx": [0, 10, 10, 0], "by": [0, 0, 10, 10]}}));
// SOLVED
// mean = 47.2399519231
// areas = [29.6650641026,32.5608974359,37.7740384615]
// weights = [0.296650641026,0.325608974359,0.377740384615]
// arithmetic_mean = 47.6666666667
// catchment_area = 100.0

show(json!({"key": "broad_crested_weir_discharge", "inputs": {"Cd": "0.95", "b": "5 m", "H": "0.6 m"}}));
// SOLVED
// Q = 3.76372687638 m^3/s
// yc = 0.4 m

show(json!({"key": "parallel_pipes_hazen_williams", "inputs": {"Q": "0.3 m^3/s", "L": [800, 1200], "D": [300, 400], "C": [120, 110]}}));
// SOLVED
// hf = 7.94406736932 m
// flows = [0.116758581689,0.183241418311] m^3/s

show(json!({"key": "degree_of_consolidation", "inputs": {"cv": "7.922021954e-08 m^2/s", "t": "31557600 s", "Hdr": "3 m"}}));
// SOLVED
// Tv = 0.277777777795
// U = 0.591370217287
// U_percent = 59.1370217287

show(json!({"key": "area_from_offsets", "inputs": {"offsets": [0, 2.5, 3.5, 5, 4.6, 3.2, 0], "d": "10 m"}}));
// SOLVED
// A_trapezoidal = 188.0 m^2
// A_simpson = 196.666666667 m^2

show(json!({"key": "coulomb_passive_thrust", "inputs": {"phi": "30", "delta": "15", "beta": "0", "gam": "18000 N/m^3", "H": "2 m"}}));
// SOLVED
// Kp = 4.97650022273
// Pp = 179154.008018 N/m
// Pp_h = 173049.483228 N/m
// Pp_v = 46368.4692815 N/m

show(json!({"key": "well_field_drawdown", "inputs": {"wx": [0, 200], "wy": [0, 0], "Qs": [2000, 1500], "T": "0.00462962963 m^2/s", "S": "0.0003", "t": "432000 s", "x0": "100 m", "y0": "50 m"}}));
// SOLVED
// s = 4.93587802669 m
// parts = [2.82050172954,2.11537629715] m

show(json!({"key": "unit_hydrograph_convolution", "inputs": {"excess": [1.5, 2.5, 1], "uh": [0, 10, 30, 22, 12, 5, 0]}}));
// SOLVED
// DRO = [0.0,15.0,70.0,118.0,103.0,59.5,24.5,5.0,0.0] m^3/s
// total = [0.0,15.0,70.0,118.0,103.0,59.5,24.5,5.0,0.0] m^3/s
// peak = 118.0 m^3/s
// peak_period = 4

show(json!({"key": "chezy_from_manning", "inputs": {"n": "0.013", "R": "0.75 m"}}));
// SOLVED
// C = 73.3218686921 m^0.5/s

show(json!({"key": "hazen_williams_head_loss", "inputs": {"Q": "0.05 m^3/s", "D": "0.3 m", "L": "1000 m", "C": "120"}}));
// SOLVED
// hf = 2.06455521352 m
// V = 0.707355302631 m/s
// Sf = 0.00206455521352

show(json!({"key": "coefficient_of_consolidation", "inputs": {"Hdr": "0.01 m", "t": "750 s", "U": "0.5"}}));
// SOLVED
// cv = 2.62307652698e-8 m^2/s
// Tv = 0.196730739524
// cv_m2_yr = 0.827779998079

show(json!({"key": "traverse_bowditch_adjustment", "inputs": {"az": [45, 135.01, 225, 315], "d": [100, 100.02, 99.98, 100]}}));
// SOLVED
// misclosure_E = 0.0159393843463 m
// misclosure_N = -0.0123427324999 m
// linear_misclosure = 0.0201595391541 m
// precision = 19841.7234116
// E = [0.0,70.7066932726,141.415182997,70.7146629647] m
// N = [0.0,70.7137638018,-0.020312884737,-70.7137638018] m

show(json!({"key": "rankine_active_thrust", "inputs": {"phi": "30", "gam": "18000 N/m^3", "H": "6 m"}}));
// SOLVED
// Ka = 0.333333333333
// sigma_base = 36000.0 Pa
// Pa = 108000.0 N/m
// z_crack = 0.0 m
// ybar = 2.0 m
```

## Guides

- Desktop app guide: the window, every calculation, scenes and charts: [PDF](https://huggingface.co/AI999X/civil-engineering-ai999x/blob/main/docs/civil_engineering_ai999x_desktop_guide.pdf) · [HTML](https://huggingface.co/AI999X/civil-engineering-ai999x/blob/main/docs/civil_engineering_ai999x_desktop_guide.html) · [Markdown](https://huggingface.co/AI999X/civil-engineering-ai999x/blob/main/docs/civil_engineering_ai999x_desktop_guide.md)
- Python guide: every calculation with a Python example: [PDF](https://huggingface.co/AI999X/civil-engineering-ai999x/blob/main/docs/civil_engineering_ai999x_capabilities.pdf) · [HTML](https://huggingface.co/AI999X/civil-engineering-ai999x/blob/main/docs/civil_engineering_ai999x_capabilities.html) · [Markdown](https://huggingface.co/AI999X/civil-engineering-ai999x/blob/main/docs/civil_engineering_ai999x_capabilities.md)
- Command-line guide: every calculation with the command and what it printed: [PDF](https://huggingface.co/AI999X/civil-engineering-ai999x/blob/main/docs/civil_engineering_ai999x_rust_guide.pdf) · [HTML](https://huggingface.co/AI999X/civil-engineering-ai999x/blob/main/docs/civil_engineering_ai999x_rust_guide.html) · [Markdown](https://huggingface.co/AI999X/civil-engineering-ai999x/blob/main/docs/civil_engineering_ai999x_rust_guide.md)

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
