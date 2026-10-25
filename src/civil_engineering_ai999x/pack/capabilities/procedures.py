"""Civil engineering procedures: open-channel hydraulics, pipe networks, hydrology, groundwater flow, soil mechanics
and foundations, and surveying (domain code, loaded by the pack; enabled only once tools/certify_procedures.py has
certified them).

Every capability states its inputs, outputs, conditions, method and LaTeX, the documented question form it reads,
worked examples (their outputs were computed by the reference, not by the procedure), an INDEPENDENT reference
computation used only by certification (the checking libraries are imported inside the reference functions: fluids,
WNTR, hydroeval, shapely, groundhog, SciPy, NumPy, SymPy) and a deliberately wrong variant that the checks must reject.
The shipped code (run, verify, parse) is plain Python (math, re, mpmath).

Conventions (stated once, used everywhere):
- SI inside: m, m^2, m^3/s, s, Pa, N/m^3. The question reader also accepts the usual spellings and converts them (mm,
  cm, km, ft, in; ha, km^2, acre; L/s, m^3/h, m^3/d, cfs; kPa, MPa, psi, psf; kN/m^3, pcf; cm/s, m/d, ft/d; mm/h, in/h;
  min, h, day, yr). A year is the Julian year (365.25 days). solve() takes SI or any unit of the shared reader.
- Gravity g = 9.81 m/s^2 and the unit weight of water 9.81 kN/m^3 (water density 1000 kg/m^3), as in most civil
  engineering texts.
- Manning's equation in SI form V = (1/n) R^(2/3) S^(1/2) (n in s/m^(1/3)); Chezy V = C (R S)^(1/2) (C in m^(1/2)/s).
  Side slopes z of trapezoidal channels are horizontal : vertical (z = 2 means 2H:1V). Channel slopes, gradients and
  fractions are plain numbers (m/m); "0.1%" is read as 0.001.
- Hazen-Williams head loss as EPANET and WNTR write it in SI: h_f = 10.666829500036352 L Q^1.852 / (C^1.852 D^4.871).
- Angles (notch angles, friction angles, bearings, deflection angles, slope angles) are plain numbers in DEGREES.
- Lists carry no unit; each list input states its unit in its meaning (for example lengths in m, rainfall in mm,
  stresses in kPa, flows in m^3/s).
- Outputs built from exp/log/trig/powers are rounded to 12 significant digits so that every platform prints the same
  digits; values that are zero by balance (below 1e-12 of their scale) are written as exactly 0.

Question form (one per procedure): the capability phrase, "with" (or a colon), then `name = value` pairs separated by
commas or "and" (`Manning discharge in a rectangular channel with b = 3 m, y = 1.2 m, S = 0.001 and n = 0.013`);
physical values carry a unit; lists are written in brackets (`depths = [1.2, 0.8, 2.5]`). Any other number or an
unknown name is refused, never guessed.
"""
from __future__ import annotations

import math
import re

# =============================================================================================== constants
G = 9.81                      # gravity, m/s^2 (civil engineering convention)
RHO_W = 1000.0                # water density, kg/m^3
GAMMA_W = RHO_W * G           # unit weight of water, N/m^3 (9.81 kN/m^3)
HW_K = 10.666829500036352     # Hazen-Williams SI coefficient (EPANET / WNTR)
YEAR = 3.15576e7              # Julian year, s
EULER_GAMMA = 0.5772156649015329


def _r(x):
    """12 significant digits, never a negative zero (lists element by element)."""
    if isinstance(x, (bool, str)):
        return x
    if isinstance(x, (list, tuple)):
        return [_r(v) for v in x]
    x = float(x)
    if x == 0.0 or not math.isfinite(x):
        return 0.0 if x == 0.0 else x
    return float(f"{x:.12g}") + 0.0


def _snap(x, scale):
    return 0.0 if abs(x) <= 1e-12 * abs(scale) else x


def _rel(a, b):
    if isinstance(a, (list, tuple)) or isinstance(b, (list, tuple)):
        if not (isinstance(a, (list, tuple)) and isinstance(b, (list, tuple))) or len(a) != len(b):
            return 1.0
        return max([_rel(x, y) for x, y in zip(a, b)] or [0.0])
    try:
        a, b = float(a), float(b)
    except (TypeError, ValueError):
        return 1.0
    if not (math.isfinite(a) and math.isfinite(b)):
        return 1.0
    d = abs(a - b)
    if d <= 1e-15:
        return 0.0
    return d / max(abs(a), abs(b))


def _need(cond, message):
    if not cond:
        raise ValueError(message)


def _finite(**kw):
    for k, v in kw.items():
        if isinstance(v, bool) or not isinstance(v, (int, float)) or not math.isfinite(v):
            raise ValueError(f"{k} must be a finite number")


def _pos(**kw):
    _finite(**kw)
    for k, v in kw.items():
        if v <= 0:
            raise ValueError(f"{k} must be positive")


def _vec(name, v, n=None, positive=False, nonneg=False):
    _need(isinstance(v, (list, tuple)) and len(v) > 0, f"{name} must be a list of numbers")
    for x in v:
        _need(isinstance(x, (int, float)) and not isinstance(x, bool) and math.isfinite(x), f"{name} must hold finite numbers")
        if positive:
            _need(x > 0, f"every entry of {name} must be positive")
        if nonneg:
            _need(x >= 0, f"every entry of {name} must be zero or positive")
    if n is not None:
        _need(len(v) == n, f"{name} must have {n} entries")
    return [float(x) for x in v]


def _bisect_root(f, a, b, iters=300):
    """Root of a continuous f with f(a), f(b) of opposite signs, to the last representable bit."""
    fa, fb = f(a), f(b)
    if fa == 0:
        return a
    if fb == 0:
        return b
    _need(fa * fb < 0, "no root in the admissible range")
    for _ in range(iters):
        m = 0.5 * (a + b)
        if m == a or m == b:
            break
        fm = f(m)
        if fm == 0:
            return m
        if (fm < 0) == (fa < 0):
            a, fa = m, fm
        else:
            b = m
    return 0.5 * (a + b)


# =============================================================================================== units of the reader
_NUM = r"[-+]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][-+]?\d+)?"
_FT = 0.3048
_IN = 0.0254
_UNITS = {
    "length": {"m": 1, "mm": 1e-3, "cm": 1e-2, "km": 1e3, "ft": _FT, "feet": _FT, "foot": _FT, "in": _IN, "inch": _IN,
               "inches": _IN, "metres": 1, "meters": 1, "metre": 1, "meter": 1},
    "area": {"m^2": 1, "m2": 1, "km^2": 1e6, "km2": 1e6, "ha": 1e4, "hectare": 1e4, "hectares": 1e4, "acre": 4046.8564224,
             "acres": 4046.8564224, "ft^2": _FT ** 2, "ft2": _FT ** 2, "cm^2": 1e-4, "cm2": 1e-4, "mm^2": 1e-6,
             "mm2": 1e-6, "mi^2": 2589988.110336, "sq mi": 2589988.110336},
    "volume": {"m^3": 1, "m3": 1, "L": 1e-3, "l": 1e-3, "litre": 1e-3, "liter": 1e-3, "litres": 1e-3, "liters": 1e-3,
               "ML": 1e3, "cm^3": 1e-6, "cm3": 1e-6, "ft^3": _FT ** 3, "ft3": _FT ** 3},
    "flow": {"m^3/s": 1, "m3/s": 1, "cumec": 1, "cumecs": 1, "L/s": 1e-3, "l/s": 1e-3, "m^3/h": 1 / 3600,
             "m3/h": 1 / 3600, "m^3/d": 1 / 86400, "m3/d": 1 / 86400, "m^3/day": 1 / 86400, "ML/d": 1e3 / 86400,
             "ML/day": 1e3 / 86400, "cfs": _FT ** 3, "ft^3/s": _FT ** 3, "ft3/s": _FT ** 3, "L/min": 1e-3 / 60},
    "speed": {"m/s": 1, "cm/s": 1e-2, "mm/s": 1e-3, "ft/s": _FT, "fps": _FT, "km/h": 1 / 3.6, "m/d": 1 / 86400,
              "m/day": 1 / 86400},
    "conductivity": {"m/s": 1, "cm/s": 1e-2, "mm/s": 1e-3, "m/d": 1 / 86400, "m/day": 1 / 86400, "cm/d": 1e-2 / 86400,
                     "cm/day": 1e-2 / 86400, "ft/d": _FT / 86400, "ft/day": _FT / 86400, "cm/h": 1e-2 / 3600,
                     "mm/h": 1e-3 / 3600, "m/yr": 1 / YEAR, "m/year": 1 / YEAR, "mm/d": 1e-3 / 86400,
                     "mm/day": 1e-3 / 86400, "mm/yr": 1e-3 / YEAR, "mm/year": 1e-3 / YEAR},
    "transmissivity": {"m^2/s": 1, "m2/s": 1, "m^2/d": 1 / 86400, "m2/d": 1 / 86400, "m^2/day": 1 / 86400,
                       "m2/day": 1 / 86400, "ft^2/d": _FT ** 2 / 86400, "ft^2/day": _FT ** 2 / 86400,
                       "cm^2/s": 1e-4, "m^2/yr": 1 / YEAR, "m2/yr": 1 / YEAR, "m^2/year": 1 / YEAR,
                       "mm^2/s": 1e-6, "mm^2/min": 1e-6 / 60, "cm^2/min": 1e-4 / 60},
    "intensity": {"m/s": 1, "mm/h": 1e-3 / 3600, "mm/hr": 1e-3 / 3600, "cm/h": 1e-2 / 3600, "cm/hr": 1e-2 / 3600,
                  "in/h": _IN / 3600, "in/hr": _IN / 3600, "mm/d": 1e-3 / 86400, "mm/day": 1e-3 / 86400,
                  "mm/min": 1e-3 / 60},
    "time": {"s": 1, "sec": 1, "min": 60, "minutes": 60, "h": 3600, "hr": 3600, "hrs": 3600, "hour": 3600,
             "hours": 3600, "d": 86400, "day": 86400, "days": 86400, "week": 604800, "weeks": 604800, "yr": YEAR,
             "year": YEAR, "years": YEAR, "month": YEAR / 12, "months": YEAR / 12},
    "stress": {"Pa": 1, "kPa": 1e3, "MPa": 1e6, "N/m^2": 1, "N/m2": 1, "kN/m^2": 1e3, "kN/m2": 1e3, "psi": 6894.757293168361,
               "psf": 47.88025898033584, "ksf": 47880.25898033584, "bar": 1e5, "atm": 101325, "t/m^2": 9806.65},
    "unit_weight": {"N/m^3": 1, "N/m3": 1, "kN/m^3": 1e3, "kN/m3": 1e3, "pcf": 157.08746061538, "lb/ft^3": 157.08746061538,
                    "lb/ft3": 157.08746061538},
    "density": {"kg/m^3": 1, "kg/m3": 1, "g/cm^3": 1e3, "g/cm3": 1e3, "Mg/m^3": 1e3, "Mg/m3": 1e3, "t/m^3": 1e3,
                "t/m3": 1e3},
    "mass": {"kg": 1, "g": 1e-3, "Mg": 1e3, "t": 1e3, "tonne": 1e3},
    "force": {"N": 1, "kN": 1e3, "MN": 1e6, "lbf": 4.4482216152605, "kip": 4448.2216152605},
    "power": {"W": 1, "kW": 1e3, "MW": 1e6},
    "number": {"": 1},
    "fraction": {"": 1, "%": 0.01, "percent": 0.01},
    "degrees": {"": 1, "°": 1, "deg": 1, "degree": 1, "degrees": 1},
    "mm": {"mm": 1, "cm": 10, "in": 25.4, "inch": 25.4, "inches": 25.4, "m": 1e3},
    "years": {"yr": 1, "yrs": 1, "year": 1, "years": 1},
    "chezy": {"m^0.5/s": 1, "m^(1/2)/s": 1, "": 1},
    "rate": {"1/s": 1, "/s": 1, "s^-1": 1, "1/min": 1 / 60, "/min": 1 / 60, "min^-1": 1 / 60, "1/h": 1 / 3600,
             "/h": 1 / 3600, "h^-1": 1 / 3600, "1/hr": 1 / 3600, "/hr": 1 / 3600, "hr^-1": 1 / 3600, "1/d": 1 / 86400,
             "/d": 1 / 86400, "1/day": 1 / 86400, "d^-1": 1 / 86400},
}


def _norm_unit(u: str) -> str:
    u = u.strip().replace("²", "^2").replace("³", "^3").replace("·", "*").replace("−", "-").replace("º", "°")
    if u.lower() in ("sq mi",):
        return "sq mi"
    return re.sub(r"\s+", "", u)


def _convert(value: float, unit: str, table: str):
    t = _UNITS[table]
    u = _norm_unit(unit)
    if u in t:
        return value * t[u]
    low = {k.lower(): v for k, v in t.items() if len(k) >= 3 and k not in ("ML", "ML/d", "ML/day", "Mg", "MPa", "MN", "MW")}
    if len(u) >= 3 and u.lower() in low:
        return value * low[u.lower()]
    return None


# =============================================================================================== the procedure registry
PROCEDURES: dict = {}
_ROUTES: list = []          # (phrase, key): a question is routed to the procedure whose phrase is the longest found
_SPEC: dict = {}            # key -> internal reading information


def _in(sym, kind, table, meaning, names=(), show="", required=True, typ=None, **extra):
    d = {"symbol": sym, "kind": kind, "meaning": meaning, "required": required}
    if typ:
        d["type"] = typ
    d.update(extra)
    return d, table, [sym] + list(names), show


def _o(sym, kind, meaning):
    return {"symbol": sym, "kind": kind, "meaning": meaning}


def _route(head: str):
    h = " " + re.sub(r"\s+", " ", head.lower().replace("-", " ")) + " "
    for phrase, key in _ROUTES:
        if re.search(r"(?<![a-z])" + re.escape(phrase.replace("-", " ")) + r"(?![a-z])", h):
            return key
    return None


def _show_value(v):
    if isinstance(v, (list, tuple)):
        return "[" + ", ".join(f"{x:g}" if isinstance(x, (int, float)) else str(x) for x in v) + "]"
    if isinstance(v, str):
        return v
    return f"{v:g}"


def _read(key, body):
    """'name = value unit, name = value unit and name = [a, b, c]' -> SI values; None for anything else."""
    info = _SPEC[key]
    body = body.strip().rstrip(".?! ")
    lists = []

    def keep(m):
        lists.append(m.group(1))
        return f"\x00{len(lists) - 1}\x00"
    body = re.sub(r"\[([^\[\]]*)\]", keep, body)
    items = [s for s in re.split(r"\s*(?:,|;|\band\b)\s*", body) if s.strip()]
    if not items:
        return None
    out = {}
    for item in items:
        m = re.fullmatch(r"([A-Za-z][\w ()'/\-]*?)\s*=\s*(.+)", item.strip())
        if not m:
            return None
        name, val = re.sub(r"\s+", " ", m.group(1).strip()), m.group(2).strip()
        sym = info["names"].get(name) or info["names"].get(name.lower())
        if sym is None or sym in out:
            return None
        spec, table = info["inputs"][sym]
        typ = spec.get("type", "number")
        lm = re.fullmatch("\x00(\\d+)\x00", val)
        if typ == "vector":
            if not lm:
                return None
            nums = [x for x in re.split(r"[\s,;]+", lists[int(lm.group(1))].strip()) if x]
            if not nums or not all(re.fullmatch(_NUM, x) for x in nums):
                return None
            out[sym] = ", ".join(nums)
            continue
        if typ in ("edges", "text"):
            if not lm:
                return None
            out[sym] = lists[int(lm.group(1))].strip()
            continue
        if lm:
            return None
        if typ in ("choice", "boolean"):
            out[sym] = val
            continue
        nm = re.match("(" + _NUM + r")\s*(.*)$", val)
        if not nm:
            return None
        number, unit = float(nm.group(1)), nm.group(2)
        si = _convert(number, unit, table)                 # an unknown unit (or a second number) is refused
        if si is None:
            return None
        if typ == "integer":
            if si != int(si):
                return None
            si = int(si)
        out[sym] = si
    return out


def _parser(key):
    def parse(question):
        if not isinstance(question, str):
            return None
        m = re.search(r"\s+with\s+|\s*:\s+", question)
        if not m:
            return None
        head, body = question[:m.start()], question[m.end():]
        if re.search(r"\d", head) or _route(head) != key:
            return None
        return _read(key, body)
    return parse


def _verifier(run, extra=None):
    def verify(inputs, outputs):
        try:
            again = run(**inputs)
        except Exception:
            return 1.0
        worst = 0.0
        for k, v in again.items():
            if k not in outputs:
                return 1.0
            if isinstance(v, str):
                worst = max(worst, 0.0 if outputs[k] == v else 1.0)
            else:
                worst = max(worst, _rel(outputs[k], v))
        if extra is not None:
            try:
                worst = max(worst, float(extra(inputs, outputs)))
            except Exception:
                return 1.0
        return worst
    return verify


def proc(key, name, topic, purpose, inputs, outputs, run, reference, wrong, rand, examples, phrases, *,
         conditions=(), method="", latex="", invalid=(), extra_verify=None, primary=None):
    """Register one procedure. `examples` are dicts of input values written in each input's display unit (lists for
    vectors); their outputs come from EXAMPLE_OUTPUTS (computed by the reference) or, until then, from `run`."""
    specs, names, shows = {}, {}, {}
    for d, table, aliases, show in inputs:
        specs[d["symbol"]] = (d, table)
        for a in aliases:
            names[a] = d["symbol"]
            names.setdefault(a.lower(), d["symbol"])
        shows[d["symbol"]] = (aliases[1] if len(aliases) > 1 else aliases[0], show, table)
    _SPEC[key] = {"inputs": specs, "names": names}
    for ph in phrases:
        _ROUTES.append((ph.lower(), key))
    _ROUTES.sort(key=lambda r: -len(r[0]))

    def to_si(ex):
        out = {}
        for s, v in ex.items():
            d, table = specs[s]
            if d.get("type") in ("vector", "choice", "boolean", "text", "edges"):
                out[s] = list(v) if isinstance(v, (list, tuple)) else v
            else:
                si = _convert(float(v), shows[s][1], table)
                out[s] = int(round(si)) if d.get("type") == "integer" else si
        return out

    def as_text(ex, drop=None):
        parts = []
        for s, v in ex.items():
            if s == drop:
                continue
            label, show, _t = shows[s]
            d = specs[s][0]
            if d.get("type") in ("edges", "text"):
                parts.append(f"{label} = [{v}]")
            else:
                parts.append(f"{label} = {_show_value(v)}" + (f" {show}" if show else ""))
        text = ", ".join(parts[:-1]) + (" and " if len(parts) > 1 else "") + parts[-1]
        return f"{phrases[0][0].upper()}{phrases[0][1:]} with {text}."

    def coerce_ex(si):
        out = {}
        for s, v in si.items():
            d = specs[s][0]
            if d.get("type") in ("vector",) and isinstance(v, list):
                out[s] = [float(x) for x in v]
            elif d.get("type") == "edges" and isinstance(v, str):
                out[s] = [tuple(p.strip().split("-")) for p in v.split(",")]
            else:
                out[s] = v
        return out

    stored = EXAMPLE_OUTPUTS.get(key)
    exs = []
    for i, ex in enumerate(examples):
        si = to_si(ex)
        outs = stored[i] if stored and i < len(stored) else {k: v for k, v in run(**coerce_ex(si)).items()}
        exs.append({"inputs": si, "outputs": outs})
    questions = [{"q": as_text(ex), "outputs": exs[i]["outputs"]} for i, ex in enumerate(examples[:2])]
    req = [d["symbol"] for d, *_ in inputs if d.get("required")]
    if len(req) >= 2:
        questions.append({"q": as_text(examples[0], drop=req[-1]), "outcome": "NEEDS_INPUT"})
    questions.append({"q": as_text(examples[0])[:-1] + " and colour = 3.", "outcome": "UNRECOGNIZED_FORMAT"})
    for bad in invalid:
        questions.append({"q": as_text(bad), "outcome": "INVALID_INPUT"})
    PROCEDURES[key] = {
        "name": name, "topic": topic, "purpose": purpose, "inputs": [d for d, *_ in inputs], "outputs": outputs,
        "primary": primary or outputs[0]["symbol"], "conditions": list(conditions), "method": method, "latex": latex,
        "run": run, "verify": _verifier(run, extra_verify), "parse": _parser(key), "reference": reference,
        "wrong": wrong, "random_inputs": rand, "templates": [q["q"] for q in questions[:2]], "examples": exs,
        "questions": questions, "keywords": [p.lower() for p in phrases]}


# =============================================================================================== reference helpers
def _brentq(f, a, b):
    from scipy.optimize import brentq
    return brentq(f, a, b, xtol=1e-300, rtol=8.9e-16, maxiter=500)


def _quad(f, a, b, **kw):
    from scipy.integrate import quad
    v, _err = quad(f, a, b, epsabs=0.0, epsrel=1e-13, limit=500, **kw)
    return v


def _fluids():
    import fluids
    return fluids


# Outputs of the documented examples, computed by each capability's INDEPENDENT reference on the build machine and
# rounded to 12 significant digits; certification checks the procedure against them.
EXAMPLE_OUTPUTS: dict = {
    "rectangular_channel_section": [{"A": 3.6, "P": 5.4, "R": 0.666666666667, "T": 3.0, "Dh": 1.2}, {"A": 0.15, "P": 1.1, "R": 0.136363636364, "T": 0.6, "Dh": 0.25}],
    "trapezoidal_channel_section": [{"A": 10.5, "P": 10.7082039325, "R": 0.98055659625, "T": 10.0, "Dh": 1.05}, {"A": 0.64, "P": 2.2627416998, "R": 0.282842712475, "T": 1.6, "Dh": 0.4}],
    "circular_channel_section": [{"A": 0.492028356752, "P": 1.77215424759, "R": 0.277644204743, "T": 0.979795897113, "Dh": 0.502174338759, "theta": 203.073918066}, {"A": 0.0263165125095, "P": 0.44179441046, "R": 0.0595673278936, "T": 0.374165738677, "Dh": 0.0703338381608, "theta": 112.502022808}],
    "manning_velocity": [{"V": 1.28465664533}, {"V": 0.744286758512}],
    "manning_rectangular_discharge": [{"Q": 6.68290020277, "V": 1.85636116744, "A": 3.6, "R": 0.666666666667}, {"Q": 9.03394587726, "V": 0.752828823105, "A": 12.0, "R": 1.2}],
    "manning_trapezoidal_discharge": [{"Q": 11.7249066655, "V": 1.11665777767, "A": 10.5, "R": 0.98055659625}, {"Q": 5.82608871714, "V": 1.93236773371, "A": 3.015, "R": 0.574833596627}],
    "manning_partially_full_pipe": [{"Q": 0.217085862993, "V": 1.53556836314, "A": 0.141371669412, "R": 0.15, "Q_full": 0.434171725987, "ratio": 0.5}, {"Q": 0.974343007569, "V": 1.07086333664, "A": 0.909866809543, "R": 0.362024500735, "Q_full": 1.06850186423, "ratio": 0.911877686123}],
    "normal_depth_rectangular": [{"yn": 1.75441668808, "V": 1.42497504554, "Fr": 0.3434838961}, {"yn": 1.48278653078, "V": 1.12400985042, "Fr": 0.294710966962}],
    "normal_depth_trapezoidal": [{"yn": 2.1655840504, "V": 0.989735022071, "A": 20.2074288108, "Fr": 0.259831254317}, {"yn": 0.538530204014, "V": 1.44832208538, "A": 0.828544984649, "Fr": 0.732145111446}],
    "chezy_velocity": [{"V": 1.15368973299}, {"V": 1.26491106407}],
    "chezy_from_manning": [{"C": 73.3218686921}, {"C": 37.4154016103}],
    "manning_from_chezy": [{"n": 0.0171808886814}, {"n": 0.0245249776838}],
    "channel_froude_number": [{"Fr": 2.70914184591, "V": 6.0, "regime": "supercritical"}, {"Fr": 0.231288845784, "V": 0.857142857143, "regime": "subcritical"}],
    "critical_depth_rectangular": [{"yc": 0.898302935855, "Vc": 2.9685605604, "Emin": 1.34745440378, "q": 2.66666666667}, {"yc": 0.260597054767, "Vc": 1.59889246269, "Emin": 0.39089558215, "q": 0.416666666667}],
    "critical_depth_trapezoidal": [{"yc": 1.07108002685, "Ac": 5.5076649284, "Vc": 2.72347722583, "Emin": 1.44912937444}, {"yc": 0.665438911247, "Ac": 0.442808944601, "Vc": 1.80664823905, "Emin": 0.831798639059}],
    "critical_depth_circular": [{"yc": 0.398841268146, "Ac": 0.292234761101, "Vc": 1.71095320117, "Emin": 0.548044166036}, {"yc": 0.158554683667, "Ac": 0.0463731548167, "Vc": 1.07821001607, "Emin": 0.217807325805}],
    "alternate_depth_rectangular": [{"E": 3.26697247706, "y_alt": 3.22280783816, "yc": 0.971682767432, "Fr": 3.7861408301, "regime": "supercritical"}, {"E": 1.94157888776, "y_alt": 0.58054737008, "yc": 0.971682767432, "Fr": 0.396623089702, "regime": "subcritical"}],
    "hydraulic_jump_rectangular": [{"y2": 3.3752062735, "Fr1": 6.31023471683, "Fr2": 0.257445331789, "dE": 4.87675503526, "power_loss": 478409.668959, "ratio": 8.43801568375}, {"y2": 1.68541502808, "Fr1": 5.10840685451, "Fr2": 0.291833537115, "dE": 1.75479183224, "power_loss": 51643.5236227, "ratio": 6.74166011233}],
    "gradually_varied_flow_length": [{"L": -4594.58618746, "yn": 2.56382386541, "yc": 1.17710984427, "profile": "M1"}, {"L": 2438.48632369, "yn": 2.56382386541, "yc": 1.17710984427, "profile": "M2"}],
    "rectangular_weir_discharge": [{"Q": 0.758196882083, "L_eff": 2.0}, {"Q": 0.636484488672, "L_eff": 1.42}],
    "v_notch_weir_discharge": [{"Q": 0.0428179868747}, {"Q": 0.0071312719763}],
    "broad_crested_weir_discharge": [{"Q": 3.76372687638, "yc": 0.4}, {"Q": 0.560285641437, "yc": 0.2}],
    "orifice_discharge": [{"Q": 0.00918903390447, "V_ideal": 7.67202711153, "area": 0.00196349540849}, {"Q": 0.0228655291551, "V_ideal": 4.8522159886, "area": 0.00785398163397}],
    "hazen_williams_head_loss": [{"hf": 2.06455521352, "V": 0.707355302631, "Sf": 0.00206455521352}, {"hf": 3.15119246573, "V": 1.01859163579, "Sf": 0.0126047698629}],
    "hazen_williams_flow": [{"Q": 0.0593636598191, "V": 0.839823991132}, {"Q": 0.0255287750682, "V": 1.44463315313}],
    "parallel_pipes_hazen_williams": [{"hf": 7.94406736932, "flows": [0.116758581689, 0.183241418311]}, {"hf": 3.64983323078, "flows": [0.0368759234991, 0.0531302920946, 0.0599937844063]}],
    "hardy_cross_network": [{"flows": [0.0714399845267, 0.0331343839504, -0.0168656160496, -0.0485600154733, -0.00169439942369], "head_loss": [0.0, 2.39874834362, 3.56973630784, 2.37675423164], "loops": 2, "iterations": 14}, {"flows": [0.090876743882, 0.040876743882, -0.019123256118, -0.059123256118], "head_loss": [0.0, 1.32566122834, 2.66183379844, 2.25278888236], "loops": 1, "iterations": 4}],
    "three_reservoir_problem": [{"h_junction": 91.9467900035, "flows": [0.299838366058, -0.0389148570849, -0.260923508973]}, {"h_junction": 87.8349045524, "flows": [0.141141557943, 0.0503454547189, -0.191487012662]}],
    "scs_curve_number_runoff": [{"Q": 50.5390583554, "S": 63.5, "Ia": 12.7, "C": 0.505390583554}, {"Q": 18.2143306733, "S": 108.857142857, "Ia": 5.44285714286, "C": 0.303572177888}],
    "unit_hydrograph_convolution": [{"DRO": [0.0, 15.0, 70.0, 118.0, 103.0, 59.5, 24.5, 5.0, 0.0], "total": [0.0, 15.0, 70.0, 118.0, 103.0, 59.5, 24.5, 5.0, 0.0], "peak": 118.0, "peak_period": 4}, {"DRO": [10.0, 54.0, 100.0, 88.0, 52.4, 24.0, 4.8], "total": [20.0, 64.0, 110.0, 98.0, 62.4, 34.0, 14.8], "peak": 110.0, "peak_period": 3}],
    "unit_hydrograph_derivation": [{"uh": [-0.502834815065, 11.0475725314, 25.6537253747, 17.3404996047, 11.3869004489, 4.27012446017, 1.55034025912], "rms_residual": 0.4011393048, "peak": 25.6537253747}, {"uh": [1.69772696982, 15.8984865275, 28.8271810427, 20.2485690516, 11.2920123639, 5.60453828918, 1.88161822656], "rms_residual": 0.212071972897, "peak": 28.8271810427}],
    "thiessen_polygon_rainfall": [{"mean": 47.2399519231, "areas": [29.6650641026, 32.5608974359, 37.7740384615], "weights": [0.296650641026, 0.325608974359, 0.377740384615], "arithmetic_mean": 47.6666666667, "catchment_area": 100.0}, {"mean": 89.608935701, "areas": [13.2277173913, 34.1398538961, 25.7055444555, 21.926884257], "weights": [0.139239130435, 0.359366883117, 0.270584678479, 0.230809307969], "arithmetic_mean": 88.75, "catchment_area": 95.0}],
    "flood_exceedance_risk": [{"risk": 0.394993932862, "p_annual": 0.01, "p_none": 0.605006067138}, {"risk": 0.335167364008, "p_annual": 0.04, "p_none": 0.664832635992}],
    "design_return_period_for_risk": [{"T": 475.061254652, "p_annual": 0.00210499170414}, {"T": 487.893314633, "p_annual": 0.00204962841262}],
    "gumbel_design_flood": [{"Q_T": 812.637120024, "K_T": 3.13666842977, "mean": 451.5, "std": 115.133979925}, {"Q_T": 1766.24691075, "K_T": 2.59227588266, "mean": 1186.66666667, "std": 223.579692253}],
    "log_pearson_iii_design_flood": [{"Q_T": 1083.77060547, "K_T": 2.75390768385, "mean_log": 2.66461076821, "std_log": 0.134473134908, "skew": 0.598210843972}, {"Q_T": 2446.88722986, "K_T": 2.65404891292, "mean_log": 3.08633550587, "std_log": 0.113893322411, "skew": 1.26855684609}],
    "weibull_plotting_positions": [{"ranked": [640.0, 510.0, 420.0, 380.0, 290.0], "T": [6.0, 3.0, 2.0, 1.5, 1.2], "P": [0.166666666667, 0.333333333333, 0.5, 0.666666666667, 0.833333333333]}, {"ranked": [22.1, 19.8, 18.2, 15.3, 12.5, 11.0, 9.7], "T": [8.0, 4.0, 2.66666666667, 2.0, 1.6, 1.33333333333, 1.14285714286], "P": [0.125, 0.25, 0.375, 0.5, 0.625, 0.75, 0.875]}],
    "nash_sutcliffe_efficiency": [{"NSE": 0.954237913643, "RMSE": 3.21455025366, "PBIAS": 0.0}, {"NSE": 0.966307628265, "RMSE": 0.72702918, "PBIAS": 2.39520958084}],
    "kling_gupta_efficiency": [{"KGE": 0.962739163589, "r": 0.976869955561, "alpha": 0.970787485998, "beta": 1.0}, {"KGE": 0.908454547122, "r": 0.986748450849, "alpha": 0.912642896716, "beta": 0.976047904192}],
    "horton_infiltration": [{"f": 3.10847681327e-06, "F": 0.0259523708681, "f_mm_h": 11.1905165278, "F_mm": 25.9523708681}, {"f": 5.16839196162e-06, "F": 0.0155575155753, "f_mm_h": 18.6062110618, "F_mm": 15.5575155753}],
    "green_ampt_infiltration": [{"F": 0.0320966925154, "f": 5.59313582187e-06, "F_mm": 32.0966925154, "f_mm_h": 20.1352889587}, {"F": 0.0324786453565, "f": 2.59356905372e-06, "F_mm": 32.4786453565, "f_mm_h": 9.33684859338}],
    "phi_index": [{"phi": 6.33333333333, "excess": [0.0, 2.66666666667, 11.6666666667, 5.66666666667, 0.0, 0.0], "contributing": 3}, {"phi": 10.5, "excess": [0.0, 2.25, 6.75, 0.0, 0.0], "contributing": 2}],
    "muskingum_routing": [{"outflow": [10.0, 10.9523809524, 21.8321995465, 42.959723572, 45.8360456806, 42.6284048803, 36.7101168421, 29.8005373935, 22.5145672061], "C0": 0.047619047619, "C1": 0.428571428571, "C2": 0.52380952381, "peak_outflow": 45.8360456806, "attenuation": 22.1639543194}, {"outflow": [20.0, 20.0, 50.0, 130.0, 185.0, 187.5, 158.75, 119.375, 82.1875, 56.09375], "C0": 0.0, "C1": 0.5, "C2": 0.5, "peak_outflow": 187.5, "attenuation": 52.5}],
    "darcy_flow_layered_soil": [{"Q": 2.5e-05, "q": 2.5e-06, "Kv": 1e-05, "Kh": 1e-05, "heads": [0.0]}, {"Q": 3.67197062423e-05, "q": 7.34394124847e-07, "Kv": 1.10159118727e-06, "Kh": 2.91111111111e-05, "heads": [2.94492044064, 0.00734394124847]}],
    "falling_head_permeability": [{"K": 2.27033610563e-06, "K_cm_s": 0.000227033610563}, {"K": 1.66355323334e-07, "K_cm_s": 1.66355323334e-05}],
    "thiem_confined_transmissivity": [{"T": 0.00375537173462, "T_m2_d": 324.464117872}, {"T": 0.00154237289326, "T_m2_d": 133.261017978, "K": 7.71186446632e-05}],
    "thiem_unconfined_conductivity": [{"K": 0.0001447648968, "K_m_d": 12.5076870835}, {"K": 0.000126687127669, "K_m_d": 10.9457678306}],
    "theis_drawdown": [{"s": 2.51923946796, "u": 0.001, "W": 6.33153936414}, {"s": 1.32988209522, "u": 0.048, "W": 2.5067686923}],
    "cooper_jacob_analysis": [{"T": 0.00743434184031, "S": 0.000329649220272, "ds_per_cycle": 0.570634204857, "t0_min": 0.821137273662, "T_m2_d": 642.327135003}, {"T": 0.00425685487816, "S": 0.000650541459489, "ds_per_cycle": 0.74743382135, "t0_min": 1.01881393362, "T_m2_d": 367.792261473}],
    "well_field_drawdown": [{"s": 4.93587802703, "parts": [2.82050172973, 2.1153762973]}, {"s": 1.3988136812, "parts": [1.32071407719, -0.733730042886, 0.811829646889]}],
    "dupuit_flow_with_recharge": [{"q0": 3.22446098563e-06, "qL": 8.92831679215e-06, "x_divide": -339.1875, "h_max": 12.0}, {"q0": -3.1688087814e-06, "qL": 3.1688087814e-06, "x_divide": 200.0, "h_max": 9.55921063014}],
    "soil_phase_relations": [{"n": 0.393939393939, "S": 0.747692307692, "gamma_d": 16052.7272727, "gamma": 18942.2181818, "gamma_sat": 19917.2727273, "gamma_sub": 10107.2727273}, {"n": 0.473684210526, "S": 0.883333333333, "gamma_d": 13682.3684211, "gamma": 17787.0789474, "gamma_sat": 18329.2105263, "gamma_sub": 8519.21052632}],
    "soil_phase_from_lab_masses": [{"w": 0.15625, "rho": 1850.0, "rho_d": 1600.0, "e": 0.6875, "n": 0.407407407407, "S": 0.613636363636, "gamma": 18148.5, "gamma_d": 15696.0}, {"w": 0.151685393258, "rho": 1952.38095238, "rho_d": 1695.23809524, "e": 0.580898876404, "n": 0.367448471926, "S": 0.699806576402, "gamma": 19152.8571429, "gamma_d": 16630.2857143}],
    "relative_density_from_unit_weights": [{"Dr": 0.566030677142, "Dr_percent": 56.6030677142}, {"Dr": 0.336744186047, "Dr_percent": 33.6744186047}],
    "liquid_limit_flow_curve": [{"LL": 39.6291715543, "flow_index": 11.2178600879, "PI": 18.6291715543}, {"LL": 52.9097336937, "flow_index": 16.1629507349}],
    "grain_size_coefficients": [{"D10": 0.106066017178, "D30": 0.276152628571, "D60": 0.744868963576, "Cu": 7.02269193653, "Cc": 0.965255361788}, {"D10": 0.271717099279, "D30": 0.883074607981, "D60": 3.22052036949, "Cu": 11.8524758951, "Cc": 0.891151975954}],
    "effective_stress_at_depth": [{"sigma": 150000.0, "u": 49050.0, "sigma_eff": 100950.0, "sigma_eff_kPa": 100.95}, {"sigma": 169500.0, "u": 73575.0, "sigma_eff": 95925.0, "sigma_eff_kPa": 95.925}],
    "consolidation_settlement_nc": [{"Sc": 0.162025365791, "de": 0.0777721755796, "e_final": 0.84222782442}, {"Sc": 0.562461260845, "de": 0.224984504338, "e_final": 1.17501549566}],
    "consolidation_settlement_oc": [{"Sc": 0.203402742893, "de": 0.0854291520153, "OCR": 1.77777777778, "case": "passes the preconsolidation stress"}, {"Sc": 0.015099588088, "de": 0.0098147322572, "OCR": 1.85714285714, "case": "stays overconsolidated"}],
    "thick_clay_layer_settlement": [{"S": 0.753129074489, "S_one_layer": 0.720373332501}, {"S": 0.996495177511, "S_one_layer": 0.888864346253}],
    "degree_of_consolidation": [{"Tv": 0.277777777778, "U": 0.59137021727, "U_percent": 59.137021727}, {"Tv": 0.32, "U": 0.631894580746, "U_percent": 63.1894580746, "St": 0.113741024534}],
    "consolidation_time": [{"Tv": 0.848085408046, "t": 55757375.152, "t_days": 645.339990185}, {"Tv": 0.196730739524, "t": 66222399.8463, "t_days": 766.462961184}],
    "coefficient_of_consolidation": [{"cv": 2.62307652698e-08, "Tv": 0.196730739524, "cv_m2_yr": 0.827779998079}, {"cv": 2.65762875264e-08, "Tv": 0.848085408046, "cv_m2_yr": 0.838683851244}],
    "excess_pore_pressure_isochrone": [{"u": 52247.3336591, "Uz": 0.477526663409, "Tv": 0.222222222222}, {"u": 47079.1346119, "Uz": 0.411510817351, "Tv": 0.3125}],
    "stress_below_circular_load": [{"dsigma": 63594.7771265, "I": 0.423965180843}, {"dsigma": 28445.82472, "I": 0.2844582472}],
    "stress_below_rectangular_load": [{"dsigma": 21414.5857952, "I": 0.107072928976}, {"dsigma": 69630.3219317, "I": 0.580252682764}],
    "stress_below_strip_load": [{"dsigma": 39581.8696409, "I": 0.395818696409}, {"dsigma": 30724.9147049, "I": 0.204832764699}],
    "strip_footing_bearing_capacity": [{"qu": 935050.042512, "qa": 311683.347504, "Nc": 30.1396277915, "Nq": 18.4011222187, "Ngamma": 22.4024862711}, {"qu": 1861485.11566, "qa": 744594.046262, "Nc": 46.123598689, "Nq": 33.2960914914, "Ngamma": 48.0287635868}],
    "rankine_active_thrust": [{"Ka": 0.333333333333, "sigma_base": 36000.0, "Pa": 108000.0, "z_crack": 0.0, "ybar": 2.0}, {"Ka": 0.405858517205, "sigma_base": 31903.0316764, "Pa": 65994.1818587, "z_crack": 0.862826923282, "ybar": 1.37905769224}],
    "rankine_passive_thrust": [{"Kp": 3.2545883033, "sigma_base": 117165.178919, "Pp": 117165.178919, "ybar": 0.666666666667}, {"Kp": 2.46391281101, "sigma_base": 178115.484078, "Pp": 323681.906894, "ybar": 1.17458090666}],
    "coulomb_active_thrust": [{"Ka": 0.340022395359, "Pa": 110167.256096, "Pa_h": 103523.357606, "Pa_v": 37679.4207198}, {"Ka": 0.244429303443, "Pa": 37153.2541233, "Pa_h": 34199.750742, "Pa_v": 14516.9329106}],
    "coulomb_passive_thrust": [{"Kp": 4.97650022273, "Pp": 179154.008018, "Pp_h": 173049.483228, "Pp_v": 46368.4692815}, {"Kp": 6.93096761756, "Pp": 592597.731301, "Pp_h": 579648.049072, "Pp_v": 123207.99629}],
    "mohr_coulomb_failure_stress": [{"s1": 326911.004008, "sigma_f": 160191.370171, "tau_f": 100175.262391, "theta": 59.0}, {"s1": 738034.466429, "sigma_f": 314715.28727, "tau_f": 220366.016536, "theta": 62.5}],
    "triaxial_strength_parameters": [{"phi": 26.8777955081, "c": 19.4378163835, "alpha": 24.3272056837}, {"phi": 27.3522329091, "c": 22.8217732294, "alpha": 24.6768631703}],
    "direct_shear_strength_parameters": [{"phi": 29.8681357032, "c": 16.0, "tan_phi": 0.574285714286}, {"phi": 32.8220160762, "c": 3.0, "tan_phi": 0.645}],
    "proctor_compaction_optimum": [{"w_opt": 12.4117647059, "rho_dmax": 1.83760084034, "rho_zav": 2.01105284531}, {"w_opt": 17.4240963855, "rho_dmax": 1.68722893718}],
    "traverse_bowditch_adjustment": [{"misclosure_E": 0.0159393843463, "misclosure_N": -0.0123427324999, "linear_misclosure": 0.0201595391541, "precision": 19841.7234116, "E": [0.0, 70.7066932726, 141.415182997, 70.7146629647], "N": [0.0, 70.7137638018, -0.020312884737, -70.7137638018], "area": 9999.99976748}, {"misclosure_E": 84.2478409189, "misclosure_N": -79.9590556282, "linear_misclosure": 116.151406692, "precision": 9.31628837585, "E": [1000.0, 1107.59249531, 1370.58665187, 1244.62735276], "N": [2000.0, 2234.25461947, 2139.70402872, 1899.55782373], "area": 71589.2432916}],
    "levelling_rise_and_fall": [{"RL": [100.0, 99.42, 99.885, 98.55], "rise": 0.465, "fall": 1.915, "last_RL": 98.55, "check": -1.45}, {"RL": [52.75, 53.175, 53.92, 53.585, 55.12], "rise": 2.705, "fall": 0.335, "last_RL": 55.12, "check": 2.37}],
    "horizontal_circular_curve": [{"T": 109.19107028, "L": 209.439510239, "LC": 205.212085995, "E": 19.2533317428, "M": 18.0922137642, "PC": 1140.80892972, "PT": 1350.24843996}, {"T": 135.766138953, "L": 267.035375555, "LC": 264.836922026, "E": 15.1686309348, "M": 14.7946076949, "PC": 3284.73386105, "PT": 3551.7692366}],
    "vertical_parabolic_curve": [{"elev": 118.1875, "x_turn": 1520.0, "elev_turn": 118.8, "r": -0.025, "BVC": 1400.0, "elev_BVC": 117.0}, {"elev": 64.95, "x_turn": 840.0, "elev_turn": 64.95, "r": 0.025, "BVC": 740.0, "elev_BVC": 66.2}],
    "earthwork_volume_end_areas": [{"V_end_area": 1561.0, "V_prismoidal": 1551.33333333}, {"V_end_area": 3287.5}],
    "area_from_offsets": [{"A_trapezoidal": 188.0, "A_simpson": 196.666666667}, {"A_trapezoidal": 444.75}],
}


# =============================================================================================== shared input builders
def _L(sym, meaning, names=(), show="m", **kw):
    return _in(sym, "length", "length", meaning, names, show, **kw)


def _Qin(sym="Q", meaning="discharge", names=("discharge", "flow"), show="m^3/s", **kw):
    return _in(sym, "flow_rate", "flow", meaning, names, show, **kw)


def _N(sym, meaning, names=(), show="", **kw):
    return _in(sym, "number", "number", meaning, names, show, **kw)


def _F(sym, meaning, names=(), show="", **kw):
    return _in(sym, "number", "fraction", meaning, names, show, **kw)


def _DEG(sym, meaning, names=(), show="degrees", **kw):
    return _in(sym, "number", "degrees", meaning, names, show, **kw)


def _STRESS(sym, meaning, names=(), show="kPa", **kw):
    return _in(sym, "stress", "stress", meaning, names, show, **kw)


def _GAM(sym, meaning, names=(), show="kN/m^3", **kw):
    return _in(sym, "unit_weight", "unit_weight", meaning, names, show, **kw)


def _VEC(sym, meaning, names=(), **kw):
    return _in(sym, "number", "number", meaning, names, "", typ="vector", **kw)


# =============================================================================================== open-channel geometry
def _rect(b, y):
    return b * y, b + 2 * y, b


def _trap(b, y, z):
    return (b + z * y) * y, b + 2 * y * math.sqrt(1 + z * z), b + 2 * z * y


def _circ(D, y):
    th = 2 * math.acos(1 - 2 * y / D)
    return D * D / 8 * (th - math.sin(th)), D * th / 2, D * math.sin(th / 2), th


def _section_out(A, P, T):
    return {"A": _r(A), "P": _r(P), "R": _r(A / P), "T": _r(T), "Dh": _r(A / T)}


SEC_OUT = [_o("A", "area", "flow area (m^2)"), _o("P", "length", "wetted perimeter (m)"),
           _o("R", "length", "hydraulic radius A/P (m)"), _o("T", "length", "top width (m)"),
           _o("Dh", "length", "hydraulic depth A/T (m)")]


def _poly_ref(pts):
    from shapely.geometry import LineString, Polygon
    A = Polygon(pts).area
    P = LineString(pts).length
    T = abs(pts[-1][0] - pts[0][0])
    return {"A": A, "P": P, "R": A / P, "T": T, "Dh": A / T}


def rect_section(b, y):
    _pos(b=b, y=y)
    return _section_out(*_rect(b, y))


proc("rectangular_channel_section", "Rectangular channel: area, wetted perimeter, hydraulic radius", "open-channel hydraulics",
     "Flow area, wetted perimeter, hydraulic radius, top width and hydraulic depth of a rectangular channel flowing at "
     "depth y.",
     [_L("b", "bottom width", ("width", "bottom width")), _L("y", "flow depth", ("depth", "flow depth"))], SEC_OUT,
     rect_section, lambda b, y: _poly_ref([(0, y), (0, 0), (b, 0), (b, y)]),
     lambda b, y: _section_out(b * y, b + y, b),
     lambda rng: {"b": rng.uniform(0.3, 20), "y": rng.uniform(0.05, 5)},
     [{"b": 3, "y": 1.2}, {"b": 0.6, "y": 0.25}], ["rectangular channel section", "hydraulic radius of a rectangular channel",
                                                    "rectangular channel geometry"],
     conditions=["b > 0", "y > 0"], invalid=[{"b": 3, "y": -1}],
     method="A = b y, P = b + 2y, R = A/P, T = b, D = A/T", latex=r"A = b\,y,\quad P = b + 2y,\quad R = \frac{A}{P}")


def trap_section(b, y, z):
    _pos(y=y)
    _finite(b=b, z=z)
    _need(b >= 0 and z >= 0 and b + z > 0, "need b >= 0, z >= 0 and not both zero")
    return _section_out(*_trap(b, y, z))


proc("trapezoidal_channel_section", "Trapezoidal channel: area, wetted perimeter, hydraulic radius", "open-channel hydraulics",
     "Flow area, wetted perimeter, hydraulic radius, top width and hydraulic depth of a trapezoidal channel with bottom "
     "width b and side slopes z (horizontal : vertical); b = 0 is a triangular channel.",
     [_L("b", "bottom width", ("width", "bottom width")), _L("y", "flow depth", ("depth", "flow depth")),
      _N("z", "side slope, horizontal per unit vertical", ("side slope",))], SEC_OUT,
     trap_section, lambda b, y, z: _poly_ref([(0, y), (z * y, 0), (z * y + b, 0), (2 * z * y + b, y)]),
     lambda b, y, z: _section_out((b + z * y) * y, b + 2 * y * z, b + 2 * z * y),
     lambda rng: {"b": rng.uniform(0.0, 15), "y": rng.uniform(0.1, 4), "z": rng.uniform(0.5, 3)},
     [{"b": 4, "y": 1.5, "z": 2}, {"b": 0, "y": 0.8, "z": 1}], ["trapezoidal channel section",
                                                                 "hydraulic radius of a trapezoidal channel",
                                                                 "trapezoidal channel geometry"],
     conditions=["y > 0", "b >= 0", "z >= 0"], invalid=[{"b": 4, "y": 1.5, "z": -1}],
     method="A = (b + z y) y, P = b + 2 y sqrt(1 + z^2), T = b + 2 z y",
     latex=r"A = (b + z y)\,y,\quad P = b + 2y\sqrt{1+z^2},\quad T = b + 2zy")


def circ_section(D, y):
    _pos(D=D, y=y)
    _need(y < D, "the depth must be below the diameter (a part-full section)")
    A, P, T, th = _circ(D, y)
    return dict(_section_out(A, P, T), theta=_r(math.degrees(th)))


def _ref_circ(D, y):
    r = D / 2
    alpha = _brentq(lambda a: r * (1 - math.cos(a)) - y, 0.0, math.pi)          # half angle at the centre
    A = _quad(lambda f: 2 * (r * math.sin(f)) * (r * math.sin(f)), 0.0, alpha)    # strips: chord width * dy/dphi
    P = 2 * _quad(lambda f: math.hypot(r * math.cos(f), r * math.sin(f)), 0.0, alpha)
    T = 2 * math.sqrt(max(r * r - (r - y) ** 2, 0.0))
    return {"A": A, "P": P, "R": A / P, "T": T, "Dh": A / T, "theta": math.degrees(2 * alpha)}


proc("circular_channel_section", "Part-full circular conduit: area, wetted perimeter, hydraulic radius",
     "open-channel hydraulics",
     "Flow area, wetted perimeter, hydraulic radius, top width and hydraulic depth of a circular pipe or culvert "
     "flowing part full at depth y, with the central angle of the wetted arc.",
     [_L("D", "pipe diameter", ("diameter",)), _L("y", "flow depth", ("depth", "flow depth"))],
     SEC_OUT + [_o("theta", "number", "central angle of the wetted arc (degrees)")],
     circ_section, _ref_circ,
     lambda D, y: (lambda th: dict(_section_out(D * D / 8 * (th - math.sin(th)), D * th / 2, D * math.sin(th / 2)),
                                   theta=_r(math.degrees(th))))(2 * math.acos(1 - y / D)),
     lambda rng: (lambda D: {"D": D, "y": D * rng.uniform(0.05, 0.95)})(rng.uniform(0.2, 3)),
     [{"D": 1.0, "y": 0.6}, {"D": 0.45, "y": 0.1}], ["circular channel section", "part full pipe section",
                                                      "partially full pipe section"],
     conditions=["0 < y < D"], invalid=[{"D": 1.0, "y": 1.2}],
     method="theta = 2 acos(1 - 2y/D), A = D^2 (theta - sin theta)/8, P = D theta/2, T = D sin(theta/2)",
     latex=r"\theta = 2\cos^{-1}\!\left(1-\frac{2y}{D}\right),\quad A = \frac{D^2}{8}(\theta-\sin\theta),\quad P = \frac{D\theta}{2}")


# --- Manning and Chezy ---------------------------------------------------------------------------------------------
def _manning_v(R, S, n):
    return R ** (2.0 / 3.0) * math.sqrt(S) / n


def _ok_mann(S, n):
    _pos(S=S, n=n)
    _need(S < 1, "the slope S is a fraction (m/m) below 1")
    _need(0.005 <= n <= 0.2, "Manning's n must lie between 0.005 and 0.2")


def manning_velocity(R, S, n):
    _pos(R=R)
    _ok_mann(S, n)
    return {"V": _r(_manning_v(R, S, n))}


proc("manning_velocity", "Manning velocity from hydraulic radius", "open-channel hydraulics",
     "Mean velocity of uniform flow from Manning's equation (SI) for a hydraulic radius R, slope S and roughness n.",
     [_L("R", "hydraulic radius", ("hydraulic radius",)), _F("S", "channel (energy) slope, m/m", ("slope", "bed slope")),
      _N("n", "Manning roughness coefficient", ("Manning n", "roughness"))],
     [_o("V", "speed", "mean velocity (m/s)")], manning_velocity,
     lambda R, S, n: {"V": _fluids().V_Manning(R, S, n)},
     lambda R, S, n: {"V": _r(R ** (2.0 / 3.0) * S / n)},
     lambda rng: {"R": rng.uniform(0.05, 5), "S": 10 ** rng.uniform(-5, -1.5), "n": rng.uniform(0.01, 0.06)},
     [{"R": 0.8, "S": 0.0005, "n": 0.015}, {"R": 2.5, "S": 0.0002, "n": 0.035}], ["manning velocity",
                                                                                 "velocity from manning"],
     conditions=["R > 0", "0 < S < 1", "0.005 <= n <= 0.2"], invalid=[{"R": 0.8, "S": 0.0005, "n": 0.5}],
     method="V = R^(2/3) S^(1/2) / n", latex=r"V = \frac{1}{n}R^{2/3}S^{1/2}")


def _mann_out(A, P, S, n):
    R = A / P
    V = _manning_v(R, S, n)
    return {"Q": _r(V * A), "V": _r(V), "A": _r(A), "R": _r(R)}


MANN_OUT = [_o("Q", "flow_rate", "discharge (m^3/s)"), _o("V", "speed", "mean velocity (m/s)"),
            _o("A", "area", "flow area (m^2)"), _o("R", "length", "hydraulic radius (m)")]


def manning_rect(b, y, S, n):
    _pos(b=b, y=y)
    _ok_mann(S, n)
    return _mann_out(*_rect(b, y)[:2], S, n)


def _ref_mann_poly(pts, S, n):
    g = _poly_ref(pts)
    V = _fluids().V_Manning(g["R"], S, n)
    return {"Q": V * g["A"], "V": V, "A": g["A"], "R": g["R"]}


proc("manning_rectangular_discharge", "Manning discharge in a rectangular channel", "open-channel hydraulics",
     "Uniform-flow discharge and velocity in a rectangular channel from Manning's equation.",
     [_L("b", "bottom width", ("width", "bottom width")), _L("y", "flow depth", ("depth", "flow depth")),
      _F("S", "bed slope, m/m", ("slope", "bed slope")), _N("n", "Manning roughness coefficient", ("Manning n", "roughness"))],
     MANN_OUT, manning_rect,
     lambda b, y, S, n: _ref_mann_poly([(0, y), (0, 0), (b, 0), (b, y)], S, n),
     lambda b, y, S, n: _mann_out(b * y, b + y, S, n),
     lambda rng: {"b": rng.uniform(0.5, 15), "y": rng.uniform(0.1, 4), "S": 10 ** rng.uniform(-5, -2),
                  "n": rng.uniform(0.011, 0.05)},
     [{"b": 3, "y": 1.2, "S": 0.001, "n": 0.013}, {"b": 6, "y": 2.0, "S": 0.0004, "n": 0.03}],
     ["manning discharge in a rectangular channel", "uniform flow in a rectangular channel"],
     conditions=["b, y > 0", "0 < S < 1", "0.005 <= n <= 0.2"],
     method="A = b y, R = A/(b + 2y), Q = A R^(2/3) S^(1/2)/n", latex=r"Q = \frac{1}{n}A R^{2/3} S^{1/2},\quad A = by,\ R = \frac{by}{b+2y}")


def manning_trap(b, y, z, S, n):
    _pos(y=y)
    _finite(b=b, z=z)
    _need(b >= 0 and z >= 0 and b + z > 0, "need b >= 0, z >= 0 and not both zero")
    _ok_mann(S, n)
    return _mann_out(*_trap(b, y, z)[:2], S, n)


proc("manning_trapezoidal_discharge", "Manning discharge in a trapezoidal channel", "open-channel hydraulics",
     "Uniform-flow discharge and velocity in a trapezoidal channel (side slopes z H:1V) from Manning's equation.",
     [_L("b", "bottom width", ("width", "bottom width")), _L("y", "flow depth", ("depth", "flow depth")),
      _N("z", "side slope, horizontal per unit vertical", ("side slope",)),
      _F("S", "bed slope, m/m", ("slope", "bed slope")), _N("n", "Manning roughness coefficient", ("Manning n", "roughness"))],
     MANN_OUT, manning_trap,
     lambda b, y, z, S, n: _ref_mann_poly([(0, y), (z * y, 0), (z * y + b, 0), (2 * z * y + b, y)], S, n),
     lambda b, y, z, S, n: _mann_out((b + z * y) * y, b + 2 * y * z, S, n),
     lambda rng: {"b": rng.uniform(0.5, 15), "y": rng.uniform(0.1, 4), "z": rng.uniform(0.5, 3),
                  "S": 10 ** rng.uniform(-5, -2), "n": rng.uniform(0.011, 0.05)},
     [{"b": 4, "y": 1.5, "z": 2, "S": 0.0008, "n": 0.025}, {"b": 2, "y": 0.9, "z": 1.5, "S": 0.002, "n": 0.016}],
     ["manning discharge in a trapezoidal channel", "uniform flow in a trapezoidal channel"],
     conditions=["y > 0", "b >= 0", "z >= 0", "0 < S < 1", "0.005 <= n <= 0.2"],
     method="A = (b + z y) y, R = A/(b + 2 y sqrt(1 + z^2)), Q = A R^(2/3) S^(1/2)/n",
     latex=r"Q = \frac{1}{n}A R^{2/3} S^{1/2},\quad A=(b+zy)y,\ P=b+2y\sqrt{1+z^2}")


def manning_pipe(D, y, S, n):
    _pos(D=D, y=y)
    _need(y <= D, "the depth cannot exceed the diameter")
    _ok_mann(S, n)
    if y == D:
        A, P = math.pi * D * D / 4, math.pi * D
    else:
        A, P = _circ(D, y)[:2]
    out = _mann_out(A, P, S, n)
    Qf = math.pi * D * D / 4 * _manning_v(D / 4, S, n)
    out.update(Q_full=_r(Qf), ratio=_r(out["Q"] / Qf))
    return out


def _ref_mann_pipe(D, y, S, n):
    fl = _fluids()
    if y == D:
        A, R = math.pi * D * D / 4, D / 4
    else:
        g = _ref_circ(D, y)
        A, R = g["A"], g["R"]
    V = fl.V_Manning(R, S, n)
    Qf = fl.V_Manning(D / 4, S, n) * math.pi * D * D / 4
    return {"Q": V * A, "V": V, "A": A, "R": R, "Q_full": Qf, "ratio": V * A / Qf}


proc("manning_partially_full_pipe", "Manning flow in a part-full circular pipe", "open-channel hydraulics",
     "Gravity (open-channel) discharge and velocity in a circular sewer or culvert flowing at depth y, from Manning's "
     "equation, with the full-bore capacity and the ratio Q/Q_full (n taken as constant with depth).",
     [_L("D", "pipe diameter", ("diameter",)), _L("y", "flow depth", ("depth", "flow depth")),
      _F("S", "pipe slope, m/m", ("slope",)), _N("n", "Manning roughness coefficient", ("Manning n", "roughness"))],
     MANN_OUT + [_o("Q_full", "flow_rate", "full-bore capacity (m^3/s)"), _o("ratio", "number", "Q/Q_full")],
     manning_pipe, _ref_mann_pipe,
     lambda D, y, S, n: (lambda out: dict(out, Q=_r(out["A"] * _manning_v(D / 4, S, n)), R=_r(D / 4)))(manning_pipe(D, y, S, n)),
     lambda rng: (lambda D: {"D": D, "y": D * rng.uniform(0.1, 0.95), "S": 10 ** rng.uniform(-4, -1.7),
                             "n": rng.uniform(0.011, 0.024)})(rng.uniform(0.15, 2.5)),
     [{"D": 0.6, "y": 0.3, "S": 0.005, "n": 0.013}, {"D": 1.2, "y": 0.9, "S": 0.001, "n": 0.015}],
     ["manning flow in a partially full pipe", "manning flow in a part full pipe", "partially full pipe flow",
      "part full pipe flow"],
     conditions=["0 < y <= D", "0 < S < 1", "n constant with depth"], invalid=[{"D": 0.6, "y": 0.8, "S": 0.005, "n": 0.013}],
     method="circular-segment geometry, Q = A R^(2/3) S^(1/2)/n; Q_full with R = D/4",
     latex=r"Q = \frac{1}{n}A R^{2/3} S^{1/2},\quad Q_{full} = \frac{1}{n}\frac{\pi D^2}{4}\left(\frac{D}{4}\right)^{2/3}S^{1/2}")


def _normal_depth(qfun, Q, ymax):
    _need(qfun(ymax) > Q, "the discharge exceeds what the channel can carry below the depth limit")
    return _bisect_root(lambda y: qfun(y) - Q, 0.0, ymax)


def normal_depth_rect(Q, b, S, n):
    _pos(Q=Q, b=b)
    _ok_mann(S, n)
    qf = lambda y: _manning_v(b * y / (b + 2 * y), S, n) * b * y if y > 0 else 0.0     # noqa: E731
    y = _normal_depth(qf, Q, 1e4)
    V = Q / (b * y)
    return {"yn": _r(y), "V": _r(V), "Fr": _r(V / math.sqrt(G * y))}


def _ref_normal_rect(Q, b, S, n):
    fl = _fluids()
    y = _brentq(lambda y: fl.V_Manning(b * y / (b + 2 * y), S, n) * b * y - Q, 1e-12, 1e4)
    V = Q / (b * y)
    return {"yn": y, "V": V, "Fr": fl.Froude(V, y, g=G)}


proc("normal_depth_rectangular", "Normal depth in a rectangular channel", "open-channel hydraulics",
     "The uniform-flow (normal) depth that carries a discharge Q in a rectangular channel (Manning), with the velocity "
     "and Froude number at that depth.",
     [_Qin(), _L("b", "bottom width", ("width", "bottom width")), _F("S", "bed slope, m/m", ("slope", "bed slope")),
      _N("n", "Manning roughness coefficient", ("Manning n", "roughness"))],
     [_o("yn", "length", "normal depth (m)"), _o("V", "speed", "velocity at normal depth (m/s)"),
      _o("Fr", "number", "Froude number at normal depth")],
     normal_depth_rect, _ref_normal_rect,
     lambda Q, b, S, n: (lambda y: {"yn": _r(y), "V": _r(Q / (b * y)), "Fr": _r(Q / (b * y) / math.sqrt(G * y))})(
         (Q * n / (b * math.sqrt(S))) ** 0.6),
     lambda rng: {"Q": 10 ** rng.uniform(-1, 2.3), "b": rng.uniform(0.5, 10), "S": 10 ** rng.uniform(-4.5, -2),
                  "n": rng.uniform(0.011, 0.05)},
     [{"Q": 10, "b": 4, "S": 0.0005, "n": 0.015}, {"Q": 2.5, "b": 1.5, "S": 0.002, "n": 0.025}],
     ["normal depth in a rectangular channel", "normal depth rectangular"],
     conditions=["Q, b > 0", "0 < S < 1"], method="bisection on Manning's Q(y) = Q (Q increases with y)",
     latex=r"Q = \frac{1}{n}\,b y\left(\frac{by}{b+2y}\right)^{2/3}S^{1/2}",
     extra_verify=lambda i, o: _rel(_manning_v(i["b"] * o["yn"] / (i["b"] + 2 * o["yn"]), i["S"], i["n"]) * i["b"] * o["yn"], i["Q"]))


def normal_depth_trap(Q, b, z, S, n):
    _pos(Q=Q)
    _finite(b=b, z=z)
    _need(b >= 0 and z >= 0 and b + z > 0, "need b >= 0, z >= 0 and not both zero")
    _ok_mann(S, n)

    def qf(y):
        if y <= 0:
            return 0.0
        A, P, _T = _trap(b, y, z)
        return _manning_v(A / P, S, n) * A
    y = _normal_depth(qf, Q, 1e4)
    A, _P, T = _trap(b, y, z)
    V = Q / A
    return {"yn": _r(y), "V": _r(V), "A": _r(A), "Fr": _r(V / math.sqrt(G * A / T))}


def _ref_normal_trap(Q, b, z, S, n):
    fl = _fluids()

    def qf(y):
        g = _poly_ref([(0, y), (z * y, 0), (z * y + b, 0), (2 * z * y + b, y)])
        return fl.V_Manning(g["R"], S, n) * g["A"] - Q
    y = _brentq(qf, 1e-9, 1e3)
    g = _poly_ref([(0, y), (z * y, 0), (z * y + b, 0), (2 * z * y + b, y)])
    V = Q / g["A"]
    return {"yn": y, "V": V, "A": g["A"], "Fr": fl.Froude(V, g["Dh"], g=G)}


proc("normal_depth_trapezoidal", "Normal depth in a trapezoidal channel", "open-channel hydraulics",
     "The uniform-flow (normal) depth that carries a discharge Q in a trapezoidal channel (Manning), with the velocity, "
     "area and Froude number (on the hydraulic depth A/T).",
     [_Qin(), _L("b", "bottom width", ("width", "bottom width")), _N("z", "side slope, horizontal per unit vertical", ("side slope",)),
      _F("S", "bed slope, m/m", ("slope", "bed slope")), _N("n", "Manning roughness coefficient", ("Manning n", "roughness"))],
     [_o("yn", "length", "normal depth (m)"), _o("V", "speed", "velocity at normal depth (m/s)"),
      _o("A", "area", "flow area (m^2)"), _o("Fr", "number", "Froude number on the hydraulic depth")],
     normal_depth_trap, _ref_normal_trap,
     lambda Q, b, z, S, n: (lambda y: {"yn": _r(y), "V": 0.0, "A": 0.0, "Fr": 0.0})(_bisect_root(
         lambda y: (b + z * y) * y * _manning_v(y, S, n) - Q, 0.0, 1e4)),           # wide-channel R = y
     lambda rng: {"Q": 10 ** rng.uniform(-1, 2.3), "b": rng.uniform(0.5, 10), "z": rng.uniform(0.5, 3),
                  "S": 10 ** rng.uniform(-4.5, -2), "n": rng.uniform(0.011, 0.05)},
     [{"Q": 20, "b": 5, "z": 2, "S": 0.0004, "n": 0.025}, {"Q": 1.2, "b": 1, "z": 1, "S": 0.003, "n": 0.018}],
     ["normal depth in a trapezoidal channel", "normal depth trapezoidal"],
     conditions=["Q > 0", "b >= 0", "z >= 0", "0 < S < 1"], method="bisection on Manning's Q(y) = Q",
     latex=r"Q = \frac{1}{n}A R^{2/3}S^{1/2},\quad A=(b+zy)y,\ P=b+2y\sqrt{1+z^2}")


def chezy_velocity(C, R, S):
    _pos(C=C, R=R, S=S)
    _need(S < 1, "the slope S is a fraction (m/m) below 1")
    return {"V": _r(C * math.sqrt(R * S))}


proc("chezy_velocity", "Chezy velocity", "open-channel hydraulics",
     "Mean velocity of uniform flow from the Chezy equation for a Chezy coefficient C (m^(1/2)/s), hydraulic radius R "
     "and slope S.",
     [_in("C", "chezy_coefficient", "chezy", "Chezy coefficient (m^(1/2)/s)", ("Chezy C", "Chezy coefficient"), "m^0.5/s"),
      _L("R", "hydraulic radius", ("hydraulic radius",)), _F("S", "channel (energy) slope, m/m", ("slope", "bed slope"))],
     [_o("V", "speed", "mean velocity (m/s)")], chezy_velocity,
     lambda C, R, S: {"V": _fluids().V_Chezy(R, S, C)}, lambda C, R, S: {"V": _r(C * R * math.sqrt(S))},
     lambda rng: {"C": rng.uniform(20, 90), "R": rng.uniform(0.05, 5), "S": 10 ** rng.uniform(-5, -1.5)},
     [{"C": 55, "R": 1.1, "S": 0.0004}, {"C": 40, "R": 0.5, "S": 0.002}], ["chezy velocity", "chezy equation"],
     conditions=["C, R > 0", "0 < S < 1"], method="V = C sqrt(R S)", latex=r"V = C\sqrt{R S}")


def chezy_from_manning(n, R):
    _pos(R=R)
    _ok_mann(0.5, n)
    return {"C": _r(R ** (1.0 / 6.0) / n)}


proc("chezy_from_manning", "Chezy coefficient from Manning's n", "open-channel hydraulics",
     "The Chezy coefficient equivalent to a Manning roughness n at hydraulic radius R (SI units).",
     [_N("n", "Manning roughness coefficient", ("Manning n", "roughness")), _L("R", "hydraulic radius", ("hydraulic radius",))],
     [_o("C", "chezy_coefficient", "Chezy coefficient (m^(1/2)/s)")], chezy_from_manning,
     lambda n, R: {"C": _fluids().n_Manning_to_C_Chezy(n, R)}, lambda n, R: {"C": _r(R ** (1.0 / 3.0) / n)},
     lambda rng: {"n": rng.uniform(0.01, 0.06), "R": rng.uniform(0.05, 5)},
     [{"n": 0.013, "R": 0.75}, {"n": 0.03, "R": 2.0}], ["chezy coefficient from manning", "chezy c from manning n"],
     conditions=["0.005 <= n <= 0.2", "R > 0"], method="C = R^(1/6)/n", latex=r"C = \frac{R^{1/6}}{n}")


def manning_from_chezy(C, R):
    _pos(C=C, R=R)
    n = R ** (1.0 / 6.0) / C
    _need(0.005 <= n <= 0.2, "the equivalent Manning n falls outside 0.005 to 0.2")
    return {"n": _r(n)}


proc("manning_from_chezy", "Manning's n from a Chezy coefficient", "open-channel hydraulics",
     "The Manning roughness n equivalent to a Chezy coefficient C at hydraulic radius R (SI units).",
     [_in("C", "chezy_coefficient", "chezy", "Chezy coefficient (m^(1/2)/s)", ("Chezy C", "Chezy coefficient"), "m^0.5/s"),
      _L("R", "hydraulic radius", ("hydraulic radius",))],
     [_o("n", "number", "Manning roughness coefficient")], manning_from_chezy,
     lambda C, R: {"n": _fluids().C_Chezy_to_n_Manning(C, R)}, lambda C, R: {"n": _r(R ** (1.0 / 3.0) / C)},
     lambda rng: {"C": rng.uniform(20, 90), "R": rng.uniform(0.1, 5)},
     [{"C": 60, "R": 1.2}, {"C": 35, "R": 0.4}], ["manning n from chezy", "manning roughness from chezy"],
     conditions=["C, R > 0"], method="n = R^(1/6)/C", latex=r"n = \frac{R^{1/6}}{C}")


# --- Froude number, critical depth, specific energy ----------------------------------------------------------------
def _regime(Fr):
    if abs(Fr - 1) <= 1e-9:
        return "critical"
    return "subcritical" if Fr < 1 else "supercritical"


def froude_rect(Q, b, y):
    _pos(Q=Q, b=b, y=y)
    V = Q / (b * y)
    Fr = V / math.sqrt(G * y)
    return {"Fr": _r(Fr), "V": _r(V), "regime": _regime(Fr)}


def _ref_froude(Q, b, y):
    V = Q / (b * y)
    Fr = _fluids().Froude(V, y, g=G)
    return {"Fr": Fr, "V": V, "regime": _regime(Fr)}


proc("channel_froude_number", "Froude number of flow in a rectangular channel", "open-channel hydraulics",
     "Froude number V/sqrt(g y) of flow at depth y in a rectangular channel, and whether the flow is subcritical, "
     "critical or supercritical.",
     [_Qin(), _L("b", "channel width", ("width", "bottom width")), _L("y", "flow depth", ("depth", "flow depth"))],
     [_o("Fr", "number", "Froude number"), _o("V", "speed", "mean velocity (m/s)"),
      _o("regime", "number", "flow regime: subcritical, critical or supercritical")],
     froude_rect, _ref_froude,
     lambda Q, b, y: (lambda V: {"Fr": _r(V * V / (G * y)), "V": _r(V), "regime": _regime(V * V / (G * y))})(Q / (b * y)),
     lambda rng: {"Q": 10 ** rng.uniform(-1, 2), "b": rng.uniform(0.5, 10), "y": rng.uniform(0.1, 3)},
     [{"Q": 12, "b": 4, "y": 0.5}, {"Q": 3, "b": 2.5, "y": 1.4}], ["froude number in a rectangular channel",
                                                                     "channel froude number", "open channel froude number"],
     conditions=["Q, b, y > 0"], method="V = Q/(b y), Fr = V/sqrt(g y)", latex=r"Fr = \frac{V}{\sqrt{g\,y}}")


def _sym_critical(Afun, ylo, yhi, Q):
    """Reference critical depth: the minimum of the specific energy E(y) = y + Q^2/(2 g A^2), found as the root of
    dE/dy obtained by symbolic differentiation (sympy) rather than from the Froude condition."""
    import sympy as sp
    y = sp.Symbol("y", positive=True)
    E = y + Q ** 2 / (2 * G * Afun(y) ** 2)
    dE = sp.lambdify(y, sp.diff(E, y), "mpmath")
    yc = _brentq(lambda v: float(dE(v)), ylo, yhi)
    return yc, float(E.subs(y, yc))


def critical_rect(Q, b):
    _pos(Q=Q, b=b)
    q = Q / b
    yc = (q * q / G) ** (1.0 / 3.0)
    return {"yc": _r(yc), "Vc": _r(q / yc), "Emin": _r(1.5 * yc), "q": _r(q)}


def _ref_critical_rect(Q, b):
    yc, E = _sym_critical(lambda y: b * y, 1e-9, 1e3, Q)
    return {"yc": yc, "Vc": Q / (b * yc), "Emin": E, "q": Q / b}


proc("critical_depth_rectangular", "Critical depth in a rectangular channel", "open-channel hydraulics",
     "Critical depth, critical velocity and minimum specific energy for a discharge Q in a rectangular channel.",
     [_Qin(), _L("b", "channel width", ("width", "bottom width"))],
     [_o("yc", "length", "critical depth (m)"), _o("Vc", "speed", "critical velocity (m/s)"),
      _o("Emin", "length", "minimum specific energy 1.5 yc (m)"),
      _o("q", "volume_transport_per_width", "discharge per unit width (m^2/s)")],
     critical_rect, _ref_critical_rect,
     lambda Q, b: (lambda q: (lambda yc: {"yc": _r(yc), "Vc": _r(q / yc), "Emin": _r(1.5 * yc), "q": _r(q)})(
         (q * q / (2 * G)) ** (1.0 / 3.0)))(Q / b),
     lambda rng: {"Q": 10 ** rng.uniform(-1.5, 2.5), "b": rng.uniform(0.3, 20)},
     [{"Q": 8, "b": 3}, {"Q": 0.5, "b": 1.2}], ["critical depth in a rectangular channel", "critical depth rectangular"],
     conditions=["Q, b > 0"], method="yc = (q^2/g)^(1/3), q = Q/b", latex=r"y_c = \left(\frac{q^2}{g}\right)^{1/3},\quad E_{min} = \tfrac{3}{2}y_c")


def critical_trap(Q, b, z):
    _pos(Q=Q)
    _finite(b=b, z=z)
    _need(b >= 0 and z > 0, "need b >= 0 and z > 0 (z = 0 is a rectangular channel)")
    f = lambda y: Q * Q * (b + 2 * z * y) - G * ((b + z * y) * y) ** 3           # noqa: E731
    yc = _bisect_root(f, 1e-12, 1e4)
    A = (b + z * yc) * yc
    return {"yc": _r(yc), "Ac": _r(A), "Vc": _r(Q / A), "Emin": _r(yc + (Q / A) ** 2 / (2 * G))}


def _ref_critical_trap(Q, b, z):
    yc, E = _sym_critical(lambda y: (b + z * y) * y, 1e-9, 1e3, Q)
    A = (b + z * yc) * yc
    return {"yc": yc, "Ac": A, "Vc": Q / A, "Emin": E}


proc("critical_depth_trapezoidal", "Critical depth in a trapezoidal channel", "open-channel hydraulics",
     "Critical depth, area, velocity and minimum specific energy for a discharge Q in a trapezoidal (or triangular, "
     "b = 0) channel, from Q^2 T = g A^3.",
     [_Qin(), _L("b", "bottom width", ("width", "bottom width")), _N("z", "side slope, horizontal per unit vertical", ("side slope",))],
     [_o("yc", "length", "critical depth (m)"), _o("Ac", "area", "critical flow area (m^2)"),
      _o("Vc", "speed", "critical velocity (m/s)"), _o("Emin", "length", "minimum specific energy (m)")],
     critical_trap, _ref_critical_trap,
     lambda Q, b, z: (lambda yc: {"yc": _r(yc), "Ac": _r((b + z * yc) * yc), "Vc": 0.0, "Emin": 0.0})(_bisect_root(
         lambda y: Q * Q * (b + z * y) - G * ((b + z * y) * y) ** 3, 1e-12, 1e4)),   # top width written b + z y
     lambda rng: {"Q": 10 ** rng.uniform(-1, 2.3), "b": rng.uniform(0.5, 10), "z": rng.uniform(0.5, 3)},
     [{"Q": 15, "b": 3, "z": 2}, {"Q": 0.8, "b": 0, "z": 1}], ["critical depth in a trapezoidal channel",
                                                               "critical depth trapezoidal"],
     conditions=["Q > 0", "b >= 0", "z > 0"], method="bisection on Q^2 T(y) = g A(y)^3",
     latex=r"\frac{Q^2\,T}{g\,A^3} = 1,\quad A = (b+zy)y,\ T = b+2zy",
     extra_verify=lambda i, o: _rel(i["Q"] ** 2 * (i["b"] + 2 * i["z"] * o["yc"]), G * o["Ac"] ** 3))


def critical_circ(Q, D):
    _pos(Q=Q, D=D)

    def f(y):
        A, _P, T, _th = _circ(D, y)
        return Q * Q * T - G * A ** 3
    _need(f(0.95 * D) < 0, "the critical depth would exceed 0.95 D: the conduit is too small for this discharge")
    yc = _bisect_root(f, 1e-12 * D, 0.95 * D)
    A = _circ(D, yc)[0]
    return {"yc": _r(yc), "Ac": _r(A), "Vc": _r(Q / A), "Emin": _r(yc + (Q / A) ** 2 / (2 * G))}


def _ref_critical_circ(Q, D):
    import sympy as sp
    Afun = lambda y: (lambda th: D ** 2 / 8 * (th - sp.sin(th)))(2 * sp.acos(1 - 2 * y / D))     # noqa: E731
    yc, E = _sym_critical(Afun, 1e-6 * D, 0.999 * D, Q)
    A = _ref_circ(D, yc)["A"]
    return {"yc": yc, "Ac": A, "Vc": Q / A, "Emin": E}


proc("critical_depth_circular", "Critical depth in a circular conduit", "open-channel hydraulics",
     "Critical depth, area, velocity and minimum specific energy for a discharge Q in a circular pipe or culvert "
     "flowing part full (refused when the critical depth would exceed 0.95 D).",
     [_Qin(), _L("D", "pipe diameter", ("diameter",))],
     [_o("yc", "length", "critical depth (m)"), _o("Ac", "area", "critical flow area (m^2)"),
      _o("Vc", "speed", "critical velocity (m/s)"), _o("Emin", "length", "minimum specific energy (m)")],
     critical_circ, _ref_critical_circ,
     lambda Q, D: (lambda r: dict(r, yc=_r((Q * Q / (G * D * D)) ** (1 / 3))))(critical_circ(Q, D)),  # as if rectangular, width D
     lambda rng: (lambda D: {"D": D, "Q": math.sqrt(G) * D ** 2.5 * rng.uniform(0.02, 0.9)})(rng.uniform(0.2, 3)),
     [{"Q": 0.5, "D": 1.0}, {"Q": 0.05, "D": 0.4}], ["critical depth in a circular pipe", "critical depth circular",
                                                      "critical depth in a culvert"],
     conditions=["Q, D > 0", "critical depth below 0.95 D"], invalid=[{"Q": 20, "D": 1.0}],
     method="bisection on Q^2 T(y) = g A(y)^3 with circular-segment geometry",
     latex=r"\frac{Q^2\,T}{g\,A^3} = 1,\quad A = \frac{D^2}{8}(\theta-\sin\theta),\ T = D\sin\frac{\theta}{2}")


def alternate_depth(Q, b, y):
    _pos(Q=Q, b=b, y=y)
    q = Q / b
    yc = (q * q / G) ** (1.0 / 3.0)
    E = y + q * q / (2 * G * y * y)
    Fr = q / (y * math.sqrt(G * y))
    _need(abs(Fr - 1) > 1e-3, "the flow is at critical depth: there is no distinct alternate depth")
    f = lambda v: v + q * q / (2 * G * v * v) - E                      # noqa: E731
    y2 = _bisect_root(f, yc, E) if y < yc else _bisect_root(f, 1e-300 + yc * 1e-9, yc)
    return {"E": _r(E), "y_alt": _r(y2), "yc": _r(yc), "Fr": _r(Fr), "regime": _regime(Fr)}


def _ref_alternate(Q, b, y):
    import numpy as np
    q = Q / b
    E = y + q * q / (2 * G * y * y)
    roots = [r.real for r in np.roots([1.0, -E, 0.0, q * q / (2 * G)]) if abs(r.imag) < 1e-9 and r.real > 0]
    y2 = max(roots, key=lambda r: abs(r - y))
    # polish the root on the cubic itself (numpy's companion-matrix roots carry ~1e-15 relative error)
    poly = lambda v: v ** 3 - E * v * v + q * q / (2 * G)                 # noqa: E731
    lo, hi = y2 * (1 - 1e-9), y2 * (1 + 1e-9)
    if poly(lo) * poly(hi) < 0:
        y2 = _brentq(poly, lo, hi)
    yc = float(np.cbrt(q * q / G))
    Fr = _fluids().Froude(q / y, y, g=G)
    return {"E": E, "y_alt": y2, "yc": yc, "Fr": Fr, "regime": _regime(Fr)}


proc("alternate_depth_rectangular", "Specific energy and alternate depth in a rectangular channel", "open-channel hydraulics",
     "The specific energy of flow at depth y in a rectangular channel and the alternate depth with the same specific "
     "energy on the other side of critical depth.",
     [_Qin(), _L("b", "channel width", ("width", "bottom width")), _L("y", "flow depth", ("depth", "flow depth"))],
     [_o("E", "length", "specific energy y + V^2/2g (m)"), _o("y_alt", "length", "alternate depth (m)"),
      _o("yc", "length", "critical depth (m)"), _o("Fr", "number", "Froude number at depth y"),
      _o("regime", "number", "flow regime at depth y")],
     alternate_depth, _ref_alternate,
     lambda Q, b, y: (lambda r: dict(r, E=_r(y + (Q / b) ** 2 / (G * y * y))))(alternate_depth(Q, b, y)),
     lambda rng: (lambda Q, b: (lambda yc: {"Q": Q, "b": b, "y": yc * rng.choice([rng.uniform(0.3, 0.8), rng.uniform(1.25, 3)])})(
         ((Q / b) ** 2 / G) ** (1 / 3)))(10 ** rng.uniform(-1, 2), rng.uniform(0.5, 10)),
     [{"Q": 6, "b": 2, "y": 0.4}, {"Q": 6, "b": 2, "y": 1.8}], ["specific energy and alternate depth",
                                                                "alternate depth in a rectangular channel",
                                                                "specific energy in a rectangular channel"],
     conditions=["Q, b, y > 0", "y is not the critical depth"],
     method="E = y + q^2/(2 g y^2); the other positive root of y^3 - E y^2 + q^2/(2g) = 0 by bisection",
     latex=r"E = y + \frac{q^2}{2 g y^2},\quad y_2^3 - E\,y_2^2 + \frac{q^2}{2g} = 0")


# --- hydraulic jump -----------------------------------------------------------------------------------------------
def hydraulic_jump(Q, b, y1):
    _pos(Q=Q, b=b, y1=y1)
    q = Q / b
    Fr1 = q / (y1 * math.sqrt(G * y1))
    _need(Fr1 > 1.0, "the upstream flow must be supercritical (Fr1 > 1) for a hydraulic jump")
    y2 = y1 / 2 * (math.sqrt(1 + 8 * Fr1 * Fr1) - 1)
    dE = (y2 - y1) ** 3 / (4 * y1 * y2)
    return {"y2": _r(y2), "Fr1": _r(Fr1), "Fr2": _r(q / (y2 * math.sqrt(G * y2))), "dE": _r(dE),
            "power_loss": _r(GAMMA_W * Q * dE), "ratio": _r(y2 / y1)}


def _ref_jump(Q, b, y1):
    q = Q / b
    M = lambda y: q * q / (G * y) + y * y / 2                      # specific force per unit width   # noqa: E731
    yc = (q * q / G) ** (1 / 3)
    y2 = _brentq(lambda y: M(y) - M(y1), yc, 1e4)
    E = lambda y: y + q * q / (2 * G * y * y)                     # noqa: E731
    dE = E(y1) - E(y2)
    fl = _fluids()
    return {"y2": y2, "Fr1": fl.Froude(q / y1, y1, g=G), "Fr2": fl.Froude(q / y2, y2, g=G), "dE": dE,
            "power_loss": RHO_W * G * Q * dE, "ratio": y2 / y1}


proc("hydraulic_jump_rectangular", "Hydraulic jump in a rectangular channel", "open-channel hydraulics",
     "Sequent (conjugate) depth, energy loss and dissipated power of a hydraulic jump in a horizontal rectangular "
     "channel (Belanger's equation from the momentum balance).",
     [_Qin(), _L("b", "channel width", ("width", "bottom width")),
      _L("y1", "upstream (supercritical) depth", ("upstream depth", "initial depth"))],
     [_o("y2", "length", "sequent depth after the jump (m)"), _o("Fr1", "number", "upstream Froude number"),
      _o("Fr2", "number", "downstream Froude number"), _o("dE", "length", "head lost in the jump (m)"),
      _o("power_loss", "power", "power dissipated gamma Q dE (W)"), _o("ratio", "number", "depth ratio y2/y1")],
     hydraulic_jump, _ref_jump,
     lambda Q, b, y1: (lambda r: (lambda y2: dict(r, y2=_r(y2), dE=_r((y2 - y1) ** 3 / (4 * y1 * y2))))(
         y1 / 2 * (math.sqrt(1 + 8 * r["Fr1"] ** 2) + 1)))(hydraulic_jump(Q, b, y1)),
     lambda rng: (lambda b, y1: {"b": b, "y1": y1, "Q": b * y1 * math.sqrt(G * y1) * rng.uniform(1.3, 9)})(
         rng.uniform(0.5, 10), rng.uniform(0.05, 1.5)),
     [{"Q": 10, "b": 2, "y1": 0.4}, {"Q": 3, "b": 1.5, "y1": 0.25}], ["hydraulic jump in a rectangular channel",
                                                                     "hydraulic jump", "sequent depth"],
     conditions=["horizontal, frictionless bed", "Fr1 > 1"], invalid=[{"Q": 1, "b": 2, "y1": 1.0}],
     method="y2 = (y1/2)(sqrt(1 + 8 Fr1^2) - 1), dE = (y2 - y1)^3/(4 y1 y2)",
     latex=r"\frac{y_2}{y_1} = \tfrac12\left(\sqrt{1+8Fr_1^2}-1\right),\quad \Delta E = \frac{(y_2-y_1)^3}{4y_1y_2}")


# --- gradually varied flow ------------------------------------------------------------------------------------------
def gvf_length(Q, b, S0, n, y1, y2):
    import mpmath
    _pos(Q=Q, b=b, y1=y1, y2=y2)
    _ok_mann(S0, n)
    _need(abs(y1 - y2) > 1e-6 * max(y1, y2), "the two depths must differ")
    q = Q / b
    yc = (q * q / G) ** (1.0 / 3.0)
    qf = lambda y: _manning_v(b * y / (b + 2 * y), S0, n) * b * y        # noqa: E731
    yn = _bisect_root(lambda y: qf(y) - Q, 0.0, 1e4)
    lo, hi = min(y1, y2), max(y1, y2)
    for c, nm in ((yn, "normal"), (yc, "critical")):
        _need(not (lo * 0.98 < c < hi * 1.02), f"the depths must stay on one side of (and 2% away from) the {nm} depth")
    zone = 1 if lo > max(yn, yc) else (3 if hi < min(yn, yc) else 2)
    kind = "M" if yn > yc else "S"

    def dxdy(y):
        A = b * y
        R = A / (b + 2 * y)
        Sf = (n * Q / A) ** 2 / R ** (4.0 / 3.0)
        Fr2 = Q * Q / (G * A * A * y)
        return (1 - Fr2) / (S0 - Sf)
    with mpmath.workdps(15):
        L = float(mpmath.quad(lambda y: dxdy(float(y)), [y1, y2]))
    return {"L": _r(L), "yn": _r(yn), "yc": _r(yc), "profile": f"{kind}{zone}"}


def _ref_gvf(Q, b, S0, n, y1, y2):
    fl = _fluids()
    yn = _brentq(lambda y: fl.V_Manning(b * y / (b + 2 * y), S0, n) * b * y - Q, 1e-12, 1e4)
    yc = ((Q / b) ** 2 / G) ** (1 / 3)

    def dxdy(y):
        V = Q / (b * y)
        Sf = (V * n / (b * y / (b + 2 * y)) ** (2 / 3)) ** 2       # Manning solved for the friction slope
        return (1 - fl.Froude(V, y, g=G) ** 2) / (S0 - Sf)
    from scipy.integrate import quad
    L = quad(dxdy, y1, y2, epsabs=0.0, epsrel=1e-12, limit=500)[0]
    lo, hi = min(y1, y2), max(y1, y2)
    zone = 1 if lo > max(yn, yc) else (3 if hi < min(yn, yc) else 2)
    return {"L": L, "yn": yn, "yc": yc, "profile": ("M" if yn > yc else "S") + str(zone)}


proc("gradually_varied_flow_length", "Length of a gradually varied flow profile (rectangular channel)", "open-channel hydraulics",
     "Distance along a prismatic rectangular channel between two sections of a gradually varied flow profile, by "
     "integrating dx/dy = (1 - Fr^2)/(S0 - Sf) exactly (not the direct-step approximation), with the profile type "
     "(M1, M2, M3, S1, S2, S3). Positive L: the section at depth y2 lies downstream of the one at depth y1.",
     [_Qin(), _L("b", "channel width", ("width", "bottom width")), _F("S0", "bed slope, m/m (positive)", ("slope", "bed slope")),
      _N("n", "Manning roughness coefficient", ("Manning n", "roughness")),
      _L("y1", "depth at the first section", ("first depth", "depth 1")), _L("y2", "depth at the second section", ("second depth", "depth 2"))],
     [_o("L", "length", "distance from section 1 to section 2, positive downstream (m)"), _o("yn", "length", "normal depth (m)"),
      _o("yc", "length", "critical depth (m)"), _o("profile", "number", "profile type (M1 ... S3)")],
     gvf_length, _ref_gvf,
     lambda Q, b, S0, n, y1, y2: dict(gvf_length(Q, b, S0, n, y1, y2), L=_r((y2 - y1) / S0)),   # friction ignored
     lambda rng: _gvf_pick(rng, 10 ** rng.uniform(0, 2), rng.uniform(2, 10), 10 ** rng.uniform(-4, -2),
                           rng.uniform(0.012, 0.035)),
     [{"Q": 20, "b": 5, "S0": 0.0004, "n": 0.015, "y1": 4.0, "y2": 3.0}, {"Q": 20, "b": 5, "S0": 0.0004, "n": 0.015, "y1": 2.4, "y2": 1.6}],
     ["gradually varied flow length", "length of a gradually varied flow profile", "backwater curve length",
      "water surface profile length"],
     conditions=["S0 > 0 (mild or steep slope)", "prismatic rectangular channel", "both depths in one zone, 2% from yn and yc"],
     invalid=[{"Q": 20, "b": 5, "S0": 0.0004, "n": 0.015, "y1": 3.0, "y2": 2.0}],
     method="x2 - x1 = integral of (1 - Fr^2)/(S0 - Sf) dy from y1 to y2, Sf from Manning (tanh-sinh quadrature)",
     latex=r"\frac{dx}{dy} = \frac{1-Fr^2}{S_0-S_f},\quad S_f = \frac{n^2V^2}{R^{4/3}}")


def _gvf_pick(rng, Q, b, S0, n):
    q = Q / b
    yc = (q * q / G) ** (1 / 3)
    yn = _bisect_root(lambda y: _manning_v(b * y / (b + 2 * y), S0, n) * b * y - Q, 0.0, 1e4)
    lo, hi = min(yn, yc), max(yn, yc)
    zone = rng.choice([1, 2, 3])
    if zone == 1:
        a, c = hi * 1.05, hi * 2.5
    elif zone == 2:
        if hi / lo < 1.15:
            a, c = hi * 1.05, hi * 2.5
        else:
            a, c = lo * 1.03, hi * 0.97
    else:
        a, c = lo * 0.2, lo * 0.95
    y1, y2 = rng.uniform(a, c), rng.uniform(a, c)
    if abs(y1 - y2) < 0.05 * (c - a):
        y2 = a + (c - a) * (0.1 if y1 > (a + c) / 2 else 0.9)
    return {"Q": Q, "b": b, "S0": S0, "n": n, "y1": y1, "y2": y2}


# --- weirs and orifices --------------------------------------------------------------------------------------------
def rect_weir(Cd, b, H, contractions=0):
    _pos(Cd=Cd, b=b, H=H)
    _need(Cd <= 1.0, "the discharge coefficient must not exceed 1")
    _need(contractions in (0, 1, 2), "the number of end contractions is 0, 1 or 2")
    Le = b - 0.1 * contractions * H
    _need(Le > 0, "the effective crest length b - 0.1 n H must be positive")
    Q = 2.0 / 3.0 * Cd * Le * math.sqrt(2 * G) * H ** 1.5
    return {"Q": _r(Q), "L_eff": _r(Le)}


def _ref_rect_weir(Cd, b, H, contractions=0):
    Le = b - 0.1 * contractions * H
    return {"Q": _quad(lambda h: Cd * Le * math.sqrt(2 * G * h), 0.0, H), "L_eff": Le}


proc("rectangular_weir_discharge", "Sharp-crested rectangular weir discharge", "open-channel hydraulics",
     "Discharge over a sharp-crested rectangular weir for a stated discharge coefficient Cd and head H, with Francis' "
     "allowance of 0.1 H per end contraction (0, 1 or 2 contractions).",
     [_N("Cd", "discharge coefficient (stated, e.g. 0.62)", ("discharge coefficient", "coefficient of discharge")),
      _L("b", "crest length", ("crest length", "weir length", "length")), _L("H", "head over the crest", ("head",)),
      _in("contractions", "number", "number", "number of end contractions (0, 1 or 2; default 0)",
          ("end contractions", "number of contractions"), "", required=False, typ="integer")],
     [_o("Q", "flow_rate", "discharge (m^3/s)"), _o("L_eff", "length", "effective crest length (m)")],
     rect_weir, _ref_rect_weir,
     lambda Cd, b, H, contractions=0: {"Q": _r(Cd * (b - 0.1 * contractions * H) * math.sqrt(2 * G) * H ** 1.5),
                                       "L_eff": _r(b - 0.1 * contractions * H)},
     lambda rng: {"Cd": rng.uniform(0.55, 0.7), "b": rng.uniform(0.5, 6), "H": rng.uniform(0.05, 0.8),
                  "contractions": rng.choice([0, 1, 2])},
     [{"Cd": 0.62, "b": 2, "H": 0.35}, {"Cd": 0.6, "b": 1.5, "H": 0.4, "contractions": 2}],
     ["rectangular weir discharge", "sharp crested weir discharge", "rectangular weir"],
     conditions=["Cd stated", "free (unsubmerged) nappe", "b - 0.1 n H > 0"], invalid=[{"Cd": 1.4, "b": 2, "H": 0.35}],
     method="Q = (2/3) Cd L_eff sqrt(2g) H^(3/2), L_eff = b - 0.1 n H",
     latex=r"Q = \tfrac{2}{3}\,C_d\,L_{e}\sqrt{2g}\,H^{3/2},\quad L_e = b - 0.1\,nH")


def vnotch_weir(Cd, theta, H):
    _pos(Cd=Cd, theta=theta, H=H)
    _need(Cd <= 1.0, "the discharge coefficient must not exceed 1")
    _need(theta < 180, "the notch angle must be below 180 degrees")
    Q = 8.0 / 15.0 * Cd * math.sqrt(2 * G) * math.tan(math.radians(theta) / 2) * H ** 2.5
    return {"Q": _r(Q)}


def _ref_vnotch(Cd, theta, H):
    t = math.tan(math.radians(theta) / 2)
    # strips at depth h below the free surface: width 2 t (H - h), velocity sqrt(2 g h)
    return {"Q": _quad(lambda h: Cd * 2 * t * (H - h) * math.sqrt(2 * G * h), 0.0, H)}


proc("v_notch_weir_discharge", "V-notch (triangular) weir discharge", "open-channel hydraulics",
     "Discharge over a sharp-crested triangular (V-notch) weir of notch angle theta for a stated discharge coefficient.",
     [_N("Cd", "discharge coefficient (stated, e.g. 0.58)", ("discharge coefficient", "coefficient of discharge")),
      _DEG("theta", "notch angle (degrees)", ("notch angle", "angle")), _L("H", "head above the vertex", ("head",))],
     [_o("Q", "flow_rate", "discharge (m^3/s)")], vnotch_weir, _ref_vnotch,
     lambda Cd, theta, H: {"Q": _r(8.0 / 15.0 * Cd * math.sqrt(2 * G) * math.tan(math.radians(theta)) * H ** 2.5)},
     lambda rng: {"Cd": rng.uniform(0.55, 0.62), "theta": rng.uniform(20, 100), "H": rng.uniform(0.03, 0.6)},
     [{"Cd": 0.58, "theta": 90, "H": 0.25}, {"Cd": 0.6, "theta": 60, "H": 0.15}],
     ["v notch weir discharge", "triangular weir discharge", "v notch weir"],
     conditions=["Cd stated", "0 < theta < 180 degrees", "free nappe"], invalid=[{"Cd": 0.58, "theta": 200, "H": 0.25}],
     method="Q = (8/15) Cd sqrt(2g) tan(theta/2) H^(5/2)",
     latex=r"Q = \tfrac{8}{15}\,C_d\sqrt{2g}\,\tan\frac{\theta}{2}\,H^{5/2}")


def broad_weir(Cd, b, H):
    _pos(Cd=Cd, b=b, H=H)
    _need(Cd <= 1.2, "the discharge coefficient must not exceed 1.2")
    yc = 2.0 * H / 3.0
    return {"Q": _r(Cd * b * math.sqrt(G) * yc ** 1.5), "yc": _r(yc)}


def _ref_broad_weir(Cd, b, H):
    from scipy.optimize import minimize_scalar
    # critical flow on the crest: the discharge per width q(y) = y sqrt(2 g (H - y)) is a maximum (its value is
    # insensitive to the location of the maximum, so it is found to full precision)
    r = minimize_scalar(lambda y: -y * math.sqrt(2 * G * (H - y)), bounds=(0.0, H), method="bounded",
                        options={"xatol": 1e-14 * H})
    return {"Q": Cd * b * (-r.fun)}


proc("broad_crested_weir_discharge", "Broad-crested weir discharge", "open-channel hydraulics",
     "Discharge over a broad-crested weir for a stated discharge coefficient (critical depth 2H/3 on the crest, "
     "approach velocity head included in H).",
     [_N("Cd", "discharge coefficient (stated, e.g. 0.95)", ("discharge coefficient", "coefficient of discharge")),
      _L("b", "crest width", ("crest width", "width", "crest length")), _L("H", "upstream head over the crest", ("head",))],
     [_o("Q", "flow_rate", "discharge (m^3/s)"), _o("yc", "length", "critical depth on the crest 2H/3 (m)")],
     broad_weir, _ref_broad_weir,
     lambda Cd, b, H: {"Q": _r(Cd * b * math.sqrt(2 * G) * (2 * H / 3) ** 1.5), "yc": _r(2 * H / 3)},
     lambda rng: {"Cd": rng.uniform(0.85, 1.0), "b": rng.uniform(0.5, 20), "H": rng.uniform(0.1, 2)},
     [{"Cd": 0.95, "b": 5, "H": 0.6}, {"Cd": 1.0, "b": 2, "H": 0.3}],
     ["broad crested weir discharge", "broad crested weir"],
     conditions=["Cd stated", "critical flow on the crest", "H includes the approach velocity head"],
     method="Q = Cd b sqrt(g) (2H/3)^(3/2)", latex=r"Q = C_d\,b\sqrt{g}\left(\tfrac{2}{3}H\right)^{3/2}")


def orifice_discharge(Cd, d, h):
    _pos(Cd=Cd, d=d, h=h)
    _need(Cd <= 1.0, "the discharge coefficient must not exceed 1")
    a = math.pi * d * d / 4
    v = math.sqrt(2 * G * h)
    return {"Q": _r(Cd * a * v), "V_ideal": _r(v), "area": _r(a)}


def _ref_orifice(Cd, d, h):
    fl = _fluids()
    # the library's orifice-meter equation with a pipe so large that the velocity of approach vanishes
    D = d * 1e4
    m = fl.flow_meter_discharge(D=D, Do=d, P1=1e5 + RHO_W * G * h, P2=1e5, rho=RHO_W, C=Cd, expansibility=1.0)
    return {"Q": m / RHO_W, "V_ideal": math.sqrt(2 * G * h), "area": math.pi * d * d / 4}


proc("orifice_discharge", "Discharge through a small orifice under a head", "open-channel hydraulics",
     "Discharge through a small sharp-edged orifice of diameter d under a constant head h for a stated discharge "
     "coefficient (Torricelli's velocity times Cd times the orifice area).",
     [_N("Cd", "discharge coefficient (stated, e.g. 0.61)", ("discharge coefficient", "coefficient of discharge")),
      _L("d", "orifice diameter", ("diameter", "orifice diameter"), "mm"), _L("h", "head above the orifice centre", ("head",))],
     [_o("Q", "flow_rate", "discharge (m^3/s)"), _o("V_ideal", "speed", "ideal jet velocity sqrt(2 g h) (m/s)"),
      _o("area", "area", "orifice area (m^2)")],
     orifice_discharge, _ref_orifice,
     lambda Cd, d, h: (lambda a: {"Q": _r(Cd * a * math.sqrt(G * h)), "V_ideal": _r(math.sqrt(G * h)), "area": _r(a)})(math.pi * d * d / 4),
     lambda rng: {"Cd": rng.uniform(0.58, 0.65), "d": rng.uniform(0.005, 0.3), "h": rng.uniform(0.2, 15)},
     [{"Cd": 0.61, "d": 50, "h": 3}, {"Cd": 0.6, "d": 100, "h": 1.2}], ["orifice discharge", "flow through an orifice",
                                                                        "discharge through an orifice"],
     conditions=["small orifice (d much smaller than h)", "free jet", "Cd stated"], invalid=[{"Cd": 1.3, "d": 50, "h": 3}],
     method="Q = Cd (pi d^2/4) sqrt(2 g h)", latex=r"Q = C_d\,\frac{\pi d^2}{4}\sqrt{2gh}")


# =============================================================================================== pipes and networks
def _hw_k(L, D, C):
    return HW_K * L / (C ** 1.852 * D ** 4.871)


def _hw_loss(k, Q):
    return math.copysign(k * abs(Q) ** 1.852, Q)


def _ok_pipe(L, D, C):
    _pos(L=L, D=D, C=C)
    _need(40 <= C <= 160, "the Hazen-Williams C must lie between 40 and 160")
    _need(D <= 5, "the diameter must not exceed 5 m")


def hazen_williams_loss(Q, D, L, C):
    _pos(Q=Q)
    _ok_pipe(L, D, C)
    hf = _hw_k(L, D, C) * Q ** 1.852
    return {"hf": _r(hf), "V": _r(Q / (math.pi * D * D / 4)), "Sf": _r(hf / L)}


def _wntr_net(reservoirs, junctions, pipes):
    import wntr
    wn = wntr.network.WaterNetworkModel()
    off = 1000.0                                    # a datum 1000 m down: WNTR failed to converge with a source head of 0
    for name, head in reservoirs:
        wn.add_reservoir(name, base_head=head + off)
    for name, demand in junctions:
        wn.add_junction(name, base_demand=demand, elevation=0.0)
    for name, a, b, L, D, C in pipes:
        wn.add_pipe(name, a, b, length=L, diameter=D, roughness=C, minor_loss=0.0)
    wn.options.hydraulic.headloss = "H-W"
    wn.options.time.duration = 0
    import warnings
    for tol in (1e-13, 1e-12, 1e-11, 1e-10):        # the tightest residual the Newton solver reaches in floating point
        with warnings.catch_warnings():
            warnings.simplefilter("ignore")
            res = wntr.sim.WNTRSimulator(wn).run_sim(solver_options={"TOL": tol, "MAXITER": 300}, HW_approx="piecewise")
        if len(res.node["head"].index):
            return res.node["head"].loc[0] - off, res.link["flowrate"].loc[0]
    raise RuntimeError("the network solver did not converge")


def _ref_hw_loss(Q, D, L, C):
    heads, _f = _wntr_net([("R", 0.0)], [("J", Q)], [("P", "R", "J", L, D, C)])
    hf = -float(heads["J"])
    return {"hf": hf, "V": Q / (math.pi * D * D / 4), "Sf": hf / L}


PIPE_IN = [_L("D", "pipe diameter", ("diameter",), "mm"), _L("L", "pipe length", ("length", "pipe length")),
           _N("C", "Hazen-Williams roughness coefficient", ("Hazen-Williams C", "roughness coefficient", "C factor"))]

proc("hazen_williams_head_loss", "Hazen-Williams head loss in a pipe", "pipe flow and water networks",
     "Friction head loss, velocity and friction slope for water flowing at Q through a pipe, from the Hazen-Williams "
     "formula (SI form used by EPANET).",
     [_Qin(show="L/s")] + PIPE_IN,
     [_o("hf", "length", "friction head loss (m)"), _o("V", "speed", "mean velocity (m/s)"),
      _o("Sf", "number", "friction slope hf/L (m/m)")],
     hazen_williams_loss, _ref_hw_loss,
     lambda Q, D, L, C: (lambda hf: {"hf": _r(hf), "V": _r(Q / (math.pi * D * D / 4)), "Sf": _r(hf / L)})(
         4.727 * L * Q ** 1.852 / (C ** 1.852 * D ** 4.871)),
     lambda rng: (lambda D: {"Q": rng.uniform(0.3, 3) * math.pi * D * D / 4, "D": D, "L": rng.uniform(10, 5000),
                             "C": rng.uniform(80, 150)})(rng.uniform(0.1, 1.2)),
     [{"Q": 50, "D": 300, "L": 1000, "C": 120}, {"Q": 8, "D": 100, "L": 250, "C": 130}],
     ["hazen williams head loss", "head loss by hazen williams"],
     conditions=["water near 15 C, turbulent flow", "40 <= C <= 160"], invalid=[{"Q": 50, "D": 300, "L": 1000, "C": 400}],
     method="hf = 10.6668 L Q^1.852/(C^1.852 D^4.871)",
     latex=r"h_f = \frac{10.67\,L\,Q^{1.852}}{C^{1.852}D^{4.871}}")


def hazen_williams_flow(hf, D, L, C):
    _pos(hf=hf)
    _ok_pipe(L, D, C)
    Q = (hf / _hw_k(L, D, C)) ** (1 / 1.852)
    return {"Q": _r(Q), "V": _r(Q / (math.pi * D * D / 4))}


def _ref_hw_flow(hf, D, L, C):
    _h, flows = _wntr_net([("R1", 0.0), ("R2", -hf)], [], [("P", "R1", "R2", L, D, C)])
    Q = float(flows["P"])
    return {"Q": Q, "V": Q / (math.pi * D * D / 4)}


proc("hazen_williams_flow", "Pipe flow from a head loss (Hazen-Williams)", "pipe flow and water networks",
     "The discharge a pipe carries under a given friction head loss (for example between two reservoirs), from the "
     "Hazen-Williams formula.",
     [_L("hf", "friction head loss", ("head loss", "head difference"))] + PIPE_IN,
     [_o("Q", "flow_rate", "discharge (m^3/s)"), _o("V", "speed", "mean velocity (m/s)")],
     hazen_williams_flow, _ref_hw_flow,
     lambda hf, D, L, C: (lambda Q: {"Q": _r(Q), "V": _r(Q / (math.pi * D * D / 4))})((hf / _hw_k(L, D, C)) ** 0.5),
     lambda rng: (lambda D, L, C: {"D": D, "L": L, "C": C, "hf": _hw_k(L, D, C) * (rng.uniform(0.3, 3) * math.pi * D * D / 4) ** 1.852})(
         rng.uniform(0.1, 1.2), rng.uniform(10, 5000), rng.uniform(80, 150)),
     [{"hf": 5, "D": 300, "L": 1500, "C": 110}, {"hf": 12, "D": 150, "L": 800, "C": 130}],
     ["flow from head loss by hazen williams", "hazen williams flow", "pipe flow from head loss"],
     conditions=["40 <= C <= 160", "hf > 0"], method="Q = (hf C^1.852 D^4.871/(10.6668 L))^(1/1.852)",
     latex=r"Q = \left(\frac{h_f\,C^{1.852}D^{4.871}}{10.67\,L}\right)^{0.54}")


def parallel_pipes(Q, L, D, C):
    _pos(Q=Q)
    n = len(L)
    _need(n >= 2, "give at least two pipes")
    L = _vec("L", L, positive=True)
    D = [d / 1000 for d in _vec("D", D, n, positive=True)]
    C = _vec("C", C, n, positive=True)
    for i in range(n):
        _ok_pipe(L[i], D[i], C[i])
    ks = [_hw_k(L[i], D[i], C[i]) for i in range(n)]
    hf = (Q / sum(k ** (-1 / 1.852) for k in ks)) ** 1.852
    flows = [(hf / k) ** (1 / 1.852) for k in ks]
    return {"hf": _r(hf), "flows": _r(flows)}


def _ref_parallel(Q, L, D, C):
    pipes = [(f"P{i}", "R", "J", L[i], D[i] / 1000, C[i]) for i in range(len(L))]
    heads, flows = _wntr_net([("R", 0.0)], [("J", Q)], pipes)
    return {"hf": -float(heads["J"]), "flows": [float(flows[f"P{i}"]) for i in range(len(L))]}


def _rand_parallel(rng):
    n = rng.choice([2, 3, 4])
    while True:
        x = {"Q": 10 ** rng.uniform(-1.5, 0), "L": [rng.uniform(100, 3000) for _ in range(n)],
             "D": [rng.uniform(100, 800) for _ in range(n)], "C": [rng.uniform(90, 140) for _ in range(n)]}
        if min(parallel_pipes(**x)["flows"]) > 3e-3:
            return x


proc("parallel_pipes_hazen_williams", "Flow division among parallel pipes (Hazen-Williams)", "pipe flow and water networks",
     "How a total flow Q divides among two or more pipes in parallel between the same two junctions, and the common "
     "head loss, from the Hazen-Williams formula.",
     [_Qin(meaning="total flow", names=("total flow", "flow", "discharge")),
      _VEC("L", "pipe lengths in m (one per pipe)", ("lengths",), min_length=2),
      _VEC("D", "pipe diameters in mm (one per pipe)", ("diameters",), min_length=2),
      _VEC("C", "Hazen-Williams C of each pipe", ("C factors", "roughness coefficients"), min_length=2)],
     [_o("hf", "length", "head loss between the junctions (m)"), _o("flows", "flow_rate", "flow in each pipe (m^3/s)")],
     parallel_pipes, _ref_parallel,
     lambda Q, L, D, C: (lambda w: {"hf": _r(_hw_k(L[0], D[0] / 1000, C[0]) * (Q * w[0] / sum(w)) ** 1.852),
                                    "flows": _r([Q * x / sum(w) for x in w])})(
         [1 / _hw_k(L[i], D[i] / 1000, C[i]) for i in range(len(L))]),
     _rand_parallel,
     [{"Q": 0.3, "L": [800, 1200], "D": [300, 400], "C": [120, 110]},
      {"Q": 0.15, "L": [500, 650, 900], "D": [200, 250, 300], "C": [130, 120, 100]}],
     ["parallel pipes", "flow division in parallel pipes", "pipes in parallel"],
     conditions=["the same head loss across every pipe", "40 <= C <= 160", "lists of equal length"],
     method="hf = (Q / sum k_i^(-1/1.852))^1.852 with k_i = 10.6668 L_i/(C_i^1.852 D_i^4.871); Q_i = (hf/k_i)^(1/1.852)",
     latex=r"Q = \sum_i \left(\frac{h_f}{k_i}\right)^{0.54},\quad k_i = \frac{10.67L_i}{C_i^{1.852}D_i^{4.871}}")


# --- Hardy Cross network balancing ------------------------------------------------------------------------------------
def _solve_linear(A, b):
    n = len(b)
    M = [row[:] + [b[i]] for i, row in enumerate(A)]
    for c in range(n):
        p = max(range(c, n), key=lambda r: abs(M[r][c]))
        _need(abs(M[p][c]) > 1e-300, "singular loop system")
        M[c], M[p] = M[p], M[c]
        for r in range(c + 1, n):
            f = M[r][c] / M[c][c]
            if f:
                for k in range(c, n + 1):
                    M[r][k] -= f * M[c][k]
    x = [0.0] * n
    for r in range(n - 1, -1, -1):
        x[r] = (M[r][n] - sum(M[r][k] * x[k] for k in range(r + 1, n))) / M[r][r]
    return x


def _network_setup(pipes, L, D, C, demands):
    m = len(pipes)
    L = _vec("L", L, m, positive=True)
    D = [d / 1000 for d in _vec("D", D, m, positive=True)]
    C = _vec("C", C, m, positive=True)
    for i in range(m):
        _ok_pipe(L[i], D[i], C[i])
    nodes = sorted({int(x) for e in pipes for x in e[:2]} if all(str(x).isdigit() for e in pipes for x in e[:2]) else [-1])
    _need(nodes != [-1] and nodes == list(range(1, len(nodes) + 1)), "name the nodes 1, 2, ..., N (node 1 is the source)")
    N = len(nodes)
    dem = _vec("demands", demands, N, nonneg=True)
    _need(dem[0] == 0, "node 1 is the source: its demand entry must be 0")
    _need(sum(dem) > 0, "give at least one positive demand")
    edges = [(int(e[0]), int(e[1])) for e in pipes]
    _need(all(u != v for u, v in edges), "a pipe must join two different nodes")
    return edges, [_hw_k(L[i], D[i], C[i]) for i in range(m)], dem, N


def _tree_and_loops(edges, N):
    adj = {i: [] for i in range(1, N + 1)}
    for j, (u, v) in enumerate(edges):
        adj[u].append((v, j, 1))
        adj[v].append((u, j, -1))
    parent = {1: None}
    order = [1]
    for x in order:
        for y, j, s in adj[x]:
            if y not in parent:
                parent[y] = (x, j, s)
                order.append(y)
    _need(len(parent) == N, "the network must be connected")
    tree = {parent[x][1] for x in order[1:]}

    def path_to_root(x):
        out = []
        while parent[x] is not None:
            p, j, s = parent[x]
            out.append((j, s, p, x))
            x = p
        return out
    loops = []
    for j, (u, v) in enumerate(edges):
        if j in tree:
            continue
        # loop: u -> v along pipe j, then v -> root -> u along the tree; coefficients +1 when the pipe's direction
        # agrees with the traversal
        coef = {j: 1}
        for jj, s, p, x in path_to_root(v):          # traverse x -> p: pipe direction s=+1 means p -> x
            coef[jj] = coef.get(jj, 0) - s
        for jj, s, p, x in path_to_root(u):          # traverse p -> x
            coef[jj] = coef.get(jj, 0) + s
        loops.append({k: c for k, c in coef.items() if c})
    return parent, order, loops


def hardy_cross(pipes, L, D, C, demands):
    edges, ks, dem, N = _network_setup(pipes, L, D, C, demands)
    parent, order, loops = _tree_and_loops(edges, N)
    m = len(edges)
    Q = [0.0] * m
    sub = {x: dem[x - 1] for x in range(1, N + 1)}
    for x in reversed(order[1:]):                    # an initial flow pattern that satisfies continuity
        p, j, s = parent[x]
        Q[j] = s * sub[x]
        sub[p] += sub[x]
    scale = sum(dem)
    for j in range(m):
        if Q[j] == 0.0:
            Q[j] = 0.0
    it = 0
    for it in range(1, 2001):                        # Hardy Cross: one loop at a time
        worst = 0.0
        for lp in loops:
            num = sum(c * _hw_loss(ks[j], Q[j]) for j, c in lp.items())
            den = sum(1.852 * ks[j] * max(abs(Q[j]), 1e-12 * scale) ** 0.852 for j in lp)
            dq = -num / den
            for j, c in lp.items():
                Q[j] += c * dq
            worst = max(worst, abs(dq))
        if worst < 1e-6 * scale:
            break
    for _ in range(60):                              # simultaneous loop corrections (Newton) to full precision
        if not loops:
            break
        F = [sum(c * _hw_loss(ks[j], Q[j]) for j, c in lp.items()) for lp in loops]
        J = [[sum(c1 * lp2.get(j, 0) * 1.852 * ks[j] * abs(Q[j]) ** 0.852 for j, c1 in lp1.items()) for lp2 in loops]
             for lp1 in loops]
        dq = _solve_linear(J, [-f for f in F])
        for lp, d in zip(loops, dq):
            for j, c in lp.items():
                Q[j] += c * d
        if max(abs(d) for d in dq) <= 1e-15 * scale:
            break
    heads = {1: 0.0}
    for x in order[1:]:
        p, j, s = parent[x]
        heads[x] = heads[p] - s * _hw_loss(ks[j], Q[j])
    Q = [_snap(q, scale) for q in Q]
    return {"flows": _r(Q), "head_loss": _r([_snap(-heads[x], 1.0) for x in range(1, N + 1)]), "loops": len(loops),
            "iterations": it}


def _ref_hardy(pipes, L, D, C, demands):
    edges = [(str(int(e[0])), str(int(e[1]))) for e in pipes]
    N = len(demands)
    heads, flows = _wntr_net([("1", 0.0)], [(str(i), demands[i - 1]) for i in range(2, N + 1)],
                             [(f"P{j}", u, v, L[j], D[j] / 1000, C[j]) for j, (u, v) in enumerate(edges)])
    return {"flows": [float(flows[f"P{j}"]) for j in range(len(edges))],
            "head_loss": [0.0] + [-float(heads[str(i)]) for i in range(2, N + 1)]}


def _rand_network(rng):
    layouts = [
        [(1, 2), (2, 3), (3, 4), (4, 1)],
        [(1, 2), (2, 3), (3, 4), (4, 1), (2, 4)],
        [(1, 2), (2, 3), (1, 4), (4, 5), (2, 5), (5, 6), (3, 6)],
        [(1, 2), (2, 3), (3, 6), (6, 5), (5, 4), (4, 1), (2, 5)],
    ]
    while True:
        lay = rng.choice(layouts)
        N = max(max(e) for e in lay)
        x = {"pipes": ", ".join(f"{u}-{v}" for u, v in lay), "L": [rng.uniform(200, 1500) for _ in lay],
             "D": [rng.uniform(150, 500) for _ in lay], "C": [rng.uniform(90, 140) for _ in lay],
             "demands": [0.0] + [rng.uniform(0.01, 0.08) for _ in range(N - 1)]}
        xs = dict(x, pipes=[(str(u), str(v)) for u, v in lay])
        if min(abs(q) for q in hardy_cross(**xs)["flows"]) > 3e-3:
            return x


def _hardy_wrong(pipes, L, D, C, demands):
    # the loop correction written with Darcy-Weisbach's exponent 2 instead of 1.852 in the head losses
    edges, ks, dem, N = _network_setup(pipes, L, D, C, demands)
    parent, order, loops = _tree_and_loops(edges, N)
    Q = [0.0] * len(edges)
    sub = {x: dem[x - 1] for x in range(1, N + 1)}
    for x in reversed(order[1:]):
        p, j, s = parent[x]
        Q[j] = s * sub[x]
        sub[p] += sub[x]
    for _ in range(500):
        for lp in loops:
            num = sum(c * ks[j] * Q[j] * abs(Q[j]) for j, c in lp.items())
            den = sum(2 * ks[j] * max(abs(Q[j]), 1e-12) for j in lp)
            for j, c in lp.items():
                Q[j] += c * (-num / den)
    heads = {1: 0.0}
    for x in order[1:]:
        p, j, s = parent[x]
        heads[x] = heads[p] - s * _hw_loss(ks[j], Q[j])
    return {"flows": _r(Q), "head_loss": _r([-heads[x] for x in range(1, N + 1)]), "loops": len(loops), "iterations": 500}


proc("hardy_cross_network", "Pipe network flows by the Hardy Cross method (Hazen-Williams)", "pipe flow and water networks",
     "Steady flows in every pipe of a looped water network and the head loss from the source to every node, by "
     "Hardy Cross loop corrections (finished with simultaneous loop corrections to full precision), Hazen-Williams "
     "friction. Nodes are numbered 1..N; node 1 is the source and supplies the total demand.",
     [_in("pipes", "number", "number", "pipes as node pairs 1-2, 2-3, ... (the flow sign follows this direction)",
          ("pipe list", "links"), "", typ="edges", min_length=2),
      _VEC("L", "pipe lengths in m (in pipe order)", ("lengths",), min_length=2),
      _VEC("D", "pipe diameters in mm (in pipe order)", ("diameters",), min_length=2),
      _VEC("C", "Hazen-Williams C of each pipe (in pipe order)", ("C factors",), min_length=2),
      _VEC("demands", "demand at nodes 1..N in m^3/s (node 1, the source, takes 0)", ("node demands",), min_length=2)],
     [_o("flows", "flow_rate", "flow in each pipe, positive from its first to its second node (m^3/s)"),
      _o("head_loss", "length", "head loss from node 1 to each node (m)"), _o("loops", "number", "independent loops"),
      _o("iterations", "number", "Hardy Cross sweeps before the Newton finish")],
     hardy_cross, _ref_hardy, _hardy_wrong, _rand_network,
     [{"pipes": "1-2, 2-3, 3-4, 4-1, 2-4", "L": [600, 500, 600, 500, 780], "D": [300, 250, 200, 250, 200],
       "C": [120, 120, 120, 120, 120], "demands": [0, 0.04, 0.05, 0.03]},
      {"pipes": "1-2, 2-3, 3-4, 4-1", "L": [1000, 800, 1000, 800], "D": [400, 300, 300, 300], "C": [130, 110, 110, 120],
       "demands": [0, 0.05, 0.06, 0.04]}],
     ["hardy cross network", "hardy cross method", "pipe network analysis", "balance a pipe network"],
     conditions=["one source (node 1)", "connected network", "40 <= C <= 160", "demands >= 0"],
     method="loop corrections dQ = -sum(k Q|Q|^0.852)/(1.852 sum(k|Q|^0.852)) until converged, then Newton on all loops",
     latex=r"\Delta Q = -\frac{\sum k\,Q|Q|^{0.852}}{1.852\sum k|Q|^{0.852}},\quad k = \frac{10.67L}{C^{1.852}D^{4.871}}")


def three_reservoirs(z, L, D, C):
    z = _vec("z", z, 3)
    L = _vec("L", L, 3, positive=True)
    D = [d / 1000 for d in _vec("D", D, 3, positive=True)]
    C = _vec("C", C, 3, positive=True)
    for i in range(3):
        _ok_pipe(L[i], D[i], C[i])
    ks = [_hw_k(L[i], D[i], C[i]) for i in range(3)]
    _need(max(z) - min(z) > 1e-6, "the reservoir levels must differ")
    inflow = lambda h: sum(math.copysign((abs(z[i] - h) / ks[i]) ** (1 / 1.852), z[i] - h) for i in range(3))  # noqa: E731
    h = _bisect_root(inflow, min(z), max(z))
    flows = [math.copysign((abs(z[i] - h) / ks[i]) ** (1 / 1.852), z[i] - h) for i in range(3)]
    scale = max(abs(f) for f in flows)
    return {"h_junction": _r(h), "flows": _r([_snap(f, scale) for f in flows])}


def _ref_three(z, L, D, C):
    heads, flows = _wntr_net([("R1", z[0]), ("R2", z[1]), ("R3", z[2])], [("J", 0.0)],
                             [(f"P{i}", f"R{i + 1}", "J", L[i], D[i] / 1000, C[i]) for i in range(3)])
    return {"h_junction": float(heads["J"]), "flows": [float(flows[f"P{i}"]) for i in range(3)]}


def _rand_three(rng):
    while True:
        x = {"z": [rng.uniform(80, 150), rng.uniform(50, 120), rng.uniform(10, 60)],
             "L": [rng.uniform(300, 4000) for _ in range(3)], "D": [rng.uniform(150, 600) for _ in range(3)],
             "C": [rng.uniform(90, 140) for _ in range(3)]}
        if min(abs(f) for f in three_reservoirs(**x)["flows"]) > 3e-3:
            return x


proc("three_reservoir_problem", "Three-reservoir problem (Hazen-Williams)", "pipe flow and water networks",
     "Head at the junction of three pipes from three reservoirs and the flow in each pipe (positive toward the "
     "junction), with Hazen-Williams friction and no demand at the junction.",
     [_VEC("z", "reservoir water levels in m (three)", ("levels", "reservoir levels"), min_length=3, max_length=3),
      _VEC("L", "pipe lengths in m (three)", ("lengths",), min_length=3, max_length=3),
      _VEC("D", "pipe diameters in mm (three)", ("diameters",), min_length=3, max_length=3),
      _VEC("C", "Hazen-Williams C of each pipe (three)", ("C factors",), min_length=3, max_length=3)],
     [_o("h_junction", "length", "piezometric head at the junction (m)"),
      _o("flows", "flow_rate", "flow in each pipe toward the junction (m^3/s)")],
     three_reservoirs, _ref_three,
     lambda z, L, D, C: (lambda ks: (lambda h: {"h_junction": _r(h), "flows": _r(
         [math.copysign((abs(z[i] - h) / ks[i]) ** 0.5, z[i] - h) for i in range(3)])})(
         _bisect_root(lambda h: sum(math.copysign((abs(z[i] - h) / ks[i]) ** 0.5, z[i] - h) for i in range(3)), min(z), max(z))))(
         [_hw_k(L[i], D[i] / 1000, C[i]) for i in range(3)]),
     _rand_three,
     [{"z": [120, 90, 40], "L": [2000, 1500, 2500], "D": [400, 300, 350], "C": [120, 120, 120]},
      {"z": [100, 95, 60], "L": [1000, 1200, 800], "D": [300, 250, 300], "C": [130, 110, 100]}],
     ["three reservoir problem", "three reservoirs"],
     conditions=["no demand at the junction", "40 <= C <= 160"],
     method="bisection on the junction head h: sum of (z_i - h)/|z_i - h| (|z_i - h|/k_i)^(1/1.852) = 0",
     latex=r"\sum_{i=1}^{3}\operatorname{sgn}(z_i-h)\left(\frac{|z_i-h|}{k_i}\right)^{0.54} = 0")


# =============================================================================================== hydrology
def _mm(sym, meaning, names=(), show="mm", **kw):
    return _in(sym, "length", "mm", meaning, names, show, **kw)


def scs_runoff(P, CN, lam=0.2):
    _finite(P=P, CN=CN, lam=lam)
    _need(P >= 0, "the rainfall depth must be zero or positive")
    _need(30 <= CN <= 100, "the curve number must lie between 30 and 100")
    _need(0 <= lam <= 0.3, "the initial-abstraction ratio must lie between 0 and 0.3")
    S = 25400.0 / CN - 254.0
    Ia = lam * S
    Q = (P - Ia) ** 2 / (P - Ia + S) if P > Ia else 0.0
    return {"Q": _r(Q), "S": _r(S), "Ia": _r(Ia), "C": _r(Q / P if P > 0 else 0.0)}


def _ref_scs(P, CN, lam=0.2):
    from scipy.integrate import solve_ivp
    S = (1000.0 / CN - 10.0) * 25.4                     # retention in inches, converted to mm
    Ia = lam * S
    Pe = P - Ia
    if Pe <= 0:
        return {"Q": 0.0, "S": S, "Ia": Ia, "C": 0.0}
    # differential form of the SCS hypothesis: the retention rate dF/dPe = (1 - F/S)^2, F = Pe - Q, integrated
    sol = solve_ivp(lambda x, F: [(1 - F[0] / S) ** 2], (0.0, Pe), [0.0], method="DOP853", rtol=1e-13, atol=1e-14)
    F = sol.y[0, -1]
    return {"Q": Pe - F, "S": S, "Ia": Ia, "C": (Pe - F) / P}


proc("scs_curve_number_runoff", "SCS curve-number runoff depth", "hydrology",
     "Direct runoff depth from a storm rainfall depth by the SCS (NRCS) curve-number method, with the potential "
     "retention S and the initial abstraction Ia = lambda S (lambda = 0.2 unless stated).",
     [_mm("P", "storm rainfall depth", ("rainfall", "rainfall depth", "precipitation")),
      _N("CN", "curve number (30 to 100)", ("curve number",)),
      _N("lam", "initial-abstraction ratio (default 0.2)", ("initial abstraction ratio", "lambda"), required=False)],
     [_o("Q", "number", "runoff depth (mm)"), _o("S", "number", "potential maximum retention (mm)"),
      _o("Ia", "number", "initial abstraction (mm)"), _o("C", "number", "runoff ratio Q/P")],
     scs_runoff, _ref_scs,
     lambda P, CN, lam=0.2: (lambda S: (lambda Ia: {"Q": _r((P - Ia) ** 2 / (P - Ia + S) if P > Ia else 0.0), "S": _r(S),
                                                  "Ia": _r(Ia), "C": 0.0})(lam * S))(1000.0 / CN - 10.0),
     lambda rng: (lambda CN: {"CN": CN, "P": (25400 / CN - 254) * rng.uniform(0.3, 3) + 5})(rng.uniform(50, 98)),
     [{"P": 100, "CN": 80}, {"P": 60, "CN": 70, "lam": 0.05}], ["scs curve number runoff", "curve number runoff",
                                                                "scs runoff"],
     conditions=["30 <= CN <= 100", "rainfall depth in mm", "0 <= lambda <= 0.3"], invalid=[{"P": 100, "CN": 120}],
     method="S = 25400/CN - 254 (mm), Ia = lambda S, Q = (P - Ia)^2/(P - Ia + S) for P > Ia, else 0",
     latex=r"Q = \frac{(P-I_a)^2}{P-I_a+S},\quad S = \frac{25400}{CN}-254,\quad I_a = \lambda S")


def uh_convolution(excess, uh, baseflow=0.0):
    P = _vec("excess", excess, nonneg=True)
    U = _vec("uh", uh, nonneg=True)
    _finite(baseflow=baseflow)
    _need(baseflow >= 0, "the base flow must be zero or positive")
    n, m = len(P), len(U)
    Q = [sum(P[i] * U[k - i] for i in range(max(0, k - m + 1), min(n, k + 1))) for k in range(n + m - 1)]
    total = [q + baseflow for q in Q]
    pk = max(range(len(total)), key=lambda k: total[k])
    return {"DRO": _r(Q), "total": _r(total), "peak": _r(total[pk]), "peak_period": pk + 1}


def _ref_uh_conv(excess, uh, baseflow=0.0):
    import numpy as np
    Q = np.convolve(np.asarray(excess, float), np.asarray(uh, float))
    total = Q + baseflow
    return {"DRO": list(Q), "total": list(total), "peak": float(total.max()), "peak_period": int(np.argmax(total)) + 1}


proc("unit_hydrograph_convolution", "Direct runoff hydrograph by unit-hydrograph convolution", "hydrology",
     "The direct runoff (and total) hydrograph of a storm: the convolution of the excess-rainfall depths of successive "
     "periods with the unit hydrograph of the same period, plus a constant base flow.",
     [_VEC("excess", "excess rainfall in cm per period (one per period)", ("excess rainfall", "rainfall excess")),
      _VEC("uh", "unit-hydrograph ordinates in m^3/s per cm of excess, one per period", ("unit hydrograph", "UH")),
      _Qin("baseflow", "constant base flow (default 0)", ("base flow",), required=False)],
     [_o("DRO", "flow_rate", "direct runoff ordinates (m^3/s)"), _o("total", "flow_rate", "total flow ordinates (m^3/s)"),
      _o("peak", "flow_rate", "peak total flow (m^3/s)"), _o("peak_period", "number", "period of the peak (1-based)")],
     uh_convolution, _ref_uh_conv,
     lambda excess, uh, baseflow=0.0: (lambda Q: {"DRO": _r(Q), "total": _r([q + baseflow for q in Q]),
                                                  "peak": _r(max(Q) + baseflow), "peak_period": 1})(
         [sum(excess[i] * uh[(k + i) % len(uh)] for i in range(len(excess)) if 0 <= k - i < len(uh)) for k in range(len(excess) + len(uh) - 1)]),
     lambda rng: {"excess": [round(rng.uniform(0.2, 3), 2) for _ in range(rng.randint(2, 5))],
                  "uh": [round(x, 2) for x in _uh_shape(rng)], "baseflow": rng.choice([0.0, rng.uniform(1, 20)])},
     [{"excess": [1.5, 2.5, 1.0], "uh": [0, 10, 30, 22, 12, 5, 0]},
      {"excess": [2.0, 0.8], "uh": [5, 25, 40, 28, 15, 6], "baseflow": 10}],
     ["unit hydrograph convolution", "direct runoff hydrograph", "storm hydrograph from a unit hydrograph"],
     conditions=["excess depths and the unit hydrograph share one period", "linear, time-invariant catchment"],
     method="Q_k = sum_i P_i U_(k-i+1) (discrete convolution)", latex=r"Q_n = \sum_{m=1}^{\min(n,M)} P_m\,U_{n-m+1}")


def _uh_shape(rng):
    m = rng.randint(5, 9)
    pk = rng.randint(1, 3)
    return [max(0.0, 40 * rng.uniform(0.6, 1.4) * (k / pk if k <= pk else (m - k) / (m - pk))) for k in range(m + 1)]


def uh_derive(dro, excess):
    Q = _vec("dro", dro)
    P = _vec("excess", excess, nonneg=True)
    n = len(P)
    m = len(Q) - n + 1
    _need(m >= 2, "the runoff hydrograph must have at least n + 1 ordinates for n excess periods")
    _need(sum(P) > 0, "the excess rainfall must not be all zero")
    # least squares on the convolution system A u = Q (normal equations, solved exactly in floating point)
    A = [[P[k - j] if 0 <= k - j < n else 0.0 for j in range(m)] for k in range(len(Q))]
    N = [[sum(A[k][i] * A[k][j] for k in range(len(Q))) for j in range(m)] for i in range(m)]
    rhs = [sum(A[k][i] * Q[k] for k in range(len(Q))) for i in range(m)]
    u = _solve_linear(N, rhs)
    fit = [sum(A[k][j] * u[j] for j in range(m)) for k in range(len(Q))]
    rms = math.sqrt(sum((fit[k] - Q[k]) ** 2 for k in range(len(Q))) / len(Q))
    scale = max(abs(x) for x in Q)
    return {"uh": _r([_snap(x, scale) for x in u]), "rms_residual": _r(_snap(rms, scale)), "peak": _r(max(u))}


def _ref_uh_derive(dro, excess):
    import numpy as np
    Q = np.asarray(dro, float)
    P = np.asarray(excess, float)
    n, m = len(P), len(Q) - len(P) + 1
    A = np.zeros((len(Q), m))
    for j in range(m):
        A[j:j + n, j] = P
    u = np.linalg.lstsq(A, Q, rcond=None)[0]
    return {"uh": list(u), "peak": float(u.max())}


def _rand_uh_derive(rng):
    P = [round(rng.uniform(0.5, 3), 2) for _ in range(rng.randint(2, 4))]
    U = _uh_shape(rng)
    Q = [sum(P[i] * U[k - i] for i in range(len(P)) if 0 <= k - i < len(U)) for k in range(len(P) + len(U) - 1)]
    return {"dro": [round(q + rng.uniform(-1.5, 1.5), 2) for q in Q], "excess": P}


def _uh_derive_wrong(dro, excess):
    # forward substitution (exact deconvolution of the first ordinates), ignoring the later ones
    P, Q = list(excess), list(dro)
    n, m = len(P), len(Q) - len(P) + 1
    u = []
    for k in range(m):
        u.append((Q[k] - sum(P[i] * u[k - i] for i in range(1, min(n, k + 1)))) / P[0] if P[0] else 0.0)
    return {"uh": _r(u), "rms_residual": 0.0, "peak": _r(max(u))}


proc("unit_hydrograph_derivation", "Unit hydrograph from a storm by least-squares deconvolution", "hydrology",
     "The unit hydrograph that best reproduces an observed direct-runoff hydrograph from its excess-rainfall depths "
     "(least-squares solution of the convolution equations), with the RMS misfit.",
     [_VEC("dro", "observed direct runoff ordinates in m^3/s (base flow removed)", ("direct runoff", "runoff hydrograph")),
      _VEC("excess", "excess rainfall in cm per period", ("excess rainfall", "rainfall excess"))],
     [_o("uh", "flow_rate", "unit-hydrograph ordinates (m^3/s per cm)"),
      _o("rms_residual", "flow_rate", "RMS misfit of the reconstructed runoff (m^3/s)"),
      _o("peak", "flow_rate", "unit-hydrograph peak (m^3/s per cm)")],
     uh_derive, _ref_uh_derive, _uh_derive_wrong, _rand_uh_derive,
     [{"dro": [0, 15, 61, 77, 52, 29, 11, 3], "excess": [1.5, 2.0]},
      {"dro": [2, 20, 70, 105, 85, 50, 25, 9, 2], "excess": [1.0, 2.5, 0.8]}],
     ["unit hydrograph derivation", "derive a unit hydrograph", "unit hydrograph by deconvolution"],
     conditions=["len(dro) >= len(excess) + 1", "the same period for both"],
     method="least squares on the convolution system: (A^T A) u = A^T Q", latex=r"\min_u \lVert A u - Q\rVert_2,\quad A_{kj} = P_{k-j+1}")


# --- Thiessen polygons ---------------------------------------------------------------------------------------------
def _clip_halfplane(poly, a, b, c):
    """Keep the part of polygon `poly` where a x + b y <= c (Sutherland-Hodgman against one half-plane)."""
    out = []
    n = len(poly)
    for i in range(n):
        P, Qp = poly[i], poly[(i + 1) % n]
        fp, fq = a * P[0] + b * P[1] - c, a * Qp[0] + b * Qp[1] - c
        if fp <= 0:
            out.append(P)
        if (fp < 0 < fq) or (fq < 0 < fp):
            t = fp / (fp - fq)
            out.append((P[0] + t * (Qp[0] - P[0]), P[1] + t * (Qp[1] - P[1])))
    return out


def _shoelace(poly):
    return 0.5 * sum(poly[i][0] * poly[(i + 1) % len(poly)][1] - poly[(i + 1) % len(poly)][0] * poly[i][1]
                     for i in range(len(poly))) if len(poly) >= 3 else 0.0


def _is_simple(poly):
    n = len(poly)
    if len(set(poly)) != n:
        return False

    def cross(o, a, b):
        return (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0])

    def hit(p1, p2, p3, p4):
        d1, d2, d3, d4 = cross(p3, p4, p1), cross(p3, p4, p2), cross(p1, p2, p3), cross(p1, p2, p4)
        return ((d1 > 0) != (d2 > 0) or d1 == 0 or d2 == 0) and ((d3 > 0) != (d4 > 0) or d3 == 0 or d4 == 0) and             min(p1[0], p2[0]) <= max(p3[0], p4[0]) and min(p3[0], p4[0]) <= max(p1[0], p2[0]) and             min(p1[1], p2[1]) <= max(p3[1], p4[1]) and min(p3[1], p4[1]) <= max(p1[1], p2[1])
    for i in range(n):
        for j in range(i + 1, n):
            if j == i + 1 or (i == 0 and j == n - 1):
                continue
            if hit(poly[i], poly[(i + 1) % n], poly[j], poly[(j + 1) % n]):
                return False
    return True


def thiessen(gx, gy, rain, bx, by):
    gx, gy = _vec("gauge x", gx), _vec("gauge y", gy, len(gx))
    rain = _vec("rainfall", rain, len(gx), nonneg=True)
    bx, by = _vec("boundary x", bx), _vec("boundary y", by, len(bx))
    _need(len(gx) >= 2, "give at least two gauges")
    _need(len(bx) >= 3, "the catchment boundary needs at least three vertices")
    pts = list(zip(gx, gy))
    _need(len(set(pts)) == len(pts), "two gauges share a location")
    poly = list(zip(bx, by))
    _need(_is_simple(poly), "the catchment boundary must be a simple polygon (no crossing or repeated vertices)")
    area = _shoelace(poly)
    _need(abs(area) > 0, "the boundary encloses no area")
    if area < 0:
        poly, area = poly[::-1], -area
    areas = []
    for i, (xi, yi) in enumerate(pts):
        cell = poly
        for j, (xj, yj) in enumerate(pts):
            if i != j and cell:
                # points closer to gauge i than to gauge j: 2(xj - xi) x + 2(yj - yi) y <= xj^2 + yj^2 - xi^2 - yi^2
                cell = _clip_halfplane(cell, 2 * (xj - xi), 2 * (yj - yi), xj * xj + yj * yj - xi * xi - yi * yi)
        areas.append(abs(_shoelace(cell)))
    areas = [_snap(a, area) for a in areas]
    w = [a / area for a in areas]
    mean = sum(wi * r for wi, r in zip(w, rain))
    return {"mean": _r(mean), "areas": _r(areas), "weights": _r(w), "arithmetic_mean": _r(sum(rain) / len(rain)),
            "catchment_area": _r(area)}


def _ref_thiessen(gx, gy, rain, bx, by):
    from shapely.geometry import MultiPoint, Point, Polygon
    from shapely.ops import voronoi_diagram
    catch = Polygon(list(zip(bx, by)))
    big = catch.envelope.buffer(10 * (catch.length + 1))
    cells = voronoi_diagram(MultiPoint(list(zip(gx, gy))), envelope=big)
    areas = []
    for x, y in zip(gx, gy):
        cell = next(c for c in cells.geoms if c.contains(Point(x, y)))
        areas.append(cell.intersection(catch).area)
    A = catch.area
    w = [a / A for a in areas]
    return {"mean": sum(wi * r for wi, r in zip(w, rain)), "areas": areas, "weights": w,
            "arithmetic_mean": sum(rain) / len(rain), "catchment_area": A}


def _rand_thiessen(rng):
    k = rng.randint(4, 7)
    R = rng.uniform(5, 30)
    ang = [2 * math.pi * (i + rng.uniform(0.1, 0.9)) / k for i in range(k)]
    bx = [round(R * rng.uniform(0.7, 1.0) * math.cos(a), 3) for a in ang]
    by = [round(R * rng.uniform(0.7, 1.0) * math.sin(a), 3) for a in ang]
    g = rng.randint(3, 6)
    gx = [round(rng.uniform(-0.5, 0.5) * R, 3) for _ in range(g)]
    gy = [round(rng.uniform(-0.5, 0.5) * R, 3) for _ in range(g)]
    return {"gx": gx, "gy": gy, "rain": [round(rng.uniform(10, 120), 1) for _ in range(g)], "bx": bx, "by": by}


proc("thiessen_polygon_rainfall", "Areal rainfall by Thiessen polygons", "hydrology",
     "Mean areal rainfall over a catchment by the Thiessen (Voronoi) polygon method: each gauge represents the part of "
     "the catchment nearer to it than to any other gauge; the areas, weights and the arithmetic mean are also given. "
     "Coordinates in km (areas in km^2), rainfall in mm; the boundary is a simple polygon listed vertex by vertex.",
     [_VEC("gx", "gauge x coordinates in km", ("gauge x",)), _VEC("gy", "gauge y coordinates in km", ("gauge y",)),
      _VEC("rain", "rainfall at each gauge in mm", ("rainfall", "gauge rainfall")),
      _VEC("bx", "catchment boundary x coordinates in km", ("boundary x",), min_length=3),
      _VEC("by", "catchment boundary y coordinates in km", ("boundary y",), min_length=3)],
     [_o("mean", "number", "Thiessen mean rainfall (mm)"), _o("areas", "number", "area represented by each gauge (km^2)"),
      _o("weights", "number", "Thiessen weight of each gauge"), _o("arithmetic_mean", "number", "arithmetic mean (mm)"),
      _o("catchment_area", "number", "catchment area (km^2)")],
     thiessen, _ref_thiessen,
     lambda gx, gy, rain, bx, by: dict(thiessen(gx, gy, rain, bx, by), mean=_r(sum(rain) / len(rain))),
     _rand_thiessen,
     [{"gx": [2, 8, 5], "gy": [2, 3, 9], "rain": [45, 60, 38], "bx": [0, 10, 10, 0], "by": [0, 0, 10, 10]},
      {"gx": [1, 6, 9, 4], "gy": [1, 2, 7, 8], "rain": [80, 95, 70, 110], "bx": [0, 12, 10, 2], "by": [0, 0, 10, 9]}],
     ["thiessen polygon rainfall", "thiessen polygons", "thiessen mean rainfall"],
     conditions=["distinct gauge locations", "simple (non-self-intersecting) boundary polygon"],
     method="clip the catchment polygon by the perpendicular-bisector half-planes of every pair of gauges; weights = cell area/catchment area",
     latex=r"\bar P = \sum_i \frac{A_i}{A} P_i")


# --- frequency analysis -----------------------------------------------------------------------------------------------
def exceedance_risk(T, n):
    _finite(T=T)
    _need(T > 1, "the return period must exceed 1 year")
    _need(isinstance(n, int) and 1 <= n <= 1000, "the design life must be a whole number of years, 1 to 1000")
    p = 1.0 / T
    risk = 1 - (1 - p) ** n
    return {"risk": _r(risk), "p_annual": _r(p), "p_none": _r(1 - risk)}


proc("flood_exceedance_risk", "Risk of exceedance over a design life", "hydrology",
     "Probability that an event of return period T is equalled or exceeded at least once in n years (independent "
     "years), with the annual exceedance probability.",
     [_in("T", "number", "years", "return period (years)", ("return period",), "years"),
      _in("n", "number", "years", "design life or period (whole years)", ("design life", "period"), "years", typ="integer")],
     [_o("risk", "number", "probability of at least one exceedance in n years"),
      _o("p_annual", "number", "annual exceedance probability 1/T"), _o("p_none", "number", "probability of no exceedance")],
     exceedance_risk,
     lambda T, n: (lambda r: {"risk": r, "p_annual": 1 / T, "p_none": 1 - r})(float(__import__("scipy.stats", fromlist=["binom"]).binom.sf(0, n, 1 / T))),
     lambda T, n: {"risk": _r(min(n / T, 1.0)), "p_annual": _r(1 / T), "p_none": _r(1 - min(n / T, 1.0))},
     lambda rng: {"T": rng.uniform(2, 1000), "n": rng.randint(2, 100)},
     [{"T": 100, "n": 50}, {"T": 25, "n": 10}], ["flood exceedance risk", "risk of exceedance", "hydrologic risk"],
     conditions=["T > 1 year", "independent years"], invalid=[{"T": 0.5, "n": 10}],
     method="risk = 1 - (1 - 1/T)^n", latex=r"R = 1 - \left(1 - \frac{1}{T}\right)^n")


def design_return_period(R, n):
    _finite(R=R)
    _need(0 < R < 1, "the acceptable risk must be a fraction between 0 and 1")
    _need(isinstance(n, int) and 1 <= n <= 1000, "the design life must be a whole number of years, 1 to 1000")
    T = 1.0 / (1 - (1 - R) ** (1.0 / n))
    return {"T": _r(T), "p_annual": _r(1 / T)}


def _ref_design_T(R, n):
    from scipy.stats import binom
    p = _brentq(lambda p: binom.sf(0, n, p) - R, 1e-12, 1 - 1e-12)
    return {"T": 1 / p, "p_annual": p}


proc("design_return_period_for_risk", "Return period for an acceptable risk over a design life", "hydrology",
     "The design return period whose event has only the stated probability R of being exceeded during an n-year life.",
     [_F("R", "acceptable risk of exceedance (fraction, or % in a question)", ("risk", "acceptable risk")),
      _in("n", "number", "years", "design life (whole years)", ("design life", "period"), "years", typ="integer")],
     [_o("T", "number", "required return period (years)"), _o("p_annual", "number", "annual exceedance probability")],
     design_return_period, _ref_design_T,
     lambda R, n: {"T": _r(n / R), "p_annual": _r(R / n)},
     lambda rng: {"R": rng.uniform(0.01, 0.6), "n": rng.randint(2, 100)},
     [{"R": 0.1, "n": 50}, {"R": 0.05, "n": 25}], ["design return period for a risk", "return period for acceptable risk"],
     conditions=["0 < R < 1", "independent years"], invalid=[{"R": 1.5, "n": 10}],
     method="T = 1/(1 - (1 - R)^(1/n))", latex=r"T = \frac{1}{1-(1-R)^{1/n}}")


def _mean_sd(x):
    n = len(x)
    m = sum(x) / n
    return m, math.sqrt(sum((v - m) ** 2 for v in x) / (n - 1))


def gumbel_flood(peaks, T):
    x = _vec("peaks", peaks, positive=True)
    _need(len(x) >= 5, "give at least five annual maxima")
    _finite(T=T)
    _need(T > 1, "the return period must exceed 1 year")
    m, s = _mean_sd(x)
    _need(s > 0, "the annual maxima must not all be equal")
    K = -math.sqrt(6) / math.pi * (EULER_GAMMA + math.log(math.log(T / (T - 1))))
    return {"Q_T": _r(m + K * s), "K_T": _r(K), "mean": _r(m), "std": _r(s)}


def _ref_gumbel(peaks, T):
    import numpy as np
    from scipy.stats import gumbel_r
    x = np.asarray(peaks, float)
    m, s = float(x.mean()), float(x.std(ddof=1))
    scale = s * math.sqrt(6) / math.pi                      # method of moments
    loc = m - float(np.euler_gamma) * scale
    q = float(gumbel_r.ppf(1 - 1 / T, loc=loc, scale=scale))
    return {"Q_T": q, "K_T": (q - m) / s, "mean": m, "std": s}


proc("gumbel_design_flood", "Gumbel (EV1) design flood", "hydrology",
     "The T-year flood from a series of annual maximum discharges by the Gumbel (extreme value type I) distribution "
     "fitted by the method of moments (Chow's frequency factor for a long record; sample standard deviation with n - 1).",
     [_VEC("peaks", "annual maximum discharges in m^3/s", ("annual maxima", "annual peaks"), min_length=5),
      _in("T", "number", "years", "return period (years)", ("return period",), "years")],
     [_o("Q_T", "flow_rate", "T-year flood (m^3/s)"), _o("K_T", "number", "frequency factor"),
      _o("mean", "flow_rate", "mean annual maximum (m^3/s)"), _o("std", "flow_rate", "sample standard deviation (m^3/s)")],
     gumbel_flood, _ref_gumbel,
     lambda peaks, T: (lambda m, s: (lambda K: {"Q_T": _r(m + K * s), "K_T": _r(K), "mean": _r(m), "std": _r(s)})(
         -math.sqrt(6) / math.pi * (EULER_GAMMA + math.log(math.log(T / (T - 1))))))(
         sum(peaks) / len(peaks), math.sqrt(sum((v - sum(peaks) / len(peaks)) ** 2 for v in peaks) / len(peaks))),
     lambda rng: {"peaks": [round(rng.uniform(100, 900), 1) for _ in range(rng.randint(8, 40))], "T": rng.uniform(2, 500)},
     [{"peaks": [420, 380, 510, 290, 640, 455, 370, 520, 600, 330], "T": 100},
      {"peaks": [1200, 980, 1450, 1100, 860, 1600, 1320, 1010, 940, 1250, 1380, 1150], "T": 50}],
     ["gumbel design flood", "gumbel flood frequency", "gumbel distribution flood"],
     conditions=["at least five annual maxima", "T > 1 year", "method of moments (no sample-size correction table)"],
     method="K_T = -(sqrt 6/pi)(0.5772 + ln ln(T/(T - 1))), Q_T = mean + K_T s",
     latex=r"Q_T = \bar Q + K_T s,\quad K_T = -\frac{\sqrt6}{\pi}\left[0.5772 + \ln\ln\frac{T}{T-1}\right]")


def _gamma_ppf(a, p):
    """Quantile of the standard gamma distribution (shape a) by bisection on mpmath's regularized incomplete gamma."""
    import statistics
    import mpmath
    with mpmath.workdps(30):
        F = lambda x: float(mpmath.gammainc(a, 0, x, regularized=True) - p)          # noqa: E731
        lg = math.lgamma(a)
        z = statistics.NormalDist().inv_cdf(p)
        x = max(a * (1 - 1 / (9 * a) + z * math.sqrt(1 / (9 * a))) ** 3, 1e-3 * a)    # Wilson-Hilferty start
        for _ in range(60):                           # Newton steps on F(x) = p with the gamma density
            fx = F(x)
            step = fx / math.exp((a - 1) * math.log(x) - x - lg)
            nx = x - step
            if nx <= 0:
                nx = x / 2
            if abs(nx - x) <= 1e-15 * x:
                return nx
            x = nx
        hi = a + 10 * math.sqrt(a) + 50
        while F(hi) < 0:
            hi *= 2
        return _bisect_root(F, 0.0, hi, iters=200)


def log_pearson3(peaks, T):
    import statistics
    x = _vec("peaks", peaks, positive=True)
    _need(len(x) >= 10, "give at least ten annual maxima")
    _finite(T=T)
    _need(T > 1.0001, "the return period must exceed 1 year")
    y = [math.log10(v) for v in x]
    n = len(y)
    m, s = _mean_sd(y)
    _need(s > 0, "the annual maxima must not all be equal")
    G = n * sum((v - m) ** 3 for v in y) / ((n - 1) * (n - 2) * s ** 3)
    p = 1 - 1 / T
    if abs(G) < 1.6e-5:
        K = statistics.NormalDist().inv_cdf(p)
    else:
        a = 4 / (G * G)
        K = (_gamma_ppf(a, p) - a) * G / 2 if G > 0 else (_gamma_ppf(a, 1 - p) - a) * G / 2
    return {"Q_T": _r(10 ** (m + K * s)), "K_T": _r(K), "mean_log": _r(m), "std_log": _r(s), "skew": _r(G)}


def _ref_lp3(peaks, T):
    import numpy as np
    from scipy.stats import pearson3, skew
    y = np.log10(np.asarray(peaks, float))
    m, s, G = float(y.mean()), float(y.std(ddof=1)), float(skew(y, bias=False))
    K = float(pearson3.ppf(1 - 1 / T, G))
    return {"Q_T": 10 ** (m + K * s), "K_T": K, "mean_log": m, "std_log": s, "skew": G}


def _rand_lp3(rng):
    while True:
        x = {"peaks": [round(10 ** rng.gauss(2.6, rng.uniform(0.1, 0.35)), 1) for _ in range(rng.randint(12, 40))],
             "T": rng.uniform(2, 500)}
        y = [math.log10(v) for v in x["peaks"]]
        n = len(y)
        m, s = _mean_sd(y)
        G = n * sum((v - m) ** 3 for v in y) / ((n - 1) * (n - 2) * s ** 3)
        if 0.1 < abs(G) < 2.5:
            return x


proc("log_pearson_iii_design_flood", "Log-Pearson type III design flood", "hydrology",
     "The T-year flood by the log-Pearson type III distribution fitted by moments of the base-10 logarithms (station "
     "skew only, no regional skew weighting or outlier tests), frequency factor from the exact Pearson III quantile.",
     [_VEC("peaks", "annual maximum discharges in m^3/s", ("annual maxima", "annual peaks"), min_length=10),
      _in("T", "number", "years", "return period (years)", ("return period",), "years")],
     [_o("Q_T", "flow_rate", "T-year flood (m^3/s)"), _o("K_T", "number", "Pearson III frequency factor"),
      _o("mean_log", "number", "mean of log10 Q"), _o("std_log", "number", "standard deviation of log10 Q"),
      _o("skew", "number", "skew coefficient of log10 Q (unbiased)")],
     log_pearson3, _ref_lp3,
     lambda peaks, T: (lambda r: dict(r, Q_T=_r(10 ** (r["mean_log"] + __import__("statistics").NormalDist().inv_cdf(1 - 1 / T) * r["std_log"]))))(
         log_pearson3(peaks, T)),
     _rand_lp3,
     [{"peaks": [420, 380, 510, 290, 640, 455, 370, 520, 600, 330, 880, 410], "T": 100},
      {"peaks": [1200, 980, 1450, 1100, 860, 1600, 1320, 1010, 940, 1250, 1380, 1150, 2400, 1050], "T": 50}],
     ["log pearson iii design flood", "log pearson type iii flood", "log pearson 3 flood frequency"],
     conditions=["at least ten annual maxima", "station skew only", "no outlier screening"],
     method="moments of log10 Q; K_T = Pearson III quantile (gamma inverse with shape 4/G^2); Q_T = 10^(m + K s)",
     latex=r"\log Q_T = \bar y + K_T(G, T)\,s_y")


def weibull_positions(peaks):
    x = _vec("peaks", peaks)
    n = len(x)
    _need(n >= 2, "give at least two values")
    xs = sorted(x, reverse=True)
    return {"ranked": _r(xs), "T": _r([(n + 1) / m for m in range(1, n + 1)]),
            "P": _r([m / (n + 1) for m in range(1, n + 1)])}


def _ref_weibull(peaks):
    import numpy as np
    from scipy.stats.mstats import plotting_positions
    x = np.asarray(peaks, float)
    pp = np.asarray(plotting_positions(x, alpha=0, beta=0))          # non-exceedance i/(n+1) by ascending rank
    P = np.sort(1 - pp)                                               # exceedance, largest value first
    return {"ranked": list(np.sort(x)[::-1]), "T": list(1 / P), "P": list(P)}


proc("weibull_plotting_positions", "Return periods by the Weibull plotting position", "hydrology",
     "Ranks annual maxima from largest to smallest and gives each its exceedance probability m/(n + 1) and return "
     "period (n + 1)/m (Weibull plotting position).",
     [_VEC("peaks", "annual maxima (any unit; the ranking keeps it)", ("annual maxima", "annual peaks", "data"), min_length=2)],
     [_o("ranked", "number", "values ranked from largest to smallest"), _o("T", "number", "return period of each (years)"),
      _o("P", "number", "exceedance probability of each")],
     weibull_positions, _ref_weibull,
     lambda peaks: (lambda n: {"ranked": _r(sorted(peaks, reverse=True)), "T": _r([n / m for m in range(1, n + 1)]),
                              "P": _r([m / n for m in range(1, n + 1)])})(len(peaks)),
     lambda rng: {"peaks": [round(rng.uniform(50, 900), 1) for _ in range(rng.randint(5, 30))]},
     [{"peaks": [420, 380, 510, 290, 640]}, {"peaks": [12.5, 18.2, 9.7, 22.1, 15.3, 11.0, 19.8]}],
     ["weibull plotting position", "weibull plotting positions", "return periods by weibull"],
     conditions=["annual series", "ties keep distinct ranks"], method="rank m from the largest; P = m/(n + 1), T = 1/P",
     latex=r"P = \frac{m}{n+1},\quad T = \frac{n+1}{m}")


def _pair(obs, sim):
    o = _vec("observed", obs)
    s = _vec("simulated", sim, len(o))
    _need(len(o) >= 3, "give at least three pairs")
    return o, s


def nse_metrics(obs, sim):
    o, s = _pair(obs, sim)
    n = len(o)
    mo = sum(o) / n
    den = sum((v - mo) ** 2 for v in o)
    _need(den > 0, "the observed series must vary")
    _need(sum(o) != 0, "the observed total must not be zero")
    sse = sum((a - b) ** 2 for a, b in zip(o, s))
    return {"NSE": _r(1 - sse / den), "RMSE": _r(math.sqrt(sse / n)), "PBIAS": _r(100 * sum(a - b for a, b in zip(o, s)) / sum(o))}


def _ref_nse(obs, sim):
    import hydroeval as he
    import numpy as np
    o, s = np.asarray(obs, float), np.asarray(sim, float)
    f = lambda fn: float(np.ravel(fn(s, o))[0])          # noqa: E731
    return {"NSE": f(he.nse), "RMSE": f(he.rmse), "PBIAS": f(he.pbias)}


def _rand_pairs(rng):
    n = rng.randint(6, 40)
    o = [round(rng.uniform(5, 200), 2) for _ in range(n)]
    return {"obs": o, "sim": [round(v * rng.uniform(0.7, 1.3) + rng.uniform(-5, 5), 2) for v in o]}


proc("nash_sutcliffe_efficiency", "Nash-Sutcliffe efficiency, RMSE and percent bias", "hydrology",
     "Goodness of fit of simulated to observed flows: Nash-Sutcliffe efficiency, root-mean-square error and percent "
     "bias (positive when the model underestimates, as hydroeval defines it).",
     [_VEC("obs", "observed values (any unit)", ("observed",), min_length=3),
      _VEC("sim", "simulated values, same unit and order", ("simulated",), min_length=3)],
     [_o("NSE", "number", "Nash-Sutcliffe efficiency"), _o("RMSE", "number", "root-mean-square error (unit of the data)"),
      _o("PBIAS", "number", "percent bias 100 sum(obs - sim)/sum(obs)")],
     nse_metrics, _ref_nse,
     lambda obs, sim: dict(nse_metrics(obs, sim), NSE=_r(1 - sum((a - b) ** 2 for a, b in zip(obs, sim)) /
                                                         sum((b - sum(sim) / len(sim)) ** 2 for b in sim))),
     _rand_pairs,
     [{"obs": [12, 30, 55, 41, 22, 15], "sim": [10, 34, 50, 44, 20, 17]},
      {"obs": [3.2, 4.1, 8.7, 15.2, 9.4, 5.5, 4.0], "sim": [3.0, 4.5, 7.9, 13.8, 10.1, 6.0, 3.6]}],
     ["nash sutcliffe efficiency", "nash sutcliffe", "model efficiency nse"],
     conditions=["paired series of equal length", "observed values not all equal"],
     method="NSE = 1 - sum(obs - sim)^2/sum(obs - mean obs)^2", latex=r"NSE = 1 - \frac{\sum (O_i-S_i)^2}{\sum (O_i-\bar O)^2}")


def kge_metrics(obs, sim):
    o, s = _pair(obs, sim)
    n = len(o)
    mo, ms = sum(o) / n, sum(s) / n
    so = math.sqrt(sum((v - mo) ** 2 for v in o) / n)
    ss = math.sqrt(sum((v - ms) ** 2 for v in s) / n)
    _need(so > 0 and ss > 0, "both series must vary")
    _need(sum(o) != 0, "the observed total must not be zero")
    r = sum((a - mo) * (b - ms) for a, b in zip(o, s)) / (n * so * ss)
    alpha, beta = ss / so, sum(s) / sum(o)
    return {"KGE": _r(1 - math.sqrt((r - 1) ** 2 + (alpha - 1) ** 2 + (beta - 1) ** 2)), "r": _r(r), "alpha": _r(alpha),
            "beta": _r(beta)}


def _ref_kge(obs, sim):
    import hydroeval as he
    import numpy as np
    k = np.ravel(he.kge(np.asarray(sim, float), np.asarray(obs, float)))
    return {"KGE": float(k[0]), "r": float(k[1]), "alpha": float(k[2]), "beta": float(k[3])}


def _kge_wrong(obs, sim):
    out = kge_metrics(obs, sim)
    g = out["alpha"] / out["beta"]                       # the coefficient-of-variation ratio of KGE' in place of alpha
    return dict(out, alpha=_r(g), KGE=_r(1 - math.sqrt((out["r"] - 1) ** 2 + (g - 1) ** 2 + (out["beta"] - 1) ** 2)))


proc("kling_gupta_efficiency", "Kling-Gupta efficiency", "hydrology",
     "Kling-Gupta efficiency of simulated against observed values (Gupta et al. 2009) with its components: the "
     "correlation r, the variability ratio alpha (population standard deviations) and the bias ratio beta.",
     [_VEC("obs", "observed values (any unit)", ("observed",), min_length=3),
      _VEC("sim", "simulated values, same unit and order", ("simulated",), min_length=3)],
     [_o("KGE", "number", "Kling-Gupta efficiency"), _o("r", "number", "Pearson correlation"),
      _o("alpha", "number", "ratio of standard deviations sim/obs"), _o("beta", "number", "ratio of means sim/obs")],
     kge_metrics, _ref_kge, _kge_wrong, _rand_pairs,
     [{"obs": [12, 30, 55, 41, 22, 15], "sim": [10, 34, 50, 44, 20, 17]},
      {"obs": [3.2, 4.1, 8.7, 15.2, 9.4, 5.5, 4.0], "sim": [3.0, 4.5, 7.9, 13.8, 10.1, 6.0, 3.6]}],
     ["kling gupta efficiency", "kge efficiency"],
     conditions=["paired series of equal length", "both series vary"],
     method="KGE = 1 - sqrt((r - 1)^2 + (alpha - 1)^2 + (beta - 1)^2)",
     latex=r"KGE = 1-\sqrt{(r-1)^2+(\alpha-1)^2+(\beta-1)^2}")


# --- infiltration and losses ------------------------------------------------------------------------------------------
def _rate_in(sym, meaning, names=(), show="mm/h", **kw):
    return _in(sym, "rainfall_intensity", "intensity", meaning, names, show, **kw)


def horton(f0, fc, k, t):
    _pos(f0=f0, k=k)
    _finite(fc=fc, t=t)
    _need(0 <= fc < f0, "need 0 <= fc < f0")
    _need(t >= 0, "the time must be zero or positive")
    e = math.exp(-k * t)
    f = fc + (f0 - fc) * e
    F = fc * t + (f0 - fc) / k * (1 - e)
    return {"f": _r(f), "F": _r(F), "f_mm_h": _r(f * 3.6e6), "F_mm": _r(F * 1e3)}


def _ref_horton(f0, fc, k, t):
    f = lambda s: fc + (f0 - fc) * math.exp(-k * s)        # noqa: E731
    F = _quad(f, 0.0, t)
    return {"f": f(t), "F": F, "f_mm_h": f(t) * 3.6e6, "F_mm": F * 1e3}


proc("horton_infiltration", "Horton infiltration rate and cumulative infiltration", "hydrology",
     "Infiltration capacity at time t and cumulative infiltration from the start of ponding by Horton's equation.",
     [_rate_in("f0", "initial infiltration capacity", ("initial rate", "initial infiltration rate")),
      _rate_in("fc", "final (constant) infiltration capacity", ("final rate", "final infiltration rate")),
      _in("k", "first_order_rate_constant", "rate", "Horton decay constant", ("decay constant",), "1/h"),
      _in("t", "time", "time", "time since ponding began", ("time",), "h")],
     [_o("f", "rainfall_intensity", "infiltration capacity at t (m/s)"), _o("F", "length", "cumulative infiltration (m)"),
      _o("f_mm_h", "number", "infiltration capacity (mm/h)"), _o("F_mm", "number", "cumulative infiltration (mm)")],
     horton, _ref_horton,
     lambda f0, fc, k, t: (lambda e: {"f": _r(fc + (f0 - fc) * e), "F": _r(fc * t + (f0 - fc) * k * (1 - e)),
                                      "f_mm_h": _r((fc + (f0 - fc) * e) * 3.6e6), "F_mm": _r((fc * t + (f0 - fc) * k * (1 - e)) * 1e3)})(math.exp(-k * t)),
     lambda rng: (lambda f0: {"f0": f0, "fc": f0 * rng.uniform(0.05, 0.6), "k": rng.uniform(0.5, 6) / 3600,
                              "t": rng.uniform(600, 4 * 3600)})(rng.uniform(20, 150) / 3.6e6),
     [{"f0": 75, "fc": 10, "k": 4, "t": 1}, {"f0": 50, "fc": 6, "k": 2.5, "t": 0.5}], ["horton infiltration"],
     conditions=["0 <= fc < f0", "ponded surface from t = 0"], invalid=[{"f0": 10, "fc": 75, "k": 4, "t": 1}],
     method="f = fc + (f0 - fc) e^(-k t), F = fc t + (f0 - fc)(1 - e^(-k t))/k",
     latex=r"f = f_c + (f_0-f_c)e^{-kt},\quad F = f_c t + \frac{f_0-f_c}{k}\left(1-e^{-kt}\right)")


def green_ampt(K, psi, dtheta, t):
    _pos(K=K, psi=psi, dtheta=dtheta, t=t)
    _need(dtheta < 1, "the moisture deficit is a fraction below 1")
    pd = psi * dtheta
    g = lambda F: F - pd * math.log1p(F / pd) - K * t          # noqa: E731
    F = _bisect_root(g, 0.0, K * t + 2 * math.sqrt(2 * K * pd * t) + pd)
    f = K * (1 + pd / F)
    return {"F": _r(F), "f": _r(f), "F_mm": _r(F * 1e3), "f_mm_h": _r(f * 3.6e6)}


def _ref_green_ampt(K, psi, dtheta, t):
    pd = psi * dtheta
    # time to infiltrate F: t(F) = integral of dF/f(F) with f = K (1 + pd/F), by quadrature; inverted by brentq
    tf = lambda F: _quad(lambda x: x / (K * (x + pd)), 0.0, F)        # noqa: E731
    F = _brentq(lambda F: tf(F) - t, 1e-15, K * t + 10 * math.sqrt(K * pd * t) + 10 * pd)
    f = K * (1 + pd / F)
    return {"F": F, "f": f, "F_mm": F * 1e3, "f_mm_h": f * 3.6e6}


proc("green_ampt_infiltration", "Green-Ampt cumulative infiltration under ponding", "hydrology",
     "Cumulative infiltration and infiltration rate at time t for a soil ponded from t = 0, by the Green-Ampt model "
     "(implicit equation solved to full precision).",
     [_in("K", "hydraulic_conductivity", "conductivity", "saturated hydraulic conductivity", ("hydraulic conductivity",), "mm/h"),
      _L("psi", "wetting-front suction head", ("suction head", "wetting front suction"), "mm"),
      _F("dtheta", "moisture deficit (porosity minus initial water content)", ("moisture deficit",)),
      _in("t", "time", "time", "time since ponding began", ("time",), "h")],
     [_o("F", "length", "cumulative infiltration (m)"), _o("f", "rainfall_intensity", "infiltration rate (m/s)"),
      _o("F_mm", "number", "cumulative infiltration (mm)"), _o("f_mm_h", "number", "infiltration rate (mm/h)")],
     green_ampt, _ref_green_ampt,
     lambda K, psi, dtheta, t: {"F": _r(K * t), "f": _r(K), "F_mm": _r(K * t * 1e3), "f_mm_h": _r(K * 3.6e6)},
     lambda rng: {"K": rng.uniform(1, 30) / 3.6e6, "psi": rng.uniform(0.05, 0.3), "dtheta": rng.uniform(0.1, 0.4),
                  "t": rng.uniform(0.25, 6) * 3600},
     [{"K": 10.9, "psi": 110.1, "dtheta": 0.247, "t": 1}, {"K": 3.4, "psi": 166.8, "dtheta": 0.34, "t": 2}],
     ["green ampt infiltration", "green ampt"],
     conditions=["ponded surface from t = 0", "homogeneous soil, sharp wetting front"],
     method="F - psi dtheta ln(1 + F/(psi dtheta)) = K t (bisection); f = K(1 + psi dtheta/F)",
     latex=r"F - \psi\Delta\theta\,\ln\!\left(1+\frac{F}{\psi\Delta\theta}\right) = Kt,\quad f = K\left(1+\frac{\psi\Delta\theta}{F}\right)")


def phi_index(rain, dt, R):
    P = _vec("rainfall", rain, nonneg=True)
    _pos(dt=dt)
    _finite(R=R)
    _need(0 < R < sum(P), "the runoff depth must be positive and less than the total rainfall")
    order = sorted(P, reverse=True)
    phi = None
    for k in range(1, len(order) + 1):               # the k largest intervals contribute
        cand = (sum(order[:k]) - R) / k               # phi * dt (mm per interval)
        if cand >= (order[k] if k < len(order) else 0.0) - 1e-15 * order[0] and cand <= order[k - 1]:
            phi = cand
            break
    _need(phi is not None, "no phi-index fits the data")
    excess = [max(p - phi, 0.0) for p in P]
    excess = [_snap(e, order[0]) for e in excess]
    return {"phi": _r(phi / (dt / 3600)), "excess": _r(excess), "contributing": sum(1 for e in excess if e > 0)}


def _ref_phi(rain, dt, R):
    import numpy as np
    P = np.asarray(rain, float)
    loss = _brentq(lambda f: float(np.maximum(P - f, 0.0).sum()) - R, 0.0, float(P.max()))
    ex = np.maximum(P - loss, 0.0)
    return {"phi": loss / (dt / 3600), "excess": list(ex)}


def _rand_phi(rng):
    while True:
        P = [round(rng.uniform(0.5, 30), 1) for _ in range(rng.randint(3, 10))]
        x = {"rain": P, "dt": rng.choice([0.5, 1.0, 2.0]) * 3600, "R": round(sum(P) * rng.uniform(0.1, 0.6), 2)}
        out = phi_index(**x)
        if out["contributing"] < len(P):
            return x


proc("phi_index", "Phi-index of a storm", "hydrology",
     "The constant loss rate (phi-index) that, subtracted from every interval of a rainfall hyetograph, leaves exactly "
     "the observed runoff depth; with the excess rainfall of each interval.",
     [_VEC("rain", "rainfall depth in each interval in mm", ("rainfall", "hyetograph")),
      _in("dt", "time", "time", "duration of one interval", ("interval", "time step"), "h"),
      _mm("R", "observed direct runoff depth", ("runoff", "runoff depth"))],
     [_o("phi", "number", "phi-index (mm/h)"), _o("excess", "number", "excess rainfall in each interval (mm)"),
      _o("contributing", "number", "number of intervals producing runoff")],
     phi_index, _ref_phi,
     lambda rain, dt, R: {"phi": _r((sum(rain) - R) / len(rain) / (dt / 3600)),
                          "excess": _r([p - (sum(rain) - R) / len(rain) for p in rain]), "contributing": len(rain)},
     _rand_phi,
     [{"rain": [4, 9, 18, 12, 3, 1], "dt": 1, "R": 20}, {"rain": [2.5, 7.5, 12.0, 5.0, 1.5], "dt": 0.5, "R": 9}],
     ["phi index", "phi-index of a storm"], conditions=["0 < runoff < total rainfall", "equal intervals"],
     method="the k largest intervals contribute: phi dt = (sum of the k largest - R)/k, with k found by consistency",
     latex=r"\sum_i \max(P_i - \phi\,\Delta t,\,0) = R")


def muskingum(inflow, K, X, dt, O1=None):
    I = _vec("inflow", inflow, nonneg=True)                 # noqa: E741
    _pos(K=K, dt=dt)
    _finite(X=X)
    _need(0 <= X <= 0.5, "the weighting factor X must lie between 0 and 0.5")
    _need(len(I) >= 2, "give at least two inflow ordinates")
    d = 2 * K * (1 - X) + dt
    C0, C1, C2 = (dt - 2 * K * X) / d, (dt + 2 * K * X) / d, (2 * K * (1 - X) - dt) / d
    _need(min(C0, C1, C2) >= -1e-12, "a routing coefficient is negative: choose 2 K X <= dt <= 2 K (1 - X)")
    O = [I[0] if O1 is None else float(O1)]                 # noqa: E741
    for j in range(1, len(I)):
        O.append(C0 * I[j] + C1 * I[j - 1] + C2 * O[-1])
    return {"outflow": _r(O), "C0": _r(C0), "C1": _r(C1), "C2": _r(C2), "peak_outflow": _r(max(O)),
            "attenuation": _r(max(I) - max(O))}


def _ref_muskingum(inflow, K, X, dt, O1=None):
    import numpy as np
    I = np.asarray(inflow, float)                          # noqa: E741
    n = len(I)
    # the whole routing at once: continuity (S_j+1 - S_j)/dt = (I_j + I_j+1)/2 - (O_j + O_j+1)/2 with
    # S = K (X I + (1 - X) O), as one linear system for O_1 .. O_n
    A = np.zeros((n, n))
    b = np.zeros(n)
    A[0, 0] = 1.0
    b[0] = I[0] if O1 is None else O1
    for j in range(n - 1):
        A[j + 1, j + 1] = K * (1 - X) / dt + 0.5
        A[j + 1, j] = -K * (1 - X) / dt + 0.5
        b[j + 1] = (I[j] + I[j + 1]) / 2 - K * X * (I[j + 1] - I[j]) / dt
    O = np.linalg.solve(A, b)                              # noqa: E741
    d = 2 * K * (1 - X) + dt
    return {"outflow": list(O), "C0": (dt - 2 * K * X) / d, "C1": (dt + 2 * K * X) / d,
            "C2": (2 * K * (1 - X) - dt) / d, "peak_outflow": float(O.max()), "attenuation": float(I.max() - O.max())}


def _musk_wrong(inflow, K, X, dt, O1=None):
    d = 2 * K * (1 - X) + dt
    C2, C1, C0 = (dt - 2 * K * X) / d, (dt + 2 * K * X) / d, (2 * K * (1 - X) - dt) / d     # C0 and C2 swapped
    O = [inflow[0] if O1 is None else O1]                  # noqa: E741
    for j in range(1, len(inflow)):
        O.append(C0 * inflow[j] + C1 * inflow[j - 1] + C2 * O[-1])
    return {"outflow": _r(O), "C0": _r(C0), "C1": _r(C1), "C2": _r(C2), "peak_outflow": _r(max(O)),
            "attenuation": _r(max(inflow) - max(O))}


def _rand_musk(rng):
    K = rng.uniform(2, 24) * 3600
    X = rng.uniform(0.05, 0.4)
    dt = rng.uniform(2 * K * X * 1.05, 2 * K * (1 - X) * 0.95)
    n = rng.randint(6, 15)
    pk = rng.randint(1, 3)
    base = rng.uniform(5, 30)
    I = [base + rng.uniform(80, 400) * (min(j / pk, max(0.0, (n - 1 - j) / (n - 1 - pk)))) for j in range(n)]    # noqa: E741
    return {"inflow": [round(v, 1) for v in I], "K": K, "X": X, "dt": dt}


proc("muskingum_routing", "Muskingum flood routing", "hydrology",
     "Routes an inflow hydrograph through a river reach by the Muskingum method (storage S = K[X I + (1 - X) O]), "
     "giving the outflow ordinates, routing coefficients, peak outflow and attenuation. The first outflow equals the "
     "first inflow unless stated.",
     [_VEC("inflow", "inflow ordinates in m^3/s at equal intervals", ("inflow", "inflow hydrograph"), min_length=2),
      _in("K", "time", "time", "storage (travel-time) constant", ("storage constant", "travel time"), "h"),
      _N("X", "weighting factor (0 to 0.5)", ("weighting factor",)),
      _in("dt", "time", "time", "routing interval", ("interval", "time step"), "h"),
      _Qin("O1", "initial outflow (default: the first inflow)", ("initial outflow",), required=False)],
     [_o("outflow", "flow_rate", "outflow ordinates (m^3/s)"), _o("C0", "number", "coefficient C0"),
      _o("C1", "number", "coefficient C1"), _o("C2", "number", "coefficient C2"),
      _o("peak_outflow", "flow_rate", "peak outflow (m^3/s)"), _o("attenuation", "flow_rate", "peak attenuation (m^3/s)")],
     muskingum, _ref_muskingum, _musk_wrong, _rand_musk,
     [{"inflow": [10, 30, 68, 50, 40, 31, 23, 15, 10], "K": 12, "X": 0.2, "dt": 6},
      {"inflow": [20, 80, 210, 240, 190, 130, 80, 45, 30, 22], "K": 8, "X": 0.25, "dt": 4}],
     ["muskingum routing", "muskingum method", "route a flood by muskingum"],
     conditions=["0 <= X <= 0.5", "2 K X <= dt <= 2 K (1 - X) (no negative coefficient)"],
     invalid=[{"inflow": [10, 30, 68, 50], "K": 12, "X": 0.2, "dt": 30}],
     method="O_j+1 = C0 I_j+1 + C1 I_j + C2 O_j, C0 = (dt - 2KX)/D, C1 = (dt + 2KX)/D, C2 = (2K(1 - X) - dt)/D, D = 2K(1 - X) + dt",
     latex=r"O_{j+1} = C_0 I_{j+1} + C_1 I_j + C_2 O_j,\quad C_0 = \frac{\Delta t-2KX}{2K(1-X)+\Delta t}")


# =============================================================================================== groundwater
def _K(sym="K", meaning="hydraulic conductivity", names=("hydraulic conductivity",), show="m/s", **kw):
    return _in(sym, "hydraulic_conductivity", "conductivity", meaning, names, show, **kw)


def _T(sym="T", meaning="transmissivity", names=("transmissivity",), show="m^2/d", **kw):
    return _in(sym, "transmissivity", "transmissivity", meaning, names, show, **kw)


def layered_darcy(H, K, A, dh):
    H = _vec("thicknesses", H, positive=True)
    Kc = _vec("conductivities", K, len(H), positive=True)
    _pos(A=A)
    _finite(dh=dh)
    _need(dh > 0, "the head difference must be positive")
    Ltot = sum(H)
    Kv = Ltot / sum(h / k for h, k in zip(H, Kc))
    Kh = sum(h * k for h, k in zip(H, Kc)) / Ltot
    q = Kv * dh / Ltot
    heads, h = [], dh
    for hi, ki in zip(H, Kc):
        h -= q * hi / ki
        heads.append(_snap(h, dh))
    return {"Q": _r(q * A), "q": _r(q), "Kv": _r(Kv), "Kh": _r(Kh), "heads": _r(heads[:-1] or [0.0])}


def _ref_layered(H, K, A, dh):
    import numpy as np
    # finite-volume solution of d/dz(K dh/dz) = 0: nodes at the layer boundaries, conductances K_i/H_i in series
    n = len(H)
    M = np.zeros((n + 1, n + 1))
    b = np.zeros(n + 1)
    M[0, 0], b[0] = 1.0, dh
    M[n, n], b[n] = 1.0, 0.0
    c = [K[i] / H[i] for i in range(n)]
    for j in range(1, n):
        M[j, j - 1], M[j, j], M[j, j + 1] = c[j - 1], -(c[j - 1] + c[j]), c[j]
    h = np.linalg.solve(M, b)
    q = c[0] * (h[0] - h[1])
    flows = [k * h_ / sum(H) for k, h_ in zip(K, H)]     # parallel flow: each layer under a unit gradient
    return {"Q": q * A, "q": q, "Kv": q * sum(H) / dh, "Kh": sum(flows), "heads": [float(v) for v in h[1:-1]] or [0.0]}


proc("darcy_flow_layered_soil", "Darcy flow across soil layers in series; equivalent conductivities", "groundwater",
     "Flow rate, specific discharge and the heads at the layer interfaces for one-dimensional Darcy flow across one or "
     "more soil layers in series (flow perpendicular to the layering), with the equivalent vertical (series) and "
     "horizontal (parallel) conductivities. Heads are measured from the downstream face.",
     [_VEC("H", "layer thicknesses in m, in flow order", ("thicknesses", "layer thicknesses")),
      _VEC("K", "hydraulic conductivity of each layer in m/s", ("conductivities", "layer conductivities")),
      _in("A", "area", "area", "flow cross-section area", ("area", "cross section area"), "m^2"),
      _L("dh", "head difference across all layers", ("head difference", "head loss"))],
     [_o("Q", "flow_rate", "flow rate (m^3/s)"), _o("q", "speed", "specific discharge (Darcy velocity, m/s)"),
      _o("Kv", "hydraulic_conductivity", "equivalent conductivity across the layers (m/s)"),
      _o("Kh", "hydraulic_conductivity", "equivalent conductivity along the layers (m/s)"),
      _o("heads", "length", "head at each interior interface above the outlet (m)")],
     layered_darcy, _ref_layered,
     lambda H, K, A, dh: (lambda Kv: dict(layered_darcy(H, K, A, dh), Q=_r(Kv * dh / sum(H) * A), q=_r(Kv * dh / sum(H)), Kv=_r(Kv)))(
         sum(h * k for h, k in zip(H, K)) / sum(H)),
     lambda rng: (lambda n: {"H": [round(rng.uniform(0.2, 5), 2) for _ in range(n)], "K": [10 ** rng.uniform(-8, -3) for _ in range(n)],
                             "A": rng.uniform(1, 500), "dh": rng.uniform(0.1, 20)})(rng.randint(2, 5)),
     [{"H": [2.0], "K": [1e-5], "A": 10, "dh": 0.5}, {"H": [1.5, 2.0, 1.0], "K": [2e-5, 5e-7, 1e-4], "A": 50, "dh": 3}],
     ["darcy flow through layered soil", "darcy flow across soil layers", "darcy flow", "equivalent hydraulic conductivity"],
     conditions=["steady, saturated, laminar flow", "flow perpendicular to the layers"],
     method="Kv = sum H / sum(H_i/K_i), Kh = sum(K_i H_i)/sum H, q = Kv dh/sum H, Q = q A",
     latex=r"K_v = \frac{\sum H_i}{\sum H_i/K_i},\quad K_h = \frac{\sum K_iH_i}{\sum H_i},\quad Q = K_v\frac{\Delta h}{\sum H_i}A")


def falling_head(d, D, L, h1, h2, t):
    _pos(d=d, D=D, L=L, h1=h1, h2=h2, t=t)
    _need(h2 < h1, "the head must fall: h2 < h1")
    a, A = math.pi * d * d / 4, math.pi * D * D / 4
    K = a * L / (A * t) * math.log(h1 / h2)
    return {"K": _r(K), "K_cm_s": _r(K * 100)}


def _ref_falling_head(d, D, L, h1, h2, t):
    a, A = math.pi * d * d / 4, math.pi * D * D / 4
    # continuity in the standpipe, -a dh/dt = K (h/L) A: the time to fall from h1 to h2 for a unit K is the integral
    # of a L/(A h) dh (quadrature); K is that time divided by the observed one
    t_unit = _quad(lambda h: a * L / (A * h), h2, h1)
    K = t_unit / t
    return {"K": K, "K_cm_s": K * 100}


proc("falling_head_permeability", "Hydraulic conductivity from a falling-head test", "groundwater",
     "Hydraulic conductivity of a soil sample from a falling-head permeameter: standpipe diameter d, sample diameter D "
     "and length L, the standpipe head falling from h1 to h2 in time t.",
     [_L("d", "standpipe diameter", ("standpipe diameter",), "mm"), _L("D", "sample diameter", ("sample diameter",), "mm"),
      _L("L", "sample length", ("sample length", "length"), "mm"), _L("h1", "initial head", ("initial head",), "cm"),
      _L("h2", "final head", ("final head",), "cm"), _in("t", "time", "time", "elapsed time", ("time",), "s")],
     [_o("K", "hydraulic_conductivity", "hydraulic conductivity (m/s)"), _o("K_cm_s", "number", "hydraulic conductivity (cm/s)")],
     falling_head, _ref_falling_head,
     lambda d, D, L, h1, h2, t: (lambda K: {"K": _r(K), "K_cm_s": _r(100 * K)})(d * d * L / (D * D * t) * math.log10(h1 / h2)),
     lambda rng: (lambda h1: {"d": rng.uniform(0.005, 0.02), "D": rng.uniform(0.05, 0.1), "L": rng.uniform(0.05, 0.2),
                              "h1": h1, "h2": h1 * rng.uniform(0.2, 0.9), "t": rng.uniform(60, 7200)})(rng.uniform(0.3, 1.5)),
     [{"d": 10, "D": 75, "L": 150, "h1": 100, "h2": 60, "t": 600}, {"d": 6, "D": 100, "L": 120, "h1": 80, "h2": 40, "t": 1800}],
     ["falling head permeability", "falling head test", "falling head permeameter"],
     conditions=["h2 < h1", "saturated sample, laminar flow"], invalid=[{"d": 10, "D": 75, "L": 150, "h1": 60, "h2": 100, "t": 600}],
     method="K = (a L/(A t)) ln(h1/h2), a and A the standpipe and sample areas",
     latex=r"K = \frac{aL}{At}\ln\frac{h_1}{h_2}")


def thiem_confined(Q, r1, r2, s1, s2, b=None):
    _pos(Q=Q, r1=r1, r2=r2)
    _finite(s1=s1, s2=s2)
    _need(r2 > r1, "the second observation well must be farther from the pumped well: r2 > r1")
    _need(s1 > s2 >= 0, "the drawdown must decrease with distance: s1 > s2 >= 0")
    T = Q * math.log(r2 / r1) / (2 * math.pi * (s1 - s2))
    out = {"T": _r(T), "T_m2_d": _r(T * 86400)}
    if b is not None:
        _pos(b=b)
        out["K"] = _r(T / b)
    return out


def _ref_thiem_conf(Q, r1, r2, s1, s2, b=None):
    # Darcy's law on cylinders: ds/dr = -Q/(2 pi T r); integrate from r1 to r2 numerically for a unit T
    unit = _quad(lambda r: Q / (2 * math.pi * r), r1, r2)
    T = unit / (s1 - s2)
    out = {"T": T, "T_m2_d": T * 86400}
    if b is not None:
        out["K"] = T / b
    return out


proc("thiem_confined_transmissivity", "Transmissivity from steady pumping (Thiem, confined aquifer)", "groundwater",
     "Transmissivity (and conductivity, if the aquifer thickness is given) of a confined aquifer from the steady "
     "drawdowns in two observation wells at distances r1 < r2 from a well pumped at Q (Thiem equation).",
     [_Qin(meaning="pumping rate", names=("pumping rate", "discharge"), show="m^3/d"),
      _L("r1", "distance of the nearer observation well", ("first distance",)),
      _L("r2", "distance of the farther observation well", ("second distance",)),
      _L("s1", "drawdown at r1", ("first drawdown",)), _L("s2", "drawdown at r2", ("second drawdown",)),
      _L("b", "aquifer thickness (optional)", ("aquifer thickness", "thickness"), required=False)],
     [_o("T", "transmissivity", "transmissivity (m^2/s)"), _o("T_m2_d", "number", "transmissivity (m^2/day)"),
      _o("K", "hydraulic_conductivity", "hydraulic conductivity T/b (m/s)")],
     thiem_confined, _ref_thiem_conf,
     lambda Q, r1, r2, s1, s2, b=None: (lambda T: dict({"T": _r(T), "T_m2_d": _r(T * 86400)}, **({"K": _r(T / b)} if b else {})))(
         Q * math.log10(r2 / r1) / (2 * math.pi * (s1 - s2))),
     lambda rng: (lambda r1: {"Q": rng.uniform(100, 5000) / 86400, "r1": r1, "r2": r1 * rng.uniform(1.5, 20),
                              "s1": rng.uniform(1, 6), "s2": rng.uniform(0.05, 0.9), "b": rng.uniform(5, 60)})(rng.uniform(5, 50)),
     [{"Q": 2500, "r1": 15, "r2": 60, "s1": 2.8, "s2": 1.1}, {"Q": 1200, "r1": 10, "r2": 100, "s1": 4.2, "s2": 0.9, "b": 20}],
     ["thiem confined transmissivity", "thiem equation confined", "steady pumping test confined"],
     conditions=["steady state", "confined aquifer, fully penetrating well", "r1 < r2, s1 > s2"],
     invalid=[{"Q": 2500, "r1": 60, "r2": 15, "s1": 2.8, "s2": 1.1}],
     method="T = Q ln(r2/r1)/(2 pi (s1 - s2))", latex=r"T = \frac{Q\ln(r_2/r_1)}{2\pi(s_1-s_2)}")


def thiem_unconfined(Q, r1, r2, h1, h2):
    _pos(Q=Q, r1=r1, r2=r2, h1=h1, h2=h2)
    _need(r2 > r1, "need r2 > r1")
    _need(h2 > h1, "the saturated thickness must rise with distance: h2 > h1")
    K = Q * math.log(r2 / r1) / (math.pi * (h2 * h2 - h1 * h1))
    return {"K": _r(K), "K_m_d": _r(K * 86400)}


def _ref_thiem_unconf(Q, r1, r2, h1, h2):
    unit = _quad(lambda r: Q / (math.pi * r), r1, r2)     # Dupuit: d(h^2)/dr = Q/(pi K r)
    K = unit / (h2 * h2 - h1 * h1)
    return {"K": K, "K_m_d": K * 86400}


proc("thiem_unconfined_conductivity", "Hydraulic conductivity from steady pumping (unconfined aquifer)", "groundwater",
     "Hydraulic conductivity of an unconfined aquifer from the steady saturated thicknesses h1 < h2 in two observation "
     "wells at distances r1 < r2 from a well pumped at Q (Thiem-Dupuit equation).",
     [_Qin(meaning="pumping rate", names=("pumping rate", "discharge"), show="m^3/d"),
      _L("r1", "distance of the nearer observation well", ("first distance",)),
      _L("r2", "distance of the farther observation well", ("second distance",)),
      _L("h1", "saturated thickness at r1", ("first head",)), _L("h2", "saturated thickness at r2", ("second head",))],
     [_o("K", "hydraulic_conductivity", "hydraulic conductivity (m/s)"), _o("K_m_d", "number", "hydraulic conductivity (m/day)")],
     thiem_unconfined, _ref_thiem_unconf,
     lambda Q, r1, r2, h1, h2: (lambda K: {"K": _r(K), "K_m_d": _r(K * 86400)})(Q * math.log(r2 / r1) / (math.pi * (h2 - h1) ** 2)),
     lambda rng: (lambda r1, h1: {"Q": rng.uniform(100, 5000) / 86400, "r1": r1, "r2": r1 * rng.uniform(1.5, 20), "h1": h1,
                                  "h2": h1 + rng.uniform(0.3, 5)})(rng.uniform(5, 50), rng.uniform(5, 40)),
     [{"Q": 1500, "r1": 20, "r2": 80, "h1": 18.2, "h2": 19.6}, {"Q": 800, "r1": 12, "r2": 45, "h1": 9.5, "h2": 11.0}],
     ["thiem unconfined conductivity", "unconfined steady pumping test", "dupuit thiem well"],
     conditions=["steady state", "unconfined aquifer, Dupuit assumptions", "r1 < r2, h1 < h2"],
     method="K = Q ln(r2/r1)/(pi (h2^2 - h1^2))", latex=r"K = \frac{Q\ln(r_2/r_1)}{\pi(h_2^2-h_1^2)}")


def _E1(u):
    """Exponential integral E1(u) = Theis well function W(u): power series for u <= 1, continued fraction above."""
    if u <= 1.0:
        s, term, k = 0.0, 1.0, 1
        while True:
            term *= -u / k if k > 1 else -u
            add = -term / k
            s += add
            if abs(add) < 1e-17 * abs(s) or k > 200:
                break
            k += 1
        return -EULER_GAMMA - math.log(u) + s
    # modified Lentz evaluation of the continued fraction E1(u) = e^-u / (u + 1/(1 + 1/(u + 2/(1 + 2/(u + ...)))))
    b = u + 1.0
    c = 1.0 / 1e-300
    d = 1.0 / b
    h = d
    for i in range(1, 500):
        an = -i * i
        b += 2.0
        d = 1.0 / (an * d + b)
        c = b + an / c
        de = c * d
        h *= de
        if abs(de - 1.0) < 1e-16:
            break
    return h * math.exp(-u)


def theis(Q, T, S, r, t):
    _pos(Q=Q, T=T, S=S, r=r, t=t)
    _need(S < 1, "the storativity is a fraction below 1")
    u = r * r * S / (4 * T * t)
    _need(u <= 50, "u exceeds 50: the drawdown is negligible (below 1e-23 of Q/(4 pi T))")
    W = _E1(u)
    return {"s": _r(Q / (4 * math.pi * T) * W), "u": _r(u), "W": _r(W)}


def _ref_theis(Q, T, S, r, t):
    from scipy.special import exp1
    u = r * r * S / (4 * T * t)
    W = float(exp1(u))
    return {"s": Q / (4 * math.pi * T) * W, "u": u, "W": W}


proc("theis_drawdown", "Theis drawdown around a pumped well", "groundwater",
     "Transient drawdown at distance r and time t from a well pumped at a constant rate in a confined aquifer (Theis), "
     "with u and the well function W(u) = E1(u).",
     [_Qin(meaning="pumping rate", names=("pumping rate", "discharge"), show="m^3/d"), _T(),
      _N("S", "storativity (storage coefficient)", ("storativity", "storage coefficient")),
      _L("r", "distance from the pumped well", ("distance", "radius")),
      _in("t", "time", "time", "time since pumping began", ("time",), "h")],
     [_o("s", "length", "drawdown (m)"), _o("u", "number", "u = r^2 S/(4 T t)"), _o("W", "number", "well function W(u)")],
     theis, _ref_theis,
     lambda Q, T, S, r, t: (lambda u: {"s": _r(Q / (4 * math.pi * T) * (-EULER_GAMMA - math.log(u))), "u": _r(u),
                                       "W": _r(-EULER_GAMMA - math.log(u))})(r * r * S / (4 * T * t)),
     lambda rng: (lambda T, S, r: {"Q": rng.uniform(200, 5000) / 86400, "T": T, "S": S, "r": r,
                                   "t": r * r * S / (4 * T * 10 ** rng.uniform(-4, 0.7))})(
         rng.uniform(50, 2000) / 86400, 10 ** rng.uniform(-5, -2.5), rng.uniform(5, 300)),
     [{"Q": 2500, "T": 500, "S": 0.0002, "r": 100, "t": 24}, {"Q": 1000, "T": 150, "S": 0.001, "r": 60, "t": 3}],
     ["theis drawdown", "theis equation", "transient drawdown around a well"],
     conditions=["confined, homogeneous, infinite aquifer", "fully penetrating well, constant rate", "u <= 50"],
     method="u = r^2 S/(4 T t), W(u) = E1(u) (series for u <= 1, continued fraction above), s = Q W/(4 pi T)",
     latex=r"s = \frac{Q}{4\pi T}W(u),\quad u = \frac{r^2S}{4Tt},\quad W(u) = \int_u^\infty \frac{e^{-x}}{x}\,dx")


def _linfit(x, y):
    n = len(x)
    mx, my = sum(x) / n, sum(y) / n
    sxx = sum((a - mx) ** 2 for a in x)
    _need(sxx > 0, "the times must not all be equal")
    b = sum((a - mx) * (c - my) for a, c in zip(x, y)) / sxx
    return b, my - b * mx


def cooper_jacob(times, drawdowns, Q, r):
    t = _vec("times", times, positive=True)
    s = _vec("drawdowns", drawdowns, len(t))
    _need(len(t) >= 2, "give at least two readings")
    _pos(Q=Q, r=r)
    slope, icpt = _linfit([math.log10(v) for v in t], s)
    _need(slope > 0, "the drawdown must increase with time")
    T = 2.303 * Q / (4 * math.pi * slope)
    t0 = 10 ** (-icpt / slope) * 60                  # minutes -> s
    S = 2.25 * T * t0 / (r * r)
    return {"T": _r(T), "S": _r(S), "ds_per_cycle": _r(slope), "t0_min": _r(t0 / 60), "T_m2_d": _r(T * 86400)}


def _ref_cooper_jacob(times, drawdowns, Q, r):
    import numpy as np
    slope, icpt = np.polyfit(np.log10(np.asarray(times, float)), np.asarray(drawdowns, float), 1)
    T = 2.303 * Q / (4 * math.pi * slope)
    t0 = 10 ** (-icpt / slope) * 60
    return {"T": T, "S": 2.25 * T * t0 / (r * r), "ds_per_cycle": float(slope), "t0_min": t0 / 60, "T_m2_d": T * 86400}


def _rand_cj(rng):
    T, S, Q, r = rng.uniform(50, 2000) / 86400, 10 ** rng.uniform(-5, -3), rng.uniform(500, 5000) / 86400, rng.uniform(10, 100)
    ts = sorted(rng.uniform(10, 2000) for _ in range(rng.randint(4, 10)))
    sd = [2.303 * Q / (4 * math.pi * T) * math.log10(2.25 * T * v * 60 / (r * r * S)) + rng.uniform(-0.02, 0.02) for v in ts]
    return {"times": [round(v, 1) for v in ts], "drawdowns": [round(v, 3) for v in sd], "Q": Q, "r": r}


proc("cooper_jacob_analysis", "Transmissivity and storativity by the Cooper-Jacob straight line", "groundwater",
     "Transmissivity and storativity of a confined aquifer from a pumping test by the Cooper-Jacob method: a "
     "least-squares straight line through drawdown against log10(time) (late-time data, u < 0.05, chosen by the user). "
     "Times are in minutes, drawdowns in m.",
     [_VEC("times", "times since pumping began, in minutes", ("times", "time readings"), min_length=2),
      _VEC("drawdowns", "drawdowns at those times, in m", ("drawdowns",), min_length=2),
      _Qin(meaning="pumping rate", names=("pumping rate", "discharge"), show="m^3/d"),
      _L("r", "distance of the observation well", ("distance", "radius"))],
     [_o("T", "transmissivity", "transmissivity (m^2/s)"), _o("S", "number", "storativity"),
      _o("ds_per_cycle", "length", "drawdown per log cycle of time (m)"), _o("t0_min", "number", "zero-drawdown intercept time (min)"),
      _o("T_m2_d", "number", "transmissivity (m^2/day)")],
     cooper_jacob, _ref_cooper_jacob,
     lambda times, drawdowns, Q, r: (lambda o: dict(o, T=_r(o["T"] / 2.303), T_m2_d=_r(o["T_m2_d"] / 2.303)))(cooper_jacob(times, drawdowns, Q, r)),
     _rand_cj,
     [{"times": [10, 20, 50, 100, 200, 500], "drawdowns": [0.62, 0.79, 1.02, 1.19, 1.36, 1.59], "Q": 2000, "r": 50},
      {"times": [30, 60, 120, 240, 480], "drawdowns": [1.10, 1.32, 1.55, 1.77, 2.00], "Q": 1500, "r": 30}],
     ["cooper jacob analysis", "cooper jacob straight line", "cooper jacob method"],
     conditions=["late-time data on the straight line (u < 0.05)", "confined aquifer"],
     method="fit s = a + b log10 t; T = 2.303 Q/(4 pi b), S = 2.25 T t0/r^2 with t0 = 10^(-a/b)",
     latex=r"T = \frac{2.303\,Q}{4\pi\,\Delta s},\quad S = \frac{2.25\,T\,t_0}{r^2}")


def well_superposition(wx, wy, Qs, T, S, t, x0, y0):
    wx = _vec("well x", wx)
    wy = _vec("well y", wy, len(wx))
    Qs = _vec("pumping rates", Qs, len(wx))
    _pos(T=T, S=S, t=t)
    _need(S < 1, "the storativity is a fraction below 1")
    _finite(x0=x0, y0=y0)
    parts = []
    for x, y, q in zip(wx, wy, Qs):
        r = math.hypot(x - x0, y - y0)
        _need(r >= 0.05, "the point is at (or within 5 cm of) a well: give a point away from the wells")
        u = r * r * S / (4 * T * t)
        parts.append(0.0 if u > 50 else q / 86400 / (4 * math.pi * T) * _E1(u))
    tot = sum(parts)
    return {"s": _r(tot), "parts": _r(parts)}


def _ref_superposition(wx, wy, Qs, T, S, t, x0, y0):
    import numpy as np
    from scipy.special import exp1
    r = np.hypot(np.asarray(wx) - x0, np.asarray(wy) - y0)
    u = r * r * S / (4 * T * t)
    parts = np.asarray(Qs) / 86400 / (4 * math.pi * T) * exp1(u)
    return {"s": float(parts.sum()), "parts": list(parts)}


def _rand_superposition(rng):
    n = rng.randint(2, 4)
    T, S = rng.uniform(100, 1500) / 86400, 10 ** rng.uniform(-4.5, -3)
    t = rng.uniform(0.5, 30) * 86400
    return {"wx": [round(rng.uniform(-300, 300), 1) for _ in range(n)], "wy": [round(rng.uniform(-300, 300), 1) for _ in range(n)],
            "Qs": [round(rng.uniform(200, 3000), 0) * rng.choice([1, 1, 1, -1]) for _ in range(n)],
            "T": T, "S": S, "t": t, "x0": round(rng.uniform(-50, 50), 1), "y0": round(rng.uniform(-50, 50), 1)}


proc("well_field_drawdown", "Drawdown from several wells by superposition (Theis)", "groundwater",
     "Total drawdown at a point from several wells pumping (positive rate) or injecting (negative rate) at constant "
     "rates in a confined aquifer, by superposing Theis solutions; with each well's contribution. Coordinates in m, "
     "rates in m^3/day.",
     [_VEC("wx", "well x coordinates in m", ("well x",)), _VEC("wy", "well y coordinates in m", ("well y",)),
      _VEC("Qs", "pumping rates in m^3/day (negative for injection)", ("pumping rates", "rates")), _T(),
      _N("S", "storativity", ("storativity", "storage coefficient")),
      _in("t", "time", "time", "time since pumping began (all wells together)", ("time",), "d"),
      _L("x0", "x coordinate of the point", ("point x",)), _L("y0", "y coordinate of the point", ("point y",))],
     [_o("s", "length", "total drawdown (m)"), _o("parts", "length", "drawdown from each well (m)")],
     well_superposition, _ref_superposition,
     lambda wx, wy, Qs, T, S, t, x0, y0: (lambda o: dict(o, s=_r(max(o["parts"]))))(well_superposition(wx, wy, Qs, T, S, t, x0, y0)),
     _rand_superposition,
     [{"wx": [0, 200], "wy": [0, 0], "Qs": [2000, 1500], "T": 400, "S": 0.0003, "t": 5, "x0": 100, "y0": 50},
      {"wx": [-150, 150, 0], "wy": [0, 0, 200], "Qs": [1800, -1000, 1200], "T": 800, "S": 0.0001, "t": 2, "x0": 0, "y0": 0}],
     ["well field drawdown", "drawdown from several wells", "superposition of wells"],
     conditions=["confined, homogeneous, infinite aquifer", "all wells started together", "point at least 5 cm from any well"],
     method="s = sum Q_i W(u_i)/(4 pi T), u_i = r_i^2 S/(4 T t)", latex=r"s = \sum_i \frac{Q_i}{4\pi T}W\!\left(\frac{r_i^2S}{4Tt}\right)")


def dupuit_recharge(K, h1, h2, L, W):
    _pos(K=K, h1=h1, h2=h2, L=L, W=W)
    c = (h1 * h1 - h2 * h2) / (2 * L)
    q0 = K * c - W * L / 2                      # flow at x = 0 (positive toward x = L)
    qL = K * c + W * L / 2
    d = L / 2 - K * (h1 * h1 - h2 * h2) / (2 * W * L)
    hsq = lambda x: h1 * h1 - (h1 * h1 - h2 * h2) * x / L + W / K * (L - x) * x      # noqa: E731
    hmax = math.sqrt(hsq(d)) if 0 < d < L else max(h1, h2)
    return {"q0": _r(_snap(q0, abs(K * c) + W * L)), "qL": _r(_snap(qL, abs(K * c) + W * L)), "x_divide": _r(d), "h_max": _r(hmax)}


def _ref_dupuit_recharge(K, h1, h2, L, W):
    from scipy.integrate import solve_ivp

    def h_at(q0, x):
        s = solve_ivp(lambda xx, h: [-(q0 + W * xx) / (K * h[0])], (0.0, x), [h1], method="DOP853", rtol=1e-13, atol=1e-14)
        if not s.success or s.t[-1] < x * (1 - 1e-12):
            return 0.0
        return s.y[0, -1]
    # the head at x = L must equal h2: shoot on the inflow q0. Bracket: a strong inflow from x = L keeps the water
    # table high; at the upper end the water table just reaches the base at x = L
    lo, hi = -W * L - K * (h1 * h1 + h2 * h2) / L, (K * h1 * h1 / (2 * L) - W * L / 2) * (1 - 1e-9)
    q0 = _brentq(lambda q: h_at(q, L) ** 2 - h2 ** 2, lo, hi)
    d = -q0 / W
    hmax = h_at(q0, d) if 0 < d < L else max(h1, h2)
    return {"q0": q0, "qL": q0 + W * L, "x_divide": d, "h_max": hmax}


proc("dupuit_flow_with_recharge", "Unconfined flow between two water bodies with recharge (Dupuit)", "groundwater",
     "Steady unconfined flow per unit width between two water bodies (heads h1 at x = 0 and h2 at x = L above the base) "
     "with uniform recharge W: the flows at both ends (positive toward x = L), the groundwater divide and the highest "
     "water table (Dupuit-Forchheimer).",
     [_K(show="m/d"), _L("h1", "head at x = 0 (above the impermeable base)", ("first head", "upstream head")),
      _L("h2", "head at x = L", ("second head", "downstream head")), _L("L", "distance between the water bodies", ("distance", "length")),
      _in("W", "rainfall_intensity", "conductivity", "recharge rate", ("recharge", "recharge rate"), "mm/yr")],
     [_o("q0", "volume_transport_per_width", "flow per unit width at x = 0 (m^2/s, positive toward x = L)"),
      _o("qL", "volume_transport_per_width", "flow per unit width at x = L (m^2/s)"),
      _o("x_divide", "length", "position of the groundwater divide (m); outside 0..L means no divide"),
      _o("h_max", "length", "highest water table between the water bodies (m)")],
     dupuit_recharge, _ref_dupuit_recharge,
     lambda K, h1, h2, L, W: (lambda o: dict(o, q0=_r(K * (h1 - h2) / L * (h1 + h2) / 2 * 2 - W * L / 2)))(dupuit_recharge(K, h1, h2, L, W)),
     lambda rng: {"K": rng.uniform(0.5, 30) / 86400, "h1": rng.uniform(5, 25), "h2": rng.uniform(5, 25), "L": rng.uniform(100, 2000),
                  "W": rng.uniform(50, 800) * 1e-3 / YEAR},
     [{"K": 10, "h1": 12, "h2": 9, "L": 600, "W": 300}, {"K": 2, "h1": 8, "h2": 8, "L": 400, "W": 500}],
     ["dupuit recharge flow", "unconfined recharge flow", "dupuit flow under recharge"],
     conditions=["steady, unconfined, homogeneous aquifer on a horizontal base", "Dupuit assumptions", "W > 0"],
     method="h^2 = h1^2 - (h1^2 - h2^2) x/L + (W/K)(L - x) x; q = K(h1^2 - h2^2)/(2L) - W(L/2 - x)",
     latex=r"h^2 = h_1^2 - \frac{(h_1^2-h_2^2)x}{L} + \frac{W}{K}(L-x)x,\quad q(x) = \frac{K(h_1^2-h_2^2)}{2L} - W\left(\frac L2 - x\right)")


# =============================================================================================== soil mechanics
def _gh_phase():
    from groundhog.siteinvestigation.classification import phaserelations as ph
    return ph


def phase_from_void_ratio(Gs, e, w):
    _finite(Gs=Gs, e=e, w=w)
    _need(2.0 <= Gs <= 3.0, "the specific gravity of solids must lie between 2.0 and 3.0")
    _need(0 < e <= 4, "the void ratio must lie between 0 and 4")
    _need(w >= 0, "the water content must be zero or positive")
    S = w * Gs / e
    _need(S <= 1 + 1e-12, "w Gs exceeds e: the soil would be more than saturated")
    gd = Gs * GAMMA_W / (1 + e)
    g = Gs * GAMMA_W * (1 + w) / (1 + e)
    gsat = (Gs + e) * GAMMA_W / (1 + e)
    return {"n": _r(e / (1 + e)), "S": _r(min(S, 1.0)), "gamma_d": _r(gd), "gamma": _r(g), "gamma_sat": _r(gsat),
            "gamma_sub": _r(gsat - GAMMA_W)}


def _ref_phase_e(Gs, e, w):
    ph = _gh_phase()
    gw = GAMMA_W / 1e3
    S = ph.saturation_watercontent(w, e, specific_gravity=Gs)["saturation [-]"]
    bulk = ph.bulkunitweight(S, e, specific_gravity=Gs, unitweight_water=gw)["bulk unit weight [kN/m3]"]
    sat = ph.bulkunitweight(1.0, e, specific_gravity=Gs, unitweight_water=gw)
    gd = ph.dryunitweight_watercontent(w, bulk)["dry unit weight [kN/m3]"]
    return {"n": ph.porosity_voidratio(e)["porosity [-]"], "S": S, "gamma_d": gd * 1e3, "gamma": bulk * 1e3,
            "gamma_sat": sat["bulk unit weight [kN/m3]"] * 1e3, "gamma_sub": sat["effective unit weight [kN/m3]"] * 1e3}


UW_OUT = lambda sym, meaning: _o(sym, "unit_weight", meaning + " (N/m^3)")      # noqa: E731

proc("soil_phase_relations", "Soil phase relations from void ratio and water content", "soil mechanics",
     "Porosity, degree of saturation and the dry, bulk, saturated and submerged unit weights of a soil from its "
     "specific gravity of solids Gs, void ratio e and water content w (unit weight of water 9.81 kN/m^3).",
     [_N("Gs", "specific gravity of the solids", ("specific gravity",)), _N("e", "void ratio", ("void ratio",)),
      _F("w", "water content (fraction, or % in a question)", ("water content", "moisture content"), "%")],
     [_o("n", "number", "porosity e/(1 + e)"), _o("S", "number", "degree of saturation w Gs/e"),
      UW_OUT("gamma_d", "dry unit weight"), UW_OUT("gamma", "bulk (moist) unit weight"),
      UW_OUT("gamma_sat", "saturated unit weight"), UW_OUT("gamma_sub", "submerged (buoyant) unit weight")],
     phase_from_void_ratio, _ref_phase_e,
     lambda Gs, e, w: dict(phase_from_void_ratio(Gs, e, w), gamma_d=_r(Gs * GAMMA_W / e)),
     lambda rng: (lambda Gs, e: {"Gs": Gs, "e": e, "w": e / Gs * rng.uniform(0.1, 1.0)})(rng.uniform(2.55, 2.8), rng.uniform(0.35, 1.3)),
     [{"Gs": 2.7, "e": 0.65, "w": 18}, {"Gs": 2.65, "e": 0.9, "w": 30}], ["soil phase relations", "phase relations of a soil",
                                                                             "soil unit weights from void ratio"],
     conditions=["w Gs <= e (not oversaturated)", "2.0 <= Gs <= 3.0"], invalid=[{"Gs": 2.7, "e": 0.5, "w": 40}],
     method="n = e/(1 + e), S = w Gs/e, gamma_d = Gs gamma_w/(1 + e), gamma = Gs gamma_w (1 + w)/(1 + e), gamma_sat = (Gs + e) gamma_w/(1 + e)",
     latex=r"S e = w G_s,\quad \gamma = \frac{G_s(1+w)}{1+e}\gamma_w,\quad \gamma_{sat} = \frac{G_s+e}{1+e}\gamma_w")


def phase_from_lab(M, Md, V, Gs):
    _pos(M=M, Md=Md, V=V)
    _finite(Gs=Gs)
    _need(M >= Md, "the wet mass cannot be less than the dry mass")
    _need(2.0 <= Gs <= 3.0, "the specific gravity of solids must lie between 2.0 and 3.0")
    w = (M - Md) / Md
    rho, rhod = M / V, Md / V
    e = Gs * RHO_W / rhod - 1
    _need(e > 0, "the dry density exceeds that of the solids: check the masses and volume")
    S = w * Gs / e
    _need(S <= 1 + 1e-9, "the data imply more than full saturation: check the masses and volume")
    return {"w": _r(w), "rho": _r(rho), "rho_d": _r(rhod), "e": _r(e), "n": _r(e / (1 + e)), "S": _r(S),
            "gamma": _r(rho * G), "gamma_d": _r(rhod * G)}


def _ref_phase_lab(M, Md, V, Gs):
    ph = _gh_phase()
    w = (M - Md) / Md
    rhod = Md / V
    e = ph.voidratio_drydensity(rhod, specific_gravity=Gs, water_density=RHO_W)["Void ratio [-]"]
    return {"w": w, "rho": M / V, "rho_d": rhod, "e": e, "n": ph.porosity_voidratio(e)["porosity [-]"],
            "S": ph.saturation_watercontent(w, e, specific_gravity=Gs)["saturation [-]"],
            "gamma": ph.unitweight_density(M / V, g=G)["Unit weight [kN/m3]"] * 1e3,
            "gamma_d": ph.unitweight_density(rhod, g=G)["Unit weight [kN/m3]"] * 1e3}


def _rand_lab(rng):
    while True:
        Gs = rng.uniform(2.55, 2.8)
        rhod = rng.uniform(1300, 1950)
        V = rng.uniform(5e-5, 1e-3)
        e = Gs * 1000 / rhod - 1
        w = e / Gs * rng.uniform(0.1, 0.99)
        if e > 0.2:
            return {"M": rhod * V * (1 + w), "Md": rhod * V, "V": V, "Gs": Gs}


proc("soil_phase_from_lab_masses", "Soil properties from a weighed sample", "soil mechanics",
     "Water content, bulk and dry densities and unit weights, void ratio, porosity and degree of saturation of a soil "
     "sample from its wet mass, oven-dry mass, volume and the specific gravity of the solids.",
     [_in("M", "mass", "mass", "wet (moist) mass of the sample", ("wet mass", "moist mass", "mass"), "g"),
      _in("Md", "mass", "mass", "oven-dry mass of the sample", ("dry mass",), "g"),
      _in("V", "volume", "volume", "volume of the sample", ("volume",), "cm^3"),
      _N("Gs", "specific gravity of the solids", ("specific gravity",))],
     [_o("w", "number", "water content"), _o("rho", "density", "bulk density (kg/m^3)"), _o("rho_d", "density", "dry density (kg/m^3)"),
      _o("e", "number", "void ratio"), _o("n", "number", "porosity"), _o("S", "number", "degree of saturation"),
      UW_OUT("gamma", "bulk unit weight"), UW_OUT("gamma_d", "dry unit weight")],
     phase_from_lab, _ref_phase_lab,
     lambda M, Md, V, Gs: dict(phase_from_lab(M, Md, V, Gs), w=_r((M - Md) / M)),
     _rand_lab,
     [{"M": 1850, "Md": 1600, "V": 1000, "Gs": 2.7}, {"M": 205, "Md": 178, "V": 105, "Gs": 2.68}],
     ["soil properties from a weighed sample", "phase relations from sample masses", "soil sample phase relations"],
     conditions=["M >= Md", "consistent data (0 < e, S <= 1)"], invalid=[{"M": 1500, "Md": 1600, "V": 1000, "Gs": 2.7}],
     method="w = (M - Md)/Md, rho_d = Md/V, e = Gs rho_w/rho_d - 1, S = w Gs/e",
     latex=r"w = \frac{M-M_d}{M_d},\quad e = \frac{G_s\rho_w}{\rho_d}-1,\quad S = \frac{wG_s}{e}")


def relative_density(gd, gd_min, gd_max):
    _pos(gd=gd, gd_min=gd_min, gd_max=gd_max)
    _need(gd_min < gd_max, "the minimum dry unit weight must be below the maximum")
    _need(gd_min <= gd <= gd_max, "the dry unit weight must lie between the minimum and the maximum")
    Dr = gd_max * (gd - gd_min) / (gd * (gd_max - gd_min))
    return {"Dr": _r(Dr), "Dr_percent": _r(100 * Dr)}


def _ref_relative_density(gd, gd_min, gd_max):
    ph = _gh_phase()
    # through void ratios with an arbitrary Gs: the relative density does not depend on it
    e = [ph.voidratio_drydensity(x / G, specific_gravity=2.65, water_density=RHO_W)["Void ratio [-]"] for x in (gd, gd_min, gd_max)]
    # groundhog returns (e - e_min)/(e_max - e_min), the complement of the usual density index (checked by hand:
    # e = 0.6 between 0.4 and 0.9 gives 0.4, where (e_max - e)/(e_max - e_min) = 0.6)
    Dr = 1.0 - ph.relative_density(e[0], e_min=e[2], e_max=e[1])["Dr [-]"]
    return {"Dr": Dr, "Dr_percent": 100 * Dr}


proc("relative_density_from_unit_weights", "Relative density of a granular soil", "soil mechanics",
     "Relative density (density index) of a sand from its in-place dry unit weight and the minimum and maximum dry "
     "unit weights of the laboratory tests.",
     [_GAM("gd", "in-place dry unit weight", ("dry unit weight",)),
      _GAM("gd_min", "minimum dry unit weight", ("minimum dry unit weight",)),
      _GAM("gd_max", "maximum dry unit weight", ("maximum dry unit weight",))],
     [_o("Dr", "number", "relative density (fraction)"), _o("Dr_percent", "number", "relative density (%)")],
     relative_density, _ref_relative_density,
     lambda gd, gd_min, gd_max: (lambda Dr: {"Dr": _r(Dr), "Dr_percent": _r(100 * Dr)})((gd - gd_min) / (gd_max - gd_min)),
     lambda rng: (lambda lo, hi: {"gd_min": lo, "gd_max": hi, "gd": rng.uniform(lo, hi)})(rng.uniform(13e3, 15.5e3), rng.uniform(16.5e3, 19e3)),
     [{"gd": 16.2, "gd_min": 14.5, "gd_max": 17.8}, {"gd": 15.0, "gd_min": 13.8, "gd_max": 18.1}],
     ["relative density from unit weights", "relative density of a sand", "density index"],
     conditions=["gd_min <= gd <= gd_max"], invalid=[{"gd": 19, "gd_min": 14.5, "gd_max": 17.8}],
     method="Dr = (gd_max/gd)(gd - gd_min)/(gd_max - gd_min) = (e_max - e)/(e_max - e_min)",
     latex=r"D_r = \frac{\gamma_{d,max}}{\gamma_d}\,\frac{\gamma_d-\gamma_{d,min}}{\gamma_{d,max}-\gamma_{d,min}}")


def liquid_limit(blows, w, PL=None):
    N = _vec("blows", blows, positive=True)
    wc = _vec("water contents", w, len(N), positive=True)
    _need(len(N) >= 2, "give at least two cup tests")
    slope, icpt = _linfit([math.log10(v) for v in N], wc)
    _need(slope < 0, "the water content must fall as the number of blows rises")
    LL = icpt + slope * math.log10(25)
    out = {"LL": _r(LL), "flow_index": _r(-slope)}
    if PL is not None:
        _finite(PL=PL)
        _need(0 <= PL < LL, "the plastic limit must lie below the liquid limit")
        out["PI"] = _r(LL - PL)
    return out


def _ref_liquid_limit(blows, w, PL=None):
    import numpy as np
    b, a = np.polyfit(np.log10(np.asarray(blows, float)), np.asarray(w, float), 1)
    LL = float(np.polyval([b, a], np.log10(25.0)))
    out = {"LL": LL, "flow_index": -float(b)}
    if PL is not None:
        out["PI"] = LL - PL
    return out


def _rand_ll(rng):
    LL = rng.uniform(25, 80)
    If = rng.uniform(5, 25)
    N = sorted(rng.sample(range(12, 45), rng.randint(3, 5)))
    return {"blows": N, "w": [round(LL - If * math.log10(n / 25) + rng.uniform(-0.6, 0.6), 1) for n in N],
            "PL": round(LL * rng.uniform(0.35, 0.7), 1)}


proc("liquid_limit_flow_curve", "Liquid limit from Casagrande cup tests", "soil mechanics",
     "Liquid limit (water content at 25 blows) and flow index from Casagrande cup tests by a least-squares line of "
     "water content against log10(blows), with the plasticity index when the plastic limit is given. Water contents "
     "in percent.",
     [_VEC("blows", "number of blows of each test", ("number of blows", "blow counts"), min_length=2),
      _VEC("w", "water content of each test in %", ("water contents",), min_length=2),
      _N("PL", "plastic limit in % (optional)", ("plastic limit",), required=False)],
     [_o("LL", "number", "liquid limit (%)"), _o("flow_index", "number", "flow index (% per log cycle of blows)"),
      _o("PI", "number", "plasticity index LL - PL (%)")],
     liquid_limit, _ref_liquid_limit,
     lambda blows, w, PL=None: (lambda o: dict(o, LL=_r(sum(w) / len(w))))(liquid_limit(blows, w, PL)),
     _rand_ll,
     [{"blows": [15, 22, 30, 38], "w": [42.1, 40.3, 38.7, 37.6], "PL": 21}, {"blows": [12, 20, 28, 35], "w": [58.0, 54.6, 52.1, 50.5]}],
     ["liquid limit from cup tests", "liquid limit flow curve", "casagrande liquid limit"],
     conditions=["water content falls with blows", "water contents in %"],
     method="least squares w = a + b log10 N; LL = a + b log10 25; flow index = -b", latex=r"LL = w(N=25),\quad PI = LL - PL")


def grain_size(sizes, passing):
    d = _vec("sizes", sizes, positive=True)
    p = _vec("percent passing", passing, len(d))
    _need(len(d) >= 2, "give at least two sieves")
    pairs = sorted(zip(d, p))
    _need(all(pairs[i][0] < pairs[i + 1][0] and pairs[i][1] < pairs[i + 1][1] for i in range(len(pairs) - 1)),
          "the percent passing must rise strictly with sieve size (no repeated sizes or equal percentages)")
    _need(pairs[0][1] <= 10 and pairs[-1][1] >= 60, "the curve must span 10% to 60% passing")
    _need(pairs[0][1] >= 0 and pairs[-1][1] <= 100, "percentages must lie between 0 and 100")

    def D(P):
        for (d1, p1), (d2, p2) in zip(pairs, pairs[1:]):
            if p1 <= P <= p2:
                return 10 ** (math.log10(d1) + (P - p1) / (p2 - p1) * (math.log10(d2) - math.log10(d1)))
        raise ValueError("percentage outside the curve")
    D10, D30, D60 = D(10), D(30), D(60)
    return {"D10": _r(D10), "D30": _r(D30), "D60": _r(D60), "Cu": _r(D60 / D10), "Cc": _r(D30 * D30 / (D10 * D60))}


def _ref_grain_size(sizes, passing):
    import numpy as np
    d = np.asarray(sizes, float)
    p = np.asarray(passing, float)
    o = np.argsort(p)
    Ds = 10 ** np.interp([10, 30, 60], p[o], np.log10(d[o]))
    return {"D10": Ds[0], "D30": Ds[1], "D60": Ds[2], "Cu": Ds[2] / Ds[0], "Cc": Ds[1] ** 2 / (Ds[0] * Ds[2])}


def _rand_gs(rng):
    sieves = [75, 37.5, 19, 9.5, 4.75, 2.36, 1.18, 0.6, 0.3, 0.15, 0.075]
    k = rng.randint(6, 11)
    s = sorted(rng.sample(sieves, k), reverse=True)
    D50, spread = 10 ** rng.uniform(-0.8, 0.8), rng.uniform(0.5, 1.5)
    p = [100 / (1 + (D50 / x) ** (1 / spread * 2)) for x in s]
    p = [round(v, 1) for v in p]
    ok = all(p[i] > p[i + 1] for i in range(len(p) - 1)) and p[-1] <= 10 and p[0] >= 60
    return {"sizes": s, "passing": p} if ok else _rand_gs(rng)


proc("grain_size_coefficients", "Effective sizes and gradation coefficients from a sieve analysis", "soil mechanics",
     "D10, D30 and D60 by log-linear interpolation of the grading curve, and the coefficients of uniformity Cu and "
     "curvature Cc. Sieve sizes in mm, percent passing in %.",
     [_VEC("sizes", "sieve sizes in mm", ("sieve sizes",), min_length=2),
      _VEC("passing", "percent passing each sieve, in %", ("percent passing",), min_length=2)],
     [_o("D10", "number", "D10 (mm)"), _o("D30", "number", "D30 (mm)"), _o("D60", "number", "D60 (mm)"),
      _o("Cu", "number", "coefficient of uniformity D60/D10"), _o("Cc", "number", "coefficient of curvature D30^2/(D10 D60)")],
     grain_size, _ref_grain_size,
     lambda sizes, passing: (lambda o: dict(o, Cc=_r(o["D30"] / (o["D10"] * o["D60"]))))(grain_size(sizes, passing)),
     _rand_gs,
     [{"sizes": [4.75, 2.0, 0.85, 0.425, 0.25, 0.15, 0.075], "passing": [95, 82, 64, 43, 27, 14, 6]},
      {"sizes": [19, 9.5, 4.75, 2.36, 1.18, 0.6, 0.3, 0.15], "passing": [100, 88, 70, 52, 36, 22, 11, 4]}],
     ["grain size coefficients", "sieve analysis coefficients", "coefficient of uniformity", "d10 d30 d60"],
     conditions=["percent passing rises strictly with size", "the curve spans 10% to 60%"],
     method="log-linear interpolation of size against percent passing", latex=r"C_u = \frac{D_{60}}{D_{10}},\quad C_c = \frac{D_{30}^2}{D_{10}D_{60}}")


def effective_stress(H, gam, zw, z):
    H = _vec("thicknesses", H, positive=True)
    g = [x * 1e3 for x in _vec("unit weights", gam, len(H), positive=True)]
    _finite(zw=zw, z=z)
    _need(zw >= 0, "the water table depth must be zero or positive")
    _need(0 < z <= sum(H) + 1e-12, "the depth must lie within the layers")
    sig, top = 0.0, 0.0
    for h, gi in zip(H, g):
        if z <= top:
            break
        sig += gi * (min(z, top + h) - top)
        top += h
    u = GAMMA_W * max(0.0, z - zw)
    return {"sigma": _r(sig), "u": _r(u), "sigma_eff": _r(sig - u), "sigma_eff_kPa": _r((sig - u) / 1e3)}


def _ref_eff_stress(H, gam, zw, z):
    import numpy as np
    edges = np.concatenate([[0.0], np.cumsum(H)])
    gamma = lambda s: gam[min(int(np.searchsorted(edges, s, side="right")) - 1, len(H) - 1)] * 1e3   # noqa: E731
    pts = [e for e in edges[1:-1] if 0 < e < z]
    from scipy.integrate import quad
    sig = quad(gamma, 0.0, z, points=pts or None, epsabs=0, epsrel=1e-13, limit=200)[0]
    u = quad(lambda s: GAMMA_W if s > zw else 0.0, 0.0, z, points=[zw] if 0 < zw < z else None, epsabs=0, epsrel=1e-13)[0]
    return {"sigma": sig, "u": u, "sigma_eff": sig - u, "sigma_eff_kPa": (sig - u) / 1e3}


proc("effective_stress_at_depth", "Total, pore-water and effective vertical stress at a depth", "soil mechanics",
     "Total vertical stress, hydrostatic pore-water pressure and effective vertical stress at depth z in a layered "
     "soil with a water table at depth zw (no capillary rise, hydrostatic water). Each layer's unit weight is the one "
     "that applies there (moist above the water table, saturated below), in kN/m^3.",
     [_VEC("H", "layer thicknesses in m, from the surface down", ("thicknesses", "layer thicknesses")),
      _VEC("gam", "unit weight of each layer in kN/m^3", ("unit weights",)),
      _L("zw", "depth of the water table", ("water table depth",)), _L("z", "depth of the point", ("depth",))],
     [_o("sigma", "stress", "total vertical stress (Pa)"), _o("u", "stress", "pore-water pressure (Pa)"),
      _o("sigma_eff", "stress", "effective vertical stress (Pa)"), _o("sigma_eff_kPa", "number", "effective vertical stress (kPa)")],
     effective_stress, _ref_eff_stress,
     lambda H, gam, zw, z: (lambda o: dict(o, sigma_eff=_r(o["sigma"] - GAMMA_W * z), sigma_eff_kPa=_r((o["sigma"] - GAMMA_W * z) / 1e3)))(
         effective_stress(H, gam, zw, z)),
     lambda rng: (lambda H: {"H": H, "gam": [round(rng.uniform(16, 21), 1) for _ in H], "zw": round(rng.uniform(0.5, sum(H) * 0.6), 2),
                             "z": round(rng.uniform(sum(H) * 0.65, sum(H)), 2)})([round(rng.uniform(1, 6), 1) for _ in range(rng.randint(1, 4))]),
     [{"H": [3, 5], "gam": [17.5, 19.5], "zw": 3, "z": 8}, {"H": [2, 4, 6], "gam": [16.8, 18.9, 20.1], "zw": 1.5, "z": 9}],
     ["effective stress at depth", "effective vertical stress", "vertical stress profile"],
     conditions=["hydrostatic pore water", "no capillary rise", "0 < z <= total thickness"], invalid=[{"H": [3, 5], "gam": [17.5, 19.5], "zw": 3, "z": 12}],
     method="sigma = sum gamma_i h_i above z, u = gamma_w (z - zw) below the water table, sigma' = sigma - u",
     latex=r"\sigma'_v = \sum \gamma_i h_i - \gamma_w (z - z_w)")


# --- consolidation -------------------------------------------------------------------------------------------------
def consolidation_nc(Cc, e0, H, s0, ds):
    _pos(Cc=Cc, e0=e0, H=H, s0=s0, ds=ds)
    de = Cc * math.log10((s0 + ds) / s0)
    return {"Sc": _r(H * de / (1 + e0)), "de": _r(de), "e_final": _r(e0 - de)}


def _ref_cons(Cc, e0, H, s0, ds, Cs=None, sp=None):
    # compressibility de/dsigma' = -C/(ln10 sigma') integrated along the loading path (Cs below sigma_p, Cc above)
    if Cs is None:
        de = _quad(lambda s: Cc / (math.log(10) * s), s0, s0 + ds)
    else:
        f = lambda s: (Cs if s < sp else Cc) / (math.log(10) * s)        # noqa: E731
        de = _quad(f, s0, min(s0 + ds, sp)) + (_quad(f, sp, s0 + ds) if s0 + ds > sp else 0.0)
    return {"Sc": H * de / (1 + e0), "de": de, "e_final": e0 - de}


proc("consolidation_settlement_nc", "Primary consolidation settlement of a normally consolidated clay", "soil mechanics",
     "Primary consolidation settlement of a normally consolidated clay layer of thickness H from the compression index "
     "Cc, the initial void ratio and the initial and added effective stresses at mid-depth.",
     [_N("Cc", "compression index", ("compression index",)), _N("e0", "initial void ratio", ("initial void ratio", "void ratio")),
      _L("H", "clay layer thickness", ("thickness", "layer thickness")),
      _STRESS("s0", "initial effective vertical stress at mid-depth", ("initial effective stress",)),
      _STRESS("ds", "added vertical stress at mid-depth", ("stress increase", "added stress"))],
     [_o("Sc", "length", "consolidation settlement (m)"), _o("de", "number", "change in void ratio"),
      _o("e_final", "number", "final void ratio")],
     consolidation_nc, lambda Cc, e0, H, s0, ds: _ref_cons(Cc, e0, H, s0, ds),
     lambda Cc, e0, H, s0, ds: (lambda de: {"Sc": _r(H * de / e0), "de": _r(de), "e_final": _r(e0 - de)})(Cc * math.log10((s0 + ds) / s0)),
     lambda rng: {"Cc": rng.uniform(0.1, 0.8), "e0": rng.uniform(0.6, 2.0), "H": rng.uniform(1, 12), "s0": rng.uniform(20e3, 250e3),
                  "ds": rng.uniform(10e3, 200e3)},
     [{"Cc": 0.32, "e0": 0.92, "H": 4, "s0": 80, "ds": 60}, {"Cc": 0.5, "e0": 1.4, "H": 6, "s0": 55, "ds": 100}],
     ["consolidation settlement of a normally consolidated clay", "primary consolidation settlement",
      "consolidation settlement normally consolidated"],
     conditions=["normally consolidated clay", "stresses at mid-depth of the layer"],
     method="Sc = Cc H/(1 + e0) log10((s0 + ds)/s0)", latex=r"S_c = \frac{C_c H}{1+e_0}\log_{10}\frac{\sigma'_0+\Delta\sigma}{\sigma'_0}")


def consolidation_oc(Cc, Cs, e0, H, s0, ds, sp):
    _pos(Cc=Cc, Cs=Cs, e0=e0, H=H, s0=s0, ds=ds, sp=sp)
    _need(Cs < Cc, "the swelling index must be smaller than the compression index")
    _need(sp >= s0, "the preconsolidation stress cannot be below the present effective stress")
    s1 = s0 + ds
    if s1 <= sp:
        de, case = Cs * math.log10(s1 / s0), "stays overconsolidated"
    else:
        de, case = Cs * math.log10(sp / s0) + Cc * math.log10(s1 / sp), "passes the preconsolidation stress"
    return {"Sc": _r(H * de / (1 + e0)), "de": _r(de), "OCR": _r(sp / s0), "case": case}


def _ref_cons_oc(Cc, Cs, e0, H, s0, ds, sp):
    r = _ref_cons(Cc, e0, H, s0, ds, Cs, sp)
    return {"Sc": r["Sc"], "de": r["de"], "OCR": sp / s0,
            "case": "stays overconsolidated" if s0 + ds <= sp else "passes the preconsolidation stress"}


def _rand_oc(rng):
    s0 = rng.uniform(30e3, 200e3)
    sp = s0 * rng.uniform(1.05, 4)
    ds = (sp - s0) * rng.choice([rng.uniform(0.2, 0.9), rng.uniform(1.2, 4)])
    return {"Cc": rng.uniform(0.2, 0.8), "Cs": rng.uniform(0.02, 0.12), "e0": rng.uniform(0.6, 1.8), "H": rng.uniform(1, 10),
            "s0": s0, "ds": ds, "sp": sp}


proc("consolidation_settlement_oc", "Primary consolidation settlement of an overconsolidated clay", "soil mechanics",
     "Primary consolidation settlement of an overconsolidated clay: recompression with the swelling index Cs up to the "
     "preconsolidation stress, then virgin compression with Cc.",
     [_N("Cc", "compression index", ("compression index",)), _N("Cs", "swelling (recompression) index", ("swelling index", "recompression index")),
      _N("e0", "initial void ratio", ("initial void ratio", "void ratio")), _L("H", "clay layer thickness", ("thickness", "layer thickness")),
      _STRESS("s0", "initial effective vertical stress at mid-depth", ("initial effective stress",)),
      _STRESS("ds", "added vertical stress at mid-depth", ("stress increase", "added stress")),
      _STRESS("sp", "preconsolidation stress", ("preconsolidation stress",))],
     [_o("Sc", "length", "consolidation settlement (m)"), _o("de", "number", "change in void ratio"),
      _o("OCR", "number", "overconsolidation ratio sp/s0"), _o("case", "number", "whether the load passes the preconsolidation stress")],
     consolidation_oc, _ref_cons_oc,
     lambda Cc, Cs, e0, H, s0, ds, sp: (lambda de: {"Sc": _r(H * de / (1 + e0)), "de": _r(de), "OCR": _r(sp / s0),
                                                    "case": "passes the preconsolidation stress"})(Cc * math.log10((s0 + ds) / s0)),
     _rand_oc,
     [{"Cc": 0.4, "Cs": 0.06, "e0": 1.1, "H": 5, "s0": 90, "ds": 150, "sp": 160}, {"Cc": 0.35, "Cs": 0.05, "e0": 0.95, "H": 3, "s0": 70, "ds": 40, "sp": 130}],
     ["consolidation settlement of an overconsolidated clay", "overconsolidated clay settlement", "consolidation settlement overconsolidated"],
     conditions=["Cs < Cc", "sp >= s0", "stresses at mid-depth"], invalid=[{"Cc": 0.4, "Cs": 0.06, "e0": 1.1, "H": 5, "s0": 200, "ds": 150, "sp": 160}],
     method="sigma1 <= sp: Sc = Cs H/(1+e0) log(s1/s0); otherwise Sc = H/(1+e0)[Cs log(sp/s0) + Cc log(s1/sp)]",
     latex=r"S_c = \frac{H}{1+e_0}\left[C_s\log\frac{\sigma'_p}{\sigma'_0} + C_c\log\frac{\sigma'_0+\Delta\sigma}{\sigma'_p}\right]")


def thick_layer_settlement(H, s_top, gsub, ds, Cc, e0):
    _pos(H=H, s_top=s_top, gsub=gsub, ds=ds, Cc=Cc, e0=e0)
    a, b = s_top, gsub
    F = lambda u: u * math.log(u) - u              # antiderivative of ln u                       # noqa: E731
    integral = (F(a + b * H + ds) - F(a + ds) - F(a + b * H) + F(a)) / b       # integral of ln((s0+ds)/s0) dz
    S = Cc / ((1 + e0) * math.log(10)) * integral
    smid = a + b * H / 2
    S1 = Cc * H / (1 + e0) * math.log10((smid + ds) / smid)
    return {"S": _r(S), "S_one_layer": _r(S1)}


def _ref_thick(H, s_top, gsub, ds, Cc, e0):
    S = _quad(lambda z: Cc / (1 + e0) * math.log10((s_top + gsub * z + ds) / (s_top + gsub * z)), 0.0, H)
    smid = s_top + gsub * H / 2
    return {"S": S, "S_one_layer": Cc * H / (1 + e0) * math.log10((smid + ds) / smid)}


proc("thick_clay_layer_settlement", "Consolidation settlement of a thick clay layer integrated over depth", "soil mechanics",
     "Primary consolidation settlement of a thick normally consolidated clay layer under a uniform added stress, with "
     "the initial effective stress rising linearly with depth (integrated exactly over the thickness), and the usual "
     "single-layer estimate at mid-depth for comparison.",
     [_L("H", "clay layer thickness", ("thickness", "layer thickness")),
      _STRESS("s_top", "initial effective vertical stress at the top of the clay", ("top effective stress",)),
      _GAM("gsub", "submerged unit weight of the clay", ("submerged unit weight", "buoyant unit weight")),
      _STRESS("ds", "uniform added vertical stress", ("stress increase", "added stress")),
      _N("Cc", "compression index", ("compression index",)), _N("e0", "initial void ratio", ("initial void ratio", "void ratio"))],
     [_o("S", "length", "settlement integrated over depth (m)"), _o("S_one_layer", "length", "single-layer estimate at mid-depth (m)")],
     thick_layer_settlement, _ref_thick,
     lambda H, s_top, gsub, ds, Cc, e0: (lambda o: dict(o, S=o["S_one_layer"]))(thick_layer_settlement(H, s_top, gsub, ds, Cc, e0)),
     lambda rng: {"H": rng.uniform(4, 25), "s_top": rng.uniform(10e3, 120e3), "gsub": rng.uniform(6e3, 11e3), "ds": rng.uniform(20e3, 250e3),
                  "Cc": rng.uniform(0.2, 0.9), "e0": rng.uniform(0.8, 2.2)},
     [{"H": 10, "s_top": 40, "gsub": 8, "ds": 100, "Cc": 0.45, "e0": 1.2}, {"H": 16, "s_top": 25, "gsub": 7, "ds": 60, "Cc": 0.6, "e0": 1.6}],
     ["thick clay layer settlement", "settlement of a thick clay layer", "consolidation settlement integrated over depth"],
     conditions=["normally consolidated clay", "uniform added stress with depth", "s0' = s_top + gsub z"],
     method="S = Cc/(1+e0) integral_0^H log10((s0'(z) + ds)/s0'(z)) dz in closed form",
     latex=r"S = \frac{C_c}{1+e_0}\int_0^H \log_{10}\frac{\sigma'_0(z)+\Delta\sigma}{\sigma'_0(z)}\,dz")


def _U_fourier(Tv):
    s, m = 0.0, 0
    while True:
        M = math.pi * (2 * m + 1) / 2
        term = 2 / (M * M) * math.exp(-M * M * Tv)
        s += term
        if term < 1e-18:
            break
        m += 1
    return 1 - s


def _U_images(Tv):
    """Average degree of consolidation from the Laplace-transform image series (erfc terms), not the Fourier series."""
    from scipy.special import erfc
    rt = math.sqrt(Tv)
    s = 2 * rt / math.sqrt(math.pi)
    n = 1
    while True:
        term = 2 * (-1) ** n * (2 * rt / math.sqrt(math.pi) * math.exp(-n * n / Tv) - 2 * n * erfc(n / rt))
        s += term
        if abs(term) < 1e-18 and n > 3:
            break
        n += 1
        if n > 400:
            break
    return s


def degree_of_consolidation(cv, t, Hdr, Sc=None):
    _pos(cv=cv, t=t, Hdr=Hdr)
    Tv = cv * t / (Hdr * Hdr)
    _need(1e-4 <= Tv <= 5, "the time factor must lie between 1e-4 and 5")
    U = _U_fourier(Tv)
    out = {"Tv": _r(Tv), "U": _r(U), "U_percent": _r(100 * U)}
    if Sc is not None:
        _pos(Sc=Sc)
        out["St"] = _r(U * Sc)
    return out


def _ref_degree(cv, t, Hdr, Sc=None):
    import mpmath
    Tv = cv * t / (Hdr * Hdr)
    with mpmath.workdps(30):
        U = float(mpmath.invertlaplace(lambda s: mpmath.tanh(mpmath.sqrt(s)) / s ** 1.5, Tv, method="talbot"))
    out = {"Tv": Tv, "U": U, "U_percent": 100 * U}
    if Sc is not None:
        out["St"] = U * Sc
    return out


CV_IN = lambda **kw: _in("cv", "consolidation_coefficient", "transmissivity", "coefficient of consolidation",      # noqa: E731
                         ("coefficient of consolidation",), "m^2/yr", **kw)

proc("degree_of_consolidation", "Average degree of consolidation after a time (Terzaghi)", "soil mechanics",
     "Time factor and average degree of one-dimensional consolidation of a clay layer after time t (Terzaghi, uniform "
     "initial excess pore pressure), and the settlement reached if the final consolidation settlement is given. Hdr is "
     "the drainage path: half the thickness for double drainage, the full thickness for single drainage.",
     [CV_IN(), _in("t", "time", "time", "time since loading", ("time",), "yr"), _L("Hdr", "drainage path length", ("drainage path", "drainage length")),
      _L("Sc", "final consolidation settlement (optional)", ("final settlement", "ultimate settlement"), "mm", required=False)],
     [_o("Tv", "number", "time factor cv t/Hdr^2"), _o("U", "number", "average degree of consolidation"),
      _o("U_percent", "number", "average degree of consolidation (%)"), _o("St", "length", "settlement at time t (m)")],
     degree_of_consolidation, _ref_degree,
     lambda cv, t, Hdr, Sc=None: degree_of_consolidation(cv, t, 2 * Hdr, Sc) if cv * t / (4 * Hdr * Hdr) >= 1e-4 else
     {"Tv": 0.0, "U": 0.0, "U_percent": 0.0},                                  # the layer thickness used as the drainage path
     lambda rng: (lambda Hdr, cv: {"Hdr": Hdr, "cv": cv, "t": 10 ** rng.uniform(-2.5, 0.3) * Hdr * Hdr / cv, "Sc": rng.uniform(0.02, 0.8)})(
         rng.uniform(0.5, 8), rng.uniform(0.5, 20) / YEAR),
     [{"cv": 2.5, "t": 1, "Hdr": 3}, {"cv": 4, "t": 0.5, "Hdr": 2.5, "Sc": 180}],
     ["degree of consolidation", "average degree of consolidation", "consolidation after a time"],
     conditions=["1e-4 <= Tv <= 5", "uniform initial excess pore pressure", "Terzaghi 1-D theory"],
     method="Tv = cv t/Hdr^2; U = 1 - sum 2/M^2 exp(-M^2 Tv), M = pi(2m + 1)/2",
     latex=r"U = 1 - \sum_{m=0}^{\infty}\frac{2}{M^2}e^{-M^2T_v},\quad M = \frac{\pi}{2}(2m+1),\quad T_v = \frac{c_v t}{H_{dr}^2}")


def _Tv_for(U):
    return _bisect_root(lambda T: _U_fourier(T) - U, 1e-7, 20.0)


def consolidation_time(cv, Hdr, U):
    _pos(cv=cv, Hdr=Hdr)
    _finite(U=U)
    _need(0.01 <= U <= 0.99, "the degree of consolidation must lie between 1% and 99%")
    Tv = _Tv_for(U)
    t = Tv * Hdr * Hdr / cv
    return {"Tv": _r(Tv), "t": _r(t), "t_days": _r(t / 86400)}


def _ref_cons_time(cv, Hdr, U):
    Tv = _brentq(lambda T: _U_images(T) - U, 1e-8, 5.0)
    t = Tv * Hdr * Hdr / cv
    return {"Tv": Tv, "t": t, "t_days": t / 86400}


proc("consolidation_time", "Time to reach a degree of consolidation", "soil mechanics",
     "Time needed for a clay layer to reach an average degree of consolidation U (Terzaghi), with the time factor.",
     [CV_IN(), _L("Hdr", "drainage path length", ("drainage path", "drainage length")),
      _F("U", "average degree of consolidation (fraction, or % in a question)", ("degree of consolidation",), "%")],
     [_o("Tv", "number", "time factor"), _o("t", "time", "time (s)"), _o("t_days", "number", "time (days)")],
     consolidation_time, _ref_cons_time,
     lambda cv, Hdr, U: (lambda Tv: {"Tv": _r(Tv), "t": _r(Tv * Hdr * Hdr / cv), "t_days": _r(Tv * Hdr * Hdr / cv / 86400)})(math.pi / 4 * U * U),
     lambda rng: {"cv": rng.uniform(0.5, 20) / YEAR, "Hdr": rng.uniform(0.5, 8), "U": rng.uniform(0.3, 0.99)},
     [{"cv": 3, "Hdr": 2.5, "U": 90}, {"cv": 1.5, "Hdr": 4, "U": 50}], ["consolidation time", "time for consolidation",
                                                                        "time to reach a degree of consolidation"],
     conditions=["1% <= U <= 99%", "Terzaghi 1-D theory"], invalid=[{"cv": 3, "Hdr": 2.5, "U": 100}],
     method="Tv from U(Tv) = U (bisection on the Fourier series), t = Tv Hdr^2/cv", latex=r"t = \frac{T_v(U)\,H_{dr}^2}{c_v}")


def cv_from_time(Hdr, t, U):
    _pos(Hdr=Hdr, t=t)
    _finite(U=U)
    _need(0.01 <= U <= 0.99, "the degree of consolidation must lie between 1% and 99%")
    Tv = _Tv_for(U)
    cv = Tv * Hdr * Hdr / t
    return {"cv": _r(cv), "Tv": _r(Tv), "cv_m2_yr": _r(cv * YEAR)}


def _ref_cv_time(Hdr, t, U):
    Tv = _brentq(lambda T: _U_images(T) - U, 1e-8, 5.0)
    cv = Tv * Hdr * Hdr / t
    return {"cv": cv, "Tv": Tv, "cv_m2_yr": cv * YEAR}


proc("coefficient_of_consolidation", "Coefficient of consolidation from a consolidation time", "soil mechanics",
     "Coefficient of consolidation from the time at which a degree of consolidation is reached (for example t50 by "
     "Casagrande's method or t90 by Taylor's), using the exact time factor rather than the rounded 0.197 or 0.848.",
     [_L("Hdr", "drainage path length (half the specimen height for double drainage)", ("drainage path", "drainage length"), "mm"),
      _in("t", "time", "time", "time to reach U", ("time",), "min"),
      _F("U", "degree of consolidation reached (fraction, or % in a question)", ("degree of consolidation",), "%")],
     [_o("cv", "consolidation_coefficient", "coefficient of consolidation (m^2/s)"), _o("Tv", "number", "time factor for U"),
      _o("cv_m2_yr", "number", "coefficient of consolidation (m^2/yr)")],
     cv_from_time, _ref_cv_time,
     lambda Hdr, t, U: (lambda Tv: {"cv": _r(Tv * Hdr * Hdr / t), "Tv": _r(Tv), "cv_m2_yr": _r(Tv * Hdr * Hdr / t * YEAR)})(
         {0.5: 0.197, 0.9: 0.848}.get(round(U, 6), math.pi / 4 * U * U)),
     lambda rng: {"Hdr": rng.uniform(0.005, 0.02), "t": rng.uniform(1, 200) * 60, "U": rng.choice([0.5, 0.9, rng.uniform(0.2, 0.95)])},
     [{"Hdr": 10, "t": 12.5, "U": 50}, {"Hdr": 9.5, "t": 48, "U": 90}], ["coefficient of consolidation", "cv from t90", "cv from t50"],
     conditions=["1% <= U <= 99%", "Terzaghi 1-D theory"], method="Tv(U) exactly (bisection), cv = Tv Hdr^2/t",
     latex=r"c_v = \frac{T_v(U)\,H_{dr}^2}{t}")


def excess_pore_pressure(u0, cv, t, Hdr, z):
    _pos(u0=u0, cv=cv, t=t, Hdr=Hdr)
    _finite(z=z)
    _need(0 <= z <= 2 * Hdr, "the depth must lie between 0 and 2 Hdr (the drainage faces)")
    Tv = cv * t / (Hdr * Hdr)
    _need(1e-3 <= Tv <= 3, "the time factor must lie between 0.001 and 3")
    Z = z / Hdr
    s, m = 0.0, 0
    while True:
        M = math.pi * (2 * m + 1) / 2
        term = 2 / M * math.sin(M * Z) * math.exp(-M * M * Tv)
        s += term
        if 2 / M * math.exp(-M * M * Tv) < 1e-18:
            break
        m += 1
    u = _snap(u0 * s, u0)
    return {"u": _r(u), "Uz": _r(1 - u / u0), "Tv": _r(Tv)}


def _ref_pore(u0, cv, t, Hdr, z):
    from scipy.special import erfc
    L = 2 * Hdr
    a = 2 * math.sqrt(cv * t)
    s, n = 1.0, 0
    while True:                                     # images: u/u0 = 1 - sum (-1)^n [erfc((nL + z)/a) + erfc(((n+1)L - z)/a)]
        term = (-1) ** n * (erfc((n * L + z) / a) + erfc(((n + 1) * L - z) / a))
        s -= term
        if abs(term) < 1e-18 and n > 2:
            break
        n += 1
        if n > 500:
            break
    u = u0 * s
    return {"u": u, "Uz": 1 - u / u0, "Tv": cv * t / (Hdr * Hdr)}


proc("excess_pore_pressure_isochrone", "Excess pore pressure at a depth during consolidation", "soil mechanics",
     "Excess pore-water pressure and local degree of consolidation at depth z and time t in a doubly drained clay "
     "layer of thickness 2 Hdr under a uniform initial excess pressure u0 (Terzaghi); z is measured from the top "
     "drainage face (for a singly drained layer of thickness Hdr, use 0 <= z <= Hdr).",
     [_STRESS("u0", "initial excess pore pressure", ("initial excess pore pressure",)), CV_IN(),
      _in("t", "time", "time", "time since loading", ("time",), "yr"), _L("Hdr", "drainage path length", ("drainage path", "drainage length")),
      _L("z", "depth below the top drainage face", ("depth",))],
     [_o("u", "stress", "excess pore pressure (Pa)"), _o("Uz", "number", "local degree of consolidation 1 - u/u0"),
      _o("Tv", "number", "time factor")],
     excess_pore_pressure, _ref_pore,
     lambda u0, cv, t, Hdr, z: (lambda Tv: (lambda u: {"u": _r(u), "Uz": _r(1 - u / u0), "Tv": _r(Tv)})(
         u0 * 4 / math.pi * math.sin(math.pi * z / (2 * Hdr)) * math.exp(-math.pi ** 2 * Tv)))(cv * t / (Hdr * Hdr)),
     lambda rng: (lambda Hdr, cv: {"u0": rng.uniform(20e3, 200e3), "cv": cv, "t": 10 ** rng.uniform(-2.5, 0.2) * Hdr * Hdr / cv, "Hdr": Hdr,
                                   "z": rng.uniform(0.05, 1.95) * Hdr})(rng.uniform(0.5, 8), rng.uniform(0.5, 20) / YEAR),
     [{"u0": 100, "cv": 2, "t": 1, "Hdr": 3, "z": 1.5}, {"u0": 80, "cv": 5, "t": 0.25, "Hdr": 2, "z": 2}],
     ["excess pore pressure during consolidation", "excess pore pressure isochrone", "pore pressure at depth during consolidation"],
     conditions=["0.001 <= Tv <= 3", "uniform initial excess pore pressure", "double drainage (or symmetric half)"],
     method="u = sum 2u0/M sin(M z/Hdr) exp(-M^2 Tv), M = pi(2m + 1)/2",
     latex=r"u(z,t) = \sum_{m=0}^{\infty}\frac{2u_0}{M}\sin\frac{Mz}{H_{dr}}\,e^{-M^2T_v}")


# --- stresses under loads -------------------------------------------------------------------------------------------
def circle_load(q, a, z):
    _pos(q=q, a=a, z=z)
    I = 1 - (1 / (1 + (a / z) ** 2)) ** 1.5                   # noqa: E741
    return {"dsigma": _r(q * I), "I": _r(I)}


def _ref_circle(q, a, z):
    # Boussinesq point-load kernel integrated over the loaded circle (rings of radius r)
    k = lambda r: 3 * q * z ** 3 / (2 * math.pi * (r * r + z * z) ** 2.5) * 2 * math.pi * r     # noqa: E731
    ds = _quad(k, 0.0, a)
    return {"dsigma": ds, "I": ds / q}


proc("stress_below_circular_load", "Vertical stress below the centre of a uniformly loaded circle", "foundations",
     "Increase in vertical stress at depth z below the centre of a flexible circular area of radius a carrying a "
     "uniform pressure q (Boussinesq, elastic half-space).",
     [_STRESS("q", "uniform applied pressure", ("pressure", "applied pressure")), _L("a", "radius of the loaded area", ("radius",)),
      _L("z", "depth below the surface", ("depth",))],
     [_o("dsigma", "stress", "vertical stress increase (Pa)"), _o("I", "number", "influence factor dsigma/q")],
     circle_load, _ref_circle,
     lambda q, a, z: (lambda I: {"dsigma": _r(q * I), "I": _r(I)})(1 - (1 / (1 + (a / z) ** 2)) ** 0.5),
     lambda rng: {"q": rng.uniform(20e3, 400e3), "a": rng.uniform(0.5, 10), "z": rng.uniform(0.2, 30)},
     [{"q": 150, "a": 2, "z": 3}, {"q": 100, "a": 5, "z": 10}], ["stress below a circular load", "boussinesq circular load",
                                                                  "stress under a circular footing"],
     conditions=["elastic, homogeneous, isotropic half-space", "flexible uniform load", "below the centre"],
     method="dsigma = q [1 - (1/(1 + (a/z)^2))^(3/2)]", latex=r"\Delta\sigma_z = q\left[1-\left(\frac{1}{1+(a/z)^2}\right)^{3/2}\right]")


def _newmark_I(m, n):
    s = m * m + n * n + 1
    return (2 * m * n * math.sqrt(s) / (s + m * m * n * n) * (s + 1) / s + math.atan2(2 * m * n * math.sqrt(s), s - m * m * n * n)) / (4 * math.pi)


def rectangle_load(q, B, L, z, position="corner"):
    _pos(q=q, B=B, L=L, z=z)
    _need(position in ("corner", "centre"), "position is corner or centre")
    if position == "corner":
        I = _newmark_I(B / z, L / z)                        # noqa: E741
    else:
        I = 4 * _newmark_I(B / 2 / z, L / 2 / z)            # noqa: E741
    return {"dsigma": _r(q * I), "I": _r(I)}


def _ref_rectangle(q, B, L, z, position="corner"):
    from scipy.integrate import dblquad
    k = lambda y, x: 3 * z ** 3 / (2 * math.pi * (x * x + y * y + z * z) ** 2.5)     # noqa: E731
    if position == "corner":
        I = dblquad(k, 0.0, B, 0.0, L, epsabs=0.0, epsrel=1e-12)[0]                # noqa: E741
    else:
        I = dblquad(k, -B / 2, B / 2, -L / 2, L / 2, epsabs=0.0, epsrel=1e-12)[0]  # noqa: E741
    return {"dsigma": q * I, "I": I}


proc("stress_below_rectangular_load", "Vertical stress below a uniformly loaded rectangle (Newmark)", "foundations",
     "Increase in vertical stress at depth z below the corner (or the centre, by superposition of four corners) of a "
     "flexible rectangular area B x L under a uniform pressure q (Boussinesq integrated by Newmark).",
     [_STRESS("q", "uniform applied pressure", ("pressure", "applied pressure")), _L("B", "width of the loaded area", ("width",)),
      _L("L", "length of the loaded area", ("length",)), _L("z", "depth below the surface", ("depth",)),
      _in("position", "number", "number", "point below the 'corner' (default) or the 'centre'", ("position", "point"), "",
          required=False, typ="choice", choices=["corner", "centre"])],
     [_o("dsigma", "stress", "vertical stress increase (Pa)"), _o("I", "number", "influence factor dsigma/q")],
     rectangle_load, _ref_rectangle,
     lambda q, B, L, z, position="corner": (lambda m, n: (lambda I: {"dsigma": _r(q * I), "I": _r(I)})(
         (2 * m * n * math.sqrt(m * m + n * n + 1) / (m * m + n * n + 1 + m * m * n * n) + math.atan(2 * m * n * math.sqrt(m * m + n * n + 1) / (m * m + n * n + 1 + m * m * n * n))) / (4 * math.pi)))(
         (B if position == "corner" else B / 2) / z, (L if position == "corner" else L / 2) / z),
     lambda rng: {"q": rng.uniform(20e3, 400e3), "B": rng.uniform(0.5, 10), "L": rng.uniform(0.5, 20), "z": rng.uniform(0.3, 20),
                  "position": rng.choice(["corner", "centre"])},
     [{"q": 200, "B": 2, "L": 3, "z": 4}, {"q": 120, "B": 4, "L": 6, "z": 3, "position": "centre"}],
     ["stress below a rectangular load", "newmark rectangular load", "stress under a rectangular footing"],
     conditions=["elastic, homogeneous, isotropic half-space", "flexible uniform load"],
     method="I = (1/4 pi)[2mn sqrt(s)/(s + m^2 n^2) (s + 1)/s + atan2(2mn sqrt(s), s - m^2 n^2)], s = m^2 + n^2 + 1, m = B/z, n = L/z",
     latex=r"\Delta\sigma_z = q\,I(m,n),\quad m = \frac{B}{z},\ n = \frac{L}{z}")


def strip_load(q, B, z, x=0.0):
    _pos(q=q, B=B, z=z)
    _finite(x=x)
    b = B / 2
    g = lambda u: math.atan(u / z) + u * z / (u * u + z * z)          # noqa: E731
    I = (g(x + b) - g(x - b)) / math.pi                               # noqa: E741
    return {"dsigma": _r(q * I), "I": _r(I)}


def _ref_strip(q, B, z, x=0.0):
    # Flamant's line-load kernel integrated across the strip
    k = lambda s: 2 * z ** 3 / (math.pi * ((x - s) ** 2 + z * z) ** 2)    # noqa: E731
    I = _quad(k, -B / 2, B / 2)                                         # noqa: E741
    return {"dsigma": q * I, "I": I}


proc("stress_below_strip_load", "Vertical stress below a uniformly loaded strip", "foundations",
     "Increase in vertical stress at depth z and horizontal offset x from the centreline of an infinitely long strip "
     "of width B under a uniform pressure q (elastic half-space).",
     [_STRESS("q", "uniform applied pressure", ("pressure", "applied pressure")), _L("B", "strip width", ("width",)),
      _L("z", "depth below the surface", ("depth",)),
      _L("x", "horizontal offset from the strip centreline (default 0)", ("offset", "horizontal offset"), required=False)],
     [_o("dsigma", "stress", "vertical stress increase (Pa)"), _o("I", "number", "influence factor dsigma/q")],
     strip_load, _ref_strip,
     lambda q, B, z, x=0.0: (lambda g: (lambda I: {"dsigma": _r(q * I), "I": _r(I)})((g(x + B / 2) - g(x - B / 2)) / math.pi))(
         lambda u: math.atan(u / z)),
     lambda rng: {"q": rng.uniform(20e3, 400e3), "B": rng.uniform(0.5, 8), "z": rng.uniform(0.3, 20), "x": rng.uniform(-6, 6)},
     [{"q": 100, "B": 2, "z": 3}, {"q": 150, "B": 3, "z": 2, "x": 2.5}], ["stress below a strip load", "strip footing stress",
                                                                          "stress under a strip load"],
     conditions=["elastic, homogeneous, isotropic half-space", "flexible uniform load, plane strain"],
     method="dsigma = (q/pi)[atan(u/z) + u z/(u^2 + z^2)] from u = x - B/2 to x + B/2",
     latex=r"\Delta\sigma_z = \frac{q}{\pi}\left[\alpha + \sin\alpha\cos(\alpha+2\delta)\right]")


# --- bearing capacity -------------------------------------------------------------------------------------------------
def bearing_strip(c, phi, gam, B, Df, FS=3.0):
    _finite(c=c, phi=phi)
    _pos(gam=gam, B=B, FS=FS)
    _finite(Df=Df)
    _need(c >= 0 and Df >= 0, "c and Df must be zero or positive")
    _need(20 <= phi <= 45, "the friction angle must lie between 20 and 45 degrees")
    p = math.radians(phi)
    Nq = math.exp(math.pi * math.tan(p)) * math.tan(math.pi / 4 + p / 2) ** 2
    Nc = (Nq - 1) / math.tan(p)
    Ng = 2 * (Nq + 1) * math.tan(p)
    qu = c * Nc + gam * Df * Nq + 0.5 * gam * B * Ng
    return {"qu": _r(qu), "qa": _r(qu / FS), "Nc": _r(Nc), "Nq": _r(Nq), "Ngamma": _r(Ng)}


def _ref_bearing(c, phi, gam, B, Df, FS=3.0):
    import warnings
    with warnings.catch_warnings():
        warnings.simplefilter("ignore")
        from groundhog.shallowfoundations import capacity as cap
    Nq = float(cap.nq_frictionangle_sand(phi)["Nq [-]"])
    Ng = float(cap.ngamma_frictionangle_vesic(phi)["Ngamma [-]"])
    Nc = (Nq - 1) / math.tan(math.radians(phi))
    qu = c * Nc + gam * Df * Nq + 0.5 * gam * B * Ng
    return {"qu": qu, "qa": qu / FS, "Nc": Nc, "Nq": Nq, "Ngamma": Ng}


proc("strip_footing_bearing_capacity", "Ultimate bearing capacity of a strip footing", "foundations",
     "Ultimate and allowable bearing capacity of a strip footing from the general bearing capacity equation with "
     "Prandtl-Reissner factors Nc, Nq and Vesic's (1973) Ngamma (no shape, depth or inclination factors; drained "
     "parameters, water table well below the footing).",
     [_STRESS("c", "cohesion", ("cohesion",)), _DEG("phi", "friction angle (degrees)", ("friction angle",)),
      _GAM("gam", "unit weight of the soil", ("unit weight",)), _L("B", "footing width", ("width", "footing width")),
      _L("Df", "embedment depth", ("depth", "embedment depth", "founding depth")),
      _N("FS", "factor of safety for the allowable pressure (default 3)", ("factor of safety",), required=False)],
     [_o("qu", "stress", "ultimate bearing capacity (Pa)"), _o("qa", "stress", "allowable bearing pressure qu/FS (Pa)"),
      _o("Nc", "number", "bearing capacity factor Nc"), _o("Nq", "number", "bearing capacity factor Nq"),
      _o("Ngamma", "number", "bearing capacity factor Ngamma (Vesic)")],
     bearing_strip, _ref_bearing,
     lambda c, phi, gam, B, Df, FS=3.0: (lambda o: (lambda Ng: dict(o, Ngamma=_r(Ng), qu=_r(c * o["Nc"] + gam * Df * o["Nq"] + 0.5 * gam * B * Ng)))(
         (o["Nq"] - 1) * math.tan(1.4 * math.radians(phi))))(bearing_strip(c, phi, gam, B, Df, FS)),
     lambda rng: {"c": rng.choice([0.0, rng.uniform(1e3, 40e3)]), "phi": rng.uniform(20, 45), "gam": rng.uniform(15e3, 21e3),
                  "B": rng.uniform(0.5, 4), "Df": rng.uniform(0, 3)},
     [{"c": 10, "phi": 30, "gam": 18, "B": 1.5, "Df": 1.0}, {"c": 0, "phi": 35, "gam": 19, "B": 2, "Df": 1.5, "FS": 2.5}],
     ["strip footing bearing capacity", "bearing capacity of a strip footing", "ultimate bearing capacity"],
     conditions=["20 <= phi <= 45 degrees", "general shear failure", "strip footing, vertical central load"],
     invalid=[{"c": 10, "phi": 60, "gam": 18, "B": 1.5, "Df": 1.0}],
     method="Nq = e^(pi tan phi) tan^2(45 + phi/2), Nc = (Nq - 1) cot phi, Ngamma = 2(Nq + 1) tan phi; qu = c Nc + gam Df Nq + B gam Ngamma/2",
     latex=r"q_u = cN_c + \gamma D_f N_q + \tfrac12\gamma B N_\gamma")


# --- earth pressure -------------------------------------------------------------------------------------------------
def _mohr_sigma3(s1, c, phi):
    """Minor principal stress at failure for a given major one: the Mohr circle tangent to tau = c + sigma tan(phi)."""
    t = math.tan(phi)
    f = lambda s3: (c + (s1 + s3) / 2 * t) * math.cos(phi) - (s1 - s3) / 2      # noqa: E731
    lo = -2 * c / t - abs(s1) - 1.0
    return _brentq(f, lo, s1)


def _mohr_sigma1(s3, c, phi):
    t = math.tan(phi)
    f = lambda s1: (s1 - s3) / 2 - (c + (s1 + s3) / 2 * t) * math.cos(phi)      # noqa: E731
    hi = s3 + 10 * (abs(s3) + c + 1.0) / max(1e-9, (1 - math.sin(phi)))
    return _brentq(f, s3, hi)


def rankine_active(phi, gam, H, c=0.0, q=0.0):
    _finite(phi=phi, c=c, q=q)
    _pos(gam=gam, H=H)
    _need(10 <= phi <= 50, "the friction angle must lie between 10 and 50 degrees")
    _need(c >= 0 and q >= 0, "c and q must be zero or positive")
    p = math.radians(phi)
    return _rankine_active_K(phi, gam, H, c, q, (1 - math.sin(p)) / (1 + math.sin(p)))


def _rankine_active_K(phi, gam, H, c, q, Ka):
    sk = math.sqrt(Ka)
    sig = lambda z: Ka * (gam * z + q) - 2 * c * sk                    # noqa: E731
    z0 = min(H, max(0.0, (2 * c / sk - q) / gam))     # a crack deeper than the wall stops at its base
    if z0 >= H:
        Pa, ybar = 0.0, 0.0
    else:
        s0, sH = sig(z0), sig(H)
        h = H - z0
        Pa = (s0 + sH) / 2 * h
        ybar = h * (2 * s0 + sH) / (3 * (s0 + sH)) if Pa > 0 else 0.0     # height of the resultant above the base
    return {"Ka": _r(Ka), "sigma_base": _r(sig(H)), "Pa": _r(Pa), "z_crack": _r(z0), "ybar": _r(ybar)}


def _ref_rankine_active(phi, gam, H, c=0.0, q=0.0):
    p = math.radians(phi)
    Ka = _mohr_sigma3(1.0, 0.0, p)
    sig = lambda z: _mohr_sigma3(gam * z + q, c, p)                  # active: the horizontal stress is the minor one   # noqa: E731
    z0 = 0.0 if sig(0.0) >= 0 else (_brentq(sig, 0.0, H) if sig(H) > 0 else H)
    if z0 >= H:
        return {"Ka": Ka, "sigma_base": sig(H), "Pa": 0.0, "z_crack": z0, "ybar": 0.0}
    Pa = _quad(sig, z0, H)
    M = _quad(lambda z: sig(z) * (H - z), z0, H)
    return {"Ka": Ka, "sigma_base": sig(H), "Pa": Pa, "z_crack": z0, "ybar": M / Pa}


proc("rankine_active_thrust", "Rankine active earth pressure and thrust on a wall", "foundations",
     "Rankine active pressure coefficient, active pressure at the base, total active thrust per metre of a smooth "
     "vertical wall retaining a horizontal backfill (with optional cohesion and surcharge; the tension zone is ignored), "
     "the tension-crack depth and the height of the resultant above the base. Dry backfill.",
     [_DEG("phi", "friction angle of the backfill (degrees)", ("friction angle",)), _GAM("gam", "unit weight of the backfill", ("unit weight",)),
      _L("H", "wall height", ("height", "wall height")), _STRESS("c", "cohesion (default 0)", ("cohesion",), required=False),
      _STRESS("q", "uniform surcharge (default 0)", ("surcharge",), required=False)],
     [_o("Ka", "number", "active earth pressure coefficient"), _o("sigma_base", "stress", "active pressure at the base (Pa)"),
      _o("Pa", "distributed_load", "active thrust per metre of wall (N/m)"), _o("z_crack", "length", "tension-crack depth (m)"),
      _o("ybar", "length", "height of the resultant above the base (m)")],
     rankine_active, _ref_rankine_active,
     lambda phi, gam, H, c=0.0, q=0.0: _rankine_active_K(phi, gam, H, c, q, 1 - math.sin(math.radians(phi))),   # at-rest K0 used
     lambda rng: {"phi": rng.uniform(20, 42), "gam": rng.uniform(15e3, 21e3), "H": rng.uniform(1.5, 10),
                  "c": rng.choice([0.0, rng.uniform(1e3, 15e3)]), "q": rng.choice([0.0, rng.uniform(5e3, 30e3)])},
     [{"phi": 30, "gam": 18, "H": 6}, {"phi": 25, "gam": 19, "H": 5, "c": 10, "q": 15}],
     ["rankine active thrust", "rankine active earth pressure", "active earth pressure on a wall"],
     conditions=["smooth vertical wall, horizontal backfill", "dry backfill", "tension zone ignored"],
     invalid=[{"phi": 70, "gam": 18, "H": 6}],
     method="Ka = (1 - sin phi)/(1 + sin phi); sigma_a = Ka(gam z + q) - 2c sqrt(Ka); Pa = area of the positive pressure diagram",
     latex=r"K_a = \frac{1-\sin\phi}{1+\sin\phi},\quad \sigma_a = K_a(\gamma z + q) - 2c\sqrt{K_a}")


def rankine_passive(phi, gam, H, c=0.0, q=0.0):
    _finite(phi=phi, c=c, q=q)
    _pos(gam=gam, H=H)
    _need(10 <= phi <= 50, "the friction angle must lie between 10 and 50 degrees")
    _need(c >= 0 and q >= 0, "c and q must be zero or positive")
    p = math.radians(phi)
    Kp = (1 + math.sin(p)) / (1 - math.sin(p))
    sk = math.sqrt(Kp)
    s0, sH = Kp * q + 2 * c * sk, Kp * (gam * H + q) + 2 * c * sk
    Pp = (s0 + sH) / 2 * H
    return {"Kp": _r(Kp), "sigma_base": _r(sH), "Pp": _r(Pp), "ybar": _r(H * (2 * s0 + sH) / (3 * (s0 + sH)))}


def _ref_rankine_passive(phi, gam, H, c=0.0, q=0.0):
    p = math.radians(phi)
    sig = lambda z: _mohr_sigma1(gam * z + q, c, p)                 # passive: the horizontal stress is the major one   # noqa: E731
    Pp = _quad(sig, 0.0, H)
    return {"Kp": _mohr_sigma1(1.0, 0.0, p), "sigma_base": sig(H), "Pp": Pp, "ybar": _quad(lambda z: sig(z) * (H - z), 0.0, H) / Pp}


proc("rankine_passive_thrust", "Rankine passive earth pressure and thrust", "foundations",
     "Rankine passive pressure coefficient, passive pressure at the base, total passive thrust per metre on a smooth "
     "vertical wall with a horizontal soil surface (optional cohesion and surcharge) and the height of the resultant.",
     [_DEG("phi", "friction angle of the soil (degrees)", ("friction angle",)), _GAM("gam", "unit weight of the soil", ("unit weight",)),
      _L("H", "height of the wall in contact", ("height", "wall height")), _STRESS("c", "cohesion (default 0)", ("cohesion",), required=False),
      _STRESS("q", "uniform surcharge (default 0)", ("surcharge",), required=False)],
     [_o("Kp", "number", "passive earth pressure coefficient"), _o("sigma_base", "stress", "passive pressure at the base (Pa)"),
      _o("Pp", "distributed_load", "passive thrust per metre of wall (N/m)"), _o("ybar", "length", "height of the resultant above the base (m)")],
     rankine_passive, _ref_rankine_passive,
     lambda phi, gam, H, c=0.0, q=0.0: (lambda Kp: {"Kp": _r(Kp), "sigma_base": _r(Kp * (gam * H + q) + 2 * c * math.sqrt(Kp)),
                                                    "Pp": _r(0.5 * Kp * gam * H * H + Kp * q * H + 2 * c * math.sqrt(Kp) * H), "ybar": _r(H / 3)})(
         1 + math.sin(math.radians(phi))),
     lambda rng: {"phi": rng.uniform(20, 42), "gam": rng.uniform(15e3, 21e3), "H": rng.uniform(0.5, 6),
                  "c": rng.choice([0.0, rng.uniform(1e3, 20e3)]), "q": rng.choice([0.0, rng.uniform(5e3, 30e3)])},
     [{"phi": 32, "gam": 18, "H": 2}, {"phi": 25, "gam": 19, "H": 3, "c": 12}],
     ["rankine passive thrust", "rankine passive earth pressure", "passive earth pressure"],
     conditions=["smooth vertical wall, horizontal soil surface", "dry soil"], invalid=[{"phi": 70, "gam": 18, "H": 2}],
     method="Kp = (1 + sin phi)/(1 - sin phi); sigma_p = Kp(gam z + q) + 2c sqrt(Kp); Pp = area of the pressure diagram",
     latex=r"K_p = \frac{1+\sin\phi}{1-\sin\phi},\quad \sigma_p = K_p(\gamma z+q) + 2c\sqrt{K_p}")


def _coulomb_check(phi, delta, beta):
    _finite(phi=phi, delta=delta, beta=beta)
    _need(15 <= phi <= 50, "the friction angle must lie between 15 and 50 degrees")
    _need(0 <= delta <= phi, "the wall friction angle must lie between 0 and phi")
    _need(0 <= beta < phi, "the backfill slope must lie between 0 and phi")


def coulomb_active(phi, delta, beta, gam, H):
    _coulomb_check(phi, delta, beta)
    _pos(gam=gam, H=H)
    p, d, b = math.radians(phi), math.radians(delta), math.radians(beta)
    Ka = math.cos(p) ** 2 / (math.cos(d) * (1 + math.sqrt(math.sin(p + d) * math.sin(p - b) / (math.cos(d) * math.cos(b)))) ** 2)
    Pa = 0.5 * Ka * gam * H * H
    return {"Ka": _r(Ka), "Pa": _r(Pa), "Pa_h": _r(Pa * math.cos(d)), "Pa_v": _r(Pa * math.sin(d))}


def _wedge(rho, phi, delta, beta, gam, H, active):
    import numpy as np
    xc = H / (math.tan(rho) - math.tan(beta))              # where the trial plane meets the ground surface
    W = gam * 0.5 * H * xc
    t = math.tan(phi)
    n = np.array([-math.sin(rho), math.cos(rho)])
    s = np.array([math.cos(rho), math.sin(rho)])
    if active:                                              # wedge slides down toward the wall
        A = np.array([[math.cos(delta), (n + t * s)[0]], [math.sin(delta), (n + t * s)[1]]])
    else:                                                   # wedge is pushed up the plane
        A = np.array([[math.cos(delta), (n - t * s)[0]], [-math.sin(delta), (n - t * s)[1]]])
    P, _N = np.linalg.solve(A, np.array([0.0, W]))
    return float(P)


def _ref_coulomb(phi, delta, beta, gam, H, active=True):
    from scipy.optimize import minimize_scalar
    p, d, b = math.radians(phi), math.radians(delta), math.radians(beta)
    if active:                                              # the critical plane gives the largest thrust
        lo, hi = max(b, p) + 1e-9, math.pi / 2 - 1e-9
        r = minimize_scalar(lambda x: -_wedge(x, p, d, b, gam, H, True), bounds=(lo, hi), method="bounded",
                            options={"xatol": 1e-13})
        P = -r.fun
    else:                                                   # the critical plane gives the smallest thrust
        lo, hi = b + 1e-9, math.pi / 2 - p - d - 1e-9
        r = minimize_scalar(lambda x: _wedge(x, p, d, b, gam, H, False), bounds=(lo, hi), method="bounded",
                            options={"xatol": 1e-13})
        P = r.fun
    K = 2 * P / (gam * H * H)
    key = "a" if active else "p"
    return {f"K{key}": K, f"P{key}": P, f"P{key}_h": P * math.cos(d), f"P{key}_v": P * math.sin(d)}


COUL_IN = [_DEG("phi", "friction angle of the backfill (degrees)", ("friction angle",)),
           _DEG("delta", "wall friction angle (degrees)", ("wall friction angle", "wall friction")),
           _DEG("beta", "backfill slope angle above horizontal (degrees)", ("backfill slope", "slope angle")),
           _GAM("gam", "unit weight of the backfill", ("unit weight",)), _L("H", "wall height", ("height", "wall height"))]

proc("coulomb_active_thrust", "Coulomb active thrust on a rough wall with sloping backfill", "foundations",
     "Coulomb active earth pressure coefficient and thrust per metre on a rough vertical wall (wall friction delta) "
     "retaining a cohesionless backfill sloping at beta, with the horizontal and vertical components of the thrust.",
     COUL_IN,
     [_o("Ka", "number", "Coulomb active coefficient"), _o("Pa", "distributed_load", "active thrust per metre, inclined at delta (N/m)"),
      _o("Pa_h", "distributed_load", "horizontal component (N/m)"), _o("Pa_v", "distributed_load", "vertical component (N/m)")],
     coulomb_active, lambda phi, delta, beta, gam, H: _ref_coulomb(phi, delta, beta, gam, H, True),
     lambda phi, delta, beta, gam, H: dict(coulomb_active(phi, 0.0, beta, gam, H), Pa_h=_r(coulomb_active(phi, 0.0, beta, gam, H)["Pa"])),
     lambda rng: (lambda phi: {"phi": phi, "delta": phi * rng.uniform(0.3, 0.9), "beta": phi * rng.uniform(0, 0.8),
                               "gam": rng.uniform(15e3, 21e3), "H": rng.uniform(1.5, 10)})(rng.uniform(25, 42)),
     [{"phi": 30, "delta": 20, "beta": 10, "gam": 18, "H": 6}, {"phi": 35, "delta": 23, "beta": 0, "gam": 19, "H": 4}],
     ["coulomb active thrust", "coulomb active earth pressure", "coulomb earth pressure"],
     conditions=["vertical wall back", "cohesionless dry backfill", "0 <= delta <= phi", "0 <= beta < phi"],
     invalid=[{"phi": 30, "delta": 20, "beta": 35, "gam": 18, "H": 6}],
     method="Ka = cos^2 phi/(cos delta [1 + sqrt(sin(phi + delta) sin(phi - beta)/(cos delta cos beta))]^2), Pa = Ka gam H^2/2",
     latex=r"K_a = \frac{\cos^2\phi}{\cos\delta\left[1+\sqrt{\frac{\sin(\phi+\delta)\sin(\phi-\beta)}{\cos\delta\cos\beta}}\right]^2}")


def coulomb_passive(phi, delta, beta, gam, H):
    _coulomb_check(phi, delta, beta)
    _pos(gam=gam, H=H)
    _need(phi + delta + beta < 90, "phi + delta + beta must stay below 90 degrees for a plane passive wedge")
    p, d, b = math.radians(phi), math.radians(delta), math.radians(beta)
    root = math.sqrt(math.sin(p + d) * math.sin(p + b) / (math.cos(d) * math.cos(b)))
    _need(root < 1, "no plane passive wedge exists for these angles")
    Kp = math.cos(p) ** 2 / (math.cos(d) * (1 - root) ** 2)
    Pp = 0.5 * Kp * gam * H * H
    return {"Kp": _r(Kp), "Pp": _r(Pp), "Pp_h": _r(Pp * math.cos(d)), "Pp_v": _r(Pp * math.sin(d))}


proc("coulomb_passive_thrust", "Coulomb passive thrust on a rough wall", "foundations",
     "Coulomb (plane-wedge) passive earth pressure coefficient and thrust per metre on a rough vertical wall with wall "
     "friction delta and a soil surface rising at beta (plane wedges overestimate the passive resistance when delta is "
     "large; keep delta below about phi/2 in design).",
     COUL_IN,
     [_o("Kp", "number", "Coulomb passive coefficient"), _o("Pp", "distributed_load", "passive thrust per metre, inclined at delta (N/m)"),
      _o("Pp_h", "distributed_load", "horizontal component (N/m)"), _o("Pp_v", "distributed_load", "vertical component (N/m)")],
     coulomb_passive, lambda phi, delta, beta, gam, H: _ref_coulomb(phi, delta, beta, gam, H, False),
     lambda phi, delta, beta, gam, H: dict(coulomb_passive(phi, 0.0, beta, gam, H), Pp_h=_r(coulomb_passive(phi, 0.0, beta, gam, H)["Pp"])),
     lambda rng: (lambda phi: {"phi": phi, "delta": phi * rng.uniform(0.2, 0.6), "beta": phi * rng.uniform(0, 0.4),
                               "gam": rng.uniform(15e3, 21e3), "H": rng.uniform(0.5, 5)})(rng.uniform(20, 38)),
     [{"phi": 30, "delta": 15, "beta": 0, "gam": 18, "H": 2}, {"phi": 34, "delta": 12, "beta": 5, "gam": 19, "H": 3}],
     ["coulomb passive thrust", "coulomb passive earth pressure"],
     conditions=["vertical wall back", "cohesionless dry soil", "plane wedge (phi + delta + beta < 90)"],
     invalid=[{"phi": 30, "delta": 40, "beta": 0, "gam": 18, "H": 2}],
     method="Kp = cos^2 phi/(cos delta [1 - sqrt(sin(phi + delta) sin(phi + beta)/(cos delta cos beta))]^2), Pp = Kp gam H^2/2",
     latex=r"K_p = \frac{\cos^2\phi}{\cos\delta\left[1-\sqrt{\frac{\sin(\phi+\delta)\sin(\phi+\beta)}{\cos\delta\cos\beta}}\right]^2}")


# --- shear strength ----------------------------------------------------------------------------------------------------
def mohr_coulomb_failure(s3, c, phi):
    _finite(s3=s3, c=c, phi=phi)
    _need(s3 >= 0 and c >= 0, "s3 and c must be zero or positive")
    _need(0 < phi < 60, "the friction angle must lie between 0 and 60 degrees")
    p = math.radians(phi)
    k = math.tan(math.pi / 4 + p / 2)
    s1 = s3 * k * k + 2 * c * k
    ctr, rad = (s1 + s3) / 2, (s1 - s3) / 2
    return {"s1": _r(s1), "sigma_f": _r(ctr - rad * math.sin(p)), "tau_f": _r(rad * math.cos(p)), "theta": _r(45 + phi / 2)}


def _ref_mohr(s3, c, phi):
    import numpy as np
    p = math.radians(phi)
    s1 = _mohr_sigma1(s3, c, p)
    ctr = (s1 + s3) / 2
    # the tangent point is the foot of the perpendicular from the circle's centre to the envelope line
    d = np.array([math.cos(p), math.sin(p)])                 # direction of tau = c + sigma tan(phi)
    a = np.array([0.0, c])
    foot = a + np.dot(np.array([ctr, 0.0]) - a, d) * d
    theta = 0.5 * math.degrees(math.atan2(foot[1], foot[0] - ctr))   # the point at 2 theta from sigma_1 on the circle
    return {"s1": s1, "sigma_f": float(foot[0]), "tau_f": float(foot[1]), "theta": theta}


proc("mohr_coulomb_failure_stress", "Major principal stress at failure (Mohr-Coulomb)", "soil mechanics",
     "Major principal stress at failure for a given minor principal stress (triaxial test), with the normal and shear "
     "stresses on the failure plane and its angle to the major principal plane (45 + phi/2).",
     [_STRESS("s3", "minor principal (confining) stress", ("confining stress", "minor principal stress")),
      _STRESS("c", "cohesion", ("cohesion",)), _DEG("phi", "friction angle (degrees)", ("friction angle",))],
     [_o("s1", "stress", "major principal stress at failure (Pa)"), _o("sigma_f", "stress", "normal stress on the failure plane (Pa)"),
      _o("tau_f", "stress", "shear stress on the failure plane (Pa)"), _o("theta", "number", "failure plane angle from the major principal plane (degrees)")],
     mohr_coulomb_failure, _ref_mohr,
     lambda s3, c, phi: (lambda k: dict(mohr_coulomb_failure(s3, c, phi), s1=_r(s3 * k + 2 * c * k)))(math.tan(math.radians(45 + phi / 2))),  # k not squared
     lambda rng: {"s3": rng.uniform(10e3, 500e3), "c": rng.choice([0.0, rng.uniform(2e3, 50e3)]), "phi": rng.uniform(15, 42)},
     [{"s3": 100, "c": 15, "phi": 28}, {"s3": 200, "c": 0, "phi": 35}], ["mohr coulomb failure stress", "major principal stress at failure",
                                                                         "triaxial failure stress"],
     conditions=["0 < phi < 60 degrees", "s3, c >= 0"], invalid=[{"s3": 100, "c": 15, "phi": 75}],
     method="s1 = s3 tan^2(45 + phi/2) + 2c tan(45 + phi/2); tangent point at sigma = p - r sin phi, tau = r cos phi",
     latex=r"\sigma_1 = \sigma_3\tan^2\!\left(45^\circ+\frac{\phi}{2}\right) + 2c\tan\!\left(45^\circ+\frac{\phi}{2}\right)")


def triaxial_params(s3, s1):
    a3 = _vec("s3", s3, nonneg=True)
    a1 = _vec("s1", s1, len(a3))
    _need(len(a3) >= 2, "give at least two tests")
    _need(all(x > y for x, y in zip(a1, a3)), "each s1 must exceed its s3")
    pp = [(x + y) / 2 for x, y in zip(a1, a3)]
    qq = [(x - y) / 2 for x, y in zip(a1, a3)]
    slope, icpt = _linfit(pp, qq)
    _need(0 < slope < 1, "the stress points do not define a friction angle between 0 and 90 degrees")
    phi = math.asin(slope)
    c = icpt / math.cos(phi)
    _need(c >= -1e-9 * max(qq), "the fitted cohesion is negative: the data do not fit Mohr-Coulomb with c >= 0")
    return {"phi": _r(math.degrees(phi)), "c": _r(max(c, 0.0) if abs(c) < 1e-9 * max(qq) else c), "alpha": _r(math.degrees(math.atan(slope)))}


def _ref_triaxial(s3, s1):
    import numpy as np
    s3, s1 = np.asarray(s3, float), np.asarray(s1, float)
    slope, icpt = np.polyfit((s1 + s3) / 2, (s1 - s3) / 2, 1)
    phi = math.asin(slope)
    return {"phi": math.degrees(phi), "c": icpt / math.cos(phi), "alpha": math.degrees(math.atan(slope))}


def _rand_triax(rng):
    phi, c = math.radians(rng.uniform(18, 40)), rng.uniform(0, 40)
    k = math.tan(math.pi / 4 + phi / 2)
    s3 = sorted(rng.uniform(25, 400) for _ in range(rng.randint(2, 5)))
    return {"s3": [round(v, 1) for v in s3], "s1": [round(v * k * k + 2 * c * k + rng.uniform(-3, 3), 1) for v in s3]}


proc("triaxial_strength_parameters", "Cohesion and friction angle from triaxial tests", "soil mechanics",
     "Mohr-Coulomb cohesion and friction angle from two or more triaxial tests at failure, by a least-squares line "
     "through the stress points (p, q) = ((s1 + s3)/2, (s1 - s3)/2): sin phi = slope, c = intercept/cos phi. Stresses "
     "in kPa (effective or total, consistently).",
     [_VEC("s3", "confining (minor principal) stresses at failure in kPa", ("confining stresses",), min_length=2),
      _VEC("s1", "major principal stresses at failure in kPa", ("major principal stresses",), min_length=2)],
     [_o("phi", "number", "friction angle (degrees)"), _o("c", "number", "cohesion (kPa)"),
      _o("alpha", "number", "slope angle of the Kf line (degrees)")],
     triaxial_params, _ref_triaxial,
     lambda s3, s1: (lambda o: dict(o, phi=_r(o["alpha"])))(triaxial_params(s3, s1)),
     _rand_triax,
     [{"s3": [100, 200, 300], "s1": [330, 590, 860]}, {"s3": [50, 150], "s1": [210, 480]}],
     ["triaxial strength parameters", "cohesion and friction angle from triaxial tests", "mohr coulomb parameters from triaxial"],
     conditions=["at least two tests", "s1 > s3", "a non-negative intercept"],
     method="least squares q = a + p tan(alpha); phi = asin(tan alpha), c = a/cos phi",
     latex=r"\sin\phi = \tan\alpha,\quad c = \frac{a}{\cos\phi}")


def direct_shear_params(sigma, tau):
    s = _vec("normal stresses", sigma, nonneg=True)
    t = _vec("shear stresses", tau, len(s), positive=True)
    _need(len(s) >= 2, "give at least two tests")
    slope, icpt = _linfit(s, t)
    _need(slope > 0, "the shear strength must rise with normal stress")
    return {"phi": _r(math.degrees(math.atan(slope))), "c": _r(_snap(icpt, max(t))), "tan_phi": _r(slope)}


def _ref_direct_shear(sigma, tau):
    import numpy as np
    slope, icpt = np.polyfit(np.asarray(sigma, float), np.asarray(tau, float), 1)
    return {"phi": math.degrees(math.atan(slope)), "c": float(icpt), "tan_phi": float(slope)}


def _rand_ds(rng):
    phi, c = math.radians(rng.uniform(18, 40)), rng.uniform(0, 30)
    s = sorted(rng.uniform(25, 400) for _ in range(rng.randint(2, 5)))
    return {"sigma": [round(v, 1) for v in s], "tau": [round(c + v * math.tan(phi) + rng.uniform(-2, 2), 1) for v in s]}


proc("direct_shear_strength_parameters", "Cohesion and friction angle from direct shear tests", "soil mechanics",
     "Mohr-Coulomb cohesion and friction angle from direct shear tests by a least-squares line of peak shear stress "
     "against normal stress. Stresses in kPa.",
     [_VEC("sigma", "normal stresses in kPa", ("normal stresses",), min_length=2),
      _VEC("tau", "shear stresses at failure in kPa", ("shear stresses",), min_length=2)],
     [_o("phi", "number", "friction angle (degrees)"), _o("c", "number", "cohesion intercept (kPa)"), _o("tan_phi", "number", "slope tan(phi)")],
     direct_shear_params, _ref_direct_shear,
     lambda sigma, tau: (lambda o: dict(o, phi=_r(math.degrees(o["tan_phi"]))))(direct_shear_params(sigma, tau)),
     _rand_ds,
     [{"sigma": [50, 100, 200], "tau": [45, 73, 131]}, {"sigma": [100, 200, 300, 400], "tau": [68, 132, 195, 262]}],
     ["direct shear strength parameters", "cohesion and friction angle from direct shear", "direct shear test"],
     conditions=["at least two tests", "shear strength rises with normal stress"],
     method="least squares tau = c + sigma tan phi", latex=r"\tau_f = c + \sigma\tan\phi")


def proctor(w, rho_d, Gs=None):
    x = _vec("water contents", w, positive=True)
    y = _vec("dry densities", rho_d, len(x), positive=True)
    _need(len(x) >= 3, "give at least three compaction points")
    S = [[sum(v ** (i + j) for v in x) for j in range(3)] for i in range(3)]
    r = [sum(yy * v ** i for v, yy in zip(x, y)) for i in range(3)]
    c0, c1, c2 = _solve_linear(S, r)
    _need(c2 < 0, "the points do not form a peak (the fitted parabola opens upward)")
    wopt = -c1 / (2 * c2)
    _need(min(x) <= wopt <= max(x), "the fitted optimum lies outside the tested water contents")
    out = {"w_opt": _r(wopt), "rho_dmax": _r(c0 + c1 * wopt + c2 * wopt * wopt)}
    if Gs is not None:
        _finite(Gs=Gs)
        _need(2.0 <= Gs <= 3.0, "the specific gravity must lie between 2.0 and 3.0")
        out["rho_zav"] = _r(Gs * 1.0 / (1 + wopt / 100 * Gs))
    return out


def _ref_proctor(w, rho_d, Gs=None):
    import numpy as np
    c2, c1, c0 = np.polyfit(np.asarray(w, float), np.asarray(rho_d, float), 2)
    wopt = -c1 / (2 * c2)
    out = {"w_opt": float(wopt), "rho_dmax": float(np.polyval([c2, c1, c0], wopt))}
    if Gs is not None:
        out["rho_zav"] = Gs / (1 + wopt / 100 * Gs)
    return out


def _rand_proctor(rng):
    wo, rm = rng.uniform(8, 22), rng.uniform(1.6, 2.1)
    k = rng.uniform(0.002, 0.006)
    ws = sorted(round(wo + d, 1) for d in rng.sample([-6, -4, -2, 0, 2, 4, 6], rng.randint(4, 6)))
    return {"w": ws, "rho_d": [round(rm - k * (v - wo) ** 2 + rng.uniform(-0.005, 0.005), 3) for v in ws], "Gs": rng.uniform(2.6, 2.75)}


proc("proctor_compaction_optimum", "Optimum water content and maximum dry density from a compaction test", "soil mechanics",
     "Optimum water content and maximum dry density from Proctor compaction points by a least-squares parabola, with "
     "the zero-air-voids dry density at the optimum when Gs is given. Water contents in %, dry densities in Mg/m^3.",
     [_VEC("w", "water contents in %", ("water contents",), min_length=3),
      _VEC("rho_d", "dry densities in Mg/m^3", ("dry densities",), min_length=3),
      _N("Gs", "specific gravity of the solids (optional)", ("specific gravity",), required=False)],
     [_o("w_opt", "number", "optimum water content (%)"), _o("rho_dmax", "number", "maximum dry density (Mg/m^3)"),
      _o("rho_zav", "number", "zero-air-voids dry density at the optimum (Mg/m^3)")],
     proctor, _ref_proctor,
     lambda w, rho_d, Gs=None: (lambda i: dict(proctor(w, rho_d, Gs), w_opt=_r(w[i]), rho_dmax=_r(rho_d[i])))(max(range(len(w)), key=lambda j: rho_d[j])),
     _rand_proctor,
     [{"w": [8, 10, 12, 14, 16], "rho_d": [1.72, 1.80, 1.84, 1.82, 1.76], "Gs": 2.68}, {"w": [12, 14, 16, 18, 20, 22], "rho_d": [1.58, 1.64, 1.68, 1.69, 1.66, 1.61]}],
     ["proctor compaction optimum", "optimum moisture content", "compaction test optimum"],
     conditions=["at least three points around the peak", "a parabola opening downward"],
     method="least-squares parabola rho_d = c0 + c1 w + c2 w^2; w_opt = -c1/(2 c2); zero air voids rho_w Gs/(1 + w Gs)",
     latex=r"w_{opt} = -\frac{c_1}{2c_2},\quad \rho_{zav} = \frac{G_s\rho_w}{1+wG_s}")


# =============================================================================================== surveying
def traverse_bowditch(az, d, E0=0.0, N0=0.0):
    A = _vec("azimuths", az)
    D = _vec("distances", d, len(A), positive=True)
    _need(len(A) >= 3, "a closed traverse needs at least three legs")
    _finite(E0=E0, N0=N0)
    for a in A:
        _need(0 <= a < 360, "azimuths are whole-circle bearings from 0 to 360 degrees")
    dep = [x * math.sin(math.radians(a)) for a, x in zip(A, D)]
    lat = [x * math.cos(math.radians(a)) for a, x in zip(A, D)]
    eE, eN = sum(dep), sum(lat)
    P = sum(D)
    lin = math.hypot(eE, eN)
    E, N = [E0], [N0]
    for i in range(len(A) - 1):
        E.append(E[-1] + dep[i] - eE * D[i] / P)
        N.append(N[-1] + lat[i] - eN * D[i] / P)
    area = abs(_shoelace(list(zip(E, N))))
    scale = P
    return {"misclosure_E": _r(_snap(eE, scale)), "misclosure_N": _r(_snap(eN, scale)), "linear_misclosure": _r(_snap(lin, scale)),
            "precision": _r(P / lin if lin > 1e-12 * scale else 0.0), "E": _r(E), "N": _r(N), "area": _r(area)}


def _ref_traverse(az, d, E0=0.0, N0=0.0):
    import numpy as np
    from shapely.geometry import Polygon
    a, D = np.radians(np.asarray(az, float)), np.asarray(d, float)
    dep, lat = D * np.sin(a), D * np.cos(a)
    n = len(D)

    def adjust(x):
        # least squares: minimise sum v_i^2/d_i subject to sum(x_i + v_i) = 0 (KKT system)
        K = np.zeros((n + 1, n + 1))
        K[:n, :n] = np.diag(1.0 / D)
        K[:n, n] = K[n, :n] = 1.0
        rhs = np.zeros(n + 1)
        rhs[n] = -x.sum()
        return x + np.linalg.solve(K, rhs)[:n]
    E = E0 + np.concatenate([[0.0], np.cumsum(adjust(dep))[:-1]])
    N = N0 + np.concatenate([[0.0], np.cumsum(adjust(lat))[:-1]])
    lin = float(np.hypot(dep.sum(), lat.sum()))
    return {"misclosure_E": float(dep.sum()), "misclosure_N": float(lat.sum()), "linear_misclosure": lin,
            "precision": float(D.sum()) / lin, "E": list(E), "N": list(N), "area": Polygon(list(zip(E, N))).area}


def _traverse_wrong(az, d, E0=0.0, N0=0.0):
    # the transit rule (corrections proportional to |departure| and |latitude|) in place of the compass rule
    dep = [x * math.sin(math.radians(a)) for a, x in zip(az, d)]
    lat = [x * math.cos(math.radians(a)) for a, x in zip(az, d)]
    eE, eN = sum(dep), sum(lat)
    sE, sN = sum(abs(v) for v in dep), sum(abs(v) for v in lat)
    E, N = [E0], [N0]
    for i in range(len(az) - 1):
        E.append(E[-1] + dep[i] - eE * abs(dep[i]) / sE)
        N.append(N[-1] + lat[i] - eN * abs(lat[i]) / sN)
    out = traverse_bowditch(az, d, E0, N0)
    return dict(out, E=_r(E), N=_r(N), area=_r(abs(_shoelace(list(zip(E, N))))))


def _rand_traverse(rng):
    n = rng.randint(4, 7)
    R = rng.uniform(100, 600)
    ang = [2 * math.pi * (i + rng.uniform(0.2, 0.8)) / n for i in range(n)]
    pts = [(R * math.cos(a), R * math.sin(a)) for a in ang]
    az, d = [], []
    for i in range(n):
        (x1, y1), (x2, y2) = pts[i], pts[(i + 1) % n]
        dx, dy = x2 - x1, y2 - y1
        az.append(round(math.degrees(math.atan2(dx, dy)) % 360 + rng.uniform(-0.005, 0.005), 4) % 360)
        d.append(round(math.hypot(dx, dy) + rng.uniform(-0.05, 0.05), 3))
    x = {"az": az, "d": d, "E0": round(rng.uniform(0, 5000), 3), "N0": round(rng.uniform(0, 5000), 3)}
    o = traverse_bowditch(**x)
    # both misclosure components well above rounding (they are differences of sums of large terms)
    if min(abs(o["misclosure_E"]), abs(o["misclosure_N"])) < 1e-6 * sum(d):
        return _rand_traverse(rng)
    return x


proc("traverse_bowditch_adjustment", "Closed traverse: misclosure and Bowditch (compass rule) adjustment", "surveying",
     "Latitudes and departures of a closed loop traverse from whole-circle bearings (azimuths clockwise from north, "
     "degrees) and distances (m), the misclosure and relative precision (1 in X), the coordinates adjusted by the "
     "Bowditch (compass) rule, and the enclosed area. The last leg returns to the starting station.",
     [_VEC("az", "azimuths (whole-circle bearings) of the legs in degrees", ("azimuths", "bearings"), min_length=3),
      _VEC("d", "leg lengths in m", ("distances", "lengths"), min_length=3),
      _L("E0", "easting of the starting station (default 0)", ("start easting",), required=False),
      _L("N0", "northing of the starting station (default 0)", ("start northing",), required=False)],
     [_o("misclosure_E", "length", "misclosure in departure (m)"), _o("misclosure_N", "length", "misclosure in latitude (m)"),
      _o("linear_misclosure", "length", "linear misclosure (m)"), _o("precision", "number", "relative precision X in 1:X (0 if it closes exactly)"),
      _o("E", "length", "adjusted eastings of the stations (m)"), _o("N", "length", "adjusted northings of the stations (m)"),
      _o("area", "area", "enclosed area (m^2)")],
     traverse_bowditch, _ref_traverse, _traverse_wrong, _rand_traverse,
     [{"az": [45, 135.01, 225, 315], "d": [100, 100.02, 99.98, 100]},
      {"az": [30.5, 112.25, 201.75, 290.1], "d": [250.4, 310.25, 280.9, 240.55], "E0": 1000, "N0": 2000}],
     ["traverse bowditch adjustment", "bowditch adjustment", "compass rule adjustment", "traverse misclosure"],
     conditions=["closed loop traverse", "azimuths 0 to 360 degrees"],
     method="dep = d sin(az), lat = d cos(az); corrections -e d_i/sum d (compass rule); area by coordinates",
     latex=r"\delta E_i = -e_E\frac{d_i}{\sum d},\quad \delta N_i = -e_N\frac{d_i}{\sum d}")


def _sights(text):
    toks = [t.strip().upper() for t in re.split(r"[\s,;]+", text) if t.strip()]
    _need(toks and all(t in ("BS", "IS", "FS") for t in toks), "sights are BS, IS or FS, in reading order")
    return toks


def levelling_rise_fall(readings, sights, RL0):
    r = _vec("readings", readings, nonneg=True)
    s = _sights(sights) if isinstance(sights, str) else list(sights)
    _need(len(s) == len(r), "give one sight type per reading")
    _need(s[0] == "BS" and s[-1] == "FS", "the first reading is a BS and the last an FS")
    for i in range(1, len(s)):
        _need(not (s[i] == "BS" and s[i - 1] != "FS"), "a BS (after the first) must follow an FS at the same change point")
        _need(not (s[i] == "FS" and s[i - 1] == "FS"), "two FS readings in a row")
    _finite(RL0=RL0)
    rl = [RL0]
    rise = fall = 0.0
    prev = r[0]
    for i in range(1, len(r)):
        if s[i] == "BS":                 # same station as the FS before it: a new instrument set-up
            prev = r[i]
            continue
        diff = prev - r[i]
        if diff >= 0:
            rise += diff
        else:
            fall -= diff
        rl.append(rl[-1] + diff)
        prev = r[i]
    sbs = sum(x for x, t in zip(r, s) if t == "BS")
    sfs = sum(x for x, t in zip(r, s) if t == "FS")
    sc = max(r)
    return {"RL": _r(rl), "rise": _r(rise), "fall": _r(fall), "last_RL": _r(rl[-1]),
            "check": _r(_snap(sbs - sfs, sc))}


def _ref_levelling(readings, sights, RL0):
    import numpy as np
    s = [t.strip().upper() for t in re.split(r"[\s,;]+", sights) if t.strip()] if isinstance(sights, str) else list(sights)
    r = np.asarray(readings, float)
    # height-of-instrument method: HI = RL + BS, RL = HI - IS/FS
    rl, HI = [RL0], RL0 + r[0]
    for x, t in zip(r[1:], s[1:]):
        if t == "BS":
            HI = rl[-1] + x
        else:
            rl.append(HI - x)
    d = np.diff(rl)
    return {"RL": rl, "rise": float(d[d > 0].sum()), "fall": float(-d[d < 0].sum()), "last_RL": rl[-1],
            "check": float(r[[t == "BS" for t in s]].sum() - r[[t == "FS" for t in s]].sum())}


def _rand_level(rng):
    n_setups = rng.randint(1, 4)
    rd, st = [], []
    for k in range(n_setups):
        rd.append(round(rng.uniform(0.5, 3.5), 3))
        st.append("BS")
        for _ in range(rng.randint(0, 3)):
            rd.append(round(rng.uniform(0.5, 3.5), 3))
            st.append("IS")
        rd.append(round(rng.uniform(0.5, 3.5), 3))
        st.append("FS")
    return {"readings": rd, "sights": ", ".join(st), "RL0": round(rng.uniform(10, 300), 3)}


proc("levelling_rise_and_fall", "Reduced levels by the rise-and-fall method", "surveying",
     "Reduced levels of every station of a levelling run from staff readings (m) and their sight types (BS, IS, FS in "
     "reading order; a BS after an FS is the new set-up on the same change point), with the total rise and fall and "
     "the arithmetic check sum BS - sum FS (= last RL - first RL).",
     [_VEC("readings", "staff readings in m, in reading order", ("readings", "staff readings"), min_length=2),
      _in("sights", "number", "number", "sight type of each reading: BS, IS or FS", ("sight types",), "", typ="text", min_length=2),
      _L("RL0", "reduced level of the first station (benchmark)", ("benchmark level", "starting level", "benchmark"))],
     [_o("RL", "length", "reduced level of each station (m)"), _o("rise", "length", "sum of rises (m)"),
      _o("fall", "length", "sum of falls (m)"), _o("last_RL", "length", "reduced level of the last station (m)"),
      _o("check", "length", "sum BS - sum FS (m)")],
     levelling_rise_fall, _ref_levelling,
     lambda readings, sights, RL0: (lambda o: dict(o, RL=_r([2 * RL0 - v for v in o["RL"]]), last_RL=_r(2 * RL0 - o["last_RL"])))(
         levelling_rise_fall(readings, sights, RL0)),
     _rand_level,
     [{"readings": [1.525, 2.105, 1.640, 0.985, 2.320], "sights": "BS, IS, FS, BS, FS", "RL0": 100},
      {"readings": [2.150, 1.725, 0.980, 1.315, 2.640, 1.105], "sights": "BS, IS, IS, FS, BS, FS", "RL0": 52.75}],
     ["levelling rise and fall", "rise and fall method", "reduced levels by rise and fall"],
     conditions=["first reading BS, last FS", "a BS after the first follows an FS (change point)"],
     method="rise or fall = previous reading - current reading within a set-up; RL = previous RL + rise - fall",
     latex=r"\sum \text{BS} - \sum \text{FS} = \sum\text{rise} - \sum\text{fall} = RL_{last} - RL_{first}")


def horizontal_curve(R, Delta, PI=0.0):
    _pos(R=R)
    _finite(Delta=Delta, PI=PI)
    _need(0 < Delta < 180, "the deflection angle must lie between 0 and 180 degrees")
    d = math.radians(Delta)
    T = R * math.tan(d / 2)
    L = R * d
    out = {"T": _r(T), "L": _r(L), "LC": _r(2 * R * math.sin(d / 2)), "E": _r(R * (1 / math.cos(d / 2) - 1)),
           "M": _r(R * (1 - math.cos(d / 2))), "PC": _r(PI - T), "PT": _r(PI - T + L)}
    return out


def _ref_hcurve(R, Delta, PI=0.0):
    import numpy as np
    d = math.radians(Delta)
    u1 = np.array([1.0, 0.0])                                 # back tangent, PI at the origin
    u2 = np.array([math.cos(d), math.sin(d)])                 # forward tangent, turned left by Delta
    n1, n2 = np.array([-u1[1], u1[0]]), np.array([-u2[1], u2[0]])
    # centre: on the inner side, at distance R from both tangent lines (n . X = R for both)
    O = np.linalg.solve(np.array([n1, n2]), np.array([R, R]))
    PC = np.dot(O, u1) * u1                                   # feet of the perpendiculars on the tangents
    PT = np.dot(O, u2) * u2
    a1, a2 = PC - O, PT - O
    ang = math.atan2(a1[0] * a2[1] - a1[1] * a2[0], float(np.dot(a1, a2)))
    T = float(np.linalg.norm(PC))
    L = R * abs(ang)
    mid = (PC + PT) / 2
    return {"T": T, "L": L, "LC": float(np.linalg.norm(PC - PT)), "E": float(np.linalg.norm(O)) - R,
            "M": R - float(np.linalg.norm(mid - O)), "PC": PI - T, "PT": PI - T + L}


proc("horizontal_circular_curve", "Elements of a simple horizontal circular curve", "surveying",
     "Tangent length, curve length, long chord, external distance, middle ordinate and the chainages of the tangent "
     "points of a simple circular curve of radius R joining tangents that deflect by Delta at the intersection point PI.",
     [_L("R", "curve radius", ("radius",)), _DEG("Delta", "deflection (intersection) angle (degrees)", ("deflection angle", "intersection angle")),
      _L("PI", "chainage of the intersection point (default 0)", ("PI chainage", "chainage of PI"), required=False)],
     [_o("T", "length", "tangent length (m)"), _o("L", "length", "curve length (m)"), _o("LC", "length", "long chord (m)"),
      _o("E", "length", "external distance (m)"), _o("M", "length", "middle ordinate (m)"),
      _o("PC", "length", "chainage of the start of the curve (m)"), _o("PT", "length", "chainage of the end of the curve (m)")],
     horizontal_curve, _ref_hcurve,
     lambda R, Delta, PI=0.0: dict(horizontal_curve(R, Delta, PI), T=_r(R * math.tan(math.radians(Delta))), PC=_r(PI - R * math.tan(math.radians(Delta)))),
     lambda rng: {"R": rng.uniform(50, 2000), "Delta": rng.uniform(5, 150), "PI": round(rng.uniform(0, 20000), 3)},
     [{"R": 300, "Delta": 40, "PI": 1250}, {"R": 600, "Delta": 25.5, "PI": 3420.5}], ["horizontal circular curve", "horizontal curve elements",
                                                                                     "simple circular curve"],
     conditions=["0 < Delta < 180 degrees", "chainage measured along the tangents then the curve"],
     invalid=[{"R": 300, "Delta": 190, "PI": 1250}],
     method="T = R tan(D/2), L = R D, LC = 2R sin(D/2), E = R(sec(D/2) - 1), M = R(1 - cos(D/2)), PC = PI - T, PT = PC + L",
     latex=r"T = R\tan\frac{\Delta}{2},\quad L = R\Delta,\quad LC = 2R\sin\frac{\Delta}{2}")


def vertical_curve(g1, g2, L, PVI, elev_PVI, x):
    _finite(g1=g1, g2=g2, PVI=PVI, elev_PVI=elev_PVI, x=x)
    _pos(L=L)
    _need(abs(g1) < 0.3 and abs(g2) < 0.3, "grades are fractions (or %) below 30%")
    _need(g1 != g2, "the grades must differ")
    bvc = PVI - L / 2
    _need(bvc - 1e-9 <= x <= bvc + L + 1e-9, "the station must lie on the curve (between BVC and EVC)")
    e_bvc = elev_PVI - g1 * L / 2
    r = (g2 - g1) / L
    elev = lambda s: e_bvc + g1 * (s - bvc) + r * (s - bvc) ** 2 / 2      # noqa: E731
    xt = bvc - g1 / r
    if not (bvc < xt < bvc + L):
        xt = bvc if (elev(bvc) > elev(bvc + L)) == (g1 > g2) else bvc + L
    return {"elev": _r(elev(x)), "x_turn": _r(xt), "elev_turn": _r(elev(xt)), "r": _r(r * 100), "BVC": _r(bvc),
            "elev_BVC": _r(e_bvc)}


def _ref_vcurve(g1, g2, L, PVI, elev_PVI, x):
    bvc = PVI - L / 2
    e0 = elev_PVI - g1 * L / 2
    grade = lambda s: g1 + (g2 - g1) * (s - bvc) / L                   # the grade changes linearly along the curve   # noqa: E731
    elev = lambda s: e0 + _quad(grade, bvc, s) if s > bvc else e0       # noqa: E731
    if grade(bvc) * grade(bvc + L) < 0:
        xt = _brentq(grade, bvc, bvc + L)
    else:
        ends = [bvc, bvc + L]
        xt = max(ends, key=elev) if g1 > g2 else min(ends, key=elev)
    return {"elev": elev(x), "x_turn": xt, "elev_turn": elev(xt), "r": (g2 - g1) / L * 100, "BVC": bvc, "elev_BVC": e0}


proc("vertical_parabolic_curve", "Elevations on a vertical parabolic curve", "surveying",
     "Elevation at a station on an equal-tangent parabolic vertical curve, with the highest (crest) or lowest (sag) "
     "point on the curve and the rate of change of grade. Grades are fractions or percentages; stations and lengths "
     "in m.",
     [_F("g1", "entering grade (fraction, or %)", ("entering grade", "first grade"), "%"),
      _F("g2", "leaving grade (fraction, or %)", ("leaving grade", "second grade"), "%"),
      _L("L", "curve length (horizontal)", ("curve length", "length")), _L("PVI", "station of the intersection point", ("PVI station",)),
      _L("elev_PVI", "elevation of the intersection point", ("PVI elevation",)), _L("x", "station where the elevation is wanted", ("station",))],
     [_o("elev", "length", "elevation at the station (m)"), _o("x_turn", "length", "station of the highest/lowest point on the curve (m)"),
      _o("elev_turn", "length", "elevation of that point (m)"), _o("r", "number", "rate of change of grade (% per m)"),
      _o("BVC", "length", "station of the beginning of the curve (m)"), _o("elev_BVC", "length", "elevation of the BVC (m)")],
     vertical_curve, _ref_vcurve,
     lambda g1, g2, L, PVI, elev_PVI, x: dict(vertical_curve(g1, g2, L, PVI, elev_PVI, x),
                                              elev=_r(elev_PVI - g1 * L / 2 + g1 * (x - PVI + L / 2) + (g2 - g1) / L * (x - PVI + L / 2) ** 2)),
     lambda rng: (lambda L, PVI: {"g1": rng.uniform(-0.06, 0.06), "g2": rng.uniform(-0.06, 0.06), "L": L, "PVI": PVI,
                                  "elev_PVI": rng.uniform(50, 500), "x": PVI - L / 2 + rng.uniform(0.05, 0.95) * L})(rng.uniform(60, 600), rng.uniform(500, 5000)),
     [{"g1": 3, "g2": -2, "L": 200, "PVI": 1500, "elev_PVI": 120, "x": 1450}, {"g1": -2.5, "g2": 1.5, "L": 160, "PVI": 820, "elev_PVI": 64.2, "x": 840}],
     ["vertical parabolic curve", "vertical curve elevation", "parabolic vertical curve"],
     conditions=["equal-tangent parabola", "BVC <= station <= EVC"], invalid=[{"g1": 3, "g2": -2, "L": 200, "PVI": 1500, "elev_PVI": 120, "x": 1700}],
     method="y = y_BVC + g1 x + (g2 - g1) x^2/(2L), x from the BVC; turning point at x = -g1 L/(g2 - g1)",
     latex=r"y = y_{BVC} + g_1x + \frac{g_2-g_1}{2L}x^2,\quad x_t = -\frac{g_1L}{g_2-g_1}")


def earthwork_volume(areas, d):
    A = _vec("areas", areas, nonneg=True)
    _need(len(A) >= 2, "give at least two cross-section areas")
    _pos(d=d)
    out = {"V_end_area": _r(d * (sum(A) - (A[0] + A[-1]) / 2))}
    if len(A) % 2 == 1:
        out["V_prismoidal"] = _r(d / 3 * (A[0] + A[-1] + 4 * sum(A[1:-1:2]) + 2 * sum(A[2:-1:2])))
    return out


def _ref_earthwork(areas, d):
    import numpy as np
    from scipy.integrate import simpson
    A = np.asarray(areas, float)
    trap = getattr(np, "trapezoid", None) or np.trapz
    out = {"V_end_area": float(trap(A, dx=d))}
    if len(A) % 2 == 1:
        out["V_prismoidal"] = float(simpson(A, dx=d))
    return out


proc("earthwork_volume_end_areas", "Earthwork volume from cross-section areas", "surveying",
     "Volume of cut or fill between equally spaced cross-sections by the average-end-area method, and by the "
     "prismoidal (Simpson) rule when the number of sections is odd. Areas in m^2.",
     [_VEC("areas", "cross-section areas in m^2, in order", ("areas", "cross section areas"), min_length=2),
      _L("d", "spacing between sections", ("spacing", "interval"))],
     [_o("V_end_area", "volume", "volume by average end areas (m^3)"), _o("V_prismoidal", "volume", "volume by the prismoidal rule (m^3)")],
     earthwork_volume, _ref_earthwork,
     lambda areas, d: dict(earthwork_volume(areas, d), V_end_area=_r(d * sum(areas))),
     lambda rng: {"areas": [round(rng.uniform(0, 120), 1) for _ in range(rng.randint(3, 12))], "d": rng.choice([10, 20, 25, 30, 50])},
     [{"areas": [12.5, 18.2, 25.6, 20.1, 15.8], "d": 20}, {"areas": [30, 42, 51, 47], "d": 25}],
     ["earthwork volume from cross sections", "average end area", "earthwork volume"],
     conditions=["equally spaced sections", "prismoidal rule only for an odd number of sections"],
     method="V = d (A1/2 + A2 + ... + An/2); prismoidal V = (d/3)(A1 + 4A2 + 2A3 + ... + An)",
     latex=r"V = d\left(\frac{A_1+A_n}{2}+\sum_{i=2}^{n-1}A_i\right)")


def offsets_area(offsets, d):
    y = _vec("offsets", offsets, nonneg=True)
    _need(len(y) >= 2, "give at least two offsets")
    _pos(d=d)
    out = {"A_trapezoidal": _r(d * (sum(y) - (y[0] + y[-1]) / 2))}
    if len(y) % 2 == 1:
        out["A_simpson"] = _r(d / 3 * (y[0] + y[-1] + 4 * sum(y[1:-1:2]) + 2 * sum(y[2:-1:2])))
    return out


def _ref_offsets(offsets, d):
    r = _ref_earthwork(offsets, d)
    out = {"A_trapezoidal": r["V_end_area"]}
    if "V_prismoidal" in r:
        out["A_simpson"] = r["V_prismoidal"]
    return out


proc("area_from_offsets", "Area between a survey line and a boundary from offsets", "surveying",
     "Area between a chain line and an irregular boundary from offsets at a regular interval, by the trapezoidal rule "
     "and, for an odd number of offsets, Simpson's one-third rule. Offsets in m.",
     [_VEC("offsets", "offsets in m at the regular interval", ("offsets", "ordinates"), min_length=2),
      _L("d", "interval between offsets", ("interval", "spacing"))],
     [_o("A_trapezoidal", "area", "area by the trapezoidal rule (m^2)"), _o("A_simpson", "area", "area by Simpson's rule (m^2)")],
     offsets_area, _ref_offsets,
     lambda offsets, d: dict(offsets_area(offsets, d), A_trapezoidal=_r(d * sum(offsets))),
     lambda rng: {"offsets": [round(rng.uniform(0, 30), 2) for _ in range(rng.randint(3, 12))], "d": rng.choice([5, 10, 15, 20])},
     [{"offsets": [0, 2.5, 3.5, 5.0, 4.6, 3.2, 0], "d": 10}, {"offsets": [3.2, 5.1, 6.8, 7.5, 6.2, 4.9], "d": 15}],
     ["area from offsets", "area by offsets", "simpsons rule area"],
     conditions=["regular interval", "Simpson's rule only for an odd number of offsets"],
     method="trapezoidal: d (y1/2 + y2 + ... + yn/2); Simpson: (d/3)(y1 + 4y2 + 2y3 + ... + yn)",
     latex=r"A = \frac{d}{3}\left(y_1 + y_n + 4\sum y_{even} + 2\sum y_{odd}\right)")


# =============================================================================================== definitional additions
# (owner, 2026-10-23: a capability that passes every gate ships; a weaker reference is documented). Rational method,
# Kirpich time of concentration, infinite-slope factor of safety, Terzaghi bearing capacity with stated factors,
# plasticity and liquidity indices, secondary compression.
_UNITS["percent_value"] = {"": 1, "%": 1, "percent": 1}       # Atterberg limits written in percent (45 or 45 %)


def _guard(fn):
    """A wrong variant that raises cannot be compared: return a failed marker instead."""
    def g(**kw):
        try:
            return fn(**kw)
        except Exception:
            return {"_failed": True}
    return g


# Outputs of the documented examples of these additions, computed by their references on the build machine and
# rounded to 12 significant digits.
EXAMPLE_OUTPUTS.update({
    "rational_method_peak_runoff": [{"Q": 1.66666666667, "Q_Ls": 1666.66666667}, {"Q": 0.708333333333, "Q_Ls": 708.333333333}],
    "kirpich_time_of_concentration": [{"tc": 1469.76551371, "tc_min": 24.4960918952, "tc_h": 0.408268198253}, {"tc": 299.112282071, "tc_min": 4.98520470118, "tc_h": 0.0830867450197}],
    "infinite_slope_factor_of_safety": [{"FS": 1.47987373845, "tau": 20683.1999642, "sigma_eff": 44355.2654615}, {"FS": 0.874716954368, "tau": 12855.7521937, "sigma_eff": 17995.9928754}],
    "terzaghi_bearing_capacity": [{"qu": 1848052.0, "qa": 616017.333333, "q_net": 1821052.0}, {"qu": 1434452.5, "qa": 478150.833333, "q_net": 1415452.5}],
    "plasticity_liquidity_index": [{"PI": 23.0, "A_line_PI": 18.25, "LI": 0.347826086957, "CI": 0.652173913043}, {"PI": 34.0, "A_line_PI": 30.66}],
    "secondary_compression_settlement": [{"Ss": 0.0619538093173, "Ss_mm": 61.9538093173, "C_alpha_mod": 0.00952380952381}, {"Ss": 0.0156812167874, "Ss_mm": 15.6812167874, "C_alpha_mod": 0.00444444444444}],
})


# --- rational method --------------------------------------------------------------------------------------------------
def rational_peak(C, i, A):
    _finite(C=C)
    _pos(i=i, A=A)
    _need(0 < C <= 1, "the runoff coefficient must lie in (0, 1]")
    _need(A <= 8e6, "the rational method is for small catchments (at most 800 ha here)")
    Q = C * i * A
    return {"Q": _r(Q), "Q_Ls": _r(Q * 1e3)}


def _ref_rational_peak(C, i, A):
    # the runoff rate C i integrated over a square catchment of area A (scipy dblquad)
    from scipy.integrate import dblquad
    s = math.sqrt(A)
    Q = dblquad(lambda y, x: C * i, 0.0, s, 0.0, s, epsabs=0, epsrel=1e-13)[0]
    return {"Q": Q, "Q_Ls": Q * 1e3}


proc("rational_method_peak_runoff", "Peak runoff by the rational method", "hydrology",
     "The peak runoff Q = C i A of a small catchment from the runoff coefficient, the rainfall intensity for a duration "
     "equal to the time of concentration, and the catchment area.",
     [_F("C", "runoff coefficient (0 to 1)", ("runoff coefficient",)), _rate_in("i", "rainfall intensity", ("rainfall intensity", "intensity")),
      _in("A", "area", "area", "catchment area", ("catchment area", "area"), "ha")],
     [_o("Q", "flow_rate", "peak runoff (m^3/s)"), _o("Q_Ls", "number", "peak runoff (L/s)")],
     rational_peak, _ref_rational_peak,
     _guard(lambda C, i, A: {"Q": _r(0.278 * C * i * A), "Q_Ls": _r(0.278 * C * i * A * 1e3)}),
     lambda rng: {"C": rng.uniform(0.1, 0.95), "i": rng.uniform(5, 200) * 1e-3 / 3600, "A": rng.uniform(1e3, 5e6)},
     [{"C": 0.6, "i": 50, "A": 20}, {"C": 0.85, "i": 120, "A": 2.5}],
     ["rational method peak runoff", "rational method", "peak runoff by the rational method"],
     conditions=["0 < C <= 1", "A <= 800 ha (small catchments)", "intensity for a duration equal to the time of concentration"],
     invalid=[{"C": 1.5, "i": 50, "A": 20}], method="Q = C i A (SI); = 0.278 C i[mm/h] A[km^2]", latex=r"Q = C\,i\,A")


# --- Kirpich time of concentration ------------------------------------------------------------------------------------
def kirpich(L, S):
    _pos(L=L, S=S)
    _need(S <= 1, "the slope must be at most 1 (m/m)")
    _need(L <= 1e5, "the flow length must be at most 100 km")
    tc = 0.0078 * (L / 0.3048) ** 0.77 * S ** -0.385
    return {"tc": _r(tc * 60), "tc_min": _r(tc), "tc_h": _r(tc / 60)}


def _ref_kirpich(L, S):
    # Kirpich's metric form with its coefficient converted exactly from the US form (sympy, 30 digits)
    import sympy as sp
    k = sp.Rational("0.0078") * (1 / sp.Rational("0.3048")) ** sp.Rational("0.77")
    tc = float(sp.N(k * sp.Float(L, 30) ** sp.Rational("0.77") * sp.Float(S, 30) ** sp.Rational("-0.385"), 25))
    return {"tc": tc * 60, "tc_min": tc, "tc_h": tc / 60}


proc("kirpich_time_of_concentration", "Time of concentration by the Kirpich formula", "hydrology",
     "The time of concentration tc = 0.0078 L^0.77 S^-0.385 minutes (L in feet; equivalent to 0.0195 L^0.77 S^-0.385 with L "
     "in metres) of a small rural catchment from the flow length and the average slope.",
     [_L("L", "length of the longest flow path", ("flow length", "length")), _F("S", "average slope (m/m)", ("slope", "average slope"))],
     [_o("tc", "time", "time of concentration (s)"), _o("tc_min", "number", "time of concentration (min)"), _o("tc_h", "number", "time of concentration (h)")],
     kirpich, _ref_kirpich,
     _guard(lambda L, S: (lambda tc: {"tc": _r(tc * 60), "tc_min": _r(tc), "tc_h": _r(tc / 60)})(0.0078 * (L / 0.3048) ** 0.77 * S ** 0.385)),
     lambda rng: {"L": rng.uniform(50, 2e4), "S": 10 ** rng.uniform(-3.5, -0.7)},
     [{"L": 1500, "S": 0.02}, {"L": 300, "S": 0.05}],
     ["kirpich time of concentration", "time of concentration by kirpich", "kirpich formula", "time of concentration"],
     conditions=["0 < S <= 1", "L <= 100 km", "small rural catchments (Kirpich's data: 0.4 to 45 ha)"], invalid=[{"L": 1500, "S": -0.02}],
     method="tc = 0.0078 L_ft^0.77 S^-0.385 min, L_ft = L/0.3048", latex=r"t_c = 0.0078\,L^{0.77}S^{-0.385}\ \text{(min, L in ft)}")


# --- infinite slope ---------------------------------------------------------------------------------------------------
def infinite_slope(c, phi, gam, z, beta, m=0.0):
    _finite(c=c, phi=phi, beta=beta, m=m)
    _pos(gam=gam, z=z)
    _need(c >= 0, "the cohesion must be zero or positive")
    _need(0 <= phi < 60, "the friction angle must lie in [0, 60) degrees")
    _need(1 <= beta <= 80, "the slope angle must lie between 1 and 80 degrees")
    _need(0 <= m <= 1, "the water-table ratio m must lie between 0 (dry) and 1 (at the surface)")
    _need(m == 0 or gam > GAMMA_W, "a submerged slope needs a unit weight above that of water")
    b, p = math.radians(beta), math.radians(phi)
    cb, sb = math.cos(b), math.sin(b)
    tau = gam * z * sb * cb
    sig_eff = (gam - m * GAMMA_W) * z * cb * cb
    FS = (c + sig_eff * math.tan(p)) / tau
    return {"FS": _r(FS), "tau": _r(tau), "sigma_eff": _r(sig_eff)}


def _ref_infinite_slope(c, phi, gam, z, beta, m=0.0):
    # a slice of unit base length: weight vector resolved onto the base normal and tangent (numpy), pore pressure of
    # seepage parallel to the slope at the water-table height m z, Mohr-Coulomb resistance over the base
    import numpy as np
    b = np.radians(beta)
    W = np.array([0.0, -gam * z * np.cos(b)])                         # weight of a slice of unit base length
    t = np.array([np.cos(b), -np.sin(b)])                              # down-slope direction along the base
    n = np.array([np.sin(b), np.cos(b)])                               # outward normal of the base
    T = float(W @ t)
    N = float(-(W @ n))
    u = m * z * GAMMA_W * np.cos(b) ** 2
    FS = (c + (N - u) * np.tan(np.radians(phi))) / T
    return {"FS": float(FS), "tau": T, "sigma_eff": N - float(u)}


proc("infinite_slope_factor_of_safety", "Factor of safety of an infinite slope", "soil mechanics",
     "The factor of safety against translational sliding of an infinite slope on a plane at depth z, with seepage "
     "parallel to the slope and the water table at a height m z above the slip plane (m = 0 dry, m = 1 at the surface).",
     [_STRESS("c", "effective cohesion", ("cohesion",)), _DEG("phi", "effective friction angle (degrees)", ("friction angle",)),
      _GAM("gam", "unit weight of the soil (saturated below the water table)", ("unit weight",)),
      _L("z", "depth of the slip plane (vertical)", ("depth", "depth of slip plane")),
      _DEG("beta", "slope angle (degrees)", ("slope angle",)),
      _F("m", "water-table height above the slip plane as a fraction of z (default 0, dry)", ("water table ratio",), required=False)],
     [_o("FS", "number", "factor of safety"), _o("tau", "stress", "driving shear stress on the slip plane (Pa)"),
      _o("sigma_eff", "stress", "effective normal stress on the slip plane (Pa)")],
     infinite_slope, _ref_infinite_slope,
     _guard(lambda c, phi, gam, z, beta, m=0.0: (lambda b: (lambda tau, s: {"FS": _r((c + s * math.tan(math.radians(phi))) / tau), "tau": _r(tau), "sigma_eff": _r(s)})(
         gam * z * math.sin(b) * math.cos(b), (gam - m * GAMMA_W) * z * math.cos(b)))(math.radians(beta))),
     lambda rng: {"c": rng.choice([0.0, rng.uniform(1e3, 30e3)]), "phi": rng.uniform(15, 40), "gam": rng.uniform(16e3, 21e3), "z": rng.uniform(0.5, 10),
                  "beta": rng.uniform(10, 45), **({"m": rng.uniform(0, 1)} if rng.random() < 0.6 else {})},
     [{"c": 5, "phi": 30, "gam": 18, "z": 3, "beta": 25}, {"c": 0, "phi": 32, "gam": 20, "z": 2, "beta": 20, "m": 1}],
     ["infinite slope factor of safety", "infinite slope stability", "infinite slope"],
     conditions=["c >= 0", "0 <= phi < 60 degrees", "1 <= beta <= 80 degrees", "0 <= m <= 1", "seepage parallel to the slope"],
     invalid=[{"c": 5, "phi": 30, "gam": 18, "z": 3, "beta": 85}],
     method="FS = [c' + (gamma - m gamma_w) z cos^2(beta) tan(phi')]/(gamma z sin(beta) cos(beta))",
     latex=r"FS = \frac{c' + (\gamma - m\gamma_w)\,z\cos^2\beta\,\tan\phi'}{\gamma z\sin\beta\cos\beta}")


# --- Terzaghi bearing capacity with stated factors --------------------------------------------------------------------
_TERZ_SHAPE = {"strip": (1.0, 0.5), "square": (1.3, 0.4), "circular": (1.3, 0.3)}


def terzaghi_bearing(c, gam, B, Df, Nc, Nq, Ngamma, shape="strip", FS=3.0):
    _finite(c=c, Df=Df, Nc=Nc, Nq=Nq, Ngamma=Ngamma)
    _pos(gam=gam, B=B, FS=FS)
    _need(c >= 0 and Df >= 0, "c and Df must be zero or positive")
    _need(Nc >= 5.14 and Nq >= 1 and Ngamma >= 0, "the bearing capacity factors must satisfy Nc >= 5.14, Nq >= 1, Ngamma >= 0")
    _need(shape in _TERZ_SHAPE, "the footing must be strip, square or circular")
    sc, sg = _TERZ_SHAPE[shape]
    q = gam * Df
    qu = sc * c * Nc + q * Nq + sg * gam * B * Ngamma
    return {"qu": _r(qu), "qa": _r(qu / FS), "q_net": _r(qu - q)}


def _ref_terzaghi_bearing(c, gam, B, Df, Nc, Nq, Ngamma, shape="strip", FS=3.0):
    # the three terms (cohesion, surcharge, self-weight) as a dot product of the factors and Terzaghi's term weights
    # (numpy); the surcharge as the overburden pressure integrated over the embedment depth (scipy quadrature)
    import numpy as np
    sc, sg = {"strip": (1.0, 0.5), "square": (1.3, 0.4), "circular": (1.3, 0.3)}[shape]
    q = _quad(lambda z: gam, 0.0, Df) if Df > 0 else 0.0
    qu = float(np.dot([sc * c, q, sg * gam * B], [Nc, Nq, Ngamma]))
    return {"qu": qu, "qa": qu / FS, "q_net": qu - q}


proc("terzaghi_bearing_capacity", "Terzaghi bearing capacity with stated factors", "foundations",
     "Ultimate, allowable and net bearing capacity of a strip, square or circular footing by Terzaghi's equation with "
     "the bearing capacity factors Nc, Nq and Ngamma stated (from a table): qu = s_c c Nc + q Nq + s_g gamma B Ngamma, "
     "s_c = 1.3 and s_g = 0.4 (square) or 0.3 (circular, B = diameter).",
     [_STRESS("c", "cohesion", ("cohesion",)), _GAM("gam", "unit weight of the soil", ("unit weight",)),
      _L("B", "footing width (diameter of a circular footing)", ("width", "footing width", "diameter")),
      _L("Df", "embedment depth", ("depth", "embedment depth")),
      _N("Nc", "bearing capacity factor Nc (stated)", ("Nc",)), _N("Nq", "bearing capacity factor Nq (stated)", ("Nq",)),
      _N("Ngamma", "bearing capacity factor Ngamma (stated)", ("Ngamma", "N gamma")),
      _in("shape", "number", "number", "footing shape: strip, square or circular (default strip)", ("footing shape",), required=False,
          typ="choice", choices=["strip", "square", "circular"]),
      _N("FS", "factor of safety (default 3)", ("factor of safety",), required=False)],
     [_o("qu", "stress", "ultimate bearing capacity (Pa)"), _o("qa", "stress", "allowable bearing capacity qu/FS (Pa)"),
      _o("q_net", "stress", "net ultimate bearing capacity qu - gamma Df (Pa)")],
     terzaghi_bearing, _ref_terzaghi_bearing,
     _guard(lambda c, gam, B, Df, Nc, Nq, Ngamma, shape="strip", FS=3.0: (lambda sc, sg: (lambda qu: {"qu": _r(qu), "qa": _r(qu / FS), "q_net": _r(qu - gam * Df)})(
         sc * c * Nc + gam * B * Nq + sg * gam * B * Ngamma))(*_TERZ_SHAPE[shape])),
     lambda rng: (lambda B: {"c": rng.choice([0.0, rng.uniform(1e3, 50e3)]), "gam": rng.uniform(15e3, 21e3), "B": B, "Df": B * rng.uniform(0.2, 2.5) + 0.1,
                             "Nc": rng.uniform(5.7, 100), "Nq": rng.uniform(1.0, 80), "Ngamma": rng.uniform(0.0, 100),
                             **({"shape": rng.choice(["strip", "square", "circular"])} if rng.random() < 0.8 else {}),
                             **({"FS": rng.uniform(2, 4)} if rng.random() < 0.3 else {})})(rng.uniform(0.5, 4)),
     [{"c": 20, "gam": 18, "B": 2, "Df": 1.5, "Nc": 37.16, "Nq": 22.46, "Ngamma": 19.13, "shape": "square"},
      {"c": 0, "gam": 19, "B": 1.5, "Df": 1, "Nc": 57.75, "Nq": 41.44, "Ngamma": 45.41}],
     ["terzaghi bearing capacity", "bearing capacity by terzaghi", "terzaghi bearing capacity with stated factors"],
     conditions=["c, Df >= 0", "factors as stated (Terzaghi's table for the friction angle)", "general shear failure, vertical central load"],
     invalid=[{"c": 20, "gam": 18, "B": 2, "Df": 1.5, "Nc": 3, "Nq": 22.46, "Ngamma": 19.13}],
     method="qu = s_c c Nc + gamma Df Nq + s_g gamma B Ngamma (strip 1, 0.5; square 1.3, 0.4; circular 1.3, 0.3)",
     latex=r"q_u = s_c\,c N_c + \gamma D_f N_q + s_\gamma\,\gamma B N_\gamma")


# --- plasticity and liquidity indices ---------------------------------------------------------------------------------
def atterberg_indices(LL, PL, w=None):
    _finite(LL=LL, PL=PL)
    _need(5 <= LL <= 1000, "give the liquid limit in percent (5 to 1000)")
    _need(0 <= PL < LL, "the plastic limit must lie below the liquid limit")
    PI = LL - PL
    out = {"PI": _r(PI), "A_line_PI": _r(0.73 * (LL - 20))}
    if w is not None:
        _finite(w=w)
        _need(0 <= w <= 2000, "give the natural water content in percent")
        out["LI"] = _r((w - PL) / PI)
        out["CI"] = _r((LL - w) / PI)
    return out


def _ref_atterberg(LL, PL, w=None):
    # exact rational arithmetic (sympy) from the limits as written
    import sympy as sp
    L, P = sp.Rational(repr(LL)), sp.Rational(repr(PL))
    out = {"PI": float(L - P), "A_line_PI": float(sp.Rational("0.73") * (L - 20))}
    if w is not None:
        W = sp.Rational(repr(w))
        out["LI"] = float((W - P) / (L - P))
        out["CI"] = float((L - W) / (L - P))
    return out


proc("plasticity_liquidity_index", "Plasticity, liquidity and consistency indices", "soil mechanics",
     "The plasticity index PI = LL - PL, the Casagrande A-line value 0.73 (LL - 20) and, with the natural water "
     "content, the liquidity index (w - PL)/PI and consistency index (LL - w)/PI. Limits and water content in percent.",
     [_in("LL", "number", "percent_value", "liquid limit in %", ("liquid limit",), "%"),
      _in("PL", "number", "percent_value", "plastic limit in %", ("plastic limit",), "%"),
      _in("w", "number", "percent_value", "natural water content in % (optional)", ("water content", "natural water content"), "%", required=False)],
     [_o("PI", "number", "plasticity index (%)"), _o("A_line_PI", "number", "plasticity index on the A-line at this LL (%)"),
      _o("LI", "number", "liquidity index"), _o("CI", "number", "consistency index")],
     atterberg_indices, _ref_atterberg,
     _guard(lambda LL, PL, w=None: dict({"PI": _r(LL - PL), "A_line_PI": _r(0.73 * (LL - 20))},
                                        **({"LI": _r((w - PL) / LL), "CI": _r((LL - w) / LL)} if w is not None else {}))),
     lambda rng: (lambda LL: (lambda PL: {"LL": LL, "PL": PL, **({"w": rng.uniform(0.5 * PL, 1.3 * LL)} if rng.random() < 0.7 else {})})(LL * rng.uniform(0.3, 0.8)))(
         rng.uniform(20, 120)),
     [{"LL": 45, "PL": 22, "w": 30}, {"LL": 62, "PL": 28}],
     ["plasticity and liquidity index", "plasticity index", "liquidity index", "atterberg indices"],
     conditions=["5 <= LL <= 1000 %", "0 <= PL < LL", "inputs in percent"], invalid=[{"LL": 30, "PL": 35}],
     method="PI = LL - PL; LI = (w - PL)/PI; CI = (LL - w)/PI; A-line PI = 0.73 (LL - 20)",
     latex=r"PI = LL - PL,\quad LI = \frac{w - PL}{PI},\quad CI = \frac{LL - w}{PI}")


# --- secondary compression --------------------------------------------------------------------------------------------
def secondary_compression(Ca, H, ep, t1, t2):
    _finite(Ca=Ca, ep=ep)
    _pos(H=H, t1=t1, t2=t2)
    _need(0 < Ca <= 0.2, "the secondary compression index must lie in (0, 0.2]")
    _need(0 < ep <= 10, "the void ratio at the end of primary consolidation must lie in (0, 10]")
    _need(t2 > t1, "the later time must exceed the end of primary consolidation")
    Cm = Ca / (1 + ep)
    S = Cm * H * math.log10(t2 / t1)
    return {"Ss": _r(S), "Ss_mm": _r(S * 1e3), "C_alpha_mod": _r(Cm)}


def _ref_secondary_compression(Ca, H, ep, t1, t2):
    # the strain rate d(eps)/dt = C_alpha/((1 + e_p) t ln 10) integrated from t1 to t2 (scipy quadrature in ln t)
    lt1, lt2 = math.log(t1), math.log(t2)
    eps = _quad(lambda s: Ca / ((1 + ep) * math.log(10)), lt1, lt2)
    return {"Ss": eps * H, "Ss_mm": eps * H * 1e3, "C_alpha_mod": Ca / (1 + ep)}


proc("secondary_compression_settlement", "Secondary compression (creep) settlement of a clay layer", "soil mechanics",
     "The secondary compression settlement S_s = C_alpha H/(1 + e_p) log10(t2/t1) of a clay layer of thickness H after "
     "the end of primary consolidation t1, up to a later time t2.",
     [_N("Ca", "secondary compression index C_alpha", ("secondary compression index", "C alpha")),
      _L("H", "layer thickness (at the end of primary consolidation)", ("thickness", "layer thickness")),
      _N("ep", "void ratio at the end of primary consolidation", ("void ratio",)),
      _in("t1", "time", "time", "time at the end of primary consolidation", ("end of primary", "first time"), "yr"),
      _in("t2", "time", "time", "later time", ("later time", "second time"), "yr")],
     [_o("Ss", "length", "secondary compression settlement (m)"), _o("Ss_mm", "number", "settlement (mm)"),
      _o("C_alpha_mod", "number", "modified secondary compression index C_alpha/(1 + e_p)")],
     secondary_compression, _ref_secondary_compression,
     _guard(lambda Ca, H, ep, t1, t2: (lambda S: {"Ss": _r(S), "Ss_mm": _r(S * 1e3), "C_alpha_mod": _r(Ca / (1 + ep))})(Ca / (1 + ep) * H * math.log(t2 / t1))),
     lambda rng: (lambda t1: {"Ca": rng.uniform(0.002, 0.05), "H": rng.uniform(0.5, 20), "ep": rng.uniform(0.4, 3.0), "t1": t1, "t2": t1 * rng.uniform(1.5, 100)})(
         rng.uniform(0.2, 10) * YEAR),
     [{"Ca": 0.02, "H": 5, "ep": 1.1, "t1": 1, "t2": 20}, {"Ca": 0.008, "H": 3, "ep": 0.8, "t1": 2, "t2": 30}],
     ["secondary compression settlement", "secondary compression", "creep settlement of clay"],
     conditions=["0 < C_alpha <= 0.2", "t2 > t1 (t1 = end of primary consolidation)"], invalid=[{"Ca": 0.02, "H": 5, "ep": 1.1, "t1": 20, "t2": 1}],
     method="S_s = C_alpha H/(1 + e_p) log10(t2/t1)", latex=r"S_s = \frac{C_\alpha H}{1 + e_p}\log_{10}\frac{t_2}{t_1}")
