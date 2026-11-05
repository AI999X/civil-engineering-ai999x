//! Civil Engineering: weirs, an orifice and pipe networks (part of figures_hyd.rs).

use super::{edges, g, hdim, ml, vdim, water};
use crate::{figs_common as c, figs_more as m};
use ax_engine::figkit::{num, Canvas, Solved};
use ax_engine::figures::Figure;

fn ls(x: f64) -> String {
    format!("{} L/s", num(x * 1000.0))
}

// ------------------------------------------------------------------------------------------------ weirs
/// Side view of a broad-crested weir: head and the returned critical depth on the crest to scale.
pub fn broad_weir(s: &Solved) -> Option<Figure> {
    let (cd, b, h) = (g(s, "Cd")?, g(s, "b")?, g(s, "H")?);
    let (qq, yc) = (s.out("Q")?, s.out("yc")?);
    if !(h > 0.0 && yc > 0.0) {
        return None;
    }
    let mut cv = Canvas::new(290.0);
    let bed = 250.0;
    let yt = 180.0;                                    // crest top (crest height schematic)
    let sc = 100.0 / h.max(yc);
    let (ph, pc) = (h * sc, yc * sc);
    let (x0, xc0, xc1) = (40.0, 210.0, 410.0);
    let mut surf = vec![(x0, yt - ph), (xc0 - 10.0, yt - ph)];
    for i in 1..=20 {
        let t = i as f64 / 20.0;
        let f = t * t * (3.0 - 2.0 * t);
        surf.push((xc0 - 10.0 + (xc1 - 60.0 - xc0 + 10.0) * t, yt - ph + (ph - pc) * f));
    }
    surf.push((xc1, yt - pc));
    let mut region = surf.clone();
    region.extend([(xc1, yt), (xc0, yt), (xc0, bed), (x0, bed)]);
    water(&mut cv, &region, &surf);
    c::rect(&mut cv, xc0, yt, xc1 - xc0, bed - yt);
    c::ground_line(&mut cv, 20.0, 500.0, bed + 1.0);
    cv.dashed((150.0, yt), (xc0, yt));
    vdim(&mut cv, 160.0, yt, yt - ph, &format!("H = {}", ml(h)), true);
    vdim(&mut cv, xc1 - 12.0, yt, yt - pc, "", true);
    cv.value(xc1 - 6.0, yt - pc - 12.0, "start", &format!("yc = {}", ml(yc)));
    cv.value(40.0, 40.0, "start", &format!("Q = {} (b = {}, Cd = {})", c::qv(qq, "m^3/s"), ml(b), num(cd)));
    cv.name((xc0 + xc1) / 2.0, (yt + bed) / 2.0 + 5.0, "middle", "crest");
    let cap = format!("Side view of a broad-crested weir with head H = {} above the crest (b = {}, Cd = {}); head and the \
                       returned critical depth over the crest yc = {} to scale, crest height and length and the shape of the \
                       drawdown schematic. Marked: the returned discharge Q = {}.", ml(h), ml(b), num(cd), ml(yc),
                      c::qv(qq, "m^3/s"));
    Some(s.figure("weir", "Broad-crested weir: critical depth on the crest", &cap, cv.body(), cv.h))
}

/// Front view of a sharp-crested rectangular weir: notch width and head to scale.
pub fn rect_weir(s: &Solved) -> Option<Figure> {
    let (cd, b, h) = (g(s, "Cd")?, g(s, "b")?, g(s, "H")?);
    let (qq, leff) = (s.out("Q")?, s.out("L_eff")?);
    if !(b > 0.0 && h > 0.0) {
        return None;
    }
    let ncon = c::inp(s, "contractions").unwrap_or(0.0);
    let mut cv = Canvas::new(300.0);
    let sc = (180.0 / b).min(90.0 / h);
    let (pw, ph) = (b * sc, h * sc);
    let notch = (ph * 1.5).max(30.0);
    let (cx, top, bot) = (250.0, 70.0, 260.0);
    let (pl, pr) = (cx - pw / 2.0 - 50.0, cx + pw / 2.0 + 50.0);
    let (nl, nr, ncrest) = (cx - pw / 2.0, cx + pw / 2.0, top + notch);
    cv.raw(&format!("<path class=\"axp-ka\" d=\"M{pl:.1},{top:.1} L{nl:.1},{top:.1} L{nl:.1},{ncrest:.1} L{nr:.1},{ncrest:.1} \
                     L{nr:.1},{top:.1} L{pr:.1},{top:.1} L{pr:.1},{bot:.1} L{pl:.1},{bot:.1} Z\"/>"));
    let yw = ncrest - ph;
    water(&mut cv, &[(nl, ncrest), (nl, yw), (nr, yw), (nr, ncrest)], &[]);
    cv.raw(&format!("<path class=\"axp-c3\" stroke-dasharray=\"6 4\" d=\"M{:.1},{yw:.1} L{:.1},{yw:.1}\"/>", pl - 15.0, pr + 15.0));
    cv.raw(&format!("<path class=\"axp-k\" d=\"M{pl:.1},{top:.1} L{nl:.1},{top:.1} L{nl:.1},{ncrest:.1} L{nr:.1},{ncrest:.1} \
                     L{nr:.1},{top:.1} L{pr:.1},{top:.1} L{pr:.1},{bot:.1} L{pl:.1},{bot:.1} Z\"/>"));
    c::ground_line(&mut cv, pl - 20.0, pr + 20.0, bot + 1.0);
    hdim(&mut cv, nl, nr, ncrest + 20.0, &format!("b = {}", ml(b)));
    vdim(&mut cv, pr + 16.0, ncrest, yw, &format!("H = {}", ml(h)), false);
    cv.dashed((nr, ncrest), (pr + 16.0, ncrest));
    cv.value(16.0, 26.0, "start", &format!("Q = {}", c::qv(qq, "m^3/s")));
    cv.value(16.0, 43.0, "start", &format!("effective length {} ({} end contractions, Cd = {})", ml(leff), num(ncon), num(cd)));
    let cap = format!("Front view of a sharp-crested rectangular weir, notch width b = {}, head H = {} above the crest (Cd = {}, \
                       end contractions {}); notch and head to scale, plate outline schematic. Marked: the returned discharge \
                       Q = {} and effective crest length {}.", ml(b), ml(h), num(cd), num(ncon), c::qv(qq, "m^3/s"),
                      ml(leff));
    Some(s.figure("weir", "Rectangular weir: head and discharge", &cap, cv.body(), cv.h))
}

/// Front view of a V-notch weir: notch angle and head to scale.
pub fn vnotch_weir(s: &Solved) -> Option<Figure> {
    let (cd, th, h) = (g(s, "Cd")?, g(s, "theta")?, g(s, "H")?);
    let qq = s.out("Q")?;
    if !(h > 0.0 && th > 0.0 && th < 180.0) {
        return None;
    }
    let t = (th.to_radians() / 2.0).tan();
    let mut cv = Canvas::new(280.0);
    // the notch drawn 1.5 heads deep, fitted into 300 px across and 120 px down
    let sc = (120.0 / (1.5 * h)).min(180.0 / (2.0 * 1.5 * h * t));
    let (cx, top) = (230.0, 80.0);
    let depth = 1.5 * h * sc;
    let hw = depth * t;
    let yv = top + depth;
    let (pl, pr, bot) = (cx - hw - 60.0, cx + hw + 60.0, yv + 50.0);
    let outline = format!("M{pl:.1},{top:.1} L{:.1},{top:.1} L{cx:.1},{yv:.1} L{:.1},{top:.1} L{pr:.1},{top:.1} L{pr:.1},{bot:.1} \
                           L{pl:.1},{bot:.1} Z", cx - hw, cx + hw);
    cv.raw(&format!("<path class=\"axp-ka\" d=\"{outline}\"/>"));
    let yw = yv - h * sc;
    let ww = h * sc * t;
    water(&mut cv, &[(cx, yv), (cx - ww, yw), (cx + ww, yw)], &[]);
    cv.raw(&format!("<path class=\"axp-c3\" stroke-dasharray=\"6 4\" d=\"M{:.1},{yw:.1} L{:.1},{yw:.1}\"/>", pl - 10.0, pr + 10.0));
    cv.raw(&format!("<path class=\"axp-k\" d=\"{outline}\"/>"));
    c::ground_line(&mut cv, pl - 15.0, pr + 15.0, bot + 1.0);
    let ar = (h * sc * 0.45).clamp(10.0, 30.0);
    let a0 = std::f64::consts::FRAC_PI_2 - th.to_radians() / 2.0;
    c::arc(&mut cv, (cx, yv), ar, a0, std::f64::consts::PI - a0, "axp-k");
    cv.name(cx, top - 10.0, "middle", &format!("θ = {}°", num(th)));
    vdim(&mut cv, pr + 16.0, yv, yw, &format!("H = {}", ml(h)), false);
    cv.dashed((cx, yv), (pr + 16.0, yv));
    cv.value(16.0, 26.0, "start", &format!("Q = {} (Cd = {})", c::qv(qq, "m^3/s"), num(cd)));
    let cap = format!("Front view of a triangular (V-notch) weir with notch angle θ = {}° and head H = {} above the vertex \
                       (Cd = {}), notch and head to scale. Marked: the returned discharge Q = {}.", num(th), ml(h), num(cd),
                      c::qv(qq, "m^3/s"));
    Some(s.figure("weir", "V-notch weir: head and discharge", &cap, cv.body(), cv.h))
}

/// A tank discharging through an orifice under a head (head and orifice marked; tank and jet schematic).
pub fn orifice(s: &Solved) -> Option<Figure> {
    let (cd, d, h) = (g(s, "Cd")?, g(s, "d")?, g(s, "h")?);
    let (qq, v, a) = (s.out("Q")?, s.out("V_ideal")?, s.out("area")?);
    if !(h > 0.0 && d > 0.0) {
        return None;
    }
    let mut cv = Canvas::new(300.0);
    let (xl, xr, top, bot) = (90.0, 300.0, 40.0, 270.0);
    // the head and the orifice drawn to one scale: the head fills 180 px, the opening at least 4 px
    let sc = 180.0 / h;
    let yo = 240.0;
    let yw = yo - h * sc;
    let od = (d * sc).clamp(4.0, 40.0);
    water(&mut cv, &[(xl, bot), (xl, yw), (xr, yw), (xr, bot)], &[(xl, yw), (xr, yw)]);
    cv.raw(&format!("<path class=\"axp-k\" stroke-width=\"3\" d=\"M{xl:.1},{top:.1} L{xl:.1},{bot:.1} L{xr:.1},{bot:.1} \
                     L{xr:.1},{:.1} M{xr:.1},{:.1} L{xr:.1},{top:.1}\"/>", yo + od / 2.0, yo - od / 2.0));
    c::ground_line(&mut cv, xl - 20.0, 480.0, bot + 1.0);
    super::surface_mark(&mut cv, xl + 30.0, yw);
    // the jet: schematic falling curve
    let mut jet = vec![];
    for i in 0..=20 {
        let t = i as f64 / 20.0;
        jet.push((xr + 150.0 * t, yo + 24.0 * t * t * 2.0));
    }
    c::poly(&mut cv, &jet, "axp-c3", false);
    vdim(&mut cv, xr - 24.0, yo, yw, &format!("h = {}", ml(h)), true);
    cv.dashed((xr - 30.0, yo), (xr, yo));
    cv.value(318.0, 70.0, "start", &format!("Q = {}", ls(qq)));
    cv.value(318.0, 87.0, "start", &format!("ideal jet speed {}", c::qv(v, "m/s")));
    cv.value(318.0, 104.0, "start", &format!("orifice area {} cm²", num(a * 1e4)));
    cv.name(318.0, 124.0, "start", &format!("d = {}, Cd = {}", ml(d), num(cd)));
    let cap = format!("A tank discharging through a sharp-edged orifice of diameter {} (Cd = {}) under a head h = {} (head to \
                       scale; tank and jet schematic). Marked: the returned discharge Q = {}, ideal jet velocity {} and orifice \
                       area {} cm².", ml(d), num(cd), ml(h), ls(qq), c::qv(v, "m/s"), num(a * 1e4));
    Some(s.figure("schematic", "Orifice discharge from a tank", &cap, cv.body(), cv.h))
}

// ------------------------------------------------------------------------------------------------ pipe networks
/// A node box with its label, centred at p.
fn node(cv: &mut Canvas, p: (f64, f64), label: &str, source: bool) {
    let w = ax_engine::figkit::text_w(label, false) + 18.0;
    cv.raw(&format!("<rect class=\"{}\" x=\"{:.1}\" y=\"{:.1}\" width=\"{w:.1}\" height=\"24\" rx=\"5\"/>",
                    if source { "axp-b2" } else { "axp-b3" }, p.0 - w / 2.0, p.1 - 12.0));
    cv.bold(p.0, p.1 + 5.0, "middle", label);
}

/// The looped network with the returned pipe flows as arrows (direction and value) and each node's demand.
pub fn hardy_cross(s: &Solved) -> Option<Figure> {
    let e = edges(s, "pipes")?;
    let flows = s.out_list("flows")?;
    let dem = m::list(s, "demands")?;
    let (l, d) = (m::list(s, "L")?, m::list(s, "D")?);
    if flows.len() != e.len() {
        return None;
    }
    let mut names: Vec<String> = vec![];
    for (u, v) in &e {
        for n in [u, v] {
            if !names.contains(n) {
                names.push(n.clone());
            }
        }
    }
    if names.len() > 10 {
        return None;
    }
    names.sort_by(|a, b| match (a.parse::<f64>(), b.parse::<f64>()) {
        (Ok(x), Ok(y)) => x.partial_cmp(&y).unwrap_or(std::cmp::Ordering::Equal),
        _ => a.cmp(b),
    });
    let n = names.len();
    let mut cv = Canvas::new(340.0);
    let (cx, cy, rx, ry) = (260.0, 185.0, 170.0, 118.0);
    let pos: Vec<(f64, f64)> = (0..n).map(|i| {
        let a = (135.0 - 360.0 * i as f64 / n as f64).to_radians();
        (cx + rx * a.cos(), cy - ry * a.sin())
    }).collect();
    // a square layout for four nodes (corners), a ring otherwise
    let pos: Vec<(f64, f64)> = if n == 4 {
        vec![(cx - rx, cy - ry), (cx + rx, cy - ry), (cx + rx, cy + ry), (cx - rx, cy + ry)]
    } else { pos };
    let at = |nm: &str| names.iter().position(|x| x == nm).map(|i| pos[i]);
    let mut lab = vec![];
    for (i, (u, v)) in e.iter().enumerate() {
        let (pu, pv) = (at(u)?, at(v)?);
        let (a, b) = if flows[i] >= 0.0 { (pu, pv) } else { (pv, pu) };
        let (dx, dy) = (b.0 - a.0, b.1 - a.1);
        let dl = (dx * dx + dy * dy).sqrt().max(1.0);
        let (ux, uy) = (dx / dl, dy / dl);
        // leave room for the boxes: 48 px across, 18 px up and down
        let gap = |ux: f64, uy: f64| (48.0 / ux.abs().max(1e-9)).min(20.0 / uy.abs().max(1e-9)) + 6.0;
        let ga = gap(ux, uy);
        let (p0, p1) = ((a.0 + ux * ga, a.1 + uy * ga), (b.0 - ux * ga, b.1 - uy * ga));
        c::varrow(&mut cv, p0, p1, "axp-c");
        let mid = ((p0.0 + p1.0) / 2.0, (p0.1 + p1.1) / 2.0);
        let (nx, ny) = (-uy, ux);
        let off = if ny.abs() > 0.5 { if ny > 0.0 { -1.0 } else { 1.0 } } else if nx > 0.0 { 1.0 } else { -1.0 };
        let tx = mid.0 + nx * 12.0 * off;
        let ty = mid.1 + ny * 12.0 * off;
        let anchor = if (nx * off).abs() < 0.3 { "middle" } else if nx * off > 0.0 { "start" } else { "end" };
        cv.name(tx, ty + 4.0, anchor, &ls(flows[i].abs()));
        lab.push(format!("{u}→{v}: {}", num(flows[i] * 1000.0)));
    }
    for (i, nm) in names.iter().enumerate() {
        let k = nm.parse::<usize>().ok().filter(|k| *k >= 1 && *k <= dem.len());
        let label = match k {
            Some(1) => format!("{nm}: source"),
            Some(k) => format!("{nm}: −{}", ls(dem[k - 1])),
            None => nm.clone(),
        };
        node(&mut cv, pos[i], &label, k == Some(1));
    }
    let loops = s.out("loops").map(num).unwrap_or_default();
    let it = s.out("iterations").map(num).unwrap_or_default();
    cv.name(16.0, 330.0, "start", &format!("{} pipes, {loops} loops, {it} iterations; layout not to scale", e.len()));
    let cap = format!("A looped pipe network ({} pipes, {loops} loops; lengths [{}] m, diameters [{}] mm) balanced by the Hardy \
                       Cross method in {it} iterations: arrows show the direction and size of each pipe flow returned by the \
                       model (L/s, a negative value flows against the stated pipe direction: {}); boxes give each node's \
                       demand. The node positions are a layout, not to scale. In SI units: flows = [{}] m³/s.", e.len(), m::nums(&l),
                      m::nums(&d), lab.join(", "), m::nums(&flows));
    Some(s.figure("network", "Flows in a looped pipe network", &cap, cv.body(), cv.h))
}

/// Pipes in parallel between two junctions: the returned flow split and common head loss.
pub fn parallel_pipes(s: &Solved) -> Option<Figure> {
    let qq = g(s, "Q")?;
    let (l, d, cc) = (m::list(s, "L")?, m::list(s, "D")?, m::list(s, "C")?);
    let (hf, flows) = (s.out("hf")?, s.out_list("flows")?);
    let n = flows.len();
    if n < 2 || n > 5 || l.len() != n || d.len() != n || cc.len() != n {
        return None;
    }
    let mut cv = Canvas::new(320.0);
    let (ax, bx, ym) = (110.0, 420.0, 180.0);
    let (ytop, ybot) = (90.0, 270.0);
    for i in 0..n {
        let y = ytop + (ybot - ytop) * i as f64 / (n - 1) as f64;
        cv.wire(&[(ax, ym), (150.0, y), (380.0, y), (bx, ym)]);
        c::varrow(&mut cv, (230.0, y), (262.0, y), "axp-c");
        cv.value(265.0, y - 8.0, "middle", &ls(flows[i]));
        cv.name(265.0, y + 17.0, "middle", &format!("L = {} m, Ø{} mm, C = {}", num(l[i]), num(d[i]), num(cc[i])));
    }
    c::varrow(&mut cv, (30.0, ym), (ax - 4.0, ym), "axp-c");
    cv.line((bx, ym), (500.0, ym));
    cv.dot((ax, ym));
    cv.dot((bx, ym));
    cv.name(ax - 6.0, ym - 10.0, "end", "A");
    cv.name(bx + 6.0, ym - 10.0, "start", "B");
    cv.name(30.0, ym + 20.0, "start", &format!("Q = {}", ls(qq)));
    cv.value(260.0, 40.0, "middle", &format!("head loss A→B: {} (the same in every pipe)", ml(hf)));
    let cap = format!("Pipes in parallel between A and B sharing a total flow Q = {} (lengths [{}] m, diameters [{}] mm, \
                       Hazen-Williams C [{}]); layout schematic. Marked: the returned split {} L/s and the common head loss {}.",
                      ls(qq), m::nums(&l), m::nums(&d), m::nums(&cc),
                      flows.iter().map(|f| num(f * 1000.0)).collect::<Vec<_>>().join(", "), ml(hf));
    Some(s.figure("network", "Parallel pipes: flow split and head loss", &cap, cv.body(), cv.h))
}

/// Three reservoirs joined at a junction: elevations to scale, the returned junction head and pipe flows.
pub fn three_reservoirs(s: &Solved) -> Option<Figure> {
    let (z, l, d) = (m::list(s, "z")?, m::list(s, "L")?, m::list(s, "D")?);
    let (hj, flows) = (s.out("h_junction")?, s.out_list("flows")?);
    if z.len() != 3 || flows.len() != 3 || l.len() != 3 || d.len() != 3 {
        return None;
    }
    let (zmin, zmax) = z.iter().chain([hj].iter()).fold((f64::INFINITY, f64::NEG_INFINITY), |a, b| (a.0.min(*b), a.1.max(*b)));
    let span = (zmax - zmin).max(1e-6);
    let lo = zmin - 0.45 * span;
    let hi = zmax + 0.22 * span;
    let (y0, y1) = (250.0, 40.0);
    let py = |v: f64| y0 + (v - lo) / (hi - lo) * (y1 - y0);
    let mut cv = Canvas::new(336.0);
    for t in m::ticks((lo, hi, false), 5) {
        let y = py(t);
        cv.raw(&format!("<line class=\"axp-g\" x1=\"70\" y1=\"{y:.1}\" x2=\"504\" y2=\"{y:.1}\"/>"));
        cv.raw(&format!("<text class=\"axp-t\" x=\"64\" y=\"{:.1}\" text-anchor=\"end\">{}</text>", y + 5.0, num(t)));
    }
    cv.raw(&format!("<rect class=\"axp-f\" x=\"70\" y=\"{y1}\" width=\"434\" height=\"{}\"/>", y0 - y1));
    cv.raw(&format!("<text class=\"axp-l\" transform=\"translate(16,{:.1}) rotate(-90)\" text-anchor=\"middle\">elevation (m)</text>",
                    (y0 + y1) / 2.0));
    let jx = 300.0;
    let jy = py(zmin - 0.32 * span);
    let xs = [130.0, 300.0, 440.0];
    cv.raw(&format!("<path class=\"axp-c\" stroke-dasharray=\"7 4\" d=\"M70,{:.1} L504,{:.1}\"/>", py(hj), py(hj)));
    for i in 0..3 {
        let (x, yw) = (xs[i], py(z[i]));
        let bot = yw + 30.0;
        cv.wire(&[(x, bot), (jx, jy)]);
        cv.raw(&format!("<rect class=\"axp-s3\" x=\"{:.1}\" y=\"{yw:.1}\" width=\"60\" height=\"30\"/>", x - 30.0));
        cv.raw(&format!("<path class=\"axp-k\" d=\"M{:.1},{:.1} L{:.1},{bot:.1} L{:.1},{bot:.1} L{:.1},{:.1}\"/>",
                        x - 30.0, yw - 10.0, x - 30.0, x + 30.0, x + 30.0, yw - 10.0));
        c::poly(&mut cv, &[(x - 30.0, yw), (x + 30.0, yw)], "axp-c3", false);
        cv.name(x, yw - 14.0, "middle", &format!("R{}: {} m", i + 1, num(z[i])));
        let f = flows[i];
        let txt = if f >= 0.0 { format!("{} → J", ls(f)) } else { format!("{} from J", ls(-f)) };
        // the pipe's number beside its middle; flow and size listed under the frame
        let (mx, my) = ((x + jx) / 2.0, (bot + jy) / 2.0);
        let (dx, dy) = (jx - x, jy - bot);
        let dl = (dx * dx + dy * dy).sqrt().max(1.0);
        let (nx, ny) = (dy / dl, -dx / dl);
        let (tx, ty) = if dx.abs() < 1.0 { (mx + 14.0, my) } else { (mx + nx * 14.0 * nx.signum(), my + ny * 14.0 * nx.signum()) };
        cv.raw(&format!("<circle class=\"axp-ka\" cx=\"{tx:.1}\" cy=\"{ty:.1}\" r=\"9\"/><circle class=\"axp-k\" cx=\"{tx:.1}\" cy=\"{ty:.1}\" r=\"9\"/>"));
        cv.bold(tx, ty + 4.5, "middle", &format!("{}", i + 1));
        cv.value(70.0, 284.0 + 17.0 * i as f64, "start", &format!("pipe {} (R{}): {txt}", i + 1, i + 1));
        cv.name(320.0, 284.0 + 17.0 * i as f64, "start", &format!("{} m, Ø{} mm", num(l[i]), num(d[i])));
    }
    cv.raw(&format!("<circle class=\"axp-kd\" cx=\"{jx:.1}\" cy=\"{jy:.1}\" r=\"5\"/>"));
    cv.bold(jx - 10.0, jy + 5.0, "end", "J");
    cv.value(500.0, py(hj) - 6.0, "end", &format!("junction head {}", ml(hj)));
    let cc = m::list(s, "C").map(|v| m::nums(&v)).unwrap_or_default();
    let cap = format!("Three reservoirs with water levels [{}] m joined at a junction J by pipes of lengths [{}] m and diameters \
                       [{}] mm (Hazen-Williams C = [{cc}]); elevations to scale, layout schematic. Marked: the returned \
                       piezometric head at the junction {} and the pipe flows {} L/s (positive towards the junction).",
                      m::nums(&z), m::nums(&l), m::nums(&d), ml(hj),
                      flows.iter().map(|f| num(f * 1000.0)).collect::<Vec<_>>().join(", "));
    Some(s.figure("network", "Three-reservoir problem: junction head and flows", &cap, cv.body(), cv.h))
}
