//! Civil Engineering: soils, foundations, earth pressure, groundwater, hydrology and surveying, drawn for the
//! visitor's inputs. Charts over measured axes here; drawings to scale (walls, footings, slopes, phase diagrams,
//! curves in plan) in figures_geo2.rs. Every value drawn is an input as read, an output of this solve, or the output
//! of a further solve of the model (curves, profiles, maps); a figure that cannot be drawn is left out.

use crate::figs_common::{self as c, esc, qv};
use crate::figs_more::{self as m, dec, fx, Plot, Rg, X0, X1, Y0, Y1};
use ax_engine::figkit::{self as k, eq, num, q, Solved};
use ax_engine::figures::Figure;
use ax_engine::Input;

#[path = "figures_geo2.rs"]
mod geo2;

pub fn live(s: &Solved) -> Vec<Figure> {
    let v: Vec<Option<Figure>> = match s.key {
        // values over two inputs
        "darcy_flow_layered_soil" => vec![m::field_half(s, "field", "A", "dh", "Q")],
        "kirpich_time_of_concentration" => vec![m::field_half(s, "field", "L", "S", "tc_min")],
        "design_return_period_for_risk" => vec![risk_field(s, "R", "T")],
        "flood_exceedance_risk" => vec![risk_field(s, "T", "risk")],
        "scs_curve_number_runoff" => vec![scs_field(s)],
        "well_field_drawdown" => vec![well_map(s)],
        // fits of test data
        "cooper_jacob_analysis" => vec![cooper_jacob(s)],
        "direct_shear_strength_parameters" => vec![direct_shear(s)],
        "liquid_limit_flow_curve" => vec![flow_curve(s)],
        "weibull_plotting_positions" => vec![weibull(s)],
        // profiles
        "effective_stress_at_depth" => vec![stress_profile(s)],
        "excess_pore_pressure_isochrone" => vec![isochrones(s)],
        "theis_drawdown" => vec![theis_cone(s)],
        "thiem_confined_transmissivity" => vec![thiem_cone(s, false)],
        "thiem_unconfined_conductivity" => vec![thiem_cone(s, true)],
        "levelling_rise_and_fall" => vec![levels(s)],
        "vertical_parabolic_curve" => vec![vertical_curve(s)],
        // hydrology bars
        "muskingum_routing" => vec![hydrograph(s)],
        "phi_index" => vec![hyetograph(s)],
        // surveying plans
        "area_from_offsets" => vec![offsets(s)],
        "traverse_bowditch_adjustment" => vec![traverse(s)],
        "thiessen_polygon_rainfall" => vec![thiessen(s)],
        _ => vec![],
    };
    let mut out: Vec<Figure> = v.into_iter().flatten().collect();
    out.extend(geo2::live(s));
    out
}

// ------------------------------------------------------------------------------------------------ small helpers
pub(crate) fn path_d(pts: &[(f64, f64)]) -> String {
    let mut d = String::new();
    for (i, p) in pts.iter().enumerate() {
        if p.0.is_finite() && p.1.is_finite() {
            d.push_str(&format!("{}{:.1},{:.1} ", if i == 0 { "M" } else { "L" }, p.0, p.1));
        }
    }
    d.trim_end().to_string()
}

pub(crate) fn pline(out: &mut String, pts: &[(f64, f64)], class: &str, dash: bool) {
    if pts.len() >= 2 {
        out.push_str(&format!("<path class=\"{class}\"{} d=\"{}\"/>", if dash { " stroke-dasharray=\"5 4\"" } else { "" },
                              path_d(pts)));
    }
}

pub(crate) fn dot(out: &mut String, p: (f64, f64), class: &str, r: f64) {
    out.push_str(&format!("<circle class=\"{class}\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"{r}\"/>", p.0, p.1));
}

/// Text inside the 520-wide canvas (moved in from the edges), in a class.
pub(crate) fn txt(out: &mut String, x: f64, y: f64, anchor: &str, class: &str, t: &str) {
    let w = t.chars().count() as f64 * if class == "axp-mt" { 9.0 } else { 7.6 };
    let left = match anchor { "middle" => x - w / 2.0, "end" => x - w, _ => x };
    let shift = if left < 4.0 { 4.0 - left } else if left + w > 516.0 { 516.0 - (left + w) } else { 0.0 };
    out.push_str(&format!("<text class=\"{class}\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"{anchor}\">{}</text>", x + shift,
                          y.clamp(14.0, 336.0), esc(t)));
}

/// A legend row above a plot: (label, swatch class, line?) items from the left at y = 34.
pub(crate) fn legend(out: &mut String, items: &[(&str, &str, bool)]) {
    // one row at y = 34, or two (y = 16 and 34) when the items do not fit across
    let w = |t: &str| 30.0 + t.chars().count() as f64 * 7.6;
    let total: f64 = items.iter().map(|i| w(i.0)).sum();
    let mut rows: Vec<Vec<&(&str, &str, bool)>> = vec![vec![]];
    if X0 + total <= 516.0 {
        rows[0] = items.iter().collect();
    } else {
        let mut x = 40.0;
        rows.push(vec![]);
        for it in items {
            let first = rows[1].is_empty() && (rows[0].is_empty() || x + w(it.0) <= 516.0);
            if first {
                x += w(it.0);
                rows[0].push(it);
            } else {
                rows[1].push(it);
            }
        }
    }
    let n = rows.len();
    for (ri, row) in rows.iter().enumerate() {
        let y = if n == 1 { 34.0 } else { 16.0 + 18.0 * ri as f64 };
        let mut x = if n == 1 { X0 } else { 40.0 };
        for (t, class, line) in row.iter().map(|x| **x) {
            if line {
                out.push_str(&format!("<path class=\"{class}\" d=\"M{:.1},{y:.1} L{:.1},{y:.1}\"/>", x, x + 18.0));
            } else {
                out.push_str(&format!("<rect class=\"{class}\" x=\"{x:.1}\" y=\"{:.1}\" width=\"14\" height=\"11\"/>", y - 6.0));
            }
            out.push_str(&format!("<text class=\"axp-lt\" x=\"{:.1}\" y=\"{:.1}\">{}</text>", x + 22.0, y + 4.0, esc(t)));
            x += w(t);
        }
    }
}

fn kpa(x: f64) -> String {
    format!("{} kPa", num(x))
}

/// The range of values, padded by a fraction on each side.
fn padded(v: &[f64], f: f64) -> (f64, f64) {
    let lo = v.iter().cloned().fold(f64::INFINITY, f64::min);
    let hi = v.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let d = if hi > lo { (hi - lo) * f } else { lo.abs().max(1.0) * 0.1 };
    (lo - d, hi + d)
}

fn fl(x: f64) -> Input {
    Input::Float(x)
}

// ------------------------------------------------------------------------------------------------ maps over two inputs
/// A value over two inputs on explicit grids (the cells' values solved by the model), shaded (logarithmically when it
/// spans more than two decades), the request marked, `overlay` drawn on top with the pixel map.
#[allow(clippy::too_many_arguments)]
fn field_grid(s: &Solved, kind: &str, x: &str, xs: &[f64], logx: bool, y: &str, ys: &[f64], logy: bool, z: &str,
              mine: (f64, f64), set: &dyn Fn(f64, f64) -> Vec<(&'static str, f64)>,
              overlay: &dyn Fn(&dyn Fn(f64, f64) -> (f64, f64)) -> String, title: &str, intro: &str) -> Option<Figure> {
    let z0 = c::get(s, z)?;
    let (nx, ny) = (xs.len(), ys.len());
    if nx < 2 || ny < 2 || s.left() < nx * ny {
        return None;
    }
    let mut vals: Vec<Vec<Option<f64>>> = vec![vec![None; ny]; nx];
    let mut n_ok = 0;
    for (i, xv) in xs.iter().enumerate() {
        for (j, yv) in ys.iter().enumerate() {
            if let Some(v) = c::at(s, &set(*xv, *yv), z) {
                vals[i][j] = Some(v);
                n_ok += 1;
            }
        }
    }
    if n_ok < nx * ny / 3 {
        return None;
    }
    let all: Vec<f64> = vals.iter().flatten().flatten().copied().collect();
    let lo = all.iter().cloned().fold(f64::INFINITY, f64::min);
    let hi = all.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let logz = lo > 0.0 && hi / lo > 100.0;
    let shade: Vec<Vec<Option<f64>>> = if logz {
        vals.iter().map(|col| col.iter().map(|v| v.map(|w| w.log10())).collect()).collect()
    } else {
        vals.clone()
    };
    let zu = c::unit(s, z);
    let xlab = if x.contains('0') && x.len() <= 2 { format!("{} (m)", &x[..1]) } else { c::axis(s, x) };
    let ylab = if y.contains('0') && y.len() <= 2 { format!("{} (m)", &y[..1]) } else if y == "n" { "n, design life (years)".to_string() } else { c::axis(s, y) };
    let mut body = k::field(xs, ys, &shade, mine, &xlab, &ylab, &c::eqs(s, z, z0), (logx, logy));
    if body.is_empty() {
        return None;
    }
    if logz {
        // the scale's two ends are written as the real values (the shading is by their logarithm)
        body = k::relabel_scale(&body, &num(lo), &format!("{} (log scale)", num(hi)));
    }
    let rx: Rg = (xs[0], xs[nx - 1], logx);
    let ry: Rg = (ys[0], ys[ny - 1], logy);
    let pl = Plot::new(rx, ry);
    let levels: Vec<f64> = if logz {
        let (a, b) = (lo.log10().ceil() as i32, hi.log10().floor() as i32);
        (a..=b).map(|e| e as f64).filter(|e| *e > lo.log10() && *e < hi.log10()).take(8).collect()
    } else {
        c::nice_levels(lo, hi, 5)
    };
    let (slo, shi) = if logz { (lo.log10(), hi.log10()) } else { (lo, hi) };
    for lv0 in &levels {
        let lv = lv0 + (shi - slo) * 1e-9;
        let mut segs: Vec<((f64, f64), (f64, f64))> = vec![];
        for i in 0..nx - 1 {
            for j in 0..ny - 1 {
                let cs = [(i, j), (i + 1, j), (i + 1, j + 1), (i, j + 1)];
                let v: Vec<Option<f64>> = cs.iter().map(|(a, b)| shade[*a][*b]).collect();
                if v.iter().any(|w| w.is_none()) {
                    continue;
                }
                let v: Vec<f64> = v.into_iter().flatten().collect();
                let mut pts = vec![];
                for e in 0..4 {
                    let (a, b) = (e, (e + 1) % 4);
                    if (v[a] - lv) * (v[b] - lv) < 0.0 {
                        let t = (lv - v[a]) / (v[b] - v[a]);
                        let (pa, pb) = (pl.p(xs[cs[a].0], ys[cs[a].1]), pl.p(xs[cs[b].0], ys[cs[b].1]));
                        pts.push((pa.0 + t * (pb.0 - pa.0), pa.1 + t * (pb.1 - pa.1)));
                    }
                }
                if pts.len() == 2 {
                    segs.push((pts[0], pts[1]));
                } else if pts.len() == 4 {
                    segs.push((pts[0], pts[1]));
                    segs.push((pts[2], pts[3]));
                }
            }
        }
        if segs.is_empty() {
            continue;
        }
        let mut d = String::new();
        for (a, b) in &segs {
            d.push_str(&format!("M{:.1},{:.1} L{:.1},{:.1} ", a.0, a.1, b.0, b.1));
        }
        body.push_str(&format!("<path class=\"axp-c4\" stroke-width=\"1.2\" d=\"{}\"/>", d.trim_end()));
        let mm = segs[segs.len() / 2];
        let (lx, ly) = ((mm.0 .0 + mm.1 .0) / 2.0, (mm.0 .1 + mm.1 .1) / 2.0);
        let lab = if logz { num(10f64.powf(*lv0)) } else { num(*lv0) };
        body.push_str(&format!("<text class=\"axp-t\" font-size=\"11\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text>",
                               lx.clamp(X0 + 18.0, X1 - 18.0), (ly - 3.0).clamp(Y0 + 12.0, Y1 - 3.0), esc(&lab)));
    }
    body.push_str(&overlay(&|a, b| pl.p(a, b)));
    let (mx, my) = pl.p(mine.0, mine.1);
    let lab = qv(z0, &zu);
    let w = k::text_w(&lab, true) * 1.15;
    let tx = if mx + 12.0 + w < X1 { mx + 12.0 } else { (mx - 12.0 - w).max(X0 + 2.0) };
    body.push_str(&format!("<text class=\"axp-mt\" x=\"{tx:.1}\" y=\"{:.1}\">{}</text>", (my - 8.0).max(Y0 + 14.0), esc(&lab)));
    let body = c::fit_right(&body);
    let cap = format!("{intro} Each of the {nx} × {ny} cells is one solve of the model, shaded by its value (light {} to dark \
                       {}{}){}; iso-lines at {z} = {}. Marked: the given point, {}.",
                      qv(lo, &zu), qv(hi, &zu), if logz { ", logarithmic shading" } else { "" },
                      if n_ok < nx * ny { "; blank cells: inputs the model refuses" } else { "" },
                      levels.iter().map(|l| if logz { num(10f64.powf(*l)) } else { num(*l) }).collect::<Vec<_>>().join(", "),
                      c::eqs(s, z, z0));
    Some(s.figure(kind, title, &cap, &body, 340.0))
}

/// Whole design lives from 1 to 100 years (logarithmically spaced, no repeats), always holding the given one.
fn lives(n0: f64) -> Vec<f64> {
    let hi = (n0 * 2.0).max(100.0).round();
    let mut v: Vec<f64> = k::span(1.0, hi, 15, true).iter().map(|x| x.round()).collect();
    v.push(n0.round());
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v.dedup();
    v
}

/// The risk over a design life: `x` (an acceptable risk, or a return period) against the design life n.
fn risk_field(s: &Solved, x: &str, z: &str) -> Option<Figure> {
    let (x0, n0) = (c::inp(s, x)?, c::inp(s, "n")?);
    if !(x0 > 0.0) || !(n0 >= 1.0) {
        return None;
    }
    let (xs, logx) = if x == "R" {
        (k::span(0.02, 0.98, 21, false), false)
    } else {
        (k::span(x0 / 10.0, x0 * 10.0, 21, true), true)
    };
    let ns = lives(n0);
    let xn: &'static str = if x == "R" { "R" } else { "T" };
    let title = format!("{}: {} over {} and the design life", c::name(s), c::meaning(s, z), c::meaning(s, x));
    let intro = format!("{z} ({}) over {x} ({}) from {} to {} and n (design life, whole years) from {} to {} (logarithmic \
                         axes where marked).", c::meaning(s, z), c::meaning(s, x), num(xs[0]), num(xs[xs.len() - 1]),
                        num(ns[0]), num(ns[ns.len() - 1]));
    field_grid(s, "field", x, &xs, logx, "n", &ns, true, z, (x0, n0), &|a, b| vec![(xn, a), ("n", b)], &|_| String::new(),
               &title, &intro)
}

/// SCS runoff over the rainfall depth (a tenth to ten times, logarithmic) and the curve number (30 to 100).
fn scs_field(s: &Solved) -> Option<Figure> {
    let (p0, cn0) = (c::inp(s, "P")?, c::inp(s, "CN")?);
    if !(p0 > 0.0) {
        return None;
    }
    let xs = k::span(p0 / 10.0, p0 * 10.0, 21, true);
    let ys = k::span(30.0, 100.0, 15, false);
    let title = format!("{}: runoff depth over rainfall depth and curve number", c::name(s));
    let intro = format!("Q (runoff depth, mm) over P (storm rainfall depth) from {} to {} (logarithmic axis) and CN (curve \
                         number) from 30 to 100.", qv(xs[0], "m"), qv(xs[20], "m"));
    field_grid(s, "field", "P", &xs, true, "CN", &ys, false, "Q", (p0, cn0), &|a, b| vec![("P", a), ("CN", b)],
               &|_| String::new(), &title, &intro)
}

/// The superposed drawdown of a well field over the plane round the wells, every cell a solve with the point moved.
fn well_map(s: &Solved) -> Option<Figure> {
    let (wx, wy, qs) = (m::list(s, "wx")?, m::list(s, "wy")?, m::list(s, "Qs")?);
    let (x0, y0) = (c::inp(s, "x0")?, c::inp(s, "y0")?);
    if wx.len() != wy.len() || wx.len() != qs.len() {
        return None;
    }
    let mut ax: Vec<f64> = wx.clone();
    ax.push(x0);
    let mut ay: Vec<f64> = wy.clone();
    ay.push(y0);
    let span = {
        let (a, b) = padded(&ax, 0.0);
        let (c_, d) = padded(&ay, 0.0);
        (b - a).max(d - c_).max(10.0)
    };
    let (xl, xh) = padded(&ax, 0.0);
    let (yl, yh) = padded(&ay, 0.0);
    let pad = span * 0.35;
    // cell centres offset from the wells (a well's own position is a singular point)
    let xs = k::span(xl - pad + span * 0.013, xh + pad + span * 0.013, 21, false);
    let ys = k::span(yl - pad + span * 0.017, yh + pad + span * 0.017, 15, false);
    let wells: Vec<(f64, f64, f64)> = wx.iter().zip(&wy).zip(&qs).map(|((a, b), q_)| (*a, *b, *q_)).collect();
    let parts = s.out_list("parts").unwrap_or_default();
    let overlay = |p: &dyn Fn(f64, f64) -> (f64, f64)| -> String {
        let mut o = String::new();
        for (a, b, q_) in &wells {
            let pp = p(*a, *b);
            o.push_str(&format!("<circle class=\"axp-kd\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"5\"/>", pp.0, pp.1));
            txt(&mut o, pp.0, pp.1 + 18.0, "middle", "axp-lt", &format!("Q = {} m³/d", num(*q_)));
        }
        o
    };
    let title = "Well field: superposed drawdown map".to_string();
    let intro = format!("Drawdown round {} pumped wells (at x = [{}], y = [{}] m, pumping [{}] m³/day) by superposition, over x \
                         from {} to {} m and y from {} to {} m. At the given point the model returns s = {}{}.",
                        wells.len(), m::nums(&wx), m::nums(&wy), m::nums(&qs), num(xs[0]), num(xs[20]), num(ys[0]),
                        num(ys[14]), q(c::get(s, "s")?, "m"),
                        if parts.is_empty() { String::new() } else { format!(", the sum of the wells' parts {}", m::nums(&parts)) });
    field_grid(s, "map", "x0", &xs, false, "y0", &ys, false, "s", (x0, y0), &|a, b| vec![("x0", a), ("y0", b)],
               &overlay, &title, &intro)
}

// ------------------------------------------------------------------------------------------------ fits of test data
fn cooper_jacob(s: &Solved) -> Option<Figure> {
    let (t, d) = (m::list(s, "times")?, m::list(s, "drawdowns")?);
    let (ds, t0) = (s.out("ds_per_cycle")?, s.out("t0_min")?);
    let (tt, ss) = (s.out("T_m2_d")?, s.out("S")?);
    if t.len() != d.len() || !(t0 > 0.0) || t.iter().any(|x| *x <= 0.0) {
        return None;
    }
    let tmax = t.iter().cloned().fold(0.0, f64::max) * 2.0;
    let line: Vec<(f64, f64)> = k::span(t0, tmax, 30, true).into_iter().map(|x| (x, ds * (x / t0).log10())).collect();
    let pts: Vec<(f64, f64)> = t.iter().copied().zip(d.iter().copied()).collect();
    let series = vec![("fitted line (returned)".to_string(), line), ("observations".to_string(), m::dots(&pts))];
    let cap = format!("Cooper-Jacob analysis of a pumping test (Q = {}, observation well at r = {}): drawdowns [{}] m at [{}] \
                       min on a semi-log plot, and the straight line with the returned slope {} m per log cycle and zero-drawdown \
                       intercept t0 = {} min. Returned: transmissivity {} m²/day, storativity {}.",
                      qv(c::inp(s, "Q")?, "m^3/s"), q(c::inp(s, "r")?, "m"), m::nums(&d), m::nums(&t), num(ds), num(t0),
                      num(tt), num(ss));
    m::chart(s, "fit", "Cooper-Jacob straight line of a pumping test", &cap, series, "t, time since pumping began (min)",
             "s, drawdown (m)", &format!("T = {} m²/day, S = {}", num(tt), num(ss)), Some((t0, 0.0)),
             &format!("t0 = {} min", num(t0)), true)
}

fn direct_shear(s: &Solved) -> Option<Figure> {
    let (sg, tau) = (m::list(s, "sigma")?, m::list(s, "tau")?);
    let (cc, tp, phi) = (s.out("c")?, s.out("tan_phi")?, s.out("phi")?);
    if sg.len() != tau.len() {
        return None;
    }
    let smax = sg.iter().cloned().fold(0.0, f64::max) * 1.15;
    let line = vec![(0.0, cc), (smax, cc + smax * tp)];
    let pts: Vec<(f64, f64)> = sg.iter().copied().zip(tau.iter().copied()).collect();
    let series = vec![("envelope (returned)".to_string(), line), ("tests (input)".to_string(), m::dots(&pts))];
    let cap = format!("Direct shear tests (σ = [{}] kPa, τ at failure = [{}] kPa) and the straight strength envelope from the \
                       returned c = {} kPa and tan φ = {} (φ = {}°).", m::nums(&sg), m::nums(&tau), num(cc), num(tp), num(phi));
    m::chart(s, "fit", "Direct shear tests and the strength envelope", &cap, series, "σ, normal stress (kPa)",
             "τ, shear stress at failure (kPa)", &format!("c = {} kPa, φ = {}°", num(cc), num(phi)), Some((0.0, cc)),
             &format!("c = {} kPa", num(cc)), false)
}

fn flow_curve(s: &Solved) -> Option<Figure> {
    let (nb, w) = (m::list(s, "blows")?, m::list(s, "w")?);
    let (ll, fi) = (s.out("LL")?, s.out("flow_index")?);
    if nb.len() != w.len() || nb.iter().any(|x| *x <= 0.0) {
        return None;
    }
    let (lo, hi) = (nb.iter().cloned().fold(f64::INFINITY, f64::min).min(25.0) * 0.8,
                    nb.iter().cloned().fold(0.0, f64::max).max(25.0) * 1.25);
    // the flow line: the returned liquid limit at 25 blows, falling by the returned flow index per log cycle
    let line: Vec<(f64, f64)> = k::span(lo, hi, 30, true).into_iter().map(|x| (x, ll - fi * (x / 25.0).log10())).collect();
    let pts: Vec<(f64, f64)> = nb.iter().copied().zip(w.iter().copied()).collect();
    let series = vec![("flow line (returned)".to_string(), line), ("cup tests (input)".to_string(), m::dots(&pts))];
    let pl = s.out("PI").zip(c::inp(s, "PL"));
    let head = match pl { Some((pi, plv)) => format!("PL = {} %, PI = {} %", num(plv), num(pi)), None => format!("LL = {} %", num(ll)) };
    let cap = format!("Casagrande cup tests ([{}] blows at water contents [{}] %) and the flow line (straight on a logarithmic \
                       axis of blows) with the returned flow index {} % per log cycle, through the returned liquid limit LL = {} % \
                       at 25 blows{}.", m::nums(&nb), m::nums(&w), num(fi), num(ll),
                      match pl { Some((pi, plv)) => format!("; with PL = {} %, the plasticity index PI = {} %", num(plv), num(pi)), None => String::new() });
    m::chart(s, "fit", "Liquid limit from the flow curve", &cap, series, "number of blows", "water content (%)",
             &head, Some((25.0, ll)), &format!("LL = {} %", num(ll)), false)
}

fn weibull(s: &Solved) -> Option<Figure> {
    let (rk, tt, pp) = (s.out_list("ranked")?, s.out_list("T")?, s.out_list("P")?);
    if rk.len() != tt.len() || rk.len() != pp.len() || rk.len() < 2 || tt.iter().any(|x| *x <= 0.0) {
        return None;
    }
    let tmax = tt.iter().cloned().fold(0.0, f64::max);
    let tmin = tt.iter().cloned().fold(f64::INFINITY, f64::min);
    let pl = Plot::new((tmin.min(1.0) * 0.9, (tmax * 1.6).max(11.0), true), {
        let (a, b) = padded(&rk, 0.12);
        (a, b, false)
    });
    let mut out = String::new();
    pl.axes(&mut out, "T, return period (years, logarithmic)", "annual peak", &format!("{} annual peaks, Weibull T = (n + 1)/m", rk.len()));
    for (i, ((v, t), p)) in rk.iter().zip(&tt).zip(&pp).enumerate() {
        let pt = pl.p(*t, *v);
        dot(&mut out, pt, "axp-d", 5.0);
        if rk.len() <= 12 || i % 2 == 0 {
            txt(&mut out, pt.0 + 9.0, pt.1 + 4.0, "start", "axp-vt", &format!("P = {}", num(*p)));
        }
    }
    let cap = format!("Annual peaks [{}] ranked from the largest and plotted at the return periods the model returned (Weibull, \
                       T = (n + 1)/m): T = {} years, exceedance probabilities {}.", m::nums(&c_vec(s, "peaks")),
                      m::nums(&tt), m::nums(&pp));
    Some(s.chart("frequency", "Weibull plotting positions of annual peaks", &cap, &out))
}

/// A large number written whole (12345 m², not 1.234e+04).
fn big(x: f64) -> String {
    if x.abs() >= 1e3 && x.abs() < 1e9 { format!("{:.0}", x) } else { num(x) }
}

/// The frame's tick labels under the horizontal axis (text at y = `y`) and its vertical grid lines removed.
fn strip_x_ticks(svg: &str, y: f64) -> String {
    let key = format!(" y=\"{:.1}\" text-anchor=\"middle\"", y);
    let mut out = String::new();
    for part in svg.split_inclusive("/>").flat_map(|p| p.split_inclusive("</text>")) {
        let is_tick = part.contains("<text class=\"axp-t\"") && part.contains(&key);
        let is_vgrid = part.contains("<line class=\"axp-g\"") && {
            let x1 = part.split(" x1=\"").nth(1).and_then(|v| v.split('"').next());
            let x2 = part.split(" x2=\"").nth(1).and_then(|v| v.split('"').next());
            x1.is_some() && x1 == x2
        };
        if !is_tick && !is_vgrid {
            out.push_str(part);
        }
    }
    out
}

fn c_vec(s: &Solved, sym: &str) -> Vec<f64> {
    m::list(s, sym).unwrap_or_default()
}

// ------------------------------------------------------------------------------------------------ profiles
fn stress_profile(s: &Solved) -> Option<Figure> {
    let (hs, gs) = (m::list(s, "H")?, m::list(s, "gam")?);
    let (zw, z0) = (c::inp(s, "zw")?, c::inp(s, "z")?);
    let (sg0, u0, se0) = (s.out("sigma")?, s.out("u")?, s.out("sigma_eff")?);
    let total: f64 = hs.iter().sum();
    let zmax = total.max(z0);
    if !(zmax > 0.0) || s.left() < 61 {
        return None;
    }
    let mut a = vec![];
    let mut b = vec![];
    let mut e = vec![];
    for zz in k::span(0.0, zmax, 61, false) {
        if let Some(ans) = s.with(&[("z", zz)]) {
            if let (Some(x1), Some(x2), Some(x3)) = (k::num_of(&ans, "sigma"), k::num_of(&ans, "u"), k::num_of(&ans, "sigma_eff")) {
                a.push((x1 / 1e3, -zz));
                b.push((x2 / 1e3, -zz));
                e.push((x3 / 1e3, -zz));
            }
        }
    }
    if a.len() < 10 {
        return None;
    }
    let xmax = a.iter().map(|p| p.0).fold(0.0, f64::max).max(1e-9);
    let pl = Plot::new((0.0, xmax * 1.08, false), (-zmax * 1.04, 0.0 + zmax * 0.02, false));
    let mut out = String::new();
    legend(&mut out, &[("total σ", "axp-c", true), ("pore pressure u", "axp-c3", true), ("effective σ′", "axp-c2", true)]);
    pl.axes(&mut out, "stress (kPa)", "depth below the surface (m)", "");
    // layers, shaded alternately, with their unit weights
    let mut top = 0.0;
    for (i, (h, g)) in hs.iter().zip(&gs).enumerate() {
        let (p1, p2) = (pl.p(0.0, -top), pl.p(xmax * 1.08, -(top + h)));
        if i % 2 == 0 {
            out.push_str(&format!("<rect class=\"axp-ka\" x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\"/>", p1.0, p1.1,
                                  p2.0 - p1.0, p2.1 - p1.1));
        }
        txt(&mut out, X1 - 8.0, (p1.1 + p2.1) / 2.0 + 4.0, "end", "axp-lt", &format!("γ = {} kN/m³", num(*g)));
        top += h;
    }
    let yw = pl.py(-zw);
    if zw <= zmax {
        out.push_str(&format!("<path class=\"axp-c3\" stroke-dasharray=\"6 4\" stroke-width=\"1.4\" d=\"M{X0},{yw:.1} L{X1},{yw:.1}\"/>"));
        txt(&mut out, X1 - 8.0, yw - 5.0, "end", "axp-lt", &format!("water table {}", q(zw, "m")));
    }
    let pa: Vec<(f64, f64)> = a.iter().map(|p| pl.p(p.0, p.1)).collect();
    let pb: Vec<(f64, f64)> = b.iter().map(|p| pl.p(p.0, p.1)).collect();
    let pe: Vec<(f64, f64)> = e.iter().map(|p| pl.p(p.0, p.1)).collect();
    pline(&mut out, &pa, "axp-c", false);
    pline(&mut out, &pb, "axp-c3", false);
    pline(&mut out, &pe, "axp-c2", false);
    let mk = pl.p(se0 / 1e3, -z0);
    dot(&mut out, mk, "axp-m", 6.0);
    let right = mk.0 < 330.0;
    txt(&mut out, if right { mk.0 + 10.0 } else { mk.0 - 10.0 }, mk.1 - 8.0, if right { "start" } else { "end" }, "axp-mt",
        &format!("σ′ = {} at {}", kpa(se0 / 1e3), q(z0, "m")));
    let cap = format!("Vertical stresses with depth in layered soil (layers [{}] m thick, unit weights [{}] kN/m³, water table at \
                       {}): total stress, pore-water pressure and effective stress at {} depths, each a solve of the model. Marked: \
                       at z = {} the returned σ = {}, u = {}, σ′ = {}.", m::nums(&hs), m::nums(&gs), q(zw, "m"), a.len(),
                      q(z0, "m"), kpa(sg0 / 1e3), kpa(u0 / 1e3), kpa(se0 / 1e3));
    Some(s.chart("profile", "Total, pore and effective stress with depth", &cap, &out))
}

fn isochrones(s: &Solved) -> Option<Figure> {
    let (u0, t0, hd, z0, uu) = (c::inp(s, "u0")?, c::inp(s, "t")?, c::inp(s, "Hdr")?, c::inp(s, "z")?, s.out("u")?);
    if !(hd > 0.0) || !(t0 > 0.0) {
        return None;
    }
    let zs = k::span(0.0, 2.0 * hd, 41, false);
    let mut series = vec![];
    let yr = 365.25 * 86400.0;
    for f in [0.25, 1.0, 4.0] {
        let pts: Vec<(f64, f64)> = zs.iter().filter_map(|zz| c::at(s, &[("z", *zz), ("t", t0 * f)], "u").map(|v| (v / 1e3, -zz))).collect();
        if pts.len() >= 2 {
            let tl = if t0 * f >= yr * 0.1 { format!("t = {} yr", num(t0 * f / yr)) } else { format!("t = {} d", num(t0 * f / 86400.0)) };
            series.push((tl, pts));
        }
    }
    if series.is_empty() {
        return None;
    }
    series.push(("initial u0".to_string(), vec![(u0 / 1e3, 0.0), (u0 / 1e3, -2.0 * hd)]));
    let cap = format!("Isochrones of excess pore pressure in a doubly drained clay layer of thickness {} (cv = {} m²/s, u0 = {}) at a \
                       quarter of, at, and at four times the given time t = {} s; 41 depths each, every point a solve of the model. \
                       Marked: at z = {} the returned u = {}, local consolidation {}, time factor Tv = {}.",
                      q(2.0 * hd, "m"), num(c::inp(s, "cv")?), kpa(u0 / 1e3), num(t0), q(z0, "m"), kpa(uu / 1e3),
                      num(s.out("Uz").unwrap_or(f64::NAN)), num(s.out("Tv").unwrap_or(f64::NAN)));
    let body = k::lines_y(&series, "u, excess pore pressure (kPa)", "z, depth below the top face (m)",
                          &format!("u = {} at z = {}", kpa(uu / 1e3), q(z0, "m")), Some((uu / 1e3, -z0)),
                          &format!("u = {}", kpa(uu / 1e3)), false, true);
    Some(s.chart("profile", "Consolidation isochrones in a clay layer", &cap, &c::fit_right(&body)))
}

fn theis_cone(s: &Solved) -> Option<Figure> {
    let (r0, t0, s0) = (c::inp(s, "r")?, c::inp(s, "t")?, s.out("s")?);
    if !(r0 > 0.0) || !(t0 > 0.0) {
        return None;
    }
    let rmin = (r0 / 20.0).max(1e-3);
    let rmax = r0 * 10.0;
    let rs = k::span(rmin, rmax, 25, true);
    let mut series = vec![];
    let lab = |tt: f64| if tt >= 86400.0 { format!("t = {} d", num(tt / 86400.0)) } else { format!("t = {} h", num(tt / 3600.0)) };
    for f in [1.0, 10.0, 100.0] {
        let right: Vec<(f64, f64)> = rs.iter().filter_map(|r| c::at(s, &[("r", *r), ("t", t0 * f)], "s").map(|v| (*r, -v))).collect();
        if right.len() < 3 {
            continue;
        }
        let mut pts: Vec<(f64, f64)> = right.iter().rev().map(|p| (-p.0, p.1)).collect();
        pts.push((f64::NAN, f64::NAN));
        pts.extend(right.iter().copied());
        series.push((lab(t0 * f), pts));
    }
    if series.is_empty() {
        return None;
    }
    series.push(("initial level".to_string(), vec![(-rmax, 0.0), (rmax, 0.0)]));
    let cap = format!("Drawdown cone round a well pumping Q = {} from a confined aquifer (T = {} m²/s, S = {}) at the given time and \
                       10 and 100 times it, drawdown solved by the model at 25 distances from {} to {} (shown on both sides of \
                       the well). Marked: r = {}, t = {} s, s = {}.", qv(c::inp(s, "Q")?, "m^3/s"), num(c::inp(s, "T")?),
                      num(c::inp(s, "S")?), q(rmin, "m"), q(rmax, "m"), q(r0, "m"), num(t0), q(s0, "m"));
    let body = k::lines_y(&series, "distance from the well (m)", "drawdown (m)",
                          &format!("s = {} at r = {}", q(s0, "m"), q(r0, "m")), Some((r0, -s0)), &q(s0, "m"), false, true);
    Some(s.chart("profile", "Drawdown cone round a pumped well", &cap, &c::fit_right(&body)))
}

/// The piezometric level (or water-table height) at distance r that gives the returned transmissivity (or
/// conductivity) when it stands in for one observation: found by the secant method on solves of the model.
fn thiem_level(s: &Solved, unconf: bool, r: f64, target: f64) -> Option<f64> {
    let (r1, r2) = (c::inp(s, "r1")?, c::inp(s, "r2")?);
    let (hs1, hs2) = if unconf { ("h1", "h2") } else { ("s1", "s2") };
    let (v1, v2) = (c::inp(s, hs1)?, c::inp(s, hs2)?);
    let out = if unconf { "K" } else { "T" };
    // the observation replaced: the nearer one inside r1's half, the farther one otherwise
    let near = r < (r1 * r2).sqrt();
    let (rk, vk, ok) = if near { ("r1", hs1, (r2, v2, "r2", hs2)) } else { ("r2", hs2, (r1, v1, "r1", hs1)) };
    if (near && r >= r2) || (!near && r <= r1) {
        return None;
    }
    let f = |v: f64| -> Option<f64> {
        let a = s.with(&[(rk, r), (vk, v), (ok.2, ok.0), (ok.3, ok.1)])?;
        k::num_of(&a, out).map(|x| 1.0 / x - 1.0 / target)
    };
    let mut a = if near { v1 } else { v2 };
    let mut b = a + (v1 - v2).abs().max(1e-3) * if near == unconf { -0.3 } else { 0.3 };
    let (mut fa, mut fb) = (f(a)?, f(b)?);
    for _ in 0..6 {
        if (fb - fa).abs() < 1e-300 {
            break;
        }
        let c_ = b - fb * (b - a) / (fb - fa);
        a = b;
        fa = fb;
        b = c_;
        fb = f(b)?;
        if fb.abs() < 1e-9 / target.abs().max(1e-300) {
            break;
        }
    }
    (fb.abs() * target.abs() < 1e-6).then_some(b)
}

fn thiem_cone(s: &Solved, unconf: bool) -> Option<Figure> {
    let (r1, r2) = (c::inp(s, "r1")?, c::inp(s, "r2")?);
    let (hs1, hs2) = if unconf { ("h1", "h2") } else { ("s1", "s2") };
    let (v1, v2) = (c::inp(s, hs1)?, c::inp(s, hs2)?);
    let target = s.out(if unconf { "K" } else { "T" })?;
    if !(r1 > 0.0 && r2 > r1) {
        return None;
    }
    let rs = k::span(r1 / 8.0, r2 * 1.5, 16, true);
    let mut right = vec![];
    for r in rs {
        if let Some(v) = thiem_level(s, unconf, r, target) {
            right.push((r, if unconf { v } else { -v }));
        }
    }
    if right.len() < 5 {
        return None;
    }
    let mut cone: Vec<(f64, f64)> = right.iter().rev().map(|p| (-p.0, p.1)).collect();
    cone.push((f64::NAN, f64::NAN));
    cone.extend(right.iter().copied());
    let obs = if unconf { vec![(r1, v1), (r2, v2)] } else { vec![(r1, -v1), (r2, -v2)] };
    let (head, mlab) = if unconf {
        (format!("K = {} m/day", num(s.out("K_m_d").unwrap_or(target * 86400.0))), format!("r = {}", q(r1, "m")))
    } else {
        (format!("T = {} m²/day", num(s.out("T_m2_d").unwrap_or(target * 86400.0))), format!("r = {}", q(r1, "m")))
    };
    let mut series = vec![("cone (model solves)".to_string(), cone), ("observations".to_string(), m::dots(&obs))];
    if !unconf {
        series.push(("initial level".to_string(), vec![(-r2 * 1.5, 0.0), (r2 * 1.5, 0.0)]));
    }
    let what = if unconf { "water-table height" } else { "drawdown" };
    let cap = format!("Steady radial flow to a well pumping Q = {}, observed at r1 = {} and r2 = {} ({}s {} and {}, the observation \
                       points marked). Each point of the cone is the {what} that, put in place of one observation, makes the model \
                       return the same {} ({}); drawn on both sides of the well.", qv(c::inp(s, "Q")?, "m^3/s"), q(r1, "m"),
                      q(r2, "m"), what, q(v1, "m"), q(v2, "m"), if unconf { "hydraulic conductivity" } else { "transmissivity" },
                      head);
    let ylab = if unconf { "water-table height (m)" } else { "level below the initial (m)" };
    let title = if unconf { "Thiem unconfined aquifer: the drawdown cone" } else { "Thiem confined aquifer: the drawdown cone" };
    m::chart(s, "profile", title, &cap, series, "r, distance from the pumped well (m)", ylab, &head, Some(obs[0]), &mlab, false)
}

fn levels(s: &Solved) -> Option<Figure> {
    let rl = s.out_list("RL")?;
    if rl.len() < 2 {
        return None;
    }
    let (rise, fall) = (s.out("rise")?, s.out("fall")?);
    let n = rl.len();
    let pl = Plot::new((0.6, n as f64 + 0.4, false), {
        let (a, b) = padded(&rl, 0.15);
        (a, b, false)
    });
    let mut out = String::new();
    pl.axes(&mut out, "station (in order)", "reduced level (m)", &format!("rise {} m, fall {} m", num(rise), num(fall)));
    // whole stations only on the horizontal axis: the frame's own ticks replaced
    out = strip_x_ticks(&out, pl.y1 + 20.0);
    let step = ((n as f64) / 10.0).ceil().max(1.0) as usize;
    for i in (1..=n).step_by(step) {
        let x = pl.px(i as f64);
        out.push_str(&format!("<text class=\"axp-t\" x=\"{x:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{i}</text>", pl.y1 + 20.0));
    }
    let pts: Vec<(f64, f64)> = rl.iter().enumerate().map(|(i, v)| pl.p(i as f64 + 1.0, *v)).collect();
    pline(&mut out, &pts, "axp-c", false);
    for (i, p) in pts.iter().enumerate() {
        dot(&mut out, *p, "axp-d", 4.5);
        if n <= 16 {
            let up = i == 0 || rl[i] >= rl[i - 1];
            txt(&mut out, p.0, if up { p.1 - 9.0 } else { p.1 + 18.0 }, "middle", "axp-vt", &num(rl[i]));
        }
    }
    let rd = c_vec(s, "readings");
    let cap = format!("Reduced levels of a levelling run (staff readings [{}] m, sights {}, benchmark RL {}) by rise and fall: the \
                       reduced level of every station as returned ({}). Marked: total rise {} m, total fall {} m, ΣBS − ΣFS = {} m.",
                      m::nums(&rd), s.txt("sights").unwrap_or_default(), q(c::inp(s, "RL0")?, "m"), m::nums(&rl), num(rise),
                      num(fall), num(s.out("check").unwrap_or(f64::NAN)));
    Some(s.chart("profile", "Levelling run: reduced levels", &cap, &out))
}

fn vertical_curve(s: &Solved) -> Option<Figure> {
    let (g1, g2, l, pvi, ev, x0) = (c::inp(s, "g1")?, c::inp(s, "g2")?, c::inp(s, "L")?, c::inp(s, "PVI")?,
                                    c::inp(s, "elev_PVI")?, c::inp(s, "x")?);
    let (e0, bvc, ebvc) = (s.out("elev")?, s.out("BVC")?, s.out("elev_BVC")?);
    if !(l > 0.0) {
        return None;
    }
    let evc = bvc + l;
    let mut curve = vec![];
    for xx in k::span(bvc, evc, 41, false) {
        if let Some(v) = c::at(s, &[("x", xx)], "elev") {
            curve.push((xx, v));
        }
    }
    if curve.len() < 10 {
        return None;
    }
    let ext = l * 0.35;
    // the tangents through the intersection point at the given grades
    let tan = vec![(bvc - ext, ev - g1 * (pvi - bvc + ext)), (pvi, ev), (evc + ext, ev + g2 * (evc + ext - pvi))];
    let mut ys: Vec<f64> = curve.iter().map(|p| p.1).collect();
    ys.extend(tan.iter().map(|p| p.1));
    let (ylo, yhi) = padded(&ys, 0.08);
    let pl = Plot::new((bvc - ext, evc + ext, false), (ylo, yhi, false));
    let mut out = String::new();
    legend(&mut out, &[(&format!("tangents g1 = {}, g2 = {}", num(g1), num(g2)), "axp-c4", true),
                       ("vertical curve (model solves)", "axp-c", true)]);
    pl.axes(&mut out, "station (m)", "elevation (m)", "");
    let tp: Vec<(f64, f64)> = tan.iter().map(|p| pl.p(p.0, p.1)).collect();
    pline(&mut out, &tp, "axp-c4", true);
    let cp: Vec<(f64, f64)> = curve.iter().map(|p| pl.p(p.0, p.1)).collect();
    pline(&mut out, &cp, "axp-c", false);
    for (name, xx, yy, below) in [("BVC", bvc, ebvc, true), ("PVI", pvi, ev, false), ("EVC", evc, curve[curve.len() - 1].1, true)] {
        let p = pl.p(xx, yy);
        dot(&mut out, p, "axp-kd", 4.0);
        txt(&mut out, p.0, if below { p.1 + 18.0 } else { p.1 - 8.0 }, "middle", "axp-lt", name);
    }
    if let (Some(xt), Some(et)) = (s.out("x_turn"), s.out("elev_turn")) {
        if xt >= bvc && xt <= evc {
            let p = pl.p(xt, et);
            dot(&mut out, p, "axp-d2", 5.0);
            txt(&mut out, p.0, p.1 - 12.0, "middle", "axp-vt", &format!("turn {} m, {} m", num(xt), num(et)));
        }
    }
    let mk = pl.p(x0, e0);
    dot(&mut out, mk, "axp-m", 6.0);
    txt(&mut out, mk.0 + 10.0, mk.1 + 18.0, "start", "axp-mt", &format!("{} m", num(e0)));
    let cap = format!("Equal-tangent parabolic vertical curve, L = {}, PVI at station {} m and elevation {} m, grades g1 = {}, g2 = {}: \
                       {} elevations along the curve, each a solve of the model, over the two tangents. Marked: the returned BVC {} m \
                       (elevation {} m){} and the elevation {} m at the given station {} m.", q(l, "m"), num(pvi), num(ev), num(g1),
                      num(g2), curve.len(), num(bvc), num(ebvc),
                      match (s.out("x_turn"), s.out("elev_turn")) { (Some(a), Some(b)) => format!(", the turning point {} m at {} m", num(a), num(b)), _ => String::new() },
                      num(e0), num(x0));
    Some(s.chart("profile", "Vertical curve: profile, BVC, PVI and turning point", &cap, &out))
}

// ------------------------------------------------------------------------------------------------ hydrology bars
fn hydrograph(s: &Solved) -> Option<Figure> {
    let (inf, outf) = (m::list(s, "inflow")?, s.out_list("outflow")?);
    let dt = c::inp(s, "dt")?;
    if inf.len() != outf.len() || inf.len() < 2 || !(dt > 0.0) {
        return None;
    }
    let h = dt / 3600.0;
    let n = inf.len();
    let ymax = inf.iter().chain(&outf).cloned().fold(0.0, f64::max).max(1e-9);
    let pl = Plot::new((-0.6 * h, (n as f64 - 0.4) * h, false), (0.0, ymax * 1.12, false));
    let mut out = String::new();
    legend(&mut out, &[("inflow (input)", "axp-b4", false), ("routed outflow (returned)", "axp-c", true)]);
    pl.axes(&mut out, "t, time (h)", "Q, discharge (m³/s)", "");
    let bw = (pl.px(h) - pl.px(0.0)) * 0.6;
    for (i, v) in inf.iter().enumerate() {
        let (x, y) = pl.p(i as f64 * h, *v);
        out.push_str(&format!("<rect class=\"axp-b4\" x=\"{:.1}\" y=\"{y:.1}\" width=\"{bw:.1}\" height=\"{:.1}\"/>", x - bw / 2.0,
                              (pl.py(0.0) - y).max(0.0)));
    }
    let mut ip = 0;
    for (i, v) in outf.iter().enumerate() {
        if *v > outf[ip] {
            ip = i;
        }
        let (x, y) = pl.p(i as f64 * h, *v);
        out.push_str(&format!("<path class=\"axp-c\" d=\"M{x:.1},{:.1} L{x:.1},{y:.1}\"/>", pl.py(0.0)));
        dot(&mut out, (x, y), "axp-d", 3.5);
    }
    let peak = s.out("peak_outflow").unwrap_or(outf[ip]);
    let pk = pl.p(ip as f64 * h, outf[ip]);
    dot(&mut out, pk, "axp-m", 6.0);
    txt(&mut out, pk.0 + 10.0, pk.1 - 8.0, "start", "axp-mt", &format!("peak {} m³/s", num(peak)));
    let cap = format!("A flood wave routed through a river reach by the Muskingum method (K = {} h, X = {}, Δt = {} h; C0 = {}, C1 = {}, \
                       C2 = {}): inflow per interval as bars, the outflow returned by the model as stems, lower and later; peak inflow \
                       {} m³/s, marked: peak outflow at t = {} h, Q = {} m³/s (attenuation {} m³/s).",
                      num(c::inp(s, "K")? / 3600.0), num(c::inp(s, "X")?), num(h), num(s.out("C0").unwrap_or(f64::NAN)),
                      num(s.out("C1").unwrap_or(f64::NAN)), num(s.out("C2").unwrap_or(f64::NAN)),
                      num(inf.iter().cloned().fold(0.0, f64::max)), num(ip as f64 * h), num(peak),
                      num(s.out("attenuation").unwrap_or(f64::NAN)));
    Some(s.chart("hydrograph", "Flood routing: inflow and outflow hydrographs", &cap, &out))
}

fn hyetograph(s: &Solved) -> Option<Figure> {
    let (rain, ex) = (m::list(s, "rain")?, s.out_list("excess")?);
    let (dt, phi) = (c::inp(s, "dt")?, s.out("phi")?);
    if rain.len() != ex.len() || !(dt > 0.0) || rain.is_empty() {
        return None;
    }
    let h = dt / 3600.0;
    let ri: Vec<f64> = rain.iter().map(|r| r / h).collect();
    let ei: Vec<f64> = ex.iter().map(|r| r / h).collect();
    let n = rain.len();
    let ymax = ri.iter().cloned().fold(phi, f64::max).max(1e-9);
    let pl = Plot::new((0.5, n as f64 + 0.5, false), (0.0, ymax * 1.12, false));
    let mut out = String::new();
    legend(&mut out, &[("rainfall (input)", "axp-b4", false), ("excess rainfall (returned)", "axp-b", false)]);
    pl.axes(&mut out, "interval", "rainfall intensity (mm/h)", "");
    let bw = (pl.px(1.0) - pl.px(0.0)) * 0.8;
    for i in 0..n {
        for (v, class) in [(ri[i], "axp-b4"), (ei[i], "axp-b")] {
            let (x, y) = pl.p(i as f64 + 1.0, v);
            out.push_str(&format!("<rect class=\"{class}\" x=\"{:.1}\" y=\"{y:.1}\" width=\"{bw:.1}\" height=\"{:.1}\"/>",
                                  x - bw / 2.0, (pl.py(0.0) - y).max(0.0)));
        }
    }
    let yp = pl.py(phi);
    out.push_str(&format!("<path class=\"axp-c3\" d=\"M{X0},{yp:.1} L{X1},{yp:.1}\"/>"));
    txt(&mut out, X1 - 6.0, yp - 6.0, "end", "axp-mt", &format!("φ = {} mm/h", num(phi)));
    let cap = format!("Hyetograph of a storm (rain [{}] mm per interval of {} h) and the returned φ-index {} mm/h: the constant loss \
                       that leaves the observed runoff of {} mm. Marked: the returned excess rainfall of each interval ({} mm), from \
                       {} contributing intervals.", m::nums(&rain), num(h), num(phi), num(c::inp(s, "R")? * 1e3), m::nums(&ex),
                      num(s.out("contributing").unwrap_or(f64::NAN)));
    let _ = Y0;
    Some(s.chart("hyetograph", "φ-index: losses and excess rainfall", &cap, &out))
}

// ------------------------------------------------------------------------------------------------ surveying plans
fn offsets(s: &Solved) -> Option<Figure> {
    let (o, d) = (m::list(s, "offsets")?, c::inp(s, "d")?);
    let (at, asimp) = (s.out("A_trapezoidal")?, s.out("A_simpson"));
    if o.len() < 2 || !(d > 0.0) {
        return None;
    }
    let n = o.len();
    let xmax = d * (n - 1) as f64;
    let ymax = o.iter().cloned().fold(0.0, f64::max);
    let ymin = o.iter().cloned().fold(0.0, f64::min);
    let pl = Plot::new((0.0, xmax, false), (ymin, ymax * 1.18 + 1e-9, false));
    let mut out = String::new();
    legend(&mut out, &[("area enclosed by the offsets", "axp-s", false)]);
    pl.axes(&mut out, "chainage along the base line (m)", "offset (m)", "");
    let pts: Vec<(f64, f64)> = o.iter().enumerate().map(|(i, v)| pl.p(i as f64 * d, *v)).collect();
    let mut poly = vec![pl.p(0.0, 0.0)];
    poly.extend(pts.iter().copied());
    poly.push(pl.p(xmax, 0.0));
    out.push_str(&format!("<path class=\"axp-s\" d=\"{} Z\"/>", path_d(&poly)));
    for (i, p) in pts.iter().enumerate() {
        out.push_str(&format!("<path class=\"axp-c4\" d=\"M{:.1},{:.1} L{:.1},{:.1}\"/>", p.0, pl.py(0.0), p.0, p.1));
        if n <= 21 {
            txt(&mut out, p.0, p.1 - 7.0, "middle", "axp-vt", &num(o[i]));
        }
    }
    pline(&mut out, &pts, "axp-c", false);
    txt(&mut out, X1 - 4.0, 20.0, "end", "axp-mt", &format!("trapezoidal {} m²", num(at)));
    if let Some(a2) = asimp {
        txt(&mut out, X1 - 4.0, 38.0, "end", "axp-vt", &format!("Simpson {} m²", num(a2)));
    }
    let cap = format!("Offsets [{}] m taken every {} from a base line, joined by straight lines (the trapezoidal reading). Marked: the \
                       returned areas by the trapezoidal rule {} m²{}.", m::nums(&o), q(d, "m"), num(at),
                      asimp.map(|a| format!(" and Simpson's rule {} m²", num(a))).unwrap_or_default());
    Some(s.chart("diagram", "Area from offsets: trapezoidal and Simpson", &cap, &out))
}

fn traverse(s: &Solved) -> Option<Figure> {
    let (e, n) = (s.out_list("E")?, s.out_list("N")?);
    if e.len() != n.len() || e.len() < 3 {
        return None;
    }
    let (xl, xh) = padded(&e, 0.12);
    let (yl, yh) = padded(&n, 0.12);
    let pl = Plot::equal(xl, xh, yl, yh);
    let mut out = String::new();
    pl.axes(&mut out, "E, adjusted easting (m)", "N, adjusted northing (m)", &format!("area {} m²", big(s.out("area").unwrap_or(f64::NAN))));
    let mut pts: Vec<(f64, f64)> = e.iter().zip(&n).map(|(a, b)| pl.p(*a, *b)).collect();
    let closing = pts[0];
    pts.push(closing);
    out.push_str(&format!("<path class=\"axp-s\" d=\"{} Z\"/>", path_d(&pts)));
    pline(&mut out, &pts, "axp-c", false);
    let (cx, cy) = (pts.iter().map(|p| p.0).sum::<f64>() / pts.len() as f64, pts.iter().map(|p| p.1).sum::<f64>() / pts.len() as f64);
    for (i, p) in pts.iter().take(e.len()).enumerate() {
        dot(&mut out, *p, "axp-d", 4.5);
        let (dx, dy) = (p.0 - cx, p.1 - cy);
        let dd = (dx * dx + dy * dy).sqrt().max(1e-9);
        txt(&mut out, p.0 + dx / dd * 14.0, p.1 + dy / dd * 14.0 + 4.0, "middle", "axp-bt", &format!("{}", i + 1));
    }
    let cap = format!("The adjusted stations (E, N) returned by the model for the traverse (azimuths [{}]°, legs [{}] m), joined in \
                       order and closed: {}. Returned: misclosure {} m, relative precision 1:{}, enclosed area {} m².",
                      m::nums(&c_vec(s, "az")), m::nums(&c_vec(s, "d")),
                      e.iter().zip(&n).enumerate().map(|(i, (a, b))| format!("{}: ({}, {})", i + 1, num(*a), num(*b))).collect::<Vec<_>>().join("; "),
                      num(s.out("linear_misclosure").unwrap_or(f64::NAN)), num(s.out("precision").unwrap_or(f64::NAN)),
                      big(s.out("area").unwrap_or(f64::NAN)));
    Some(s.chart("path", "Closed traverse: misclosure and Bowditch (compass rule) adjustment: the returned points", &cap, &out))
}

/// The part of a convex or simple polygon on the near side of the bisector of a and b (Sutherland-Hodgman).
fn clip_half(poly: &[(f64, f64)], a: (f64, f64), b: (f64, f64)) -> Vec<(f64, f64)> {
    let (mx, my) = ((a.0 + b.0) / 2.0, (a.1 + b.1) / 2.0);
    let (nx, ny) = (b.0 - a.0, b.1 - a.1);
    let side = |p: (f64, f64)| (p.0 - mx) * nx + (p.1 - my) * ny; // <= 0: a's side
    let mut out = vec![];
    for i in 0..poly.len() {
        let (p, q_) = (poly[i], poly[(i + 1) % poly.len()]);
        let (sp, sq) = (side(p), side(q_));
        if sp <= 0.0 {
            out.push(p);
        }
        if sp * sq < 0.0 {
            let t = sp / (sp - sq);
            out.push((p.0 + t * (q_.0 - p.0), p.1 + t * (q_.1 - p.1)));
        }
    }
    out
}

fn area_of(p: &[(f64, f64)]) -> f64 {
    let mut a = 0.0;
    for i in 0..p.len() {
        let (u, v) = (p[i], p[(i + 1) % p.len()]);
        a += u.0 * v.1 - v.0 * u.1;
    }
    (a / 2.0).abs()
}

fn thiessen(s: &Solved) -> Option<Figure> {
    let (gx, gy, rain) = (m::list(s, "gx")?, m::list(s, "gy")?, m::list(s, "rain")?);
    let (bx, by) = (m::list(s, "bx")?, m::list(s, "by")?);
    let areas = s.out_list("areas")?;
    let (mean, amean) = (s.out("mean")?, s.out("arithmetic_mean"));
    if gx.len() != gy.len() || gx.len() != rain.len() || bx.len() != by.len() || areas.len() != gx.len() {
        return None;
    }
    let boundary: Vec<(f64, f64)> = bx.iter().copied().zip(by.iter().copied()).collect();
    let g: Vec<(f64, f64)> = gx.iter().copied().zip(gy.iter().copied()).collect();
    let mut polys = vec![];
    for i in 0..g.len() {
        let mut p = boundary.clone();
        for j in 0..g.len() {
            if i != j {
                p = clip_half(&p, g[i], g[j]);
            }
        }
        // drawn only when every polygon's area is the area the model returned
        let a = area_of(&p);
        if (a - areas[i]).abs() > 1e-3 * areas.iter().sum::<f64>().max(1e-12) {
            return None;
        }
        polys.push(p);
    }
    let mut xs: Vec<f64> = bx.clone();
    xs.extend(&gx);
    let mut ys: Vec<f64> = by.clone();
    ys.extend(&gy);
    let (xl, xh) = padded(&xs, 0.06);
    let (yl, yh) = padded(&ys, 0.06);
    let mut pl = Plot::equal(xl, xh, yl, yh);
    pl.x1 = 380.0;
    pl.x0 = 40.0;
    let pl = {
        // equal scales in the narrower box
        let (w, h) = (pl.x1 - pl.x0, pl.y1 - pl.y0);
        let (dx, dy) = (xh - xl, yh - yl);
        let sc = (w / dx).min(h / dy);
        let (cx, cy) = ((xl + xh) / 2.0, (yl + yh) / 2.0);
        let mut p2 = Plot::new((cx - w / sc / 2.0, cx + w / sc / 2.0, false), (cy - h / sc / 2.0, cy + h / sc / 2.0, false));
        p2.x0 = pl.x0;
        p2.x1 = pl.x1;
        p2
    };
    let mut out = String::new();
    let fills = ["axp-s", "axp-s2", "axp-s3", "axp-s4"];
    let lines = ["axp-c", "axp-c2", "axp-c3", "axp-c4"];
    let total: f64 = areas.iter().sum();
    for (i, p) in polys.iter().enumerate() {
        let pp: Vec<(f64, f64)> = p.iter().map(|v| pl.p(v.0, v.1)).collect();
        if pp.len() >= 3 {
            out.push_str(&format!("<path class=\"{}\" d=\"{} Z\"/>", fills[i % 4], path_d(&pp)));
            let mut cl = pp.clone();
            cl.push(pp[0]);
            pline(&mut out, &cl, lines[i % 4], false);
        }
    }
    let bp: Vec<(f64, f64)> = boundary.iter().chain(std::iter::once(&boundary[0])).map(|v| pl.p(v.0, v.1)).collect();
    pline(&mut out, &bp, "axp-k", false);
    for (i, gg) in g.iter().enumerate() {
        let p = pl.p(gg.0, gg.1);
        dot(&mut out, p, "axp-kd", 4.5);
        if g.len() <= 8 {
            txt(&mut out, p.0, p.1 - 9.0, "middle", "axp-bt", &format!("{} mm", num(rain[i])));
            txt(&mut out, p.0, p.1 + 17.0, "middle", "axp-vt", &format!("A = {}", num(areas[i])));
        }
    }
    let mut y = 60.0;
    for t in [format!("Thiessen mean {} mm", num(mean)),
              amean.map(|a| format!("arithmetic {} mm", num(a))).unwrap_or_default(),
              format!("catchment area {}", num(s.out("catchment_area").unwrap_or(total)))] {
        if !t.is_empty() {
            txt(&mut out, 392.0, y, "start", "axp-vt", &t);
            y += 18.0;
        }
    }
    let cap = format!("Thiessen polygons of {} rain gauges (at x = [{}], y = [{}], rainfall [{}] mm) inside the catchment boundary \
                       (x = [{}], y = [{}]); each polygon is the boundary cut by the perpendicular bisectors to the other gauges, drawn \
                       only because its area agrees with the area the model returned. Marked: areas {}, the returned Thiessen mean {} \
                       mm{}.", g.len(), m::nums(&gx), m::nums(&gy), m::nums(&rain), m::nums(&bx), m::nums(&by), m::nums(&areas),
                      num(mean), amean.map(|a| format!(" and arithmetic mean {} mm", num(a))).unwrap_or_default());
    let _ = fl;
    Some(s.figure("map", "Thiessen polygons: area-weighted rainfall", &cap, &out, 300.0))
}

#[allow(dead_code)]
fn unused() -> (Rg, fn(f64) -> Rg, String) {
    ((0.0, 1.0, false), dec, eq("a", fx(1.0, 1.0, 2.0).0, ""))
}
