"""civil-engineering-ai999x — offline civil engineering answers with their formula, steps and verification.

    import civil_engineering_ai999x as m
    m.ask("Manning discharge in a rectangular channel with b = 3 m, y = 1.2 m, S = 0.001 and n = 0.013.").show()
    # Q = 6.6829 m^3/s, V = 1.85636 m/s, A = 3.6 m^2, R = 0.666667 m
    m.solve("consolidation_settlement_nc", Cc=0.32, e0=0.92, H="4 m", s0="80 kPa", ds="60 kPa").show()
    # Sc = 0.162025 m, de = 0.0777722, e_final = 0.842228
    m.capabilities()                   # every certified capability (82)

What it covers, from school to graduate level:
- open-channel hydraulics: rectangular, trapezoidal and part-full circular sections, Manning and Chezy flow, normal
  and critical depth, Froude number, specific energy and alternate depths, hydraulic jumps, gradually varied flow
  profile lengths (M1 ... S3), sharp-crested rectangular and V-notch weirs, broad-crested weirs, orifices;
- pipes and water networks: Hazen-Williams head loss and flow, flow division in parallel pipes, looped networks by
  the Hardy Cross method, the three-reservoir problem;
- hydrology: SCS curve-number runoff, unit-hydrograph convolution and derivation, Thiessen-polygon rainfall, risk and
  return period, Gumbel and log-Pearson III design floods, Weibull plotting positions, Nash-Sutcliffe and Kling-Gupta
  efficiencies, Horton and Green-Ampt infiltration, the phi-index, Muskingum flood routing;
- groundwater: Darcy flow across soil layers, falling-head permeability, Thiem (confined and unconfined) pumping
  tests, Theis drawdown, Cooper-Jacob analysis, well fields by superposition, Dupuit flow with recharge;
- soil mechanics and foundations: phase relations, relative density, liquid limit, grading coefficients, effective
  stress, consolidation settlement (normally and overconsolidated, thick layers), degree and time of consolidation,
  coefficient of consolidation, excess pore pressure, stresses under circular, rectangular and strip loads, strip
  footing bearing capacity, Rankine and Coulomb earth pressure, Mohr-Coulomb strength and test parameters, Proctor
  compaction;
- surveying: closed traverses with the Bowditch adjustment, rise-and-fall levelling, horizontal and vertical curves,
  earthwork volumes and areas from offsets.

Questions take one documented form: the capability's name, then "with name = value unit, ..." (lists in brackets;
each list's unit is stated in its meaning). Inputs are SI in solve(); angles are degrees; g = 9.81 m/s^2 and the
unit weight of water 9.81 kN/m^3. Every capability was certified against an independent computation at build time
(fluids, WNTR, hydroeval, shapely, groundhog, SciPy, NumPy, SymPy); those libraries are not needed to use the package.
A question that is not determined is refused rather than guessed.

Creator / engineer: Ibnul Jaif Farabi. Company: AI999X.
"""
from __future__ import annotations

from pathlib import Path

try:                                       # the installed package carries its own copy of the engine
    from ._engine import Answer
    from ._engine.product import Product
except ImportError:                        # the factory's source tree, during development
    from ai999x_factory.runtime import Answer
    from ai999x_factory.runtime.product import Product

__version__ = "1.0.2"
_P = Product("civil_engineering", [Path(__file__).resolve().parent / "pack", Path(__file__).resolve().parents[3]])
pack, ask, solve, compile_problem = _P.pack, _P.ask, _P.solve, _P.compile_problem
formula, formulas, quantities, conditions, capabilities = _P.formula, _P.formulas, _P.quantities, _P.conditions, _P.capabilities
examples, family_examples, structured_example, to_si = _P.examples, _P.family_examples, _P.structured_example, _P.to_si
templates, procedure_examples = _P.templates, _P.procedure_examples
prove, derivations, about = _P.prove, _P.derivations, (lambda: {**_P.about(), "version": __version__})
knowledge, lookup, related = _P.knowledge, _P.lookup, _P.related
plot, plot_relation, search = _P.plot, _P.plot_relation, _P.search   # figures (SVG) and capability search ([search])
__all__ = ["ask", "solve", "prove", "derivations", "about", "formula", "formulas", "quantities", "conditions", "capabilities",
           "compile_problem", "pack", "Answer", "plot", "plot_relation", "search", "__version__"]
