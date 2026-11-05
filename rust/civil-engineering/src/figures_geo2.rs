//! Civil Engineering drawings: retaining walls with their earth pressure, footings, an infinite slope, soil phase
//! diagrams, a horizontal curve in plan and Mohr circles. Lengths and angles are the inputs as read, values the
//! outputs of this solve or further solves of the model (a pressure diagram from walls of every height).

use super::{dot, legend, path_d, pline, txt};
use crate::figs_common::{self as c};
use crate::figs_more::{self as m, Plot, X1};
use ax_engine::figkit::{num, q, Solved};
use ax_engine::figures::Figure;

pub fn live(s: &Solved) -> Vec<Figure> {
    let v: Vec<Option<Figure>> = match s.key {
        "coulomb_active_thrust" => vec![wall(s, Wall::CoulombA)],
        "coulomb_passive_thrust" => vec![wall(s, Wall::CoulombP)],
        "rankine_active_thrust" => vec![wall(s, Wall::RankineA)],
        "rankine_passive_thrust" => vec![wall(s, Wall::RankineP)],
        "strip_footing_bearing_capacity" => vec![footing(s, false)],
        "terzaghi_bearing_capacity" => vec![footing(s, true)],
        "infinite_slope_factor_of_safety" => vec![slope(s)],
        "soil_phase_relations" => vec![phases(s, false)],
        "soil_phase_from_lab_masses" => vec![phases(s, true)],
        "horizontal_circular_curve" => vec![hcurve(s)],
        "mohr_coulomb_failure_stress" => vec![mohr_failure(s)],
        "triaxial_strength_parameters" => vec![mohr_tests(s)],
        _ => vec![],
    };
    v.into_iter().flatten().collect()
}

fn kpa(pa: f64) -> String {
    format!("{} kPa", num(pa / 1e3))
}

fn knm(n: f64) -> String {
    format!("{} kN/m", num(n / 1e3))
}

fn deg(a: f64) -> String {
    format!("{}°", num(a))
}

fn rect(out: &mut String, x: f64, y: f64, w: f64, h: f64, fill: &str) {
    out.push_str(&format!("<rect class=\"{fill}\" x=\"{x:.1}\" y=\"{y:.1}\" width=\"{w:.1}\" height=\"{h:.1}\"/>\
                           <rect class=\"axp-k\" x=\"{x:.1}\" y=\"{y:.1}\" width=\"{w:.1}\" height=\"{h:.1}\"/>"));
}

fn arrow(out: &mut String, a: (f64, f64), b: (f64, f64), class: &str) {
    let mut cv = ax_engine::figkit::Canvas::new(340.0);
    c::varrow(&mut cv, a, b, class);
    out.push_str(cv.body());
}

fn hatch(out: &mut String, x0: f64, x1: f64, y: f64) {
    let mut cv = ax_engine::figkit::Canvas::new(340.0);
    c::ground_line(&mut cv, x0, x1, y);
    out.push_str(cv.body());
}

// ------------------------------------------------------------------------------------------------ retaining walls
#[derive(Clone, Copy, PartialEq)]
enum Wall {
    CoulombA,
    CoulombP,
    RankineA,
    RankineP,
}

/// A wall of height H holding soil on its right, the earth-pressure diagram on its left and the thrust at its
/// returned height. Coulomb: the triangle of the returned thrust (base 2P/H), the thrust inclined at the wall
/// friction δ; Rankine: the pressure at every depth is the base pressure the model returns for a wall that high.
fn wall(s: &Solved, w: Wall) -> Option<Figure> {
    let hh = c::inp(s, "H")?;
    let phi = c::inp(s, "phi")?;
    let gam = c::inp(s, "gam")?;
    if !(hh > 0.0) {
        return None;
    }
    let active = matches!(w, Wall::CoulombA | Wall::RankineA);
    let coulomb = matches!(w, Wall::CoulombA | Wall::CoulombP);
    let (kname, pname) = if active { ("Ka", "Pa") } else { ("Kp", "Pp") };
    let kv = s.out(kname)?;
    let pv = s.out(pname)?;
    let (delta, beta) = if coulomb { (c::inp(s, "delta")?, c::inp(s, "beta")?) } else { (0.0, 0.0) };
    let ybar = if coulomb { hh / 3.0 } else { s.out("ybar")? };
    // pressure profiles: (depth below the top, pressure) pairs
    let mut profiles: Vec<(String, Vec<(f64, f64)>, bool)> = vec![];
    let base: f64;
    if coulomb {
        base = 2.0 * pv / hh;
        profiles.push((String::new(), vec![(0.0, 0.0), (hh, base)], false));
    } else {
        base = s.out("sigma_base")?;
        let mut own = vec![];
        for d in ax_engine::figkit::span(0.0, hh, 21, false) {
            let p = if d <= 0.0 { c::at(s, &[("H", hh * 1e-6)], "sigma_base") } else { c::at(s, &[("H", d)], "sigma_base") };
            own.push((d, p?));
        }
        profiles.push((String::new(), own, false));
        if active {
            let q0 = c::inp(s, "q").unwrap_or(0.0);
            let q2 = if q0 == 0.0 { 20e3 } else { 0.0 };
            let mut alt = vec![];
            for d in ax_engine::figkit::span(0.0, hh, 21, false) {
                if let Some(p) = c::at(s, &[("H", d.max(hh * 1e-6)), ("q", q2)], "sigma_base") {
                    alt.push((d, p));
                }
            }
            if alt.len() == 21 {
                let lab = if q2 > 0.0 { format!("with a {} surcharge", kpa(q2)) } else { "without surcharge".to_string() };
                profiles.push((lab, alt, true));
            }
        }
    }
    let pmax = profiles.iter().flat_map(|p| p.1.iter().map(|x| x.1.abs())).fold(0.0, f64::max);
    if !(pmax > 0.0) {
        return None;
    }
    // layout
    let (xw, ww) = (262.0, 16.0);
    let (yt, yb) = (82.0, 272.0);
    let sc = (yb - yt) / hh;
    let ps = 160.0 / pmax;
    let mut out = String::new();
    // backfill (right), its surface rising at beta
    let xr = 500.0;
    let rise = ((xr - xw - ww) * beta.to_radians().tan()).clamp(-40.0, yt - 50.0);
    out.push_str(&format!("<path class=\"axp-ka\" d=\"M{:.1},{yt:.1} L{xr:.1},{:.1} L{xr:.1},{yb:.1} L{:.1},{yb:.1} Z\"/>",
                          xw + ww, yt - rise, xw + ww));
    pline(&mut out, &[(xw + ww, yt), (xr, yt - rise)], "axp-k", false);
    if coulomb && beta != 0.0 {
        txt(&mut out, 430.0, yt - rise * 0.75 - 10.0, "middle", "axp-lt", &format!("β = {}", deg(beta)));
    } else {
        txt(&mut out, 430.0, yt - 8.0, "middle", "axp-lt", "ground surface");
    }
    // wall and its base
    rect(&mut out, xw, yt - 4.0, ww, yb - yt + 4.0, "axp-ka");
    rect(&mut out, xw - 32.0, yb, 130.0, 14.0, "axp-ka");
    hatch(&mut out, 24.0, xw - 32.0, yb + 14.0);
    // the pressure diagrams on the left face
    for (i, (lab, pr, dash)) in profiles.iter().enumerate() {
        let pts: Vec<(f64, f64)> = pr.iter().map(|(d, p)| (xw - p * ps, yt + d * sc)).collect();
        if i == 0 {
            let mut poly = vec![(xw, yt)];
            poly.extend(pts.iter().copied());
            poly.push((xw, yb));
            out.push_str(&format!("<path class=\"axp-s\" d=\"{} Z\"/>", path_d(&poly)));
            pline(&mut out, &pts, "axp-c", false);
        } else {
            pline(&mut out, &pts, "axp-c3", *dash);
            let last = pr[pr.len() - 1].1;
            txt(&mut out, xw - last.max(base) * ps - 8.0, yb - 24.0, "end", "axp-lt", &kpa(last));
            txt(&mut out, 24.0, 64.0, "start", "axp-lt", &format!("dashed: {lab}"));
        }
    }
    let far = profiles.iter().flat_map(|p| p.1.last().map(|x| x.1)).fold(base, f64::max);
    txt(&mut out, xw - far * ps - 8.0, yb - 6.0, "end", "axp-vt", &kpa(base));
    // the thrust at its height
    let yp = yb - ybar * sc;
    out.push_str(&format!("<path class=\"axp-c4\" stroke-dasharray=\"5 4\" d=\"M{:.1},{yp:.1} L{xw:.1},{yp:.1}\"/>", xw - 60.0));
    let ang = delta.to_radians();
    let len = 92.0;
    let tip = (xw + ww + 2.0, yp);
    let tail = if active { (tip.0 + len * ang.cos(), yp - len * ang.sin()) } else { (tip.0 + len * ang.cos(), yp + len * ang.sin()) };
    arrow(&mut out, tail, tip, "axp-c2");
    txt(&mut out, tail.0 + 4.0, tail.1 + if active { -8.0 } else { 18.0 }, "start", "axp-vt", &format!("{pname} = {}", knm(pv)));
    txt(&mut out, xw - 6.0, yp - 6.0, "end", "axp-lt", &q(ybar, "m"));
    txt(&mut out, xw + ww + 8.0, (yt + yb) / 2.0 - 24.0, "start", "axp-lt", &format!("H = {}", q(hh, "m")));
    // values
    let mut lines = vec![format!("{kname} = {}", num(kv))];
    if coulomb {
        let (ph, pvv) = if active { ("Pa_h", "Pa_v") } else { ("Pp_h", "Pp_v") };
        if let (Some(a), Some(b)) = (s.out(ph), s.out(pvv)) {
            lines.push(format!("horizontal {}", knm(a)));
            lines.push(format!("vertical {}", knm(b)));
        }
    }
    for (i, l) in lines.iter().enumerate() {
        txt(&mut out, 24.0, 22.0 + 17.0 * i as f64, "start", "axp-vt", l);
    }
    let what = if active { "active" } else { "passive" };
    let (title, cap) = if coulomb {
        (format!("Coulomb {what} thrust on a rough wall"),
         format!("Coulomb {what} thrust on a rough vertical wall of height H = {} (φ = {}, wall friction δ = {}, backfill slope β = \
                  {}, γ = {} kN/m³). The triangular pressure distribution is drawn with the thrust the model returned (base value \
                  2P/H = {}, derived), the thrust at H/3 inclined at δ. Marked: {kname} = {}, {pname} = {}{}.",
                 q(hh, "m"), deg(phi), deg(delta), deg(beta), num(gam / 1e3), kpa(base), num(kv), knm(pv),
                 if lines.len() > 1 { format!(", {}, {}", lines[1], lines[2]) } else { String::new() }))
    } else {
        (format!("Rankine {what} earth pressure on a wall"),
         format!("Rankine {what} earth pressure on a wall of height H = {} (φ = {}, γ = {} kN/m³{}): the pressure at each of 21 depths is \
                  the base pressure the model returns for a wall of that height, growing to {} at the base; the thrust {pname} = {} acts \
                  {} above the base (arrow). Marked: {kname} = {}.{}",
                 q(hh, "m"), deg(phi), num(gam / 1e3),
                 match (c::inp(s, "c"), c::inp(s, "q")) {
                     (Some(cc), Some(qq)) => format!(", c = {}, surcharge {}", kpa(cc), kpa(qq)),
                     (Some(cc), None) => format!(", c = {}", kpa(cc)),
                     (None, Some(qq)) => format!(", surcharge {}", kpa(qq)),
                     _ => String::new(),
                 },
                 kpa(base), knm(pv), q(ybar, "m"), num(kv),
                 if profiles.len() > 1 { format!(" Dashed: the same wall {} (solved again), base {}.", profiles[1].0,
                                                 kpa(profiles[1].1[20].1)) } else { String::new() }))
    };
    Some(s.figure("diagram", &title, &cap, &out, 322.0))
}

// ------------------------------------------------------------------------------------------------ footings
fn footing(s: &Solved, terzaghi: bool) -> Option<Figure> {
    let (b, df) = (c::inp(s, "B")?, c::inp(s, "Df")?);
    let (qu, qa) = (s.out("qu")?, s.out("qa")?);
    if !(b > 0.0) || !(df >= 0.0) {
        return None;
    }
    let sc = (170.0 / b).min(88.0 / df.max(1e-9));
    let (cx, yg) = (200.0, 92.0);
    let (bw, dh) = (b * sc, df * sc);
    let yf = yg + dh;
    let mut out = String::new();
    // soil
    out.push_str(&format!("<rect class=\"axp-ka\" x=\"30\" y=\"{yg:.1}\" width=\"350\" height=\"{:.1}\"/>", 288.0 - yg));
    hatch(&mut out, 30.0, cx - bw / 2.0 - 6.0, yg);
    hatch(&mut out, cx + bw / 2.0 + 6.0, 380.0, yg);
    // column and footing
    rect(&mut out, cx - bw * 0.15, 48.0, bw * 0.3, yf - 16.0 - 48.0, "axp-ka");
    rect(&mut out, cx - bw / 2.0, yf - 16.0, bw, 16.0, "axp-ka");
    // bearing pressure arrows under the footing
    let n = ((bw / 16.0) as usize).clamp(4, 14);
    for i in 0..=n {
        let x = cx - bw / 2.0 + bw * i as f64 / n as f64;
        arrow(&mut out, (x, yf + 46.0), (x, yf + 2.0), "axp-c");
    }
    out.push_str(&format!("<path class=\"axp-c\" d=\"M{:.1},{:.1} L{:.1},{:.1}\"/>", cx - bw / 2.0, yf + 46.0, cx + bw / 2.0, yf + 46.0));
    txt(&mut out, cx, yf + 64.0, "middle", "axp-mt", &format!("qu = {}", kpa(qu)));
    // dimensions
    let yd = (yf + 88.0).min(276.0);
    pline(&mut out, &[(cx - bw / 2.0, yd), (cx + bw / 2.0, yd)], "axp-k", false);
    for x in [cx - bw / 2.0, cx + bw / 2.0] {
        pline(&mut out, &[(x, yd - 5.0), (x, yd + 5.0)], "axp-k", false);
    }
    txt(&mut out, cx, yd + 16.0, "middle", "axp-lt", &format!("B = {}", q(b, "m")));
    if dh > 4.0 {
        let xd = cx - bw / 2.0 - 30.0;
        pline(&mut out, &[(xd, yg), (xd, yf)], "axp-k", false);
        for y in [yg, yf] {
            pline(&mut out, &[(xd - 5.0, y), (xd + 5.0, y)], "axp-k", false);
        }
        txt(&mut out, xd - 8.0, (yg + yf) / 2.0 + 5.0, "end", "axp-lt", &format!("Df = {}", q(df, "m")));
    }
    let mut vals = vec![format!("qu = {}", kpa(qu)), format!("qa = {}", kpa(qa))];
    if terzaghi {
        if let Some(qn) = s.out("q_net") {
            vals.push(format!("net qu = {}", kpa(qn)));
        }
        if let Some(sh) = s.txt("shape") {
            vals.push(format!("{sh} footing"));
        }
    } else {
        for (sym, lab) in [("Nc", "Nc"), ("Nq", "Nq"), ("Ngamma", "Nγ")] {
            if let Some(v) = s.out(sym) {
                vals.push(format!("{lab} = {}", num(v)));
            }
        }
    }
    for (i, v) in vals.iter().enumerate() {
        txt(&mut out, 396.0, 60.0 + 19.0 * i as f64, "start", "axp-vt", v);
    }
    let (title, cap) = if terzaghi {
        ("Footing: Terzaghi bearing capacity".to_string(),
         format!("A {} footing B = {} founded at Df = {} (c = {}, γ = {} kN/m³, Nc = {}, Nq = {}, Nγ = {}). Marked: the returned \
                  ultimate bearing capacity qu = {}, allowable qa = {}{}. Footing width and depth to scale.",
                 s.txt("shape").unwrap_or_else(|| "strip".into()), q(b, "m"), q(df, "m"), kpa(c::inp(s, "c")?),
                 num(c::inp(s, "gam")? / 1e3), num(c::inp(s, "Nc")?), num(c::inp(s, "Nq")?), num(c::inp(s, "Ngamma")?), kpa(qu),
                 kpa(qa), s.out("q_net").map(|v| format!(" and net {}", kpa(v))).unwrap_or_default()))
    } else {
        ("Strip footing: bearing capacity".to_string(),
         format!("A strip footing B = {} wide founded at Df = {} in soil with c = {}, φ = {}, γ = {} kN/m³. Marked: the returned \
                  ultimate bearing capacity qu = {}, allowable qa = {} and factors {}. Footing width and depth to scale.",
                 q(b, "m"), q(df, "m"), kpa(c::inp(s, "c")?), deg(c::inp(s, "phi")?), num(c::inp(s, "gam")? / 1e3), kpa(qu),
                 kpa(qa), vals[2..].join(", ")))
    };
    Some(s.figure("schematic", &title, &cap, &out, 300.0))
}

// ------------------------------------------------------------------------------------------------ infinite slope
fn slope(s: &Solved) -> Option<Figure> {
    let (z, beta) = (c::inp(s, "z")?, c::inp(s, "beta")?);
    let (fs, tau, se) = (s.out("FS")?, s.out("tau")?, s.out("sigma_eff")?);
    if !(z > 0.0) || !(beta > 0.0 && beta < 80.0) {
        return None;
    }
    let mw = c::inp(s, "m").unwrap_or(0.0);
    let t = beta.to_radians().tan();
    // the slope from (40, y0) rising to the right; run limited so the slope fits
    let run = (420.0f64).min(200.0 / t);
    let (x0, y0) = (50.0, 270.0);
    let (x1, y1) = (x0 + run, y0 - run * t);
    let dz = 46.0; // the vertical depth of the slip plane on the page (an infinite slope has no other length)
    let mut out = String::new();
    out.push_str(&format!("<path class=\"axp-ka\" d=\"M{x0:.1},{y0:.1} L{x1:.1},{y1:.1} L{x1:.1},{:.1} L{x0:.1},{:.1} Z\"/>",
                          y1 + dz, y0 + dz));
    pline(&mut out, &[(x0, y0), (x1, y1)], "axp-k", false);
    // the slip plane, hatched below
    out.push_str(&format!("<path class=\"axp-c\" d=\"M{x0:.1},{:.1} L{x1:.1},{:.1}\"/>", y0 + dz, y1 + dz));
    let mut d = String::new();
    let mut x = x0 + 6.0;
    while x < x1 {
        let y = y0 + dz - (x - x0) * t;
        d.push_str(&format!("M{x:.1},{y:.1} L{:.1},{:.1} ", x - 6.0, y + 7.0));
        x += 10.0;
    }
    out.push_str(&format!("<path class=\"axp-c4\" stroke-width=\"1\" d=\"{}\"/>", d.trim_end()));
    if mw > 0.0 {
        let hw = dz * mw.min(1.0);
        out.push_str(&format!("<path class=\"axp-c3\" stroke-dasharray=\"6 4\" d=\"M{x0:.1},{:.1} L{x1:.1},{:.1}\"/>",
                              y0 + dz - hw, y1 + dz - hw));
        txt(&mut out, x0 + run * 0.25, y0 + dz - hw - run * 0.25 * t - 6.0, "middle", "axp-lt", &format!("water table, m = {}", num(mw)));
    }
    // the slice: vertical depth z
    let xs = x0 + run * 0.55;
    let ys = y0 - run * 0.55 * t;
    pline(&mut out, &[(xs, ys), (xs, ys + dz)], "axp-k", false);
    txt(&mut out, xs - 6.0, ys + dz / 2.0 + 5.0, "end", "axp-lt", &format!("z = {}", q(z, "m")));
    txt(&mut out, xs + 50.0, ys - 4.0 - 50.0 * t, "start", "axp-lt", "ground surface");
    txt(&mut out, xs + 40.0, ys + dz + 22.0 - 40.0 * t, "start", "axp-lt", "slip plane");
    // the angle at the toe
    pline(&mut out, &[(x0, y0 + dz), (x0 + 90.0, y0 + dz)], "axp-c4", true);
    let mut arc = vec![];
    for i in 0..=16 {
        let a = beta.to_radians() * i as f64 / 16.0;
        arc.push((x0 + 60.0 * a.cos(), y0 + dz - 60.0 * a.sin()));
    }
    pline(&mut out, &arc, "axp-k", false);
    txt(&mut out, x0 + 66.0, y0 + dz - 8.0, "start", "axp-lt", &format!("β = {}", deg(beta)));
    for (i, l) in [format!("FS = {}", num(fs)), format!("τ = {}", kpa(tau)), format!("σ′ = {}", kpa(se))].iter().enumerate() {
        txt(&mut out, 24.0, 26.0 + 18.0 * i as f64, "start", if i == 0 { "axp-mt" } else { "axp-vt" }, l);
    }
    let cap = format!("An infinite slope at β = {} with a slip plane at vertical depth z = {} (c′ = {}, φ′ = {}, γ = {} kN/m³, water table \
                       ratio m = {}); the depth is drawn as a fixed length, the angle to scale. Marked: the returned factor of safety \
                       FS = {}, driving shear stress {} and effective normal stress {} on the slip plane.",
                      deg(beta), q(z, "m"), kpa(c::inp(s, "c")?), deg(c::inp(s, "phi")?), num(c::inp(s, "gam")? / 1e3), num(mw),
                      num(fs), kpa(tau), kpa(se));
    Some(s.figure("schematic", "Infinite slope: slip plane and factor of safety", &cap, &out, 340.0))
}

// ------------------------------------------------------------------------------------------------ soil phases
fn phases(s: &Solved, lab: bool) -> Option<Figure> {
    let gs = c::inp(s, "Gs")?;
    let (e, sat, n) = if lab { (s.out("e")?, s.out("S")?, s.out("n")?) } else { (c::inp(s, "e")?, s.out("S")?, s.out("n")?) };
    if !(e > 0.0) || !(0.0..=1.0 + 1e-9).contains(&sat) {
        return None;
    }
    let vs = 1.0 / (1.0 + e);
    let vw = sat * e / (1.0 + e);
    let va = (1.0 - vs - vw).max(0.0);
    let (xl, xr, yt, yb) = (210.0, 320.0, 40.0, 250.0);
    let h = yb - yt;
    let mut out = String::new();
    let mut y = yt;
    let parts = [("air", va, "axp-ka"), ("water", vw, "axp-s3"), ("solids", vs, "axp-b4")];
    let mut mids = vec![];
    for (name, f, class) in parts {
        let hh = f * h;
        if hh > 0.3 {
            rect(&mut out, xl, y, xr - xl, hh, class);
            if hh > 15.0 {
                txt(&mut out, (xl + xr) / 2.0, y + hh / 2.0 + 5.0, "middle", "axp-bt", name);
            }
        }
        mids.push((y + hh / 2.0, f, name));
        y += hh;
    }
    // volumes left, masses right
    for (ym, f, name) in &mids {
        if *f > 0.0 {
            txt(&mut out, xl - 10.0, ym + 5.0, "end", "axp-vt", &format!("V = {}", num(*f)));
        }
        let mass = match (*name, lab) {
            ("water", true) => Some(format!("Mw = {}", q(c::inp(s, "M")? - c::inp(s, "Md")?, "kg").replace(" kkg", " kg"))),
            ("solids", true) => Some(format!("Ms = {} kg", num(c::inp(s, "Md")?))),
            ("water", false) => Some(format!("Mw/(ρw Vs) = {}", num(c::inp(s, "w")? * gs))),
            ("solids", false) => Some(format!("Ms/(ρw Vs) = Gs = {}", num(gs))),
            _ => None,
        };
        if let Some(t) = mass {
            txt(&mut out, xr + 10.0, ym + 5.0, "start", "axp-vt", &t);
        }
    }
    let mut lines = vec![];
    if lab {
        lines.push(format!("w = {}, e = {}, n = {}, S = {}", num(s.out("w")?), num(e), num(n), num(sat)));
        lines.push(format!("ρ = {} kg/m³, ρd = {} kg/m³", num(s.out("rho")?), num(s.out("rho_d")?)));
        lines.push(format!("γ = {} kN/m³, γd = {} kN/m³", num(s.out("gamma")? / 1e3), num(s.out("gamma_d")? / 1e3)));
    } else {
        lines.push(format!("n = {}, S = {}", num(n), num(sat)));
        lines.push(format!("γ = {} kN/m³, γd = {} kN/m³", num(s.out("gamma")? / 1e3), num(s.out("gamma_d")? / 1e3)));
        lines.push(format!("γsat = {}, γ′ = {} kN/m³", num(s.out("gamma_sat")? / 1e3), num(s.out("gamma_sub")? / 1e3)));
    }
    for (i, l) in lines.iter().enumerate() {
        txt(&mut out, 60.0, yb + 24.0 + 17.0 * i as f64, "start", "axp-vt", l);
    }
    let (title, cap) = if lab {
        ("Lab specimen phase diagram".to_string(),
         format!("Phase diagram of a lab specimen (mass M = {} kg, dry mass {} kg, volume {} m³, Gs = {}): volumes as fractions of the \
                  specimen volume, solids 1/(1 + e) and water S·e/(1 + e) from the returned void ratio e = {} and saturation S = {}; \
                  masses as stated (water = M − Md). Marked: {}; {}; {}.", num(c::inp(s, "M")?), num(c::inp(s, "Md")?),
                 num(c::inp(s, "V")?), num(gs), num(e), num(sat), lines[0], lines[1], lines[2]))
    } else {
        ("Soil phase diagram: air, water, solids".to_string(),
         format!("Phase diagram of a soil with Gs = {}, void ratio e = {} and water content w = {}: volumes as fractions of the total, \
                  solids 1/(1 + e), water S·e/(1 + e) with the returned degree of saturation S = {}, the rest air; masses relative to \
                  ρw·Vs. Marked: {}; {}; {}.", num(gs), num(e), num(c::inp(s, "w")?), num(sat), lines[0], lines[1], lines[2]))
    };
    let _ = X1;
    Some(s.figure("diagram", &title, &cap, &out, yb + 24.0 + 17.0 * lines.len() as f64))
}

// ------------------------------------------------------------------------------------------------ horizontal curve
fn hcurve(s: &Solved) -> Option<Figure> {
    let (r, dl) = (c::inp(s, "R")?, c::inp(s, "Delta")?);
    let (t, l, lc, e, mo) = (s.out("T")?, s.out("L")?, s.out("LC")?, s.out("E")?, s.out("M")?);
    let (pc, pt) = (s.out("PC")?, s.out("PT")?);
    if !(r > 0.0) || !(dl > 0.0 && dl < 170.0) {
        return None;
    }
    let d = dl.to_radians();
    // plan coordinates (m): PC at the origin, the back tangent along x, the curve turning left round (0, R)
    let arc: Vec<(f64, f64)> = (0..=48).map(|i| {
        let a = d * i as f64 / 48.0;
        (r * a.sin(), r - r * a.cos())
    }).collect();
    let pi = (t, 0.0);
    let ptp = (t + t * d.cos(), t * d.sin());
    let mid = (r * (d / 2.0).sin(), r - r * (d / 2.0).cos());
    let chm = (ptp.0 / 2.0, ptp.1 / 2.0);
    let ext = t * 0.45;
    let pts = [(-ext, 0.0), pi, ptp, (ptp.0 + ext * d.cos(), ptp.1 + ext * d.sin()), mid];
    let xs: Vec<f64> = pts.iter().map(|p| p.0).collect();
    let ys: Vec<f64> = pts.iter().map(|p| p.1).collect();
    let (xl, xh) = (xs.iter().cloned().fold(f64::INFINITY, f64::min), xs.iter().cloned().fold(f64::NEG_INFINITY, f64::max));
    let (yl, yh) = (ys.iter().cloned().fold(f64::INFINITY, f64::min), ys.iter().cloned().fold(f64::NEG_INFINITY, f64::max));
    let (bx0, bx1, by0, by1) = (170.0, 500.0, 40.0, 290.0);
    let sc = ((bx1 - bx0) / (xh - xl).max(1e-9)).min((by1 - by0) / (yh - yl).max(1e-9));
    let ox = bx0 + ((bx1 - bx0) - (xh - xl) * sc) / 2.0;
    let oy = by1 - ((by1 - by0) - (yh - yl) * sc) / 2.0;
    let p = |v: (f64, f64)| (ox + (v.0 - xl) * sc, oy - (v.1 - yl) * sc);
    let mut out = String::new();
    pline(&mut out, &[p((-ext, 0.0)), p(pi)], "axp-k", false);
    pline(&mut out, &[p(pi), p((ptp.0 + ext * d.cos(), ptp.1 + ext * d.sin()))], "axp-k", false);
    pline(&mut out, &[p(pi), p((t + ext, 0.0))], "axp-c4", true);
    let ap: Vec<(f64, f64)> = arc.iter().map(|v| p(*v)).collect();
    pline(&mut out, &ap, "axp-c", false);
    pline(&mut out, &[p((0.0, 0.0)), p(ptp)], "axp-c3", true);
    pline(&mut out, &[p(chm), p(mid)], "axp-c2", false);
    pline(&mut out, &[p(mid), p(pi)], "axp-c2", false);
    for (v, name, dy) in [((0.0, 0.0), format!("PC {}", num(pc)), 18.0), (pi, format!("PI {}", num(c::inp(s, "PI").unwrap_or(pc + t))), 18.0),
                          (ptp, format!("PT {}", num(pt)), -10.0)] {
        let q_ = p(v);
        dot(&mut out, q_, "axp-kd", 4.0);
        txt(&mut out, q_.0 + 6.0, q_.1 + dy, "start", "axp-vt", &name);
    }
    let pm = p(mid);
    txt(&mut out, pm.0 - 8.0, pm.1 - 8.0, "end", "axp-lt", "M, E");
    let pp = p(pi);
    txt(&mut out, pp.0 + 30.0, pp.1 - 8.0, "start", "axp-lt", &format!("Δ = {}", deg(dl)));
    let lines = [format!("R = {}", q(r, "m")), format!("T = {}", q(t, "m")), format!("L = {}", q(l, "m")),
                 format!("LC = {} (dashed)", q(lc, "m")), format!("E = {}", q(e, "m")), format!("M = {}", q(mo, "m")),
                 format!("PC {}", num(pc)), format!("PT {}", num(pt))];
    for (i, l_) in lines.iter().enumerate() {
        txt(&mut out, 20.0, 56.0 + 19.0 * i as f64, "start", "axp-vt", l_);
    }
    let cap = format!("Plan of a simple circular curve, radius R = {}, deflection angle Δ = {}, PI at chainage {} m, drawn to scale with the \
                       returned elements: tangent length T = {}, curve length L = {}, long chord LC = {} (dashed), external distance E = {} \
                       and middle ordinate M = {} (green), and the chainages PC = {} m, PT = {} m.", q(r, "m"), deg(dl),
                      num(c::inp(s, "PI").unwrap_or(pc + t)), q(t, "m"), q(l, "m"), q(lc, "m"), q(e, "m"), q(mo, "m"), num(pc), num(pt));
    Some(s.figure("geometry", "Horizontal curve: tangents, chord and chainages", &cap, &out, 310.0))
}

// ------------------------------------------------------------------------------------------------ Mohr circles
fn semicircle(pl: &Plot, c0: f64, r: f64) -> Vec<(f64, f64)> {
    (0..=60).map(|i| {
        let a = std::f64::consts::PI * i as f64 / 60.0;
        pl.p(c0 + r * a.cos(), r * a.sin())
    }).collect()
}

fn mohr_failure(s: &Solved) -> Option<Figure> {
    let (s3, cc, phi) = (c::inp(s, "s3")? / 1e3, c::inp(s, "c")? / 1e3, c::inp(s, "phi")?);
    let (s1, sf, tf) = (s.out("s1")? / 1e3, s.out("sigma_f")? / 1e3, s.out("tau_f")? / 1e3);
    if !(s1 > s3) {
        return None;
    }
    let (c0, r) = ((s1 + s3) / 2.0, (s1 - s3) / 2.0);
    let tp = phi.to_radians().tan();
    let x_lo = if tp > 0.0 { (-cc / tp).max(-r * 0.6).min(0.0) } else { 0.0 };
    let pl = Plot::equal(x_lo - r * 0.1, s1 + r * 0.25, -r * 0.12, (cc + (s1 + r * 0.25) * tp).min(r * 1.6).max(r * 1.15));
    let mut out = String::new();
    legend(&mut out, &[("Mohr circle at failure", "axp-c", true), ("envelope τ = c + σ tan φ", "axp-c3", true)]);
    pl.axes(&mut out, "σ, normal stress (kPa)", "τ, shear stress (kPa)", "");
    pl.clip(&mut out);
    pline(&mut out, &[pl.p(pl.xr.0, 0.0), pl.p(pl.xr.1, 0.0)], "axp-k", false);
    pline(&mut out, &semicircle(&pl, c0, r), "axp-c", false);
    let xe0 = x_lo.max(pl.xr.0);
    pline(&mut out, &[pl.p(xe0, cc + xe0 * tp), pl.p(pl.xr.1, cc + pl.xr.1 * tp)], "axp-c3", false);
    pline(&mut out, &[pl.p(sf, tf), pl.p(c0, 0.0)], "axp-c4", true);
    out.push_str("</g>");
    let pf = pl.p(sf, tf);
    dot(&mut out, pf, "axp-m", 6.0);
    txt(&mut out, pf.0 - 10.0, pf.1 - 10.0, "end", "axp-mt", &format!("({}, {})", num(sf), num(tf)));
    for (v, lab, cl) in [(s3, format!("σ3 = {}", num(s3)), "axp-lt"), (s1, format!("σ1 = {}", num(s1)), "axp-vt")] {
        let pp = pl.p(v, 0.0);
        dot(&mut out, pp, "axp-d", 4.0);
        txt(&mut out, pp.0, pp.1 + 17.0, "middle", cl, &lab);
    }
    let cap = format!("Mohr-Coulomb failure for σ3 = {} kPa, c = {} kPa, φ = {}: the circle through σ3 and the returned major principal stress \
                       at failure σ1 = {} kPa touches the envelope at the returned stresses on the failure plane σf = {} kPa, τf = {} kPa \
                       (plane at θ = {} to the major principal plane). Equal scales on both axes.", num(s3), num(cc), deg(phi), num(s1),
                      num(sf), num(tf), deg(s.out("theta").unwrap_or(f64::NAN)));
    Some(s.chart("mohr", "Mohr-Coulomb failure: circle and envelope", &cap, &out))
}

fn mohr_tests(s: &Solved) -> Option<Figure> {
    let (s3, s1) = (m::list(s, "s3")?, m::list(s, "s1")?);
    let (cc, phi) = (s.out("c")?, s.out("phi")?);
    if s3.len() != s1.len() || s3.is_empty() || s3.iter().zip(&s1).any(|(a, b)| b <= a) {
        return None;
    }
    let smax = s1.iter().cloned().fold(0.0, f64::max);
    let rmax = s3.iter().zip(&s1).map(|(a, b)| (b - a) / 2.0).fold(0.0, f64::max);
    let tp = phi.to_radians().tan();
    let pl = Plot::equal(-smax * 0.04, smax * 1.08, -rmax * 0.1, (cc + smax * 1.08 * tp).min(rmax * 1.8).max(rmax * 1.15));
    let mut out = String::new();
    legend(&mut out, &[("test circles (input)", "axp-c", true), (&format!("fitted envelope c = {}, φ = {}", num(cc), deg(phi)), "axp-c3", true)]);
    pl.axes(&mut out, "σ, normal stress (kPa)", "τ, shear stress (kPa)", "");
    pl.clip(&mut out);
    pline(&mut out, &[pl.p(pl.xr.0, 0.0), pl.p(pl.xr.1, 0.0)], "axp-k", false);
    for (a, b) in s3.iter().zip(&s1) {
        pline(&mut out, &semicircle(&pl, (a + b) / 2.0, (b - a) / 2.0), "axp-c", false);
    }
    pline(&mut out, &[pl.p(0.0, cc), pl.p(pl.xr.1, cc + pl.xr.1 * tp)], "axp-c3", false);
    out.push_str("</g>");
    let pc = pl.p(0.0, cc);
    dot(&mut out, pc, "axp-m", 6.0);
    txt(&mut out, pc.0 + 10.0, pc.1 - 10.0, "start", "axp-mt", &format!("c = {} kPa", num(cc)));
    let cap = format!("Triaxial tests at confining stresses σ3 = [{}] kPa failing at σ1 = [{}] kPa (circles from these inputs) and the \
                       strength envelope from the returned parameters c = {} kPa and φ = {}{}. Equal scales on both axes.",
                      m::nums(&s3), m::nums(&s1), num(cc), deg(phi),
                      s.out("alpha").map(|a| format!(" (slope angle of the Kf line {})", deg(a))).unwrap_or_default());
    Some(s.chart("mohr", "Triaxial tests: Mohr circles and strength envelope", &cap, &out))
}
