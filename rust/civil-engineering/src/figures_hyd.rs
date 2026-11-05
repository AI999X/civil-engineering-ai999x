//! Civil Engineering: open channels, weirs, orifices, pipes and pipe networks, drawn for the visitor's inputs.
//! Sections are drawn to scale from the inputs and the returned depths; specific-energy curves, profiles and the
//! critical depths beside a section are further solves of the model (this capability again, or a neighbouring one).

use crate::{figs_common as c, figs_more as m};
use ax_engine::figkit::{self as k, num, q, Canvas, Solved};
use ax_engine::figures::Figure;
use ax_engine::{Input, Val};

#[path = "figures_hyd2.rs"]
mod hyd2;

pub fn live(s: &Solved) -> Vec<Figure> {
    let v: Vec<Option<Figure>> = match s.key {
        // sections
        "rectangular_channel_section" => vec![rect_section(s)],
        "trapezoidal_channel_section" => vec![trap_section(s)],
        "circular_channel_section" => vec![circ_section(s)],
        "manning_rectangular_discharge" => vec![manning_rect(s)],
        "manning_trapezoidal_discharge" => vec![manning_trap(s)],
        "manning_partially_full_pipe" => vec![manning_pipe(s)],
        "channel_froude_number" => vec![froude_channel(s)],
        "normal_depth_rectangular" => vec![normal_rect(s)],
        "normal_depth_trapezoidal" => vec![normal_trap(s)],
        "critical_depth_trapezoidal" => vec![critical_trap(s)],
        "critical_depth_circular" => vec![critical_circ(s)],
        // specific energy and profiles
        "alternate_depth_rectangular" => vec![alternate_diagram(s)],
        "critical_depth_rectangular" => vec![critical_rect_diagram(s)],
        "gradually_varied_flow_length" => vec![gvf_profile(s)],
        "hydraulic_jump_rectangular" => vec![jump_profile(s)],
        // roughness maps
        "chezy_from_manning" => vec![m::field_half(s, "field", "n", "R", "C")],
        "manning_from_chezy" => vec![m::field_half(s, "field", "C", "R", "n")],
        // weirs, orifice, pipes
        "broad_crested_weir_discharge" => vec![hyd2::broad_weir(s)],
        "rectangular_weir_discharge" => vec![hyd2::rect_weir(s)],
        "v_notch_weir_discharge" => vec![hyd2::vnotch_weir(s)],
        "orifice_discharge" => vec![hyd2::orifice(s)],
        "hardy_cross_network" => vec![hyd2::hardy_cross(s)],
        "parallel_pipes_hazen_williams" => vec![hyd2::parallel_pipes(s)],
        "three_reservoir_problem" => vec![hyd2::three_reservoirs(s)],
        _ => vec![],
    };
    v.into_iter().flatten().collect()
}

// ------------------------------------------------------------------------------------------------ helpers
/// A number input (SI) or output of the request.
pub(super) fn g(s: &Solved, sym: &str) -> Option<f64> {
    c::get(s, sym)
}

/// Another capability's number output, solved with numbers (SI).
pub(super) fn other(s: &Solved, key: &str, set: &[(&str, f64)], y: &str) -> Option<f64> {
    if set.iter().any(|x| !x.1.is_finite()) {
        return None;
    }
    let v: Vec<(&str, Input)> = set.iter().map(|(n, x)| (*n, Input::Float(*x))).collect();
    let a = s.solve_other(key, &v)?;
    k::num_of(&a, y)
}

/// Water: a filled region with its surface line.
pub(super) fn water(cv: &mut Canvas, region: &[(f64, f64)], surface: &[(f64, f64)]) {
    c::area(cv, region, "axp-s3");
    c::poly(cv, surface, "axp-c3", false);
}

/// The surface mark (an open triangle pointing down) at (x, y).
pub(super) fn surface_mark(cv: &mut Canvas, x: f64, y: f64) {
    cv.raw(&format!("<path class=\"axp-c3\" stroke-width=\"1.6\" d=\"M{:.1},{:.1} L{:.1},{:.1} L{:.1},{:.1} Z\"/>",
                    x - 6.0, y - 10.0, x + 6.0, y - 10.0, x, y - 2.0));
}

/// A vertical dimension from y0 to y1 at x, its label beside it (`left`: to the left of the line).
pub(super) fn vdim(cv: &mut Canvas, x: f64, y0: f64, y1: f64, label: &str, left: bool) {
    cv.line((x, y0), (x, y1));
    cv.line((x - 5.0, y0), (x + 5.0, y0));
    cv.line((x - 5.0, y1), (x + 5.0, y1));
    let ym = (y0 + y1) / 2.0 + 5.0;
    if left {
        cv.name(x - 8.0, ym, "end", label);
    } else {
        cv.name(x + 8.0, ym, "start", label);
    }
}

/// A horizontal dimension from x0 to x1 at y, its label under it.
pub(super) fn hdim(cv: &mut Canvas, x0: f64, x1: f64, y: f64, label: &str) {
    cv.line((x0, y), (x1, y));
    cv.line((x0, y - 5.0), (x0, y + 5.0));
    cv.line((x1, y - 5.0), (x1, y + 5.0));
    cv.name((x0 + x1) / 2.0, y + 17.0, "middle", label);
}

/// Values in two columns from (x, y) down, 17 px a row; returns the height used.
pub(super) fn values(cv: &mut Canvas, x0: f64, x1: f64, y: f64, vals: &[String]) -> f64 {
    let rows = vals.len().div_ceil(2);
    for (i, v) in vals.iter().enumerate() {
        let (x, r) = if i < rows { (x0, i) } else { (x1, i - rows) };
        cv.value(x, y + 17.0 * r as f64, "start", v);
    }
    17.0 * rows as f64
}

/// A length in metres, 4 digits, no prefix (depths read better as 0.4 m than 400 mm).
pub(super) fn ml(x: f64) -> String {
    format!("{} m", num(x))
}

fn deg(t: f64) -> String {
    format!("{}°", num(t))
}

// ------------------------------------------------------------------------------------------------ sections to scale
#[derive(Clone, Copy)]
enum Shape {
    Rect(f64),
    Trap(f64, f64),
    Circ(f64),
}

/// A channel section to scale with water to depth `y`; `levels`: dashed levels above the invert (value, label,
/// class); `vals`: the returned values listed under the drawing. Kind "channel".
#[allow(clippy::too_many_arguments)]
fn section(s: &Solved, title: &str, cap: &str, shape: Shape, y: f64, ylab: &str, levels: &[(f64, String, &str)],
           vals: &[String]) -> Option<Figure> {
    if !(y > 0.0) || !y.is_finite() || levels.iter().any(|l| !l.0.is_finite()) {
        return None;
    }
    let top_level = levels.iter().map(|l| l.0).fold(y, f64::max);
    let rows = vals.len().div_ceil(2) as f64;
    // the drawing's height to scale decides where the invert lies; the values follow under it
    let hpx = match shape {
        Shape::Rect(b) => { let h = top_level * 1.3; h * (250.0 / b).min(150.0 / h) }
        Shape::Trap(b, z) => { let h = top_level * 1.25; h * (330.0 / (b + 2.0 * z * h)).min(150.0 / h) + 14.0 }
        Shape::Circ(d) => { let h = top_level.max(d); h * (180.0 / d).min(176.0 / h) }
    };
    if !hpx.is_finite() {
        return None;
    }
    let yb = 34.0 + hpx;
    let vy = yb + 50.0;
    let mut cv = Canvas::new(vy + 17.0 * rows + 4.0);
    let cx = 270.0;
    match shape {
        Shape::Rect(b) => {
            if !(b > 0.0) {
                return None;
            }
            let h = top_level * 1.3;
            let sc = (250.0 / b).min(150.0 / h);
            let (xl, xr) = (cx - b * sc / 2.0, cx + b * sc / 2.0);
            let yt = yb - h * sc;
            c::ground_line(&mut cv, xl - 14.0, xr + 14.0, yb + 1.0);
            let yw = yb - y * sc;
            water(&mut cv, &[(xl, yb), (xl, yw), (xr, yw), (xr, yb)], &[(xl, yw), (xr, yw)]);
            cv.raw(&format!("<path class=\"axp-k\" stroke-width=\"3\" d=\"M{xl:.1},{yt:.1} L{xl:.1},{yb:.1} L{xr:.1},{yb:.1} L{xr:.1},{yt:.1}\"/>"));
            surface_mark(&mut cv, xr - 22.0, yw);
            vdim(&mut cv, xl - 22.0, yb, yw, ylab, true);
            cv.name(cx, yb + 28.0, "middle", &format!("b = {}", ml(b)));
            for (lv, lab, class) in levels {
                let yl = yb - lv * sc;
                cv.raw(&format!("<path class=\"{class}\" stroke-dasharray=\"6 4\" d=\"M{:.1},{yl:.1} L{:.1},{yl:.1}\"/>",
                                xl + 2.0, xr - 2.0));
                cv.name(xr + 10.0, yl + 4.0, "start", lab);
            }
        }
        Shape::Trap(b, z) => {
            if !(b >= 0.0) || !(z >= 0.0) || b + z <= 0.0 {
                return None;
            }
            let h = top_level * 1.25;
            let sc = (330.0 / (b + 2.0 * z * h)).min(150.0 / h);
            let half = |d: f64| (b / 2.0 + z * d) * sc;
            let yt = yb - h * sc;
            let yw = yb - y * sc;
            let (bl, br) = (cx - b * sc / 2.0, cx + b * sc / 2.0);
            water(&mut cv, &[(bl, yb), (cx - half(y), yw), (cx + half(y), yw), (br, yb)],
                  &[(cx - half(y), yw), (cx + half(y), yw)]);
            cv.raw(&format!("<path class=\"axp-k\" stroke-width=\"3\" d=\"M{:.1},{yt:.1} L{bl:.1},{yb:.1} L{br:.1},{yb:.1} L{:.1},{yt:.1}\"/>",
                            cx - half(h), cx + half(h)));
            c::ground_line(&mut cv, bl, br, yb + 1.0);
            surface_mark(&mut cv, cx + half(y) - 22.0, yw);
            vdim(&mut cv, cx - half(h) - 18.0, yb, yw, ylab, true);
            cv.name(cx, yb + 28.0, "middle", &format!("b = {}", ml(b)));
            cv.name(cx - half(h), yt - 8.0, "middle", &format!("side slope {}H : 1V", num(z)));
            for (lv, lab, class) in levels {
                let yl = yb - lv * sc;
                cv.raw(&format!("<path class=\"{class}\" stroke-dasharray=\"6 4\" d=\"M{:.1},{yl:.1} L{:.1},{yl:.1}\"/>",
                                cx - half(*lv), cx + half(*lv)));
                cv.name(cx + half(*lv) - 30.0, yl - 6.0, "end", lab);
            }
        }
        Shape::Circ(d) => {
            if !(d > 0.0) || y > d * (1.0 + 1e-9) {
                return None;
            }
            let h = top_level.max(d);
            let sc = (180.0 / d).min(176.0 / h);
            let r = d * sc / 2.0;
            let cy = yb - r;
            c::ground_line(&mut cv, cx - r - 20.0, cx + r + 20.0, yb + 1.0);
            let yw = yb - y * sc;
            if y >= d * (1.0 - 1e-9) {
                cv.raw(&format!("<circle class=\"axp-s3\" cx=\"{cx:.1}\" cy=\"{cy:.1}\" r=\"{r:.1}\"/>"));
            } else {
                let a = (r * r - (cy - yw).powi(2)).max(0.0).sqrt();
                let large = if yw < cy { 1 } else { 0 };
                cv.raw(&format!("<path class=\"axp-s3\" d=\"M{:.1},{yw:.1} A{r:.1},{r:.1} 0 {large} 0 {:.1},{yw:.1} Z\"/>",
                                cx - a, cx + a));
                c::poly(&mut cv, &[(cx - a, yw), (cx + a, yw)], "axp-c3", false);
            }
            cv.raw(&format!("<circle class=\"axp-k\" stroke-width=\"2.4\" cx=\"{cx:.1}\" cy=\"{cy:.1}\" r=\"{r:.1}\"/>"));
            vdim(&mut cv, cx - r - 24.0, yb, yw, ylab, true);
            for (lv, lab, class) in levels {
                let yl = yb - lv * sc;
                cv.raw(&format!("<path class=\"{class}\" stroke-dasharray=\"6 4\" d=\"M{:.1},{yl:.1} L{:.1},{yl:.1}\"/>",
                                cx - r - 12.0, cx + r + 12.0));
                cv.name(cx + r + 18.0, yl + 4.0, "start", lab);
            }
            cv.name(cx + r + 14.0, yb - 6.0, "start", &format!("D = {}", ml(d)));
        }
    }
    values(&mut cv, 90.0, 320.0, vy, vals);
    Some(s.figure("channel", title, cap, cv.body(), cv.h))
}

fn list_vals(s: &Solved, outs: &[(&str, &str, &str)]) -> Vec<String> {
    outs.iter().filter_map(|(sym, lab, u)| {
        let v = s.out(sym)?;
        Some(match *u {
            "°" => format!("{lab} = {}", deg(v)),
            "m" => format!("{lab} = {}", ml(v)),
            _ => format!("{lab} = {}", c::qv(v, u)),
        })
    }).collect()
}

const GEOM: [(&str, &str, &str); 5] = [("A", "A", "m^2"), ("P", "P", "m"), ("R", "R", "m"), ("T", "T", "m"), ("Dh", "D_h", "m")];

fn rect_section(s: &Solved) -> Option<Figure> {
    let (b, y) = (g(s, "b")?, g(s, "y")?);
    let vals = list_vals(s, &GEOM);
    let cap = format!("Rectangular channel (b = {}, y = {}) drawn to scale with its water. Marked: the returned {}.",
                      ml(b), ml(y), vals.join(", "));
    section(s, "Rectangular channel: wetted section", &cap, Shape::Rect(b), y, &format!("y = {}", ml(y)), &[], &vals)
}

fn trap_section(s: &Solved) -> Option<Figure> {
    let (b, y, z) = (g(s, "b")?, g(s, "y")?, g(s, "z")?);
    let vals = list_vals(s, &GEOM);
    let cap = format!("Trapezoidal channel (b = {}, y = {}, side slope z = {} horizontal to 1 vertical) drawn to scale with \
                       its water. Marked: the returned {}.", ml(b), ml(y), num(z), vals.join(", "));
    section(s, "Trapezoidal channel: wetted section", &cap, Shape::Trap(b, z), y, &format!("y = {}", ml(y)), &[], &vals)
}

fn circ_section(s: &Solved) -> Option<Figure> {
    let (d, y) = (g(s, "D")?, g(s, "y")?);
    let mut o = GEOM.to_vec();
    o.push(("theta", "θ", "°"));
    let vals = list_vals(s, &o);
    let cap = format!("Circular conduit flowing part full (D = {}, y = {}), to scale. Marked: the returned {} (θ: the central \
                       angle of the wetted arc).", ml(d), ml(y), vals.join(", "));
    section(s, "Part-full circular conduit: wetted section", &cap, Shape::Circ(d), y, &format!("y = {}", ml(y)), &[], &vals)
}

const MANNING: [(&str, &str, &str); 4] = [("Q", "Q", "m^3/s"), ("V", "V", "m/s"), ("A", "A", "m^2"), ("R", "R", "m")];

fn manning_rect(s: &Solved) -> Option<Figure> {
    let (b, y, sl, n) = (g(s, "b")?, g(s, "y")?, g(s, "S")?, g(s, "n")?);
    let vals = list_vals(s, &MANNING);
    let cap = format!("Uniform flow in a rectangular channel (b = {}, y = {}, S = {}, n = {}), section to scale. Marked: the \
                       returned {} (Manning).", ml(b), ml(y), num(sl), num(n), vals.join(", "));
    section(s, "Manning discharge of a rectangular channel", &cap, Shape::Rect(b), y, &format!("y = {}", ml(y)), &[], &vals)
}

fn manning_trap(s: &Solved) -> Option<Figure> {
    let (b, y, z, sl, n) = (g(s, "b")?, g(s, "y")?, g(s, "z")?, g(s, "S")?, g(s, "n")?);
    let vals = list_vals(s, &MANNING);
    let cap = format!("Uniform flow in a trapezoidal channel (b = {}, y = {}, z = {} H:1V, S = {}, n = {}), section to scale. \
                       Marked: the returned {} (Manning).", ml(b), ml(y), num(z), num(sl), num(n), vals.join(", "));
    section(s, "Manning discharge of a trapezoidal channel", &cap, Shape::Trap(b, z), y, &format!("y = {}", ml(y)), &[], &vals)
}

fn manning_pipe(s: &Solved) -> Option<Figure> {
    let (d, y, sl, n) = (g(s, "D")?, g(s, "y")?, g(s, "S")?, g(s, "n")?);
    let vals = list_vals(s, &[("Q", "Q", "m^3/s"), ("V", "V", "m/s"), ("A", "A", "m^2"), ("R", "R", "m"),
                              ("Q_full", "Q full", "m^3/s"), ("ratio", "Q/Q full", "")]);
    let cap = format!("A pipe flowing part full (D = {}, y = {}, S = {}, n = {}), section to scale. Marked: the returned {} \
                       (Manning).", ml(d), ml(y), num(sl), num(n), vals.join(", "));
    section(s, "Part-full pipe: Manning discharge", &cap, Shape::Circ(d), y, &format!("y = {}", ml(y)), &[], &vals)
}

fn froude_channel(s: &Solved) -> Option<Figure> {
    let (qq, b, y) = (g(s, "Q")?, g(s, "b")?, g(s, "y")?);
    let yc = other(s, "critical_depth_rectangular", &[("Q", qq), ("b", b)], "yc")?;
    let mut vals = list_vals(s, &[("V", "V", "m/s"), ("Fr", "Fr", "")]);
    vals.push(s.out_text("regime")?);
    let cap = format!("Rectangular channel carrying Q = {}, b = {} at depth y = {} (to scale); dashed: the critical depth \
                       yc = {} from the model's critical-depth capability. Marked: the returned {}.",
                      c::qv(qq, "m^3/s"), ml(b), ml(y), ml(yc), vals.join(", "));
    section(s, "Froude number and flow regime", &cap, Shape::Rect(b), y, &format!("y = {}", ml(y)),
            &[(yc, format!("yc = {}", ml(yc)), "axp-c")], &vals)
}

fn normal_rect(s: &Solved) -> Option<Figure> {
    let (qq, b, sl, n, yn) = (g(s, "Q")?, g(s, "b")?, g(s, "S")?, g(s, "n")?, s.out("yn")?);
    let yc = other(s, "critical_depth_rectangular", &[("Q", qq), ("b", b)], "yc")?;
    let vals = list_vals(s, &[("yn", "yn", "m"), ("V", "V", "m/s"), ("Fr", "Fr", "")]);
    let cap = format!("Rectangular channel carrying Q = {}, b = {}, S = {}, n = {}, drawn to scale at the returned normal \
                       depth ({}); dashed: the critical depth yc = {} from the model's critical-depth capability.",
                      c::qv(qq, "m^3/s"), ml(b), num(sl), num(n), vals.join(", "), ml(yc));
    section(s, "Normal depth in a rectangular channel", &cap, Shape::Rect(b), yn, &format!("y = {}", ml(yn)),
            &[(yc, format!("yc = {}", ml(yc)), "axp-c")], &vals)
}

fn normal_trap(s: &Solved) -> Option<Figure> {
    let (qq, b, z, sl, n, yn) = (g(s, "Q")?, g(s, "b")?, g(s, "z")?, g(s, "S")?, g(s, "n")?, s.out("yn")?);
    let yc = other(s, "critical_depth_trapezoidal", &[("Q", qq), ("b", b), ("z", z)], "yc")?;
    let vals = list_vals(s, &[("yn", "yn", "m"), ("V", "V", "m/s"), ("A", "A", "m^2"), ("Fr", "Fr", "")]);
    let cap = format!("Trapezoidal channel carrying Q = {}, b = {}, z = {} H:1V, S = {}, n = {}, drawn to scale at the \
                       returned normal depth ({}); dashed: the critical depth yc = {} from the model's critical-depth \
                       capability.", c::qv(qq, "m^3/s"), ml(b), num(z), num(sl), num(n), vals.join(", "), ml(yc));
    section(s, "Normal depth in a trapezoidal channel", &cap, Shape::Trap(b, z), yn, &format!("y = {}", ml(yn)),
            &[(yc, format!("yc = {}", ml(yc)), "axp-c")], &vals)
}

fn critical_trap(s: &Solved) -> Option<Figure> {
    let (qq, b, z, yc, emin) = (g(s, "Q")?, g(s, "b")?, g(s, "z")?, s.out("yc")?, s.out("Emin")?);
    let vals = list_vals(s, &[("yc", "yc", "m"), ("Ac", "Ac", "m^2"), ("Vc", "Vc", "m/s"), ("Emin", "Emin", "m")]);
    let cap = format!("Trapezoidal channel carrying Q = {}, b = {}, z = {} H:1V, drawn to scale at the returned critical \
                       depth. Marked: {} (the minimum specific energy as the dashed level).",
                      c::qv(qq, "m^3/s"), ml(b), num(z), vals.join(", "));
    section(s, "Critical depth in a trapezoidal channel", &cap, Shape::Trap(b, z), yc, &format!("y = {}", ml(yc)),
            &[(emin, format!("Emin = {}", ml(emin)), "axp-c4")], &vals)
}

fn critical_circ(s: &Solved) -> Option<Figure> {
    let (qq, d, yc, emin) = (g(s, "Q")?, g(s, "D")?, s.out("yc")?, s.out("Emin")?);
    let vals = list_vals(s, &[("yc", "yc", "m"), ("Ac", "Ac", "m^2"), ("Vc", "Vc", "m/s"), ("Emin", "Emin", "m")]);
    let cap = format!("Circular conduit carrying Q = {}, D = {}, to scale at the returned critical depth. Marked: {} (the \
                       minimum specific energy as the dashed level).", c::qv(qq, "m^3/s"), ml(d), vals.join(", "));
    section(s, "Critical depth in a circular conduit", &cap, Shape::Circ(d), yc, &format!("y = {}", ml(yc)),
            &[(emin, format!("Emin = {}", ml(emin)), "axp-c4")], &vals)
}

// ------------------------------------------------------------------------------------------------ specific energy
/// (E, y) at depths `ys`, E from the model (`key` with Q, b and the depth y).
fn energy_curve(s: &Solved, key: &str, qq: f64, b: f64, ys: &[f64]) -> Vec<(f64, f64)> {
    ys.iter().filter_map(|y| other(s, key, &[("Q", qq), ("b", b), ("y", *y)], "E").map(|e| (e, *y))).collect()
}

fn energy_chart(s: &Solved, title: &str, cap: &str, curve: Vec<(f64, f64)>, curve_name: String, extra: Vec<(String, Vec<(f64, f64)>)>,
                mark: (f64, f64), mlabel: &str, header: &str) -> Option<Figure> {
    if curve.len() < 10 {
        return None;
    }
    let ymax = curve.iter().map(|p| p.1).fold(0.0, f64::max);
    let mut series = vec![(curve_name, curve), ("E = y".to_string(), vec![(0.0, 0.0), (ymax, ymax)])];
    series.extend(extra);
    m::chart(s, "diagram", title, cap, series, "E, specific energy (m)", "y, depth (m)", header, Some(mark), mlabel, false)
}

fn alternate_diagram(s: &Solved) -> Option<Figure> {
    let (qq, b, y) = (g(s, "Q")?, g(s, "b")?, g(s, "y")?);
    let (e, ya, yc, fr) = (s.out("E")?, s.out("y_alt")?, s.out("yc")?, s.out("Fr")?);
    let regime = s.out_text("regime").unwrap_or_default();
    let lo = y.min(ya).min(yc * 0.5) * 0.85;
    let hi = y.max(ya).max(yc * 2.5) * 1.15;
    let ys = k::span(lo, hi, 33, true);
    let curve = energy_curve(s, s.key, qq, b, &ys);
    // at the critical depth itself this capability has no alternate depth and refuses: the minimum energy from the
    // model's critical-depth capability
    let emin = other(s, "critical_depth_rectangular", &[("Q", qq), ("b", b)], "Emin")?;
    let header = format!("Q = {}, b = {}", c::qv(qq, "m^3/s"), ml(b));
    let cap = format!("Specific-energy diagram of a rectangular channel carrying Q = {} in b = {} (E computed by the model at \
                       {} depths). Depth y = {} ({regime}, Fr = {}) has E = {} and the alternate depth {}; marked: critical \
                       depth yc = {} at the minimum energy E = {} (from the model's critical-depth capability).", c::qv(qq, "m^3/s"), ml(b), curve.len(), ml(y),
                      num(fr), ml(e), ml(ya), ml(yc), ml(emin));
    energy_chart(s, "Specific-energy diagram and alternate depths", &cap, curve, header.clone(),
                 vec![("alternate depths".to_string(), m::dots(&[(e, y), (e, ya)]))], (emin, yc),
                 &format!("yc = {}", ml(yc)), &format!("E = {} at y = {}", ml(e), ml(y)))
}

fn critical_rect_diagram(s: &Solved) -> Option<Figure> {
    let (qq, b) = (g(s, "Q")?, g(s, "b")?);
    let (yc, emin, vc, qu) = (s.out("yc")?, s.out("Emin")?, s.out("Vc")?, s.out("q")?);
    let ys = k::span(yc * 0.4, yc * 3.0, 33, true);
    let curve = energy_curve(s, "alternate_depth_rectangular", qq, b, &ys);
    let cap = format!("Specific-energy diagram of a rectangular channel carrying Q = {} in b = {}: each point is the specific \
                       energy the model returns at a depth ({} depths). Marked: the returned critical depth yc = {}, minimum \
                       specific energy Emin = {}, critical velocity {}, unit discharge q = {}.", c::qv(qq, "m^3/s"),
                      ml(b), curve.len(), ml(yc), ml(emin), c::qv(vc, "m/s"), c::qv(qu, "m^2/s"));
    energy_chart(s, "Specific energy and the critical depth", &cap, curve,
                 format!("specific energy, q = {}", c::qv(qu, "m^2/s")), vec![], (emin, yc),
                 &format!("yc = {}", ml(yc)), &format!("Emin = {}", ml(emin)))
}

// ------------------------------------------------------------------------------------------------ profiles
fn gvf_profile(s: &Solved) -> Option<Figure> {
    let (qq, b, s0, n, y1, y2) = (g(s, "Q")?, g(s, "b")?, g(s, "S0")?, g(s, "n")?, g(s, "y1")?, g(s, "y2")?);
    let (l, yn, yc) = (s.out("L")?, s.out("yn")?, s.out("yc")?);
    let prof = s.out_text("profile").unwrap_or_default();
    let mut pts = vec![(0.0, y1)];
    for i in 1..=10 {
        let yi = y1 + (y2 - y1) * i as f64 / 10.0;
        let li = if i == 10 { Some(l) } else { s.with(&[("y2", yi)]).and_then(|a| k::num_of(&a, "L")) };
        if let Some(li) = li {
            pts.push((li, yi));
        }
    }
    if pts.len() < 7 {
        return None;
    }
    let (xa, xb) = pts.iter().fold((0.0f64, 0.0f64), |a, p| (a.0.min(p.0), a.1.max(p.0)));
    let bed = |x: f64| -s0 * x;
    let surf: Vec<(f64, f64)> = pts.iter().map(|(x, y)| (*x, bed(*x) + y)).collect();
    let line = |d: f64| vec![(xa, bed(xa) + d), (xb, bed(xb) + d)];
    let series = vec![(format!("surface ({prof})"), surf), ("normal yn".to_string(), line(yn)),
                      ("critical yc".to_string(), line(yc)), ("bed".to_string(), line(0.0))];
    let cap = format!("Gradually varied flow profile in a rectangular channel (Q = {}, b = {}, S0 = {}, n = {}) from y1 = {} \
                       at x = 0: the position of each of {} intermediate depths is the length the model returns from y1 to \
                       that depth. Marked: the returned length L = {} to y2 = {}, normal depth {}, critical depth {}, profile \
                       type {prof}.", c::qv(qq, "m^3/s"), ml(b), num(s0), num(n), ml(y1), pts.len() - 2, ml(l),
                      ml(y2), ml(yn), ml(yc));
    m::chart(s, "profile", &format!("Water-surface profile ({prof}) along the channel"), &cap, series,
             "x, distance along the channel (m), downstream positive", "elevation (m), bed at x = 0",
             &format!("L = {}, yn = {}, yc = {}", ml(l), ml(yn), ml(yc)), Some((l, bed(l) + y2)),
             &format!("y2 = {}", ml(y2)), false)
}

fn jump_profile(s: &Solved) -> Option<Figure> {
    let (qq, b, y1) = (g(s, "Q")?, g(s, "b")?, g(s, "y1")?);
    let (y2, fr1, fr2, de, pw, ratio) = (s.out("y2")?, s.out("Fr1")?, s.out("Fr2")?, s.out("dE")?, s.out("power_loss")?,
                                          s.out("ratio")?);
    if !(y2 > 0.0 && y1 > 0.0) {
        return None;
    }
    let mut cv = Canvas::new(300.0);
    let bed = 250.0;
    let sc = 150.0 / y2.max(y1);
    let (p1, p2) = ((y1 * sc).max(2.0), y2 * sc);
    let (xa, xj0, xj1, xb) = (40.0, 210.0, 330.0, 490.0);
    let mut surf = vec![(xa, bed - p1), (xj0, bed - p1)];
    for i in 1..=20 {
        let t = i as f64 / 20.0;
        let f = t * t * (3.0 - 2.0 * t);
        surf.push((xj0 + (xj1 - xj0) * t, bed - p1 - (p2 - p1) * f));
    }
    surf.push((xb, bed - p2));
    let mut region = surf.clone();
    region.push((xb, bed));
    region.push((xa, bed));
    water(&mut cv, &region, &surf);
    c::ground_line(&mut cv, xa - 10.0, xb, bed + 1.0);
    cv.arrow((60.0, bed - p1 / 2.0 - 0.5), (120.0, bed - p1 / 2.0 - 0.5));
    vdim(&mut cv, 175.0, bed, bed - p1, "", true);
    vdim(&mut cv, 470.0, bed, bed - p2, "", true);
    cv.value(48.0, bed - p1 - 14.0, "start", &format!("y1 = {}, Fr1 = {}", ml(y1), num(fr1)));
    cv.value(482.0, bed - p2 - 14.0, "end", &format!("y2 = {}, Fr2 = {}", ml(y2), num(fr2)));
    cv.value(48.0, 30.0, "start", &format!("head loss ΔE = {}", ml(de)));
    cv.value(48.0, 47.0, "start", &format!("power lost {}", q(pw, "W")));
    cv.value(48.0, 64.0, "start", &format!("y2 / y1 = {}", num(ratio)));
    cv.name(482.0, 30.0, "end", &format!("Q = {}, b = {}", c::qv(qq, "m^3/s"), ml(b)));
    let cap = format!("Hydraulic jump in a rectangular channel (Q = {}, b = {}) from the supercritical depth y1 = {} to the \
                       returned sequent depth y2 = {}, depths to scale; the shape and length of the rise are schematic (not \
                       computed). Marked: Fr1 = {}, Fr2 = {}, head loss {}, power lost {}, depth ratio {}.",
                      c::qv(qq, "m^3/s"), ml(b), ml(y1), ml(y2), num(fr1), num(fr2), ml(de), q(pw, "W"), num(ratio));
    Some(s.figure("profile", "Hydraulic jump: sequent depths and energy loss", &cap, cv.body(), cv.h))
}

/// A list input, an edge list input.
pub(super) fn edges(s: &Solved, sym: &str) -> Option<Vec<(String, String)>> {
    match s.val(sym)? {
        Val::Edges(e) if !e.is_empty() => Some(e.into_iter().map(|x| (x.0, x.1)).collect()),
        _ => None,
    }
}
