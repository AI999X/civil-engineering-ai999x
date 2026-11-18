# Civil Engineering: the desktop app, user guide

**Civil Engineering**, desktop app 1.6.5 for Windows and Linux. **Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X · support@ai999x.com

Civil Engineering covers pipe and open-channel hydraulics, hydrology, groundwater flow, soil mechanics and foundations, water networks and surveying. Its desktop app holds all 88 calculations in 7 topics: pick one, change its inputs and the answer, a live scene and charts follow at once. It runs on your own computer with no network connection and nothing else to install. The pictures in this guide were taken in this app.

## Contents

- 1. Get it and start it
- 2. The window
- 3. Compute a calculation
- 4. Ask in words
- 5. The live scene and the charts
- 6. Light and dark
- 7. Every calculation
- 8. Other ways to use it

## 1. Get it and start it

| system | download |
|---|---|
| Windows x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-civil-engineering-1.6.5-windows-x86_64.zip |
| Linux x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-civil-engineering-1.6.5-linux-x86_64.tar.gz |

Windows: Download ai999x-civil-engineering-1.6.5-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x-civil-engineering.exe. If Windows SmartScreen asks, choose More info → Run anyway.

Linux:

```shell
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-civil-engineering-1.6.5-linux-x86_64.tar.gz
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-civil-engineering-1.6.5-linux-x86_64.tar.gz
./ai999x-civil-engineering-1.6.5-linux-x86_64/ai999x-civil-engineering
```

Checksums of every download: https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS. Install steps for every app: https://www.ai999x.com/desktop-cli/

## 2. The window

![The Civil Engineering app, opened on Manning discharge in a rectangular channel with its worked example.](https://www.ai999x.com/?ax_shot=civil-engineering-ai999x/hero.webp)

*The Civil Engineering app, opened on Manning discharge in a rectangular channel with its worked example.*

On the left, a search box and the topics with their calculations. In the middle, the chosen calculation: its answer at the top, then the live scene (or its charts), what the answer means and the formula. On the right, its inputs, each with a slider where that makes sense, and the Compute and Reset buttons. At the bottom, a box for a question in words.

## 3. Compute a calculation

Open Open-channel hydraulics › Manning discharge in a rectangular channel (or type part of its name in the search box). It opens with a worked example filled in: “Manning discharge in a rectangular channel with width = 3 m, depth = 1.2 m, slope = 0.001 and Manning n = 0.013.” The answer: Q = 6.682900203 m^3/s, V = 1.856361167 m/s, A = 3.6 m^2, R = 0.6666666667 m.

Change any input (type a number with its unit, or drag its slider) and the answer, the scene and the charts follow; Compute recomputes after typing, Reset brings back the worked example. An input outside the range the formula holds for is refused with the reason, never answered with a guessed number.

![Manning discharge in a rectangular channel: the live scene of the worked example.](https://www.ai999x.com/?ax_shot=civil-engineering-ai999x/scene-1.webp)

*Manning discharge in a rectangular channel: the live scene of the worked example.*

## 4. Ask in words

Type a question as you would say it, for example “Manning discharge in a rectangular channel with b = 3 m, y = 1.2 m, S = 0.001 and n = 0.013.” The app reads the quantities and their units, picks the calculation and answers; the command-line program reads it the same way and printed:

```text
Read as: Manning discharge in a rectangular channel
Q = 6.682900203 m^3/s
V = 1.856361167 m/s
```

When a question names a quantity the app cannot place, it says what is missing instead of guessing.

## 5. The live scene and the charts

Every calculation draws a scene from its answer: it moves as an input changes. The Charts button shows charts that vary one input around your values; Enlarge opens one in a large view. Scenes from other topics:

![Theis drawdown around a pumped well](https://www.ai999x.com/?ax_shot=civil-engineering-ai999x/scene-2.webp)

*Theis drawdown around a pumped well*

![Primary consolidation settlement of a normally consolidated clay](https://www.ai999x.com/?ax_shot=civil-engineering-ai999x/scene-3.webp)

*Primary consolidation settlement of a normally consolidated clay*

![Pipe network flows by the Hardy Cross method (Hazen-Williams)](https://www.ai999x.com/?ax_shot=civil-engineering-ai999x/scene-4.webp)

*Pipe network flows by the Hardy Cross method (Hazen-Williams)*

![Elements of a simple horizontal circular curve](https://www.ai999x.com/?ax_shot=civil-engineering-ai999x/scene-5.webp)

*Elements of a simple horizontal circular curve*

![Coulomb active thrust on a rough wall with sloping backfill](https://www.ai999x.com/?ax_shot=civil-engineering-ai999x/scene-6.webp)

*Coulomb active thrust on a rough wall with sloping backfill*

![Return period for an acceptable risk over a design life](https://www.ai999x.com/?ax_shot=civil-engineering-ai999x/scene-7.webp)

*Return period for an acceptable risk over a design life*

![Direct runoff hydrograph by unit-hydrograph convolution](https://www.ai999x.com/?ax_shot=civil-engineering-ai999x/scene-8.webp)

*Direct runoff hydrograph by unit-hydrograph convolution*

![Manning discharge in a rectangular channel: a chart, enlarged.](https://www.ai999x.com/?ax_shot=civil-engineering-ai999x/chart-1.webp)

*Manning discharge in a rectangular channel: a chart, enlarged.*

![Theis drawdown around a pumped well: a chart, enlarged.](https://www.ai999x.com/?ax_shot=civil-engineering-ai999x/chart-2.webp)

*Theis drawdown around a pumped well: a chart, enlarged.*

## 6. Light and dark

The app follows a light or a dark theme (the switch is in its top bar).

![The Civil Engineering app in the dark theme.](https://www.ai999x.com/?ax_shot=civil-engineering-ai999x/hero-dark.webp)

*The Civil Engineering app in the dark theme.*

## 7. Every calculation

88 calculations in 7 topics; each has its own entry on the model's page (https://www.ai999x.com/models/civil-engineering-ai999x/) with its formula, worked example and charts.

| topic | calculations |
|---|---|
| Foundations | Coulomb active thrust on a rough wall with sloping backfill, Coulomb passive thrust on a rough wall, Rankine active earth pressure and thrust on a wall, Rankine passive earth pressure and thrust, Terzaghi bearing capacity with stated factors, Ultimate bearing capacity of a strip footing, Vertical stress below a uniformly loaded rectangle (Newmark), Vertical stress below a uniformly loaded strip, Vertical stress below the centre of a uniformly loaded circle |
| Groundwater | Darcy flow across soil layers in series; equivalent conductivities, Drawdown from several wells by superposition (Theis), Hydraulic conductivity from a falling-head test, Hydraulic conductivity from steady pumping (unconfined aquifer), Theis drawdown around a pumped well, Transmissivity and storativity by the Cooper-Jacob straight line, Transmissivity from steady pumping (Thiem, confined aquifer), Unconfined flow between two water bodies with recharge (Dupuit) |
| Hydrology | Areal rainfall by Thiessen polygons, Direct runoff hydrograph by unit-hydrograph convolution, Green-Ampt cumulative infiltration under ponding, Gumbel (EV1) design flood, Horton infiltration rate and cumulative infiltration, Kling-Gupta efficiency, Log-Pearson type III design flood, Muskingum flood routing, Nash-Sutcliffe efficiency, RMSE and percent bias, Peak runoff by the rational method, Phi-index of a storm, Return period for an acceptable risk over a design life, Return periods by the Weibull plotting position, Risk of exceedance over a design life, SCS curve-number runoff depth, Time of concentration by the Kirpich formula, Unit hydrograph from a storm by least-squares deconvolution |
| Open-channel hydraulics | Broad-crested weir discharge, Chezy coefficient from Manning's n, Chezy velocity, Critical depth in a circular conduit, Critical depth in a rectangular channel, Critical depth in a trapezoidal channel, Discharge through a small orifice under a head, Froude number of flow in a rectangular channel, Hydraulic jump in a rectangular channel, Length of a gradually varied flow profile (rectangular channel), Manning discharge in a rectangular channel, Manning discharge in a trapezoidal channel, Manning flow in a part-full circular pipe, Manning velocity from hydraulic radius, Manning's n from a Chezy coefficient, Normal depth in a rectangular channel, Normal depth in a trapezoidal channel, Part-full circular conduit: area, wetted perimeter, hydraulic radius, Rectangular channel: area, wetted perimeter, hydraulic radius, Sharp-crested rectangular weir discharge, Specific energy and alternate depth in a rectangular channel, Trapezoidal channel: area, wetted perimeter, hydraulic radius, V-notch (triangular) weir discharge |
| Pipe flow and water networks | Flow division among parallel pipes (Hazen-Williams), Hazen-Williams head loss in a pipe, Pipe flow from a head loss (Hazen-Williams), Pipe network flows by the Hardy Cross method (Hazen-Williams), Three-reservoir problem (Hazen-Williams) |
| Soil mechanics | Average degree of consolidation after a time (Terzaghi), Coefficient of consolidation from a consolidation time, Cohesion and friction angle from direct shear tests, Cohesion and friction angle from triaxial tests, Consolidation settlement of a thick clay layer integrated over depth, Effective sizes and gradation coefficients from a sieve analysis, Excess pore pressure at a depth during consolidation, Factor of safety of an infinite slope, Liquid limit from Casagrande cup tests, Major principal stress at failure (Mohr-Coulomb), Optimum water content and maximum dry density from a compaction test, Plasticity, liquidity and consistency indices, Primary consolidation settlement of a normally consolidated clay, Primary consolidation settlement of an overconsolidated clay, Relative density of a granular soil, Secondary compression (creep) settlement of a clay layer, Soil phase relations from void ratio and water content, Soil properties from a weighed sample, Time to reach a degree of consolidation, Total, pore-water and effective vertical stress at a depth |
| Surveying | Area between a survey line and a boundary from offsets, Closed traverse: misclosure and Bowditch (compass rule) adjustment, Earthwork volume from cross-section areas, Elements of a simple horizontal circular curve, Elevations on a vertical parabolic curve, Reduced levels by the rise-and-fall method |

## 8. Other ways to use it

The same engine runs on the command line (`civil-engineering`, its own guide), in Python (`pip install civil-engineering-ai999x`, `import civil_engineering_ai999x`, its own guide) and in the browser on https://www.ai999x.com/models/civil-engineering-ai999x/.

Licence: Apache-2.0 · AI999X · https://www.ai999x.com/models/civil-engineering-ai999x/
