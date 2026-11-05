//! civil_engineering: procedures translated from the package's Python by tools/py2rust.py (do not edit; regenerate).
//! Source: models/civil_engineering/capabilities/procedures.py. Each keeps Python's number rules through
//! ax_engine::py and is checked against the package by tools/rust_accuracy.py before the build is listed.
#![allow(clippy::all, non_snake_case, unused_parens, unused_braces, unused_mut, unused_variables, unreachable_code)]

use ax_engine::py::{self, Py, PyErr, R};
use ax_engine::Val;
use serde_json::Value;
use std::collections::HashMap;

/// The procedures this crate implements (tools/export_rust_pack.py reads this list).
pub const NATIVE: &[&str] = &[
    "alternate_depth_rectangular",
    "area_from_offsets",
    "broad_crested_weir_discharge",
    "channel_froude_number",
    "chezy_from_manning",
    "chezy_velocity",
    "circular_channel_section",
    "coefficient_of_consolidation",
    "consolidation_settlement_nc",
    "consolidation_settlement_oc",
    "consolidation_time",
    "cooper_jacob_analysis",
    "coulomb_active_thrust",
    "coulomb_passive_thrust",
    "critical_depth_circular",
    "critical_depth_rectangular",
    "critical_depth_trapezoidal",
    "darcy_flow_layered_soil",
    "degree_of_consolidation",
    "design_return_period_for_risk",
    "direct_shear_strength_parameters",
    "dupuit_flow_with_recharge",
    "earthwork_volume_end_areas",
    "effective_stress_at_depth",
    "excess_pore_pressure_isochrone",
    "falling_head_permeability",
    "flood_exceedance_risk",
    "gradually_varied_flow_length",
    "grain_size_coefficients",
    "green_ampt_infiltration",
    "gumbel_design_flood",
    "hardy_cross_network",
    "hazen_williams_flow",
    "hazen_williams_head_loss",
    "horizontal_circular_curve",
    "horton_infiltration",
    "hydraulic_jump_rectangular",
    "infinite_slope_factor_of_safety",
    "kirpich_time_of_concentration",
    "kling_gupta_efficiency",
    "levelling_rise_and_fall",
    "liquid_limit_flow_curve",
    "log_pearson_iii_design_flood",
    "manning_from_chezy",
    "manning_partially_full_pipe",
    "manning_rectangular_discharge",
    "manning_trapezoidal_discharge",
    "manning_velocity",
    "mohr_coulomb_failure_stress",
    "muskingum_routing",
    "nash_sutcliffe_efficiency",
    "normal_depth_rectangular",
    "normal_depth_trapezoidal",
    "orifice_discharge",
    "parallel_pipes_hazen_williams",
    "phi_index",
    "plasticity_liquidity_index",
    "proctor_compaction_optimum",
    "rankine_active_thrust",
    "rankine_passive_thrust",
    "rational_method_peak_runoff",
    "rectangular_channel_section",
    "rectangular_weir_discharge",
    "relative_density_from_unit_weights",
    "scs_curve_number_runoff",
    "secondary_compression_settlement",
    "soil_phase_from_lab_masses",
    "soil_phase_relations",
    "stress_below_circular_load",
    "stress_below_rectangular_load",
    "stress_below_strip_load",
    "strip_footing_bearing_capacity",
    "terzaghi_bearing_capacity",
    "theis_drawdown",
    "thick_clay_layer_settlement",
    "thiem_confined_transmissivity",
    "thiem_unconfined_conductivity",
    "thiessen_polygon_rainfall",
    "three_reservoir_problem",
    "trapezoidal_channel_section",
    "traverse_bowditch_adjustment",
    "triaxial_strength_parameters",
    "unit_hydrograph_convolution",
    "unit_hydrograph_derivation",
    "v_notch_weir_discharge",
    "vertical_parabolic_curve",
    "weibull_plotting_positions",
    "well_field_drawdown",
];

type Out = Result<(Vec<(String, Value)>, f64), String>;

/// The run's result as the engine's answer: a dict of outputs and the residual of the package's own check; a
/// ValueError is the refusal's reason; any other error is an internal error (None).
fn finish(r: R, residual: impl Fn(&Py, &Py) -> f64, v: &HashMap<String, Val>) -> Option<Out> {
    let o = match r {
        Ok(o) => o,
        Err(PyErr::Value(m)) => return Some(Err(m)),
        Err(e) => {
            if std::env::var_os("AX_DEBUG").is_some() {        // the error behind an internal error, for diagnosis
                eprintln!("internal error: {e:?}");
            }
            return None;
        }
    };
    let Py::Dict(d) = &o else { return None };
    let outs: Vec<(String, Value)> = d.borrow().iter().map(|(k, x)| (py::to_str(k), py::to_json(x))).collect();
    let i = Py::dict(v.iter().map(|(k, x)| (Py::str(k), py::from_val(x))).collect());
    let res = residual(&i, &o);
    if std::env::var_os("AX_DEBUG").is_some() && !(res <= 1e-9) {
        eprintln!("check failed: residual {res}");
    }
    Some(Ok((outs, res)))
}

/// The engine's entry point (ax_engine::pack::NativeFn): None for a key that is not one of these procedures.
pub fn native(key: &str, v: &HashMap<String, Val>) -> Option<Out> {
    match key {
        "alternate_depth_rectangular" => {
            let r = f_alternate_depth(match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "area_from_offsets" => {
            let r = f_offsets_area(match v.get("offsets") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "broad_crested_weir_discharge" => {
            let r = f_broad_weir(match v.get("Cd") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("H") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "channel_froude_number" => {
            let r = f_froude_rect(match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "chezy_from_manning" => {
            let r = f_chezy_from_manning(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "chezy_velocity" => {
            let r = f_chezy_velocity(match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "circular_channel_section" => {
            let r = f_circ_section(match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "coefficient_of_consolidation" => {
            let r = f_cv_from_time(match v.get("Hdr") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("U") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "consolidation_settlement_nc" => {
            let r = f_consolidation_nc(match v.get("Cc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("e0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("H") { Some(x) => py::from_val(x), None => { return None } }, match v.get("s0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ds") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "consolidation_settlement_oc" => {
            let r = f_consolidation_oc(match v.get("Cc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Cs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("e0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("H") { Some(x) => py::from_val(x), None => { return None } }, match v.get("s0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ds") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sp") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "consolidation_time" => {
            let r = f_consolidation_time(match v.get("cv") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Hdr") { Some(x) => py::from_val(x), None => { return None } }, match v.get("U") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "cooper_jacob_analysis" => {
            let r = f_cooper_jacob(match v.get("times") { Some(x) => py::from_val(x), None => { return None } }, match v.get("drawdowns") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "coulomb_active_thrust" => {
            let r = f_coulomb_active(match v.get("phi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("delta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gam") { Some(x) => py::from_val(x), None => { return None } }, match v.get("H") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "coulomb_passive_thrust" => {
            let r = f_coulomb_passive(match v.get("phi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("delta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gam") { Some(x) => py::from_val(x), None => { return None } }, match v.get("H") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "critical_depth_circular" => {
            let r = f_critical_circ(match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "critical_depth_rectangular" => {
            let r = f_critical_rect(match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "critical_depth_trapezoidal" => {
            let r = f_critical_trap(match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("z") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match x_extra488(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => if x > 0.0 || x.is_nan() { x } else { 0.0 }, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } 1.0 } } }, v)
        }
        "darcy_flow_layered_soil" => {
            let r = f_layered_darcy(match v.get("H") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dh") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "degree_of_consolidation" => {
            let r = f_degree_of_consolidation(match v.get("cv") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Hdr") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Sc") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "design_return_period_for_risk" => {
            let r = f_design_return_period(match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "direct_shear_strength_parameters" => {
            let r = f_direct_shear_params(match v.get("sigma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tau") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "dupuit_flow_with_recharge" => {
            let r = f_dupuit_recharge(match v.get("K") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("W") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "earthwork_volume_end_areas" => {
            let r = f_earthwork_volume(match v.get("areas") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "effective_stress_at_depth" => {
            let r = f_effective_stress(match v.get("H") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gam") { Some(x) => py::from_val(x), None => { return None } }, match v.get("zw") { Some(x) => py::from_val(x), None => { return None } }, match v.get("z") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "excess_pore_pressure_isochrone" => {
            let r = f_excess_pore_pressure(match v.get("u0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cv") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Hdr") { Some(x) => py::from_val(x), None => { return None } }, match v.get("z") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "falling_head_permeability" => {
            let r = f_falling_head(match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "flood_exceedance_risk" => {
            let r = f_exceedance_risk(match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "gradually_varied_flow_length" => {
            let r = f_gvf_length(match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "grain_size_coefficients" => {
            let r = f_grain_size(match v.get("sizes") { Some(x) => py::from_val(x), None => { return None } }, match v.get("passing") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "green_ampt_infiltration" => {
            let r = f_green_ampt(match v.get("K") { Some(x) => py::from_val(x), None => { return None } }, match v.get("psi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dtheta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "gumbel_design_flood" => {
            let r = f_gumbel_flood(match v.get("peaks") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "hardy_cross_network" => {
            let r = f_hardy_cross(match v.get("pipes") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("demands") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "hazen_williams_flow" => {
            let r = f_hazen_williams_flow(match v.get("hf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "hazen_williams_head_loss" => {
            let r = f_hazen_williams_loss(match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "horizontal_circular_curve" => {
            let r = f_horizontal_curve(match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Delta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("PI") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "horton_infiltration" => {
            let r = f_horton(match v.get("f0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "hydraulic_jump_rectangular" => {
            let r = f_hydraulic_jump(match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y1") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "infinite_slope_factor_of_safety" => {
            let r = f_infinite_slope(match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("phi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gam") { Some(x) => py::from_val(x), None => { return None } }, match v.get("z") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "kirpich_time_of_concentration" => {
            let r = f_kirpich(match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "kling_gupta_efficiency" => {
            let r = f_kge_metrics(match v.get("obs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sim") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "levelling_rise_and_fall" => {
            let r = f_levelling_rise_fall(match v.get("readings") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sights") { Some(x) => py::from_val(x), None => { return None } }, match v.get("RL0") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "liquid_limit_flow_curve" => {
            let r = f_liquid_limit(match v.get("blows") { Some(x) => py::from_val(x), None => { return None } }, match v.get("w") { Some(x) => py::from_val(x), None => { return None } }, match v.get("PL") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "log_pearson_iii_design_flood" => {
            let r = f_log_pearson3(match v.get("peaks") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "manning_from_chezy" => {
            let r = f_manning_from_chezy(match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "manning_partially_full_pipe" => {
            let r = f_manning_pipe(match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "manning_rectangular_discharge" => {
            let r = f_manning_rect(match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "manning_trapezoidal_discharge" => {
            let r = f_manning_trap(match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("z") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "manning_velocity" => {
            let r = f_manning_velocity(match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "mohr_coulomb_failure_stress" => {
            let r = f_mohr_coulomb_failure(match v.get("s3") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("phi") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "muskingum_routing" => {
            let r = f_muskingum(match v.get("inflow") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K") { Some(x) => py::from_val(x), None => { return None } }, match v.get("X") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("O1") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "nash_sutcliffe_efficiency" => {
            let r = f_nse_metrics(match v.get("obs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sim") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "normal_depth_rectangular" => {
            let r = f_normal_depth_rect(match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match x_extra1692(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => if x > 0.0 || x.is_nan() { x } else { 0.0 }, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } 1.0 } } }, v)
        }
        "normal_depth_trapezoidal" => {
            let r = f_normal_depth_trap(match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("z") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "orifice_discharge" => {
            let r = f_orifice_discharge(match v.get("Cd") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "parallel_pipes_hazen_williams" => {
            let r = f_parallel_pipes(match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "phi_index" => {
            let r = f_phi_index(match v.get("rain") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "plasticity_liquidity_index" => {
            let r = f_atterberg_indices(match v.get("LL") { Some(x) => py::from_val(x), None => { return None } }, match v.get("PL") { Some(x) => py::from_val(x), None => { return None } }, match v.get("w") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "proctor_compaction_optimum" => {
            let r = f_proctor(match v.get("w") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho_d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Gs") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "rankine_active_thrust" => {
            let r = f_rankine_active(match v.get("phi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gam") { Some(x) => py::from_val(x), None => { return None } }, match v.get("H") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("q") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "rankine_passive_thrust" => {
            let r = f_rankine_passive(match v.get("phi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gam") { Some(x) => py::from_val(x), None => { return None } }, match v.get("H") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("q") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "rational_method_peak_runoff" => {
            let r = f_rational_peak(match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("i") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "rectangular_channel_section" => {
            let r = f_rect_section(match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "rectangular_weir_discharge" => {
            let r = f_rect_weir(match v.get("Cd") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("H") { Some(x) => py::from_val(x), None => { return None } }, match v.get("contractions") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(0i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "relative_density_from_unit_weights" => {
            let r = f_relative_density(match v.get("gd") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gd_min") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gd_max") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "scs_curve_number_runoff" => {
            let r = f_scs_runoff(match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CN") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lam") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.2)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "secondary_compression_settlement" => {
            let r = f_secondary_compression(match v.get("Ca") { Some(x) => py::from_val(x), None => { return None } }, match v.get("H") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ep") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "soil_phase_from_lab_masses" => {
            let r = f_phase_from_lab(match v.get("M") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Md") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Gs") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "soil_phase_relations" => {
            let r = f_phase_from_void_ratio(match v.get("Gs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("e") { Some(x) => py::from_val(x), None => { return None } }, match v.get("w") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "stress_below_circular_load" => {
            let r = f_circle_load(match v.get("q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("z") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "stress_below_rectangular_load" => {
            let r = f_rectangle_load(match v.get("q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("B") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("z") { Some(x) => py::from_val(x), None => { return None } }, match v.get("position") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("corner")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "stress_below_strip_load" => {
            let r = f_strip_load(match v.get("q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("B") { Some(x) => py::from_val(x), None => { return None } }, match v.get("z") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "strip_footing_bearing_capacity" => {
            let r = f_bearing_strip(match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("phi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gam") { Some(x) => py::from_val(x), None => { return None } }, match v.get("B") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Df") { Some(x) => py::from_val(x), None => { return None } }, match v.get("FS") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(3.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "terzaghi_bearing_capacity" => {
            let r = f_terzaghi_bearing(match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gam") { Some(x) => py::from_val(x), None => { return None } }, match v.get("B") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Df") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Nc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Nq") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ngamma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("shape") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("strip")) })().ok()? } }, match v.get("FS") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(3.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "theis_drawdown" => {
            let r = f_theis(match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "thick_clay_layer_settlement" => {
            let r = f_thick_layer_settlement(match v.get("H") { Some(x) => py::from_val(x), None => { return None } }, match v.get("s_top") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gsub") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ds") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Cc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("e0") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "thiem_confined_transmissivity" => {
            let r = f_thiem_confined(match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("s1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("s2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "thiem_unconfined_conductivity" => {
            let r = f_thiem_unconfined(match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "thiessen_polygon_rainfall" => {
            let r = f_thiessen(match v.get("gx") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gy") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rain") { Some(x) => py::from_val(x), None => { return None } }, match v.get("bx") { Some(x) => py::from_val(x), None => { return None } }, match v.get("by") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "three_reservoir_problem" => {
            let r = f_three_reservoirs(match v.get("z") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "trapezoidal_channel_section" => {
            let r = f_trap_section(match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("z") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "traverse_bowditch_adjustment" => {
            let r = f_traverse_bowditch(match v.get("az") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E0") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("N0") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "triaxial_strength_parameters" => {
            let r = f_triaxial_params(match v.get("s3") { Some(x) => py::from_val(x), None => { return None } }, match v.get("s1") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "unit_hydrograph_convolution" => {
            let r = f_uh_convolution(match v.get("excess") { Some(x) => py::from_val(x), None => { return None } }, match v.get("uh") { Some(x) => py::from_val(x), None => { return None } }, match v.get("baseflow") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "unit_hydrograph_derivation" => {
            let r = f_uh_derive(match v.get("dro") { Some(x) => py::from_val(x), None => { return None } }, match v.get("excess") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "v_notch_weir_discharge" => {
            let r = f_vnotch_weir(match v.get("Cd") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("H") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "vertical_parabolic_curve" => {
            let r = f_vertical_curve(match v.get("g1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("g2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("PVI") { Some(x) => py::from_val(x), None => { return None } }, match v.get("elev_PVI") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "weibull_plotting_positions" => {
            let r = f_weibull_positions(match v.get("peaks") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        "well_field_drawdown" => {
            let r = f_well_superposition(match v.get("wx") { Some(x) => py::from_val(x), None => { return None } }, match v.get("wy") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Qs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y0") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { 0.0 }, v)
        }
        _ => None,
    }
}

#[allow(non_snake_case)]
fn c_EULER_GAMMA() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(0.5772156649015329)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_G() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(9.81)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_GAMMA_W() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(9810.0)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_HW_K() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(10.666829500036352)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_RHO_W() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1000.0)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_YEAR() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(31557600.0)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__TERZ_SHAPE() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("strip"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.5)])), (Py::str("square"), Py::tuple(vec![Py::Float(1.3), Py::Float(0.4)])), (Py::str("circular"), Py::tuple(vec![Py::Float(1.3), Py::Float(0.3)]))])) })());
    V.with(|v| v.clone())
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__E1(mut v_u: Py) -> R {
    let mut v_add: Py = Py::None;
    let mut v_an: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_de: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_term: Py = Py::None;
    if ({ let t2848 = v_u.clone(); Py::Bool({ let t2849 = Py::Float(1.0); if py::le(&t2848, &t2849)? { true } else { false } }) }).truthy() {
        let t2850 = (Py::tuple(vec![Py::Float(0.0), Py::Float(1.0), Py::Int(1i64)])).items()?;
        if t2850.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_s = t2850[0].clone();
        v_term = t2850[1].clone();
        v_k = t2850[2].clone();
        while (Py::Bool(true)).truthy() {
            v_term = py::mul(&v_term, &(if ({ let t2851 = v_k.clone(); Py::Bool({ let t2852 = Py::Int(1i64); if py::gt(&t2851, &t2852)? { true } else { false } }) }).truthy() { py::truediv(&py::neg(&v_u.clone())?, &v_k.clone())? } else { py::neg(&v_u.clone())? }))?;
            v_add = py::truediv(&py::neg(&v_term.clone())?, &v_k.clone())?;
            v_s = py::iadd(&v_s, &v_add.clone())?;
            if ({ let t2857 = { let t2853 = py::abs(&v_add.clone())?; Py::Bool({ let t2854 = py::mul(&Py::Float(1e-17), &py::abs(&v_s.clone())?)?; if py::lt(&t2853, &t2854)? { true } else { false } }) }; if t2857.truthy() { t2857 } else { { let t2855 = v_k.clone(); Py::Bool({ let t2856 = Py::Int(200i64); if py::gt(&t2855, &t2856)? { true } else { false } }) } } }).truthy() {
                break;
            }
            v_k = py::iadd(&v_k, &Py::Int(1i64))?;
        }
        return Ok(py::add(&py::sub(&py::neg(&c_EULER_GAMMA()?)?, &py::math1("log", &v_u.clone())?)?, &v_s.clone())?);
    }
    v_b = py::add(&v_u.clone(), &Py::Float(1.0))?;
    v_c = py::truediv(&Py::Float(1.0), &Py::Float(1e-300))?;
    v_d = py::truediv(&Py::Float(1.0), &v_b.clone())?;
    v_h = v_d.clone();
    for t2858 in py::range_iter(&[(Py::Int(1i64)).i()?, (Py::Int(500i64)).i()?]) {
        v_i = t2858;
        v_an = py::mul(&py::neg(&v_i.clone())?, &v_i.clone())?;
        v_b = py::iadd(&v_b, &Py::Float(2.0))?;
        v_d = py::truediv(&Py::Float(1.0), &py::add(&py::mul(&v_an.clone(), &v_d.clone())?, &v_b.clone())?)?;
        v_c = py::add(&v_b.clone(), &py::truediv(&v_an.clone(), &v_c.clone())?)?;
        v_de = py::mul(&v_c.clone(), &v_d.clone())?;
        v_h = py::mul(&v_h, &v_de.clone())?;
        if ({ let t2859 = py::abs(&py::sub(&v_de.clone(), &Py::Float(1.0))?)?; Py::Bool({ let t2860 = Py::Float(1e-16); if py::lt(&t2859, &t2860)? { true } else { false } }) }).truthy() {
            break;
        }
    }
    return Ok(py::mul(&v_h.clone(), &py::math1("exp", &py::neg(&v_u.clone())?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__Tv_for(mut v_U: Py) -> R {
    return Ok(f__bisect_root({ let v_U = v_U.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_T: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&f__U_fourier(v_T.clone())?, &v_U.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, Py::Float(1e-07), Py::Float(20.0), Py::Int(300i64))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__U_fourier(mut v_Tv: Py) -> R {
    let mut v_M: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_term: Py = Py::None;
    let t556 = (Py::tuple(vec![Py::Float(0.0), Py::Int(0i64)])).items()?;
    if t556.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s = t556[0].clone();
    v_m = t556[1].clone();
    while (Py::Bool(true)).truthy() {
        v_M = py::truediv(&py::mul(&Py::Float(std::f64::consts::PI), &py::add(&py::mul(&Py::Int(2i64), &v_m.clone())?, &Py::Int(1i64))?)?, &Py::Int(2i64))?;
        v_term = py::mul(&py::truediv(&Py::Int(2i64), &py::mul(&v_M.clone(), &v_M.clone())?)?, &py::math1("exp", &py::mul(&py::mul(&py::neg(&v_M.clone())?, &v_M.clone())?, &v_Tv.clone())?)?)?;
        v_s = py::iadd(&v_s, &v_term.clone())?;
        if ({ let t557 = v_term.clone(); Py::Bool({ let t558 = Py::Float(1e-18); if py::lt(&t557, &t558)? { true } else { false } }) }).truthy() {
            break;
        }
        v_m = py::iadd(&v_m, &Py::Int(1i64))?;
    }
    return Ok(py::sub(&Py::Int(1i64), &v_s.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__bisect_root(mut v_f: Py, mut v_a: Py, mut v_b: Py, mut v_iters: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_fa: Py = Py::None;
    let mut v_fb: Py = Py::None;
    let mut v_fm: Py = Py::None;
    let mut v_m: Py = Py::None;
    let t2508 = (Py::tuple(vec![py::call(&v_f.clone(), &[v_a.clone()])?, py::call(&v_f.clone(), &[v_b.clone()])?])).items()?;
    if t2508.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fa = t2508[0].clone();
    v_fb = t2508[1].clone();
    if ({ let t2509 = v_fa.clone(); Py::Bool({ let t2510 = Py::Int(0i64); if py::eq(&t2509, &t2510) { true } else { false } }) }).truthy() {
        return Ok(v_a.clone());
    }
    if ({ let t2511 = v_fb.clone(); Py::Bool({ let t2512 = Py::Int(0i64); if py::eq(&t2511, &t2512) { true } else { false } }) }).truthy() {
        return Ok(v_b.clone());
    }
    let _ = f__need({ let t2513 = py::mul(&v_fa.clone(), &v_fb.clone())?; Py::Bool({ let t2514 = Py::Int(0i64); if py::lt(&t2513, &t2514)? { true } else { false } }) }, Py::str("no root in the admissible range"))?;
    for t2515 in py::range_iter(&[(v_iters.clone()).i()?]) {
        v__ = t2515;
        v_m = py::mul(&Py::Float(0.5), &py::add(&v_a.clone(), &v_b.clone())?)?;
        if ({ let t2520 = { let t2516 = v_m.clone(); Py::Bool({ let t2517 = v_a.clone(); if py::eq(&t2516, &t2517) { true } else { false } }) }; if t2520.truthy() { t2520 } else { { let t2518 = v_m.clone(); Py::Bool({ let t2519 = v_b.clone(); if py::eq(&t2518, &t2519) { true } else { false } }) } } }).truthy() {
            break;
        }
        v_fm = py::call(&v_f.clone(), &[v_m.clone()])?;
        if ({ let t2521 = v_fm.clone(); Py::Bool({ let t2522 = Py::Int(0i64); if py::eq(&t2521, &t2522) { true } else { false } }) }).truthy() {
            return Ok(v_m.clone());
        }
        if ({ let t2523 = { let t2525 = v_fm.clone(); Py::Bool({ let t2526 = Py::Int(0i64); if py::lt(&t2525, &t2526)? { true } else { false } }) }; Py::Bool({ let t2524 = { let t2527 = v_fa.clone(); Py::Bool({ let t2528 = Py::Int(0i64); if py::lt(&t2527, &t2528)? { true } else { false } }) }; if py::eq(&t2523, &t2524) { true } else { false } }) }).truthy() {
            let t2529 = (Py::tuple(vec![v_m.clone(), v_fm.clone()])).items()?;
            if t2529.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t2529[0].clone();
            v_fa = t2529[1].clone();
        } else {
            v_b = v_m.clone();
        }
    }
    return Ok(py::mul(&Py::Float(0.5), &py::add(&v_a.clone(), &v_b.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__circ(mut v_D: Py, mut v_y: Py) -> R {
    let mut v_th: Py = Py::None;
    v_th = py::mul(&Py::Int(2i64), &py::math1("acos", &py::sub(&Py::Int(1i64), &py::truediv(&py::mul(&Py::Int(2i64), &v_y.clone())?, &v_D.clone())?)?)?)?;
    return Ok(Py::tuple(vec![py::mul(&py::truediv(&py::mul(&v_D.clone(), &v_D.clone())?, &Py::Int(8i64))?, &py::sub(&v_th.clone(), &py::math1("sin", &v_th.clone())?)?)?, py::truediv(&py::mul(&v_D.clone(), &v_th.clone())?, &Py::Int(2i64))?, py::mul(&v_D.clone(), &py::math1("sin", &py::truediv(&v_th.clone(), &Py::Int(2i64))?)?)?, v_th.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__clip_halfplane(mut v_poly: Py, mut v_a: Py, mut v_b: Py, mut v_c: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_Qp: Py = Py::None;
    let mut v_fp: Py = Py::None;
    let mut v_fq: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_t: Py = Py::None;
    v_out = Py::list(vec![]);
    v_n = py::len(&v_poly.clone())?;
    for t2445 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t2445;
        let t2446 = (Py::tuple(vec![py::getitem(&v_poly.clone(), &v_i.clone())?, py::getitem(&v_poly.clone(), &py::modulo(&py::add(&v_i.clone(), &Py::Int(1i64))?, &v_n.clone())?)?])).items()?;
        if t2446.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_P = t2446[0].clone();
        v_Qp = t2446[1].clone();
        let t2447 = (Py::tuple(vec![py::sub(&py::add(&py::mul(&v_a.clone(), &py::getitem(&v_P.clone(), &Py::Int(0i64))?)?, &py::mul(&v_b.clone(), &py::getitem(&v_P.clone(), &Py::Int(1i64))?)?)?, &v_c.clone())?, py::sub(&py::add(&py::mul(&v_a.clone(), &py::getitem(&v_Qp.clone(), &Py::Int(0i64))?)?, &py::mul(&v_b.clone(), &py::getitem(&v_Qp.clone(), &Py::Int(1i64))?)?)?, &v_c.clone())?])).items()?;
        if t2447.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_fp = t2447[0].clone();
        v_fq = t2447[1].clone();
        if ({ let t2448 = v_fp.clone(); Py::Bool({ let t2449 = Py::Int(0i64); if py::le(&t2448, &t2449)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_out.clone(), "append", &[v_P.clone()])?;
        }
        if ({ let t2456 = { let t2450 = v_fp.clone(); Py::Bool({ let t2451 = Py::Int(0i64); if py::lt(&t2450, &t2451)? { { let t2452 = v_fq.clone(); if py::lt(&t2451, &t2452)? { true } else { false } } } else { false } }) }; if t2456.truthy() { t2456 } else { { let t2453 = v_fq.clone(); Py::Bool({ let t2454 = Py::Int(0i64); if py::lt(&t2453, &t2454)? { { let t2455 = v_fp.clone(); if py::lt(&t2454, &t2455)? { true } else { false } } } else { false } }) } } }).truthy() {
            v_t = py::truediv(&v_fp.clone(), &py::sub(&v_fp.clone(), &v_fq.clone())?)?;
            let _ = py::method(&v_out.clone(), "append", &[Py::tuple(vec![py::add(&py::getitem(&v_P.clone(), &Py::Int(0i64))?, &py::mul(&v_t.clone(), &py::sub(&py::getitem(&v_Qp.clone(), &Py::Int(0i64))?, &py::getitem(&v_P.clone(), &Py::Int(0i64))?)?)?)?, py::add(&py::getitem(&v_P.clone(), &Py::Int(1i64))?, &py::mul(&v_t.clone(), &py::sub(&py::getitem(&v_Qp.clone(), &Py::Int(1i64))?, &py::getitem(&v_P.clone(), &Py::Int(1i64))?)?)?)?])])?;
        }
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__coulomb_check(mut v_phi: Py, mut v_delta: Py, mut v_beta: Py) -> R {
    let _ = f__finite(Py::dict(vec![(Py::str("phi"), v_phi.clone()), (Py::str("delta"), v_delta.clone()), (Py::str("beta"), v_beta.clone())]))?;
    let _ = f__need({ let t366 = Py::Int(15i64); Py::Bool({ let t367 = v_phi.clone(); if py::le(&t366, &t367)? { { let t368 = Py::Int(50i64); if py::le(&t367, &t368)? { true } else { false } } } else { false } }) }, Py::str("the friction angle must lie between 15 and 50 degrees"))?;
    let _ = f__need({ let t369 = Py::Int(0i64); Py::Bool({ let t370 = v_delta.clone(); if py::le(&t369, &t370)? { { let t371 = v_phi.clone(); if py::le(&t370, &t371)? { true } else { false } } } else { false } }) }, Py::str("the wall friction angle must lie between 0 and phi"))?;
    let _ = f__need({ let t372 = Py::Int(0i64); Py::Bool({ let t373 = v_beta.clone(); if py::le(&t372, &t373)? { { let t374 = v_phi.clone(); if py::lt(&t373, &t374)? { true } else { false } } } else { false } }) }, Py::str("the backfill slope must lie between 0 and phi"))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__finite(mut v_kw: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    for t2832 in py::iter(&(py::method(&v_kw.clone(), "items", &[])?))? {
        let t2835 = (t2832).items()?;
        if t2835.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2835[0].clone();
        v_v = t2835[1].clone();
        if ({ let t2834 = Py::Bool(matches!(v_v.clone(), Py::Bool(_))); if t2834.truthy() { t2834 } else { { let t2833 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_)))); if t2833.truthy() { t2833 } else { py::not(&py::math1("isfinite", &v_v.clone())?) } } } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_k.clone(), "")?, " must be a finite number".to_string()].concat()))));
        }
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__gamma_ppf(mut v_a: Py, mut v_p: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_fx: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_lg: Py = Py::None;
    let mut v_nx: Py = Py::None;
    let mut v_step: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_z: Py = Py::None;
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(30i64))?;
        v_F = { let v_a = v_a.clone(); let v_p = v_p.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::float(&py::sub(&ax_engine::pymp::call("gammainc", &[v_a.clone(), Py::Int(0i64), v_x.clone()], &[(String::from("regularized"), Py::Bool(true))])?, &v_p.clone())?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
        v_lg = py::math1("lgamma", &v_a.clone())?;
        v_z = py::normal_dist("inv_cdf", &Py::Float(0.0), &Py::Float(1.0), &[v_p.clone()])?;
        v_x = py::minmax(&[py::mul(&v_a.clone(), &py::pow(&py::add(&py::sub(&Py::Int(1i64), &py::truediv(&Py::Int(1i64), &py::mul(&Py::Int(9i64), &v_a.clone())?)?)?, &py::mul(&v_z.clone(), &py::math1("sqrt", &py::truediv(&Py::Int(1i64), &py::mul(&Py::Int(9i64), &v_a.clone())?)?)?)?)?, &Py::Int(3i64))?)?, py::mul(&Py::Float(0.001), &v_a.clone())?], true)?;
        for t1405 in py::range_iter(&[(Py::Int(60i64)).i()?]) {
            v__ = t1405;
            v_fx = py::call(&v_F.clone(), &[v_x.clone()])?;
            v_step = py::truediv(&v_fx.clone(), &py::math1("exp", &py::sub(&py::sub(&py::mul(&py::sub(&v_a.clone(), &Py::Int(1i64))?, &py::math1("log", &v_x.clone())?)?, &v_x.clone())?, &v_lg.clone())?)?)?;
            v_nx = py::sub(&v_x.clone(), &v_step.clone())?;
            if ({ let t1406 = v_nx.clone(); Py::Bool({ let t1407 = Py::Int(0i64); if py::le(&t1406, &t1407)? { true } else { false } }) }).truthy() {
                v_nx = py::truediv(&v_x.clone(), &Py::Int(2i64))?;
            }
            if ({ let t1408 = py::abs(&py::sub(&v_nx.clone(), &v_x.clone())?)?; Py::Bool({ let t1409 = py::mul(&Py::Float(1e-15), &v_x.clone())?; if py::le(&t1408, &t1409)? { true } else { false } }) }).truthy() {
                return Ok(v_nx.clone());
            }
            v_x = v_nx.clone();
        }
        v_hi = py::add(&py::add(&v_a.clone(), &py::mul(&Py::Int(10i64), &py::math1("sqrt", &v_a.clone())?)?)?, &Py::Int(50i64))?;
        while ({ let t1410 = py::call(&v_F.clone(), &[v_hi.clone()])?; Py::Bool({ let t1411 = Py::Int(0i64); if py::lt(&t1410, &t1411)? { true } else { false } }) }).truthy() {
            v_hi = py::mul(&v_hi, &Py::Int(2i64))?;
        }
        return Ok(f__bisect_root(v_F.clone(), Py::Float(0.0), v_hi.clone(), Py::Int(200i64))?);
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__hw_k(mut v_L: Py, mut v_D: Py, mut v_C: Py) -> R {
    return Ok(py::truediv(&py::mul(&c_HW_K()?, &v_L.clone())?, &py::mul(&py::pow(&v_C.clone(), &Py::Float(1.852))?, &py::pow(&v_D.clone(), &Py::Float(4.871))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__hw_loss(mut v_k: Py, mut v_Q: Py) -> R {
    return Ok(py::copysign(&py::mul(&v_k.clone(), &py::pow(&py::abs(&v_Q.clone())?, &Py::Float(1.852))?)?, &v_Q.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__is_simple(mut v_poly: Py) -> R {
    let mut v_cross: Py = Py::None;
    let mut v_hit: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_n: Py = Py::None;
    v_n = py::len(&v_poly.clone())?;
    if ({ let t2382 = py::len(&py::set(&v_poly.clone())?)?; Py::Bool({ let t2383 = v_n.clone(); if py::ne(&t2382, &t2383) { true } else { false } }) }).truthy() {
        return Ok(Py::Bool(false));
    }
    v_cross = { let v_cross = v_cross.clone(); let v_n = v_n.clone(); let v_poly = v_poly.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_o: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_a: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_b: Py = a.get(2).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::mul(&py::sub(&py::getitem(&v_a.clone(), &Py::Int(0i64))?, &py::getitem(&v_o.clone(), &Py::Int(0i64))?)?, &py::sub(&py::getitem(&v_b.clone(), &Py::Int(1i64))?, &py::getitem(&v_o.clone(), &Py::Int(1i64))?)?)?, &py::mul(&py::sub(&py::getitem(&v_a.clone(), &Py::Int(1i64))?, &py::getitem(&v_o.clone(), &Py::Int(1i64))?)?, &py::sub(&py::getitem(&v_b.clone(), &Py::Int(0i64))?, &py::getitem(&v_o.clone(), &Py::Int(0i64))?)?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_hit = { let v_cross = v_cross.clone(); let v_hit = v_hit.clone(); let v_n = v_n.clone(); let v_poly = v_poly.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_p1: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_p2: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_p3: Py = a.get(2).cloned().unwrap_or(Py::None);
            let mut v_p4: Py = a.get(3).cloned().unwrap_or(Py::None);
            let mut v_d1: Py = Py::None;
            let mut v_d2: Py = Py::None;
            let mut v_d3: Py = Py::None;
            let mut v_d4: Py = Py::None;
            let t2384 = (Py::tuple(vec![py::call(&v_cross.clone(), &[v_p3.clone(), v_p4.clone(), v_p1.clone()])?, py::call(&v_cross.clone(), &[v_p3.clone(), v_p4.clone(), v_p2.clone()])?, py::call(&v_cross.clone(), &[v_p1.clone(), v_p2.clone(), v_p3.clone()])?, py::call(&v_cross.clone(), &[v_p1.clone(), v_p2.clone(), v_p4.clone()])?])).items()?;
            if t2384.len() != 4 { return Err(PyErr::Value("unpack".into())); }
            v_d1 = t2384[0].clone();
            v_d2 = t2384[1].clone();
            v_d3 = t2384[2].clone();
            v_d4 = t2384[3].clone();
            return Ok({ let t2421 = { let t2396 = { let t2385 = { let t2387 = v_d1.clone(); Py::Bool({ let t2388 = Py::Int(0i64); if py::gt(&t2387, &t2388)? { true } else { false } }) }; Py::Bool({ let t2386 = { let t2389 = v_d2.clone(); Py::Bool({ let t2390 = Py::Int(0i64); if py::gt(&t2389, &t2390)? { true } else { false } }) }; if py::ne(&t2385, &t2386) { true } else { false } }) }; if t2396.truthy() { t2396 } else { { let t2395 = { let t2391 = v_d1.clone(); Py::Bool({ let t2392 = Py::Int(0i64); if py::eq(&t2391, &t2392) { true } else { false } }) }; if t2395.truthy() { t2395 } else { { let t2393 = v_d2.clone(); Py::Bool({ let t2394 = Py::Int(0i64); if py::eq(&t2393, &t2394) { true } else { false } }) } } } } }; if !t2421.truthy() { t2421 } else { { let t2420 = { let t2408 = { let t2397 = { let t2399 = v_d3.clone(); Py::Bool({ let t2400 = Py::Int(0i64); if py::gt(&t2399, &t2400)? { true } else { false } }) }; Py::Bool({ let t2398 = { let t2401 = v_d4.clone(); Py::Bool({ let t2402 = Py::Int(0i64); if py::gt(&t2401, &t2402)? { true } else { false } }) }; if py::ne(&t2397, &t2398) { true } else { false } }) }; if t2408.truthy() { t2408 } else { { let t2407 = { let t2403 = v_d3.clone(); Py::Bool({ let t2404 = Py::Int(0i64); if py::eq(&t2403, &t2404) { true } else { false } }) }; if t2407.truthy() { t2407 } else { { let t2405 = v_d4.clone(); Py::Bool({ let t2406 = Py::Int(0i64); if py::eq(&t2405, &t2406) { true } else { false } }) } } } } }; if !t2420.truthy() { t2420 } else { { let t2419 = { let t2409 = py::minmax(&[py::getitem(&v_p1.clone(), &Py::Int(0i64))?, py::getitem(&v_p2.clone(), &Py::Int(0i64))?], false)?; Py::Bool({ let t2410 = py::minmax(&[py::getitem(&v_p3.clone(), &Py::Int(0i64))?, py::getitem(&v_p4.clone(), &Py::Int(0i64))?], true)?; if py::le(&t2409, &t2410)? { true } else { false } }) }; if !t2419.truthy() { t2419 } else { { let t2418 = { let t2411 = py::minmax(&[py::getitem(&v_p3.clone(), &Py::Int(0i64))?, py::getitem(&v_p4.clone(), &Py::Int(0i64))?], false)?; Py::Bool({ let t2412 = py::minmax(&[py::getitem(&v_p1.clone(), &Py::Int(0i64))?, py::getitem(&v_p2.clone(), &Py::Int(0i64))?], true)?; if py::le(&t2411, &t2412)? { true } else { false } }) }; if !t2418.truthy() { t2418 } else { { let t2417 = { let t2413 = py::minmax(&[py::getitem(&v_p1.clone(), &Py::Int(1i64))?, py::getitem(&v_p2.clone(), &Py::Int(1i64))?], false)?; Py::Bool({ let t2414 = py::minmax(&[py::getitem(&v_p3.clone(), &Py::Int(1i64))?, py::getitem(&v_p4.clone(), &Py::Int(1i64))?], true)?; if py::le(&t2413, &t2414)? { true } else { false } }) }; if !t2417.truthy() { t2417 } else { { let t2415 = py::minmax(&[py::getitem(&v_p3.clone(), &Py::Int(1i64))?, py::getitem(&v_p4.clone(), &Py::Int(1i64))?], false)?; Py::Bool({ let t2416 = py::minmax(&[py::getitem(&v_p1.clone(), &Py::Int(1i64))?, py::getitem(&v_p2.clone(), &Py::Int(1i64))?], true)?; if py::le(&t2415, &t2416)? { true } else { false } }) } } } } } } } } } } });
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    for t2422 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t2422;
        for t2423 in py::range_iter(&[(py::add(&v_i.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) {
            v_j = t2423;
            if ({ let t2431 = { let t2424 = v_j.clone(); Py::Bool({ let t2425 = py::add(&v_i.clone(), &Py::Int(1i64))?; if py::eq(&t2424, &t2425) { true } else { false } }) }; if t2431.truthy() { t2431 } else { { let t2430 = { let t2426 = v_i.clone(); Py::Bool({ let t2427 = Py::Int(0i64); if py::eq(&t2426, &t2427) { true } else { false } }) }; if !t2430.truthy() { t2430 } else { { let t2428 = v_j.clone(); Py::Bool({ let t2429 = py::sub(&v_n.clone(), &Py::Int(1i64))?; if py::eq(&t2428, &t2429) { true } else { false } }) } } } } }).truthy() {
                continue;
            }
            if (py::call(&v_hit.clone(), &[py::getitem(&v_poly.clone(), &v_i.clone())?, py::getitem(&v_poly.clone(), &py::modulo(&py::add(&v_i.clone(), &Py::Int(1i64))?, &v_n.clone())?)?, py::getitem(&v_poly.clone(), &v_j.clone())?, py::getitem(&v_poly.clone(), &py::modulo(&py::add(&v_j.clone(), &Py::Int(1i64))?, &v_n.clone())?)?])?).truthy() {
                return Ok(Py::Bool(false));
            }
        }
    }
    return Ok(Py::Bool(true));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__linfit(mut v_x: Py, mut v_y: Py) -> R {
    let mut v_b: Py = Py::None;
    let mut v_mx: Py = Py::None;
    let mut v_my: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_sxx: Py = Py::None;
    v_n = py::len(&v_x.clone())?;
    let t2634 = (Py::tuple(vec![py::truediv(&py::sum(&v_x.clone(), None)?, &v_n.clone())?, py::truediv(&py::sum(&v_y.clone(), None)?, &v_n.clone())?])).items()?;
    if t2634.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mx = t2634[0].clone();
    v_my = t2634[1].clone();
    v_sxx = py::sum(&{ let mut out = vec![]; for t2635 in py::iter(&(v_x.clone()))? { let mut v_a: Py; 
v_a = t2635;
out.push(py::pow(&py::sub(&v_a.clone(), &v_mx.clone())?, &Py::Int(2i64))?); } Py::list(out) }, None)?;
    let _ = f__need({ let t2636 = v_sxx.clone(); Py::Bool({ let t2637 = Py::Int(0i64); if py::gt(&t2636, &t2637)? { true } else { false } }) }, Py::str("the times must not all be equal"))?;
    v_b = py::truediv(&py::sum(&{ let mut out = vec![]; for t2638 in py::iter(&(py::zip(&[v_x.clone(), v_y.clone()])?))? { let mut v_a: Py; let mut v_c: Py; 
let t2639 = (t2638).items()?;
if t2639.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t2639[0].clone();
v_c = t2639[1].clone();
out.push(py::mul(&py::sub(&v_a.clone(), &v_mx.clone())?, &py::sub(&v_c.clone(), &v_my.clone())?)?); } Py::list(out) }, None)?, &v_sxx.clone())?;
    return Ok(Py::tuple(vec![v_b.clone(), py::sub(&v_my.clone(), &py::mul(&v_b.clone(), &v_mx.clone())?)?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__mann_out(mut v_A: Py, mut v_P: Py, mut v_S: Py, mut v_n: Py) -> R {
    let mut v_R: Py = Py::None;
    let mut v_V: Py = Py::None;
    v_R = py::truediv(&v_A.clone(), &v_P.clone())?;
    v_V = f__manning_v(v_R.clone(), v_S.clone(), v_n.clone())?;
    return Ok(Py::dict(vec![(Py::str("Q"), f__r(py::mul(&v_V.clone(), &v_A.clone())?)?), (Py::str("V"), f__r(v_V.clone())?), (Py::str("A"), f__r(v_A.clone())?), (Py::str("R"), f__r(v_R.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__manning_v(mut v_R: Py, mut v_S: Py, mut v_n: Py) -> R {
    return Ok(py::truediv(&py::mul(&py::pow(&v_R.clone(), &py::truediv(&Py::Float(2.0), &Py::Float(3.0))?)?, &py::math1("sqrt", &v_S.clone())?)?, &v_n.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__mean_sd(mut v_x: Py) -> R {
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    v_n = py::len(&v_x.clone())?;
    v_m = py::truediv(&py::sum(&v_x.clone(), None)?, &v_n.clone())?;
    return Ok(Py::tuple(vec![v_m.clone(), py::math1("sqrt", &py::truediv(&py::sum(&{ let mut out = vec![]; for t1396 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t1396;
out.push(py::pow(&py::sub(&v_v.clone(), &v_m.clone())?, &Py::Int(2i64))?); } Py::list(out) }, None)?, &py::sub(&v_n.clone(), &Py::Int(1i64))?)?)?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__need(mut v_cond: Py, mut v_message: Py) -> R {
    if (py::not(&v_cond.clone())).truthy() {
        return Err(py::value_error(py::to_str(&v_message.clone())));
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__network_setup(mut v_pipes: Py, mut v_L: Py, mut v_D: Py, mut v_C: Py, mut v_demands: Py) -> R {
    let mut v_N: Py = Py::None;
    let mut v_dem: Py = Py::None;
    let mut v_edges: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_nodes: Py = Py::None;
    v_m = py::len(&v_pipes.clone())?;
    v_L = f__vec(Py::str("L"), v_L.clone(), v_m.clone(), Py::Bool(true), Py::Bool(false))?;
    v_D = { let mut out = vec![]; for t958 in py::iter(&(f__vec(Py::str("D"), v_D.clone(), v_m.clone(), Py::Bool(true), Py::Bool(false))?))? { let mut v_d: Py; 
v_d = t958;
out.push(py::truediv(&v_d.clone(), &Py::Int(1000i64))?); } Py::list(out) };
    v_C = f__vec(Py::str("C"), v_C.clone(), v_m.clone(), Py::Bool(true), Py::Bool(false))?;
    for t959 in py::range_iter(&[(v_m.clone()).i()?]) {
        v_i = t959;
        let _ = f__ok_pipe(py::getitem(&v_L.clone(), &v_i.clone())?, py::getitem(&v_D.clone(), &v_i.clone())?, py::getitem(&v_C.clone(), &v_i.clone())?)?;
    }
    v_nodes = py::sorted(&(if (Py::Bool(({ let mut out = vec![]; for t974 in py::iter(&(v_pipes.clone()))? { let mut v_e: Py; 
v_e = t974;
for t975 in py::iter(&(py::slice(&v_e.clone(), None, Some((Py::Int(2i64)).i()?), None)?))? { let mut v_x: Py; 
v_x = t975;
out.push(py::method(&Py::str(&py::to_str(&v_x.clone())), "isdigit", &[])?); }} Py::list(out) }).items()?.iter().all(|x| x.truthy()))).truthy() { py::set(&{ let mut out = vec![]; for t976 in py::iter(&(v_pipes.clone()))? { let mut v_e: Py; 
v_e = t976;
for t977 in py::iter(&(py::slice(&v_e.clone(), None, Some((Py::Int(2i64)).i()?), None)?))? { let mut v_x: Py; 
v_x = t977;
out.push(py::int(&v_x.clone())?); }} Py::list(out) })? } else { Py::list(vec![py::neg(&Py::Int(1i64))?]) }), (Py::Bool(false)).truthy())?;
    let _ = f__need({ let t982 = { let t978 = v_nodes.clone(); Py::Bool({ let t979 = Py::list(vec![py::neg(&Py::Int(1i64))?]); if py::ne(&t978, &t979) { true } else { false } }) }; if !t982.truthy() { t982 } else { { let t980 = v_nodes.clone(); Py::Bool({ let t981 = Py::list((Py::list((py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&py::len(&v_nodes.clone())?, &Py::Int(1i64))?).i()?])).into_iter().collect::<Vec<Py>>())).items()?); if py::eq(&t980, &t981) { true } else { false } }) } } }, Py::str("name the nodes 1, 2, ..., N (node 1 is the source)"))?;
    v_N = py::len(&v_nodes.clone())?;
    v_dem = f__vec(Py::str("demands"), v_demands.clone(), v_N.clone(), Py::Bool(false), Py::Bool(true))?;
    let _ = f__need({ let t983 = py::getitem(&v_dem.clone(), &Py::Int(0i64))?; Py::Bool({ let t984 = Py::Int(0i64); if py::eq(&t983, &t984) { true } else { false } }) }, Py::str("node 1 is the source: its demand entry must be 0"))?;
    let _ = f__need({ let t985 = py::sum(&v_dem.clone(), None)?; Py::Bool({ let t986 = Py::Int(0i64); if py::gt(&t985, &t986)? { true } else { false } }) }, Py::str("give at least one positive demand"))?;
    v_edges = { let mut out = vec![]; for t987 in py::iter(&(v_pipes.clone()))? { let mut v_e: Py; 
v_e = t987;
out.push(Py::tuple(vec![py::int(&py::getitem(&v_e.clone(), &Py::Int(0i64))?)?, py::int(&py::getitem(&v_e.clone(), &Py::Int(1i64))?)?])); } Py::list(out) };
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t988 in py::iter(&(v_edges.clone()))? { let mut v_u: Py; let mut v_v: Py; 
let t989 = (t988).items()?;
if t989.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_u = t989[0].clone();
v_v = t989[1].clone();
out.push({ let t990 = v_u.clone(); Py::Bool({ let t991 = v_v.clone(); if py::ne(&t990, &t991) { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("a pipe must join two different nodes"))?;
    return Ok(Py::tuple(vec![v_edges.clone(), { let mut out = vec![]; for t992 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t992;
out.push(f__hw_k(py::getitem(&v_L.clone(), &v_i.clone())?, py::getitem(&v_D.clone(), &v_i.clone())?, py::getitem(&v_C.clone(), &v_i.clone())?)?); } Py::list(out) }, v_dem.clone(), v_N.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__newmark_I(mut v_m: Py, mut v_n: Py) -> R {
    let mut v_s: Py = Py::None;
    v_s = py::add(&py::add(&py::mul(&v_m.clone(), &v_m.clone())?, &py::mul(&v_n.clone(), &v_n.clone())?)?, &Py::Int(1i64))?;
    return Ok(py::truediv(&py::add(&py::truediv(&py::mul(&py::truediv(&py::mul(&py::mul(&py::mul(&Py::Int(2i64), &v_m.clone())?, &v_n.clone())?, &py::math1("sqrt", &v_s.clone())?)?, &py::add(&v_s.clone(), &py::mul(&py::mul(&py::mul(&v_m.clone(), &v_m.clone())?, &v_n.clone())?, &v_n.clone())?)?)?, &py::add(&v_s.clone(), &Py::Int(1i64))?)?, &v_s.clone())?, &py::atan2(&py::mul(&py::mul(&py::mul(&Py::Int(2i64), &v_m.clone())?, &v_n.clone())?, &py::math1("sqrt", &v_s.clone())?)?, &py::sub(&v_s.clone(), &py::mul(&py::mul(&py::mul(&v_m.clone(), &v_m.clone())?, &v_n.clone())?, &v_n.clone())?)?)?)?, &py::mul(&Py::Int(4i64), &Py::Float(std::f64::consts::PI))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__normal_depth(mut v_qfun: Py, mut v_Q: Py, mut v_ymax: Py) -> R {
    let _ = f__need({ let t1731 = py::call(&v_qfun.clone(), &[v_ymax.clone()])?; Py::Bool({ let t1732 = v_Q.clone(); if py::gt(&t1731, &t1732)? { true } else { false } }) }, Py::str("the discharge exceeds what the channel can carry below the depth limit"))?;
    return Ok(f__bisect_root({ let v_Q = v_Q.clone(); let v_qfun = v_qfun.clone(); let v_ymax = v_ymax.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_y: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::call(&v_qfun.clone(), &[v_y.clone()])?, &v_Q.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, Py::Float(0.0), v_ymax.clone(), Py::Int(300i64))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ok_mann(mut v_S: Py, mut v_n: Py) -> R {
    let _ = f__pos(Py::dict(vec![(Py::str("S"), v_S.clone()), (Py::str("n"), v_n.clone())]))?;
    let _ = f__need({ let t1723 = v_S.clone(); Py::Bool({ let t1724 = Py::Int(1i64); if py::lt(&t1723, &t1724)? { true } else { false } }) }, Py::str("the slope S is a fraction (m/m) below 1"))?;
    let _ = f__need({ let t1725 = Py::Float(0.005); Py::Bool({ let t1726 = v_n.clone(); if py::le(&t1725, &t1726)? { { let t1727 = Py::Float(0.2); if py::le(&t1726, &t1727)? { true } else { false } } } else { false } }) }, Py::str("Manning's n must lie between 0.005 and 0.2"))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ok_pipe(mut v_L: Py, mut v_D: Py, mut v_C: Py) -> R {
    let _ = f__pos(Py::dict(vec![(Py::str("L"), v_L.clone()), (Py::str("D"), v_D.clone()), (Py::str("C"), v_C.clone())]))?;
    let _ = f__need({ let t2499 = Py::Int(40i64); Py::Bool({ let t2500 = v_C.clone(); if py::le(&t2499, &t2500)? { { let t2501 = Py::Int(160i64); if py::le(&t2500, &t2501)? { true } else { false } } } else { false } }) }, Py::str("the Hazen-Williams C must lie between 40 and 160"))?;
    let _ = f__need({ let t2502 = v_D.clone(); Py::Bool({ let t2503 = Py::Int(5i64); if py::le(&t2502, &t2503)? { true } else { false } }) }, Py::str("the diameter must not exceed 5 m"))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pair(mut v_obs: Py, mut v_sim: Py) -> R {
    let mut v_o: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_o = f__vec(Py::str("observed"), v_obs.clone(), Py::None, Py::Bool(false), Py::Bool(false))?;
    v_s = f__vec(Py::str("simulated"), v_sim.clone(), py::len(&v_o.clone())?, Py::Bool(false), Py::Bool(false))?;
    let _ = f__need({ let t1628 = py::len(&v_o.clone())?; Py::Bool({ let t1629 = Py::Int(3i64); if py::ge(&t1628, &t1629)? { true } else { false } }) }, Py::str("give at least three pairs"))?;
    return Ok(Py::tuple(vec![v_o.clone(), v_s.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pos(mut v_kw: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__finite(v_kw.clone())?;
    for t2837 in py::iter(&(py::method(&v_kw.clone(), "items", &[])?))? {
        let t2840 = (t2837).items()?;
        if t2840.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2840[0].clone();
        v_v = t2840[1].clone();
        if ({ let t2838 = v_v.clone(); Py::Bool({ let t2839 = Py::Int(0i64); if py::le(&t2838, &t2839)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_k.clone(), "")?, " must be positive".to_string()].concat()))));
        }
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__r(mut v_x: Py) -> R {
    if (Py::Bool(matches!(v_x.clone(), Py::Bool(_) | Py::Str(_)))).truthy() {
        return Ok(v_x.clone());
    }
    if (Py::Bool(matches!(v_x.clone(), Py::List(_) | Py::Tuple(_)))).truthy() {
        return Ok({ let mut out = vec![]; for t2862 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t2862;
out.push(f__r(v_v.clone())?); } Py::list(out) });
    }
    v_x = py::float(&v_x.clone())?;
    if ({ let t2865 = { let t2863 = v_x.clone(); Py::Bool({ let t2864 = Py::Float(0.0); if py::eq(&t2863, &t2864) { true } else { false } }) }; if t2865.truthy() { t2865 } else { py::not(&py::math1("isfinite", &v_x.clone())?) } }).truthy() {
        return Ok((if ({ let t2866 = v_x.clone(); Py::Bool({ let t2867 = Py::Float(0.0); if py::eq(&t2866, &t2867) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_x.clone() }));
    }
    return Ok(py::add(&py::float(&Py::str(&[py::format(&v_x.clone(), ".12g")?].concat()))?, &Py::Float(0.0))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__rankine_active_K(mut v_phi: Py, mut v_gam: Py, mut v_H: Py, mut v_c: Py, mut v_q: Py, mut v_Ka: Py) -> R {
    let mut v_Pa: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_s0: Py = Py::None;
    let mut v_sH: Py = Py::None;
    let mut v_sig: Py = Py::None;
    let mut v_sk: Py = Py::None;
    let mut v_ybar: Py = Py::None;
    let mut v_z0: Py = Py::None;
    v_sk = py::math1("sqrt", &v_Ka.clone())?;
    v_sig = { let v_H = v_H.clone(); let v_Ka = v_Ka.clone(); let v_c = v_c.clone(); let v_gam = v_gam.clone(); let v_phi = v_phi.clone(); let v_q = v_q.clone(); let v_sk = v_sk.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_z: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::mul(&v_Ka.clone(), &py::add(&py::mul(&v_gam.clone(), &v_z.clone())?, &v_q.clone())?)?, &py::mul(&py::mul(&Py::Int(2i64), &v_c.clone())?, &v_sk.clone())?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_z0 = py::minmax(&[v_H.clone(), py::minmax(&[Py::Float(0.0), py::truediv(&py::sub(&py::truediv(&py::mul(&Py::Int(2i64), &v_c.clone())?, &v_sk.clone())?, &v_q.clone())?, &v_gam.clone())?], true)?], false)?;
    if ({ let t1965 = v_z0.clone(); Py::Bool({ let t1966 = v_H.clone(); if py::ge(&t1965, &t1966)? { true } else { false } }) }).truthy() {
        let t1967 = (Py::tuple(vec![Py::Float(0.0), Py::Float(0.0)])).items()?;
        if t1967.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_Pa = t1967[0].clone();
        v_ybar = t1967[1].clone();
    } else {
        let t1968 = (Py::tuple(vec![py::call(&v_sig.clone(), &[v_z0.clone()])?, py::call(&v_sig.clone(), &[v_H.clone()])?])).items()?;
        if t1968.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_s0 = t1968[0].clone();
        v_sH = t1968[1].clone();
        v_h = py::sub(&v_H.clone(), &v_z0.clone())?;
        v_Pa = py::mul(&py::truediv(&py::add(&v_s0.clone(), &v_sH.clone())?, &Py::Int(2i64))?, &v_h.clone())?;
        v_ybar = (if ({ let t1969 = v_Pa.clone(); Py::Bool({ let t1970 = Py::Int(0i64); if py::gt(&t1969, &t1970)? { true } else { false } }) }).truthy() { py::truediv(&py::mul(&v_h.clone(), &py::add(&py::mul(&Py::Int(2i64), &v_s0.clone())?, &v_sH.clone())?)?, &py::mul(&Py::Int(3i64), &py::add(&v_s0.clone(), &v_sH.clone())?)?)? } else { Py::Float(0.0) });
    }
    return Ok(Py::dict(vec![(Py::str("Ka"), f__r(v_Ka.clone())?), (Py::str("sigma_base"), f__r(py::call(&v_sig.clone(), &[v_H.clone()])?)?), (Py::str("Pa"), f__r(v_Pa.clone())?), (Py::str("z_crack"), f__r(v_z0.clone())?), (Py::str("ybar"), f__r(v_ybar.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__rect(mut v_b: Py, mut v_y: Py) -> R {
    return Ok(Py::tuple(vec![py::mul(&v_b.clone(), &v_y.clone())?, py::add(&v_b.clone(), &py::mul(&Py::Int(2i64), &v_y.clone())?)?, v_b.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__regime(mut v_Fr: Py) -> R {
    if ({ let t112 = py::abs(&py::sub(&v_Fr.clone(), &Py::Int(1i64))?)?; Py::Bool({ let t113 = Py::Float(1e-09); if py::le(&t112, &t113)? { true } else { false } }) }).truthy() {
        return Ok(Py::str("critical"));
    }
    return Ok((if ({ let t114 = v_Fr.clone(); Py::Bool({ let t115 = Py::Int(1i64); if py::lt(&t114, &t115)? { true } else { false } }) }).truthy() { Py::str("subcritical") } else { Py::str("supercritical") }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__rel(mut v_a: Py, mut v_b: Py) -> R {
    let mut v_d: Py = Py::None;
    if ({ let t1693 = Py::Bool(matches!(v_a.clone(), Py::List(_) | Py::Tuple(_))); if t1693.truthy() { t1693 } else { Py::Bool(matches!(v_b.clone(), Py::List(_) | Py::Tuple(_))) } }).truthy() {
        if ({ let t1697 = py::not(&{ let t1694 = Py::Bool(matches!(v_a.clone(), Py::List(_) | Py::Tuple(_))); if !t1694.truthy() { t1694 } else { Py::Bool(matches!(v_b.clone(), Py::List(_) | Py::Tuple(_))) } }); if t1697.truthy() { t1697 } else { { let t1695 = py::len(&v_a.clone())?; Py::Bool({ let t1696 = py::len(&v_b.clone())?; if py::ne(&t1695, &t1696) { true } else { false } }) } } }).truthy() {
            return Ok(Py::Float(1.0));
        }
        return Ok(py::minmax(&[{ let t1700 = { let mut out = vec![]; for t1698 in py::iter(&(py::zip(&[v_a.clone(), v_b.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t1699 = (t1698).items()?;
if t1699.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t1699[0].clone();
v_y = t1699[1].clone();
out.push(f__rel(v_x.clone(), v_y.clone())?); } Py::list(out) }; if t1700.truthy() { t1700 } else { Py::list(vec![Py::Float(0.0)]) } }], true)?);
    }
    let t1702: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            let t1701 = (Py::tuple(vec![py::float(&v_a.clone())?, py::float(&v_b.clone())?])).items()?;
            if t1701.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t1701[0].clone();
            v_b = t1701[1].clone();
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1702 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if (py::not(&{ let t1703 = py::math1("isfinite", &v_a.clone())?; if !t1703.truthy() { t1703 } else { py::math1("isfinite", &v_b.clone())? } })).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_d = py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?;
    if ({ let t1704 = v_d.clone(); Py::Bool({ let t1705 = Py::Float(1e-15); if py::le(&t1704, &t1705)? { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    return Ok(py::truediv(&v_d.clone(), &py::minmax(&[py::abs(&v_a.clone())?, py::abs(&v_b.clone())?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__section_out(mut v_A: Py, mut v_P: Py, mut v_T: Py) -> R {
    return Ok(Py::dict(vec![(Py::str("A"), f__r(v_A.clone())?), (Py::str("P"), f__r(v_P.clone())?), (Py::str("R"), f__r(py::truediv(&v_A.clone(), &v_P.clone())?)?), (Py::str("T"), f__r(v_T.clone())?), (Py::str("Dh"), f__r(py::truediv(&v_A.clone(), &v_T.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__shoelace(mut v_poly: Py) -> R {
    return Ok((if ({ let t2596 = py::len(&v_poly.clone())?; Py::Bool({ let t2597 = Py::Int(3i64); if py::ge(&t2596, &t2597)? { true } else { false } }) }).truthy() { py::mul(&Py::Float(0.5), &py::sum(&{ let mut out = vec![]; for t2598 in py::range_iter(&[(py::len(&v_poly.clone())?).i()?]) { let mut v_i: Py; 
v_i = t2598;
out.push(py::sub(&py::mul(&py::getitem(&py::getitem(&v_poly.clone(), &v_i.clone())?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_poly.clone(), &py::modulo(&py::add(&v_i.clone(), &Py::Int(1i64))?, &py::len(&v_poly.clone())?)?)?, &Py::Int(1i64))?)?, &py::mul(&py::getitem(&py::getitem(&v_poly.clone(), &py::modulo(&py::add(&v_i.clone(), &Py::Int(1i64))?, &py::len(&v_poly.clone())?)?)?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_poly.clone(), &v_i.clone())?, &Py::Int(1i64))?)?)?); } Py::list(out) }, None)?)? } else { Py::Float(0.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__sights(mut v_text: Py) -> R {
    let mut v_toks: Py = Py::None;
    v_toks = { let mut out = vec![]; for t1281 in py::iter(&(ax_engine::pyre::module_call("split", &Py::str("[\\s,;]+"), &[v_text.clone(), Py::Int(0)], 0)?))? { let mut v_t: Py; 
v_t = t1281;
if (py::method(&v_t.clone(), "strip", &[])?).truthy() { out.push(py::method(&py::method(&v_t.clone(), "strip", &[])?, "upper", &[])?); }} Py::list(out) };
    let _ = f__need({ let t1285 = v_toks.clone(); if !t1285.truthy() { t1285 } else { Py::Bool(({ let mut out = vec![]; for t1282 in py::iter(&(v_toks.clone()))? { let mut v_t: Py; 
v_t = t1282;
out.push({ let t1283 = v_t.clone(); Py::Bool({ let t1284 = Py::tuple(vec![Py::str("BS"), Py::str("IS"), Py::str("FS")]); if py::contains(&t1284, &t1283)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())) } }, Py::str("sights are BS, IS or FS, in reading order"))?;
    return Ok(v_toks.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__snap(mut v_x: Py, mut v_scale: Py) -> R {
    return Ok((if ({ let t2737 = py::abs(&v_x.clone())?; Py::Bool({ let t2738 = py::mul(&Py::Float(1e-12), &py::abs(&v_scale.clone())?)?; if py::le(&t2737, &t2738)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_x.clone() }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__solve_linear(mut v_A: Py, mut v_b: Py) -> R {
    let mut v_M: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_n = py::len(&v_b.clone())?;
    v_M = { let mut out = vec![]; for t2714 in py::iter(&(py::enumerate(&v_A.clone(), 0)?))? { let mut v_i: Py; let mut v_row: Py; 
let t2715 = (t2714).items()?;
if t2715.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_i = t2715[0].clone();
v_row = t2715[1].clone();
out.push(py::add(&py::slice(&v_row.clone(), None, None, None)?, &Py::list(vec![py::getitem(&v_b.clone(), &v_i.clone())?]))?); } Py::list(out) };
    for t2716 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_c = t2716;
        v_p = py::minmax_key(&[Py::list((py::range_iter(&[(v_c.clone()).i()?, (v_n.clone()).i()?])).into_iter().collect::<Vec<Py>>())], &{ let v_A = v_A.clone(); let v_M = v_M.clone(); let v_b = v_b.clone(); let v_c = v_c.clone(); let v_n = v_n.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_r: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::abs(&py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_c.clone())?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, true)?;
        let _ = f__need({ let t2717 = py::abs(&py::getitem(&py::getitem(&v_M.clone(), &v_p.clone())?, &v_c.clone())?)?; Py::Bool({ let t2718 = Py::Float(1e-300); if py::gt(&t2717, &t2718)? { true } else { false } }) }, Py::str("singular loop system"))?;
        let t2719 = (Py::tuple(vec![py::getitem(&v_M.clone(), &v_p.clone())?, py::getitem(&v_M.clone(), &v_c.clone())?])).items()?;
        if t2719.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        py::setitem(&v_M.clone(), &v_c.clone(), t2719[0].clone())?;
        py::setitem(&v_M.clone(), &v_p.clone(), t2719[1].clone())?;
        for t2720 in py::range_iter(&[(py::add(&v_c.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) {
            v_r = t2720;
            v_f = py::truediv(&py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_c.clone())?, &py::getitem(&py::getitem(&v_M.clone(), &v_c.clone())?, &v_c.clone())?)?;
            if (v_f.clone()).truthy() {
                for t2721 in py::range_iter(&[(v_c.clone()).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
                    v_k = t2721;
                    let t2722 = py::getitem(&v_M.clone(), &v_r.clone())?; let t2723 = v_k.clone();
                    py::setitem(&t2722, &t2723, py::sub(&py::getitem(&t2722, &t2723)?, &py::mul(&v_f.clone(), &py::getitem(&py::getitem(&v_M.clone(), &v_c.clone())?, &v_k.clone())?)?)?)?;
                }
            }
        }
    }
    v_x = py::mul(&Py::list(vec![Py::Float(0.0)]), &v_n.clone())?;
    for t2724 in py::range_iter(&[(py::sub(&v_n.clone(), &Py::Int(1i64))?).i()?, (py::neg(&Py::Int(1i64))?).i()?, (py::neg(&Py::Int(1i64))?).i()?]) {
        v_r = t2724;
        py::setitem(&v_x.clone(), &v_r.clone(), py::truediv(&py::sub(&py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_n.clone())?, &py::sum(&{ let mut out = vec![]; for t2725 in py::range_iter(&[(py::add(&v_r.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t2725;
out.push(py::mul(&py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_k.clone())?, &py::getitem(&v_x.clone(), &v_k.clone())?)?); } Py::list(out) }, None)?)?, &py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_r.clone())?)?)?;
    }
    return Ok(v_x.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__trap(mut v_b: Py, mut v_y: Py, mut v_z: Py) -> R {
    return Ok(Py::tuple(vec![py::mul(&py::add(&v_b.clone(), &py::mul(&v_z.clone(), &v_y.clone())?)?, &v_y.clone())?, py::add(&v_b.clone(), &py::mul(&py::mul(&Py::Int(2i64), &v_y.clone())?, &py::math1("sqrt", &py::add(&Py::Int(1i64), &py::mul(&v_z.clone(), &v_z.clone())?)?)?)?)?, py::add(&v_b.clone(), &py::mul(&py::mul(&Py::Int(2i64), &v_z.clone())?, &v_y.clone())?)?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__tree_and_loops(mut v_edges: Py, mut v_N: Py) -> R {
    let mut v_adj: Py = Py::None;
    let mut v_coef: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_jj: Py = Py::None;
    let mut v_loops: Py = Py::None;
    let mut v_order: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_parent: Py = Py::None;
    let mut v_path_to_root: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_tree: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    v_adj = { let mut out = vec![]; for t994 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_N.clone(), &Py::Int(1i64))?).i()?]) { let mut v_i: Py; 
v_i = t994;
out.push((v_i.clone(), Py::list(vec![]))); } Py::dict(out) };
    for t995 in py::iter(&(py::enumerate(&v_edges.clone(), 0)?))? {
        let t996 = (t995).items()?;
        if t996.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_j = t996[0].clone();
        let t997 = (t996[1].clone()).items()?;
        if t997.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_u = t997[0].clone();
        v_v = t997[1].clone();
        let _ = py::method(&py::getitem(&v_adj.clone(), &v_u.clone())?, "append", &[Py::tuple(vec![v_v.clone(), v_j.clone(), Py::Int(1i64)])])?;
        let _ = py::method(&py::getitem(&v_adj.clone(), &v_v.clone())?, "append", &[Py::tuple(vec![v_u.clone(), v_j.clone(), py::neg(&Py::Int(1i64))?])])?;
    }
    v_parent = Py::dict(vec![(Py::Int(1i64), Py::None)]);
    v_order = Py::list(vec![Py::Int(1i64)]);
    for t998 in py::iter(&(v_order.clone()))? {
        v_x = t998;
        for t999 in py::iter(&(py::getitem(&v_adj.clone(), &v_x.clone())?))? {
            let t1002 = (t999).items()?;
            if t1002.len() != 3 { return Err(PyErr::Value("unpack".into())); }
            v_y = t1002[0].clone();
            v_j = t1002[1].clone();
            v_s = t1002[2].clone();
            if ({ let t1000 = v_y.clone(); Py::Bool({ let t1001 = v_parent.clone(); if !py::contains(&t1001, &t1000)? { true } else { false } }) }).truthy() {
                py::setitem(&v_parent.clone(), &v_y.clone(), Py::tuple(vec![v_x.clone(), v_j.clone(), v_s.clone()]))?;
                let _ = py::method(&v_order.clone(), "append", &[v_y.clone()])?;
            }
        }
    }
    let _ = f__need({ let t1003 = py::len(&v_parent.clone())?; Py::Bool({ let t1004 = v_N.clone(); if py::eq(&t1003, &t1004) { true } else { false } }) }, Py::str("the network must be connected"))?;
    v_tree = py::set(&{ let mut out = vec![]; for t1005 in py::iter(&(py::slice(&v_order.clone(), Some((Py::Int(1i64)).i()?), None, None)?))? { let mut v_x: Py; 
v_x = t1005;
out.push(py::getitem(&py::getitem(&v_parent.clone(), &v_x.clone())?, &Py::Int(1i64))?); } Py::list(out) })?;
    v_path_to_root = { let v_N = v_N.clone(); let v_adj = v_adj.clone(); let v_edges = v_edges.clone(); let v_j = v_j.clone(); let v_order = v_order.clone(); let v_parent = v_parent.clone(); let v_path_to_root = v_path_to_root.clone(); let v_s = v_s.clone(); let v_tree = v_tree.clone(); let v_u = v_u.clone(); let v_v = v_v.clone(); let v_y = v_y.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_j: Py = Py::None;
            let mut v_out: Py = Py::None;
            let mut v_p: Py = Py::None;
            let mut v_s: Py = Py::None;
            v_out = Py::list(vec![]);
            while ({ let t1007 = py::getitem(&v_parent.clone(), &v_x.clone())?; Py::Bool({ let t1008 = Py::None; if !py::is(&t1007, &t1008) { true } else { false } }) }).truthy() {
                let t1006 = (py::getitem(&v_parent.clone(), &v_x.clone())?).items()?;
                if t1006.len() != 3 { return Err(PyErr::Value("unpack".into())); }
                v_p = t1006[0].clone();
                v_j = t1006[1].clone();
                v_s = t1006[2].clone();
                let _ = py::method(&v_out.clone(), "append", &[Py::tuple(vec![v_j.clone(), v_s.clone(), v_p.clone(), v_x.clone()])])?;
                v_x = v_p.clone();
            }
            return Ok(v_out.clone());
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_loops = Py::list(vec![]);
    for t1009 in py::iter(&(py::enumerate(&v_edges.clone(), 0)?))? {
        let t1018 = (t1009).items()?;
        if t1018.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_j = t1018[0].clone();
        let t1019 = (t1018[1].clone()).items()?;
        if t1019.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_u = t1019[0].clone();
        v_v = t1019[1].clone();
        if ({ let t1010 = v_j.clone(); Py::Bool({ let t1011 = v_tree.clone(); if py::contains(&t1011, &t1010)? { true } else { false } }) }).truthy() {
            continue;
        }
        v_coef = Py::dict(vec![(v_j.clone(), Py::Int(1i64))]);
        for t1012 in py::iter(&(py::call(&v_path_to_root.clone(), &[v_v.clone()])?))? {
            let t1013 = (t1012).items()?;
            if t1013.len() != 4 { return Err(PyErr::Value("unpack".into())); }
            v_jj = t1013[0].clone();
            v_s = t1013[1].clone();
            v_p = t1013[2].clone();
            v_x = t1013[3].clone();
            py::setitem(&v_coef.clone(), &v_jj.clone(), py::sub(&py::method(&v_coef.clone(), "get", &[v_jj.clone(), Py::Int(0i64)])?, &v_s.clone())?)?;
        }
        for t1014 in py::iter(&(py::call(&v_path_to_root.clone(), &[v_u.clone()])?))? {
            let t1015 = (t1014).items()?;
            if t1015.len() != 4 { return Err(PyErr::Value("unpack".into())); }
            v_jj = t1015[0].clone();
            v_s = t1015[1].clone();
            v_p = t1015[2].clone();
            v_x = t1015[3].clone();
            py::setitem(&v_coef.clone(), &v_jj.clone(), py::add(&py::method(&v_coef.clone(), "get", &[v_jj.clone(), Py::Int(0i64)])?, &v_s.clone())?)?;
        }
        let _ = py::method(&v_loops.clone(), "append", &[{ let mut out = vec![]; for t1016 in py::iter(&(py::method(&v_coef.clone(), "items", &[])?))? { let mut v_c: Py; let mut v_k: Py; 
let t1017 = (t1016).items()?;
if t1017.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t1017[0].clone();
v_c = t1017[1].clone();
if (v_c.clone()).truthy() { out.push((v_k.clone(), v_c.clone())); }} Py::dict(out) }])?;
    }
    return Ok(Py::tuple(vec![v_parent.clone(), v_order.clone(), v_loops.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__vec(mut v_name: Py, mut v_v: Py, mut v_n: Py, mut v_positive: Py, mut v_nonneg: Py) -> R {
    let mut v_x: Py = Py::None;
    let _ = f__need({ let t2819 = Py::Bool(matches!(v_v.clone(), Py::List(_) | Py::Tuple(_))); if !t2819.truthy() { t2819 } else { { let t2817 = py::len(&v_v.clone())?; Py::Bool({ let t2818 = Py::Int(0i64); if py::gt(&t2817, &t2818)? { true } else { false } }) } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be a list of numbers".to_string()].concat()))?;
    for t2820 in py::iter(&(v_v.clone()))? {
        v_x = t2820;
        let _ = f__need({ let t2822 = Py::Bool(matches!(v_x.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t2822.truthy() { t2822 } else { { let t2821 = py::not(&Py::Bool(matches!(v_x.clone(), Py::Bool(_)))); if !t2821.truthy() { t2821 } else { py::math1("isfinite", &v_x.clone())? } } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must hold finite numbers".to_string()].concat()))?;
        if (v_positive.clone()).truthy() {
            let _ = f__need({ let t2823 = v_x.clone(); Py::Bool({ let t2824 = Py::Int(0i64); if py::gt(&t2823, &t2824)? { true } else { false } }) }, Py::str(&["every entry of ".to_string(), py::format(&v_name.clone(), "")?, " must be positive".to_string()].concat()))?;
        }
        if (v_nonneg.clone()).truthy() {
            let _ = f__need({ let t2825 = v_x.clone(); Py::Bool({ let t2826 = Py::Int(0i64); if py::ge(&t2825, &t2826)? { true } else { false } }) }, Py::str(&["every entry of ".to_string(), py::format(&v_name.clone(), "")?, " must be zero or positive".to_string()].concat()))?;
        }
    }
    if ({ let t2827 = v_n.clone(); Py::Bool({ let t2828 = Py::None; if !py::is(&t2827, &t2828) { true } else { false } }) }).truthy() {
        let _ = f__need({ let t2829 = py::len(&v_v.clone())?; Py::Bool({ let t2830 = v_n.clone(); if py::eq(&t2829, &t2830) { true } else { false } }) }, Py::str(&[py::format(&v_name.clone(), "")?, " must have ".to_string(), py::format(&v_n.clone(), "")?, " entries".to_string()].concat()))?;
    }
    return Ok({ let mut out = vec![]; for t2831 in py::iter(&(v_v.clone()))? { let mut v_x: Py; 
v_x = t2831;
out.push(py::float(&v_x.clone())?); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_alternate_depth(mut v_Q: Py, mut v_b: Py, mut v_y: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_Fr: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_y2: Py = Py::None;
    let mut v_yc: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("Q"), v_Q.clone()), (Py::str("b"), v_b.clone()), (Py::str("y"), v_y.clone())]))?;
    v_q = py::truediv(&v_Q.clone(), &v_b.clone())?;
    v_yc = py::pow(&py::truediv(&py::mul(&v_q.clone(), &v_q.clone())?, &c_G()?)?, &py::truediv(&Py::Float(1.0), &Py::Float(3.0))?)?;
    v_E = py::add(&v_y.clone(), &py::truediv(&py::mul(&v_q.clone(), &v_q.clone())?, &py::mul(&py::mul(&py::mul(&Py::Int(2i64), &c_G()?)?, &v_y.clone())?, &v_y.clone())?)?)?;
    v_Fr = py::truediv(&v_q.clone(), &py::mul(&v_y.clone(), &py::math1("sqrt", &py::mul(&c_G()?, &v_y.clone())?)?)?)?;
    let _ = f__need({ let t10 = py::abs(&py::sub(&v_Fr.clone(), &Py::Int(1i64))?)?; Py::Bool({ let t11 = Py::Float(0.001); if py::gt(&t10, &t11)? { true } else { false } }) }, Py::str("the flow is at critical depth: there is no distinct alternate depth"))?;
    v_f = { let v_E = v_E.clone(); let v_Fr = v_Fr.clone(); let v_Q = v_Q.clone(); let v_b = v_b.clone(); let v_q = v_q.clone(); let v_y = v_y.clone(); let v_yc = v_yc.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_v: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::add(&v_v.clone(), &py::truediv(&py::mul(&v_q.clone(), &v_q.clone())?, &py::mul(&py::mul(&py::mul(&Py::Int(2i64), &c_G()?)?, &v_v.clone())?, &v_v.clone())?)?)?, &v_E.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_y2 = (if ({ let t12 = v_y.clone(); Py::Bool({ let t13 = v_yc.clone(); if py::lt(&t12, &t13)? { true } else { false } }) }).truthy() { f__bisect_root(v_f.clone(), v_yc.clone(), v_E.clone(), Py::Int(300i64))? } else { f__bisect_root(v_f.clone(), py::add(&Py::Float(1e-300), &py::mul(&v_yc.clone(), &Py::Float(1e-09))?)?, v_yc.clone(), Py::Int(300i64))? });
    return Ok(Py::dict(vec![(Py::str("E"), f__r(v_E.clone())?), (Py::str("y_alt"), f__r(v_y2.clone())?), (Py::str("yc"), f__r(v_yc.clone())?), (Py::str("Fr"), f__r(v_Fr.clone())?), (Py::str("regime"), f__regime(v_Fr.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_atterberg_indices(mut v_LL: Py, mut v_PL: Py, mut v_w: Py) -> R {
    let mut v_PI: Py = Py::None;
    let mut v_out: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("LL"), v_LL.clone()), (Py::str("PL"), v_PL.clone())]))?;
    let _ = f__need({ let t1875 = Py::Int(5i64); Py::Bool({ let t1876 = v_LL.clone(); if py::le(&t1875, &t1876)? { { let t1877 = Py::Int(1000i64); if py::le(&t1876, &t1877)? { true } else { false } } } else { false } }) }, Py::str("give the liquid limit in percent (5 to 1000)"))?;
    let _ = f__need({ let t1878 = Py::Int(0i64); Py::Bool({ let t1879 = v_PL.clone(); if py::le(&t1878, &t1879)? { { let t1880 = v_LL.clone(); if py::lt(&t1879, &t1880)? { true } else { false } } } else { false } }) }, Py::str("the plastic limit must lie below the liquid limit"))?;
    v_PI = py::sub(&v_LL.clone(), &v_PL.clone())?;
    v_out = Py::dict(vec![(Py::str("PI"), f__r(v_PI.clone())?), (Py::str("A_line_PI"), f__r(py::mul(&Py::Float(0.73), &py::sub(&v_LL.clone(), &Py::Int(20i64))?)?)?)]);
    if ({ let t1887 = v_w.clone(); Py::Bool({ let t1888 = Py::None; if !py::is(&t1887, &t1888) { true } else { false } }) }).truthy() {
        let _ = f__finite(Py::dict(vec![(Py::str("w"), v_w.clone())]))?;
        let _ = f__need({ let t1889 = Py::Int(0i64); Py::Bool({ let t1890 = v_w.clone(); if py::le(&t1889, &t1890)? { { let t1891 = Py::Int(2000i64); if py::le(&t1890, &t1891)? { true } else { false } } } else { false } }) }, Py::str("give the natural water content in percent"))?;
        py::setitem(&v_out.clone(), &Py::str("LI"), f__r(py::truediv(&py::sub(&v_w.clone(), &v_PL.clone())?, &v_PI.clone())?)?)?;
        py::setitem(&v_out.clone(), &Py::str("CI"), f__r(py::truediv(&py::sub(&v_LL.clone(), &v_w.clone())?, &v_PI.clone())?)?)?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bearing_strip(mut v_c: Py, mut v_phi: Py, mut v_gam: Py, mut v_B: Py, mut v_Df: Py, mut v_FS: Py) -> R {
    let mut v_Nc: Py = Py::None;
    let mut v_Ng: Py = Py::None;
    let mut v_Nq: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_qu: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("c"), v_c.clone()), (Py::str("phi"), v_phi.clone())]))?;
    let _ = f__pos(Py::dict(vec![(Py::str("gam"), v_gam.clone()), (Py::str("B"), v_B.clone()), (Py::str("FS"), v_FS.clone())]))?;
    let _ = f__finite(Py::dict(vec![(Py::str("Df"), v_Df.clone())]))?;
    let _ = f__need({ let t2229 = { let t2225 = v_c.clone(); Py::Bool({ let t2226 = Py::Int(0i64); if py::ge(&t2225, &t2226)? { true } else { false } }) }; if !t2229.truthy() { t2229 } else { { let t2227 = v_Df.clone(); Py::Bool({ let t2228 = Py::Int(0i64); if py::ge(&t2227, &t2228)? { true } else { false } }) } } }, Py::str("c and Df must be zero or positive"))?;
    let _ = f__need({ let t2230 = Py::Int(20i64); Py::Bool({ let t2231 = v_phi.clone(); if py::le(&t2230, &t2231)? { { let t2232 = Py::Int(45i64); if py::le(&t2231, &t2232)? { true } else { false } } } else { false } }) }, Py::str("the friction angle must lie between 20 and 45 degrees"))?;
    v_p = py::math1("radians", &v_phi.clone())?;
    v_Nq = py::mul(&py::math1("exp", &py::mul(&Py::Float(std::f64::consts::PI), &py::math1("tan", &v_p.clone())?)?)?, &py::pow(&py::math1("tan", &py::add(&py::truediv(&Py::Float(std::f64::consts::PI), &Py::Int(4i64))?, &py::truediv(&v_p.clone(), &Py::Int(2i64))?)?)?, &Py::Int(2i64))?)?;
    v_Nc = py::truediv(&py::sub(&v_Nq.clone(), &Py::Int(1i64))?, &py::math1("tan", &v_p.clone())?)?;
    v_Ng = py::mul(&py::mul(&Py::Int(2i64), &py::add(&v_Nq.clone(), &Py::Int(1i64))?)?, &py::math1("tan", &v_p.clone())?)?;
    v_qu = py::add(&py::add(&py::mul(&v_c.clone(), &v_Nc.clone())?, &py::mul(&py::mul(&v_gam.clone(), &v_Df.clone())?, &v_Nq.clone())?)?, &py::mul(&py::mul(&py::mul(&Py::Float(0.5), &v_gam.clone())?, &v_B.clone())?, &v_Ng.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("qu"), f__r(v_qu.clone())?), (Py::str("qa"), f__r(py::truediv(&v_qu.clone(), &v_FS.clone())?)?), (Py::str("Nc"), f__r(v_Nc.clone())?), (Py::str("Nq"), f__r(v_Nq.clone())?), (Py::str("Ngamma"), f__r(v_Ng.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_broad_weir(mut v_Cd: Py, mut v_b: Py, mut v_H: Py) -> R {
    let mut v_yc: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("Cd"), v_Cd.clone()), (Py::str("b"), v_b.clone()), (Py::str("H"), v_H.clone())]))?;
    let _ = f__need({ let t89 = v_Cd.clone(); Py::Bool({ let t90 = Py::Float(1.2); if py::le(&t89, &t90)? { true } else { false } }) }, Py::str("the discharge coefficient must not exceed 1.2"))?;
    v_yc = py::truediv(&py::mul(&Py::Float(2.0), &v_H.clone())?, &Py::Float(3.0))?;
    return Ok(Py::dict(vec![(Py::str("Q"), f__r(py::mul(&py::mul(&py::mul(&v_Cd.clone(), &v_b.clone())?, &py::math1("sqrt", &c_G()?)?)?, &py::pow(&v_yc.clone(), &Py::Float(1.5))?)?)?), (Py::str("yc"), f__r(v_yc.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_chezy_from_manning(mut v_n: Py, mut v_R: Py) -> R {
    let _ = f__pos(Py::dict(vec![(Py::str("R"), v_R.clone())]))?;
    let _ = f__ok_mann(Py::Float(0.5), v_n.clone())?;
    return Ok(Py::dict(vec![(Py::str("C"), f__r(py::truediv(&py::pow(&v_R.clone(), &py::truediv(&Py::Float(1.0), &Py::Float(6.0))?)?, &v_n.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_chezy_velocity(mut v_C: Py, mut v_R: Py, mut v_S: Py) -> R {
    let _ = f__pos(Py::dict(vec![(Py::str("C"), v_C.clone()), (Py::str("R"), v_R.clone()), (Py::str("S"), v_S.clone())]))?;
    let _ = f__need({ let t145 = v_S.clone(); Py::Bool({ let t146 = Py::Int(1i64); if py::lt(&t145, &t146)? { true } else { false } }) }, Py::str("the slope S is a fraction (m/m) below 1"))?;
    return Ok(Py::dict(vec![(Py::str("V"), f__r(py::mul(&v_C.clone(), &py::math1("sqrt", &py::mul(&v_R.clone(), &v_S.clone())?)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_circ_section(mut v_D: Py, mut v_y: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_T: Py = Py::None;
    let mut v_th: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("D"), v_D.clone()), (Py::str("y"), v_y.clone())]))?;
    let _ = f__need({ let t162 = v_y.clone(); Py::Bool({ let t163 = v_D.clone(); if py::lt(&t162, &t163)? { true } else { false } }) }, Py::str("the depth must be below the diameter (a part-full section)"))?;
    let t164 = (f__circ(v_D.clone(), v_y.clone())?).items()?;
    if t164.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_A = t164[0].clone();
    v_P = t164[1].clone();
    v_T = t164[2].clone();
    v_th = t164[3].clone();
    return Ok(py::dict_from(&f__section_out(v_A.clone(), v_P.clone(), v_T.clone())?, vec![(Py::str("theta"), f__r(py::math1("degrees", &v_th.clone())?)?)])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_circle_load(mut v_q: Py, mut v_a: Py, mut v_z: Py) -> R {
    let mut v_I: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("q"), v_q.clone()), (Py::str("a"), v_a.clone()), (Py::str("z"), v_z.clone())]))?;
    v_I = py::sub(&Py::Int(1i64), &py::pow(&py::truediv(&Py::Int(1i64), &py::add(&Py::Int(1i64), &py::pow(&py::truediv(&v_a.clone(), &v_z.clone())?, &Py::Int(2i64))?)?)?, &Py::Float(1.5))?)?;
    return Ok(Py::dict(vec![(Py::str("dsigma"), f__r(py::mul(&v_q.clone(), &v_I.clone())?)?), (Py::str("I"), f__r(v_I.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_consolidation_nc(mut v_Cc: Py, mut v_e0: Py, mut v_H: Py, mut v_s0: Py, mut v_ds: Py) -> R {
    let mut v_de: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("Cc"), v_Cc.clone()), (Py::str("e0"), v_e0.clone()), (Py::str("H"), v_H.clone()), (Py::str("s0"), v_s0.clone()), (Py::str("ds"), v_ds.clone())]))?;
    v_de = py::mul(&v_Cc.clone(), &py::math1("log10", &py::truediv(&py::add(&v_s0.clone(), &v_ds.clone())?, &v_s0.clone())?)?)?;
    return Ok(Py::dict(vec![(Py::str("Sc"), f__r(py::truediv(&py::mul(&v_H.clone(), &v_de.clone())?, &py::add(&Py::Int(1i64), &v_e0.clone())?)?)?), (Py::str("de"), f__r(v_de.clone())?), (Py::str("e_final"), f__r(py::sub(&v_e0.clone(), &v_de.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_consolidation_oc(mut v_Cc: Py, mut v_Cs: Py, mut v_e0: Py, mut v_H: Py, mut v_s0: Py, mut v_ds: Py, mut v_sp: Py) -> R {
    let mut v_case: Py = Py::None;
    let mut v_de: Py = Py::None;
    let mut v_s1: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("Cc"), v_Cc.clone()), (Py::str("Cs"), v_Cs.clone()), (Py::str("e0"), v_e0.clone()), (Py::str("H"), v_H.clone()), (Py::str("s0"), v_s0.clone()), (Py::str("ds"), v_ds.clone()), (Py::str("sp"), v_sp.clone())]))?;
    let _ = f__need({ let t238 = v_Cs.clone(); Py::Bool({ let t239 = v_Cc.clone(); if py::lt(&t238, &t239)? { true } else { false } }) }, Py::str("the swelling index must be smaller than the compression index"))?;
    let _ = f__need({ let t240 = v_sp.clone(); Py::Bool({ let t241 = v_s0.clone(); if py::ge(&t240, &t241)? { true } else { false } }) }, Py::str("the preconsolidation stress cannot be below the present effective stress"))?;
    v_s1 = py::add(&v_s0.clone(), &v_ds.clone())?;
    if ({ let t242 = v_s1.clone(); Py::Bool({ let t243 = v_sp.clone(); if py::le(&t242, &t243)? { true } else { false } }) }).truthy() {
        let t244 = (Py::tuple(vec![py::mul(&v_Cs.clone(), &py::math1("log10", &py::truediv(&v_s1.clone(), &v_s0.clone())?)?)?, Py::str("stays overconsolidated")])).items()?;
        if t244.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_de = t244[0].clone();
        v_case = t244[1].clone();
    } else {
        let t245 = (Py::tuple(vec![py::add(&py::mul(&v_Cs.clone(), &py::math1("log10", &py::truediv(&v_sp.clone(), &v_s0.clone())?)?)?, &py::mul(&v_Cc.clone(), &py::math1("log10", &py::truediv(&v_s1.clone(), &v_sp.clone())?)?)?)?, Py::str("passes the preconsolidation stress")])).items()?;
        if t245.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_de = t245[0].clone();
        v_case = t245[1].clone();
    }
    return Ok(Py::dict(vec![(Py::str("Sc"), f__r(py::truediv(&py::mul(&v_H.clone(), &v_de.clone())?, &py::add(&Py::Int(1i64), &v_e0.clone())?)?)?), (Py::str("de"), f__r(v_de.clone())?), (Py::str("OCR"), f__r(py::truediv(&v_sp.clone(), &v_s0.clone())?)?), (Py::str("case"), v_case.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_consolidation_time(mut v_cv: Py, mut v_Hdr: Py, mut v_U: Py) -> R {
    let mut v_Tv: Py = Py::None;
    let mut v_t: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("cv"), v_cv.clone()), (Py::str("Hdr"), v_Hdr.clone())]))?;
    let _ = f__finite(Py::dict(vec![(Py::str("U"), v_U.clone())]))?;
    let _ = f__need({ let t261 = Py::Float(0.01); Py::Bool({ let t262 = v_U.clone(); if py::le(&t261, &t262)? { { let t263 = Py::Float(0.99); if py::le(&t262, &t263)? { true } else { false } } } else { false } }) }, Py::str("the degree of consolidation must lie between 1% and 99%"))?;
    v_Tv = f__Tv_for(v_U.clone())?;
    v_t = py::truediv(&py::mul(&py::mul(&v_Tv.clone(), &v_Hdr.clone())?, &v_Hdr.clone())?, &v_cv.clone())?;
    return Ok(Py::dict(vec![(Py::str("Tv"), f__r(v_Tv.clone())?), (Py::str("t"), f__r(v_t.clone())?), (Py::str("t_days"), f__r(py::truediv(&v_t.clone(), &Py::Int(86400i64))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cooper_jacob(mut v_times: Py, mut v_drawdowns: Py, mut v_Q: Py, mut v_r: Py) -> R {
    let mut v_S: Py = Py::None;
    let mut v_T: Py = Py::None;
    let mut v_icpt: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_slope: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_t0: Py = Py::None;
    v_t = f__vec(Py::str("times"), v_times.clone(), Py::None, Py::Bool(true), Py::Bool(false))?;
    v_s = f__vec(Py::str("drawdowns"), v_drawdowns.clone(), py::len(&v_t.clone())?, Py::Bool(false), Py::Bool(false))?;
    let _ = f__need({ let t310 = py::len(&v_t.clone())?; Py::Bool({ let t311 = Py::Int(2i64); if py::ge(&t310, &t311)? { true } else { false } }) }, Py::str("give at least two readings"))?;
    let _ = f__pos(Py::dict(vec![(Py::str("Q"), v_Q.clone()), (Py::str("r"), v_r.clone())]))?;
    let t328 = (f__linfit({ let mut out = vec![]; for t327 in py::iter(&(v_t.clone()))? { let mut v_v: Py; 
v_v = t327;
out.push(py::math1("log10", &v_v.clone())?); } Py::list(out) }, v_s.clone())?).items()?;
    if t328.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_slope = t328[0].clone();
    v_icpt = t328[1].clone();
    let _ = f__need({ let t329 = v_slope.clone(); Py::Bool({ let t330 = Py::Int(0i64); if py::gt(&t329, &t330)? { true } else { false } }) }, Py::str("the drawdown must increase with time"))?;
    v_T = py::truediv(&py::mul(&Py::Float(2.303), &v_Q.clone())?, &py::mul(&py::mul(&Py::Int(4i64), &Py::Float(std::f64::consts::PI))?, &v_slope.clone())?)?;
    v_t0 = py::mul(&py::pow(&Py::Int(10i64), &py::truediv(&py::neg(&v_icpt.clone())?, &v_slope.clone())?)?, &Py::Int(60i64))?;
    v_S = py::truediv(&py::mul(&py::mul(&Py::Float(2.25), &v_T.clone())?, &v_t0.clone())?, &py::mul(&v_r.clone(), &v_r.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("T"), f__r(v_T.clone())?), (Py::str("S"), f__r(v_S.clone())?), (Py::str("ds_per_cycle"), f__r(v_slope.clone())?), (Py::str("t0_min"), f__r(py::truediv(&v_t0.clone(), &Py::Int(60i64))?)?), (Py::str("T_m2_d"), f__r(py::mul(&v_T.clone(), &Py::Int(86400i64))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_coulomb_active(mut v_phi: Py, mut v_delta: Py, mut v_beta: Py, mut v_gam: Py, mut v_H: Py) -> R {
    let mut v_Ka: Py = Py::None;
    let mut v_Pa: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_p: Py = Py::None;
    let _ = f__coulomb_check(v_phi.clone(), v_delta.clone(), v_beta.clone())?;
    let _ = f__pos(Py::dict(vec![(Py::str("gam"), v_gam.clone()), (Py::str("H"), v_H.clone())]))?;
    let t355 = (Py::tuple(vec![py::math1("radians", &v_phi.clone())?, py::math1("radians", &v_delta.clone())?, py::math1("radians", &v_beta.clone())?])).items()?;
    if t355.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_p = t355[0].clone();
    v_d = t355[1].clone();
    v_b = t355[2].clone();
    v_Ka = py::truediv(&py::pow(&py::math1("cos", &v_p.clone())?, &Py::Int(2i64))?, &py::mul(&py::math1("cos", &v_d.clone())?, &py::pow(&py::add(&Py::Int(1i64), &py::math1("sqrt", &py::truediv(&py::mul(&py::math1("sin", &py::add(&v_p.clone(), &v_d.clone())?)?, &py::math1("sin", &py::sub(&v_p.clone(), &v_b.clone())?)?)?, &py::mul(&py::math1("cos", &v_d.clone())?, &py::math1("cos", &v_b.clone())?)?)?)?)?, &Py::Int(2i64))?)?)?;
    v_Pa = py::mul(&py::mul(&py::mul(&py::mul(&Py::Float(0.5), &v_Ka.clone())?, &v_gam.clone())?, &v_H.clone())?, &v_H.clone())?;
    return Ok(Py::dict(vec![(Py::str("Ka"), f__r(v_Ka.clone())?), (Py::str("Pa"), f__r(v_Pa.clone())?), (Py::str("Pa_h"), f__r(py::mul(&v_Pa.clone(), &py::math1("cos", &v_d.clone())?)?)?), (Py::str("Pa_v"), f__r(py::mul(&v_Pa.clone(), &py::math1("sin", &v_d.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_coulomb_passive(mut v_phi: Py, mut v_delta: Py, mut v_beta: Py, mut v_gam: Py, mut v_H: Py) -> R {
    let mut v_Kp: Py = Py::None;
    let mut v_Pp: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_root: Py = Py::None;
    let _ = f__coulomb_check(v_phi.clone(), v_delta.clone(), v_beta.clone())?;
    let _ = f__pos(Py::dict(vec![(Py::str("gam"), v_gam.clone()), (Py::str("H"), v_H.clone())]))?;
    let _ = f__need({ let t380 = py::add(&py::add(&v_phi.clone(), &v_delta.clone())?, &v_beta.clone())?; Py::Bool({ let t381 = Py::Int(90i64); if py::lt(&t380, &t381)? { true } else { false } }) }, Py::str("phi + delta + beta must stay below 90 degrees for a plane passive wedge"))?;
    let t382 = (Py::tuple(vec![py::math1("radians", &v_phi.clone())?, py::math1("radians", &v_delta.clone())?, py::math1("radians", &v_beta.clone())?])).items()?;
    if t382.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_p = t382[0].clone();
    v_d = t382[1].clone();
    v_b = t382[2].clone();
    v_root = py::math1("sqrt", &py::truediv(&py::mul(&py::math1("sin", &py::add(&v_p.clone(), &v_d.clone())?)?, &py::math1("sin", &py::add(&v_p.clone(), &v_b.clone())?)?)?, &py::mul(&py::math1("cos", &v_d.clone())?, &py::math1("cos", &v_b.clone())?)?)?)?;
    let _ = f__need({ let t383 = v_root.clone(); Py::Bool({ let t384 = Py::Int(1i64); if py::lt(&t383, &t384)? { true } else { false } }) }, Py::str("no plane passive wedge exists for these angles"))?;
    v_Kp = py::truediv(&py::pow(&py::math1("cos", &v_p.clone())?, &Py::Int(2i64))?, &py::mul(&py::math1("cos", &v_d.clone())?, &py::pow(&py::sub(&Py::Int(1i64), &v_root.clone())?, &Py::Int(2i64))?)?)?;
    v_Pp = py::mul(&py::mul(&py::mul(&py::mul(&Py::Float(0.5), &v_Kp.clone())?, &v_gam.clone())?, &v_H.clone())?, &v_H.clone())?;
    return Ok(Py::dict(vec![(Py::str("Kp"), f__r(v_Kp.clone())?), (Py::str("Pp"), f__r(v_Pp.clone())?), (Py::str("Pp_h"), f__r(py::mul(&v_Pp.clone(), &py::math1("cos", &v_d.clone())?)?)?), (Py::str("Pp_v"), f__r(py::mul(&v_Pp.clone(), &py::math1("sin", &v_d.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_critical_circ(mut v_Q: Py, mut v_D: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_yc: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("Q"), v_Q.clone()), (Py::str("D"), v_D.clone())]))?;
    v_f = { let v_D = v_D.clone(); let v_Q = v_Q.clone(); let v_f = v_f.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_y: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_A: Py = Py::None;
            let mut v_T: Py = Py::None;
            let mut v__P: Py = Py::None;
            let mut v__th: Py = Py::None;
            let t400 = (f__circ(v_D.clone(), v_y.clone())?).items()?;
            if t400.len() != 4 { return Err(PyErr::Value("unpack".into())); }
            v_A = t400[0].clone();
            v__P = t400[1].clone();
            v_T = t400[2].clone();
            v__th = t400[3].clone();
            return Ok(py::sub(&py::mul(&py::mul(&v_Q.clone(), &v_Q.clone())?, &v_T.clone())?, &py::mul(&c_G()?, &py::pow(&v_A.clone(), &Py::Int(3i64))?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    let _ = f__need({ let t401 = py::call(&v_f.clone(), &[py::mul(&Py::Float(0.95), &v_D.clone())?])?; Py::Bool({ let t402 = Py::Int(0i64); if py::lt(&t401, &t402)? { true } else { false } }) }, Py::str("the critical depth would exceed 0.95 D: the conduit is too small for this discharge"))?;
    v_yc = f__bisect_root(v_f.clone(), py::mul(&Py::Float(1e-12), &v_D.clone())?, py::mul(&Py::Float(0.95), &v_D.clone())?, Py::Int(300i64))?;
    v_A = py::getitem(&f__circ(v_D.clone(), v_yc.clone())?, &Py::Int(0i64))?;
    return Ok(Py::dict(vec![(Py::str("yc"), f__r(v_yc.clone())?), (Py::str("Ac"), f__r(v_A.clone())?), (Py::str("Vc"), f__r(py::truediv(&v_Q.clone(), &v_A.clone())?)?), (Py::str("Emin"), f__r(py::add(&v_yc.clone(), &py::truediv(&py::pow(&py::truediv(&v_Q.clone(), &v_A.clone())?, &Py::Int(2i64))?, &py::mul(&Py::Int(2i64), &c_G()?)?)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_critical_rect(mut v_Q: Py, mut v_b: Py) -> R {
    let mut v_q: Py = Py::None;
    let mut v_yc: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("Q"), v_Q.clone()), (Py::str("b"), v_b.clone())]))?;
    v_q = py::truediv(&v_Q.clone(), &v_b.clone())?;
    v_yc = py::pow(&py::truediv(&py::mul(&v_q.clone(), &v_q.clone())?, &c_G()?)?, &py::truediv(&Py::Float(1.0), &Py::Float(3.0))?)?;
    return Ok(Py::dict(vec![(Py::str("yc"), f__r(v_yc.clone())?), (Py::str("Vc"), f__r(py::truediv(&v_q.clone(), &v_yc.clone())?)?), (Py::str("Emin"), f__r(py::mul(&Py::Float(1.5), &v_yc.clone())?)?), (Py::str("q"), f__r(v_q.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_critical_trap(mut v_Q: Py, mut v_b: Py, mut v_z: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_yc: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("Q"), v_Q.clone())]))?;
    let _ = f__finite(Py::dict(vec![(Py::str("b"), v_b.clone()), (Py::str("z"), v_z.clone())]))?;
    let _ = f__need({ let t459 = { let t455 = v_b.clone(); Py::Bool({ let t456 = Py::Int(0i64); if py::ge(&t455, &t456)? { true } else { false } }) }; if !t459.truthy() { t459 } else { { let t457 = v_z.clone(); Py::Bool({ let t458 = Py::Int(0i64); if py::gt(&t457, &t458)? { true } else { false } }) } } }, Py::str("need b >= 0 and z > 0 (z = 0 is a rectangular channel)"))?;
    v_f = { let v_Q = v_Q.clone(); let v_b = v_b.clone(); let v_z = v_z.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_y: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::mul(&py::mul(&v_Q.clone(), &v_Q.clone())?, &py::add(&v_b.clone(), &py::mul(&py::mul(&Py::Int(2i64), &v_z.clone())?, &v_y.clone())?)?)?, &py::mul(&c_G()?, &py::pow(&py::mul(&py::add(&v_b.clone(), &py::mul(&v_z.clone(), &v_y.clone())?)?, &v_y.clone())?, &Py::Int(3i64))?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_yc = f__bisect_root(v_f.clone(), Py::Float(1e-12), Py::Float(10000.0), Py::Int(300i64))?;
    v_A = py::mul(&py::add(&v_b.clone(), &py::mul(&v_z.clone(), &v_yc.clone())?)?, &v_yc.clone())?;
    return Ok(Py::dict(vec![(Py::str("yc"), f__r(v_yc.clone())?), (Py::str("Ac"), f__r(v_A.clone())?), (Py::str("Vc"), f__r(py::truediv(&v_Q.clone(), &v_A.clone())?)?), (Py::str("Emin"), f__r(py::add(&v_yc.clone(), &py::truediv(&py::pow(&py::truediv(&v_Q.clone(), &v_A.clone())?, &Py::Int(2i64))?, &py::mul(&Py::Int(2i64), &c_G()?)?)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cv_from_time(mut v_Hdr: Py, mut v_t: Py, mut v_U: Py) -> R {
    let mut v_Tv: Py = Py::None;
    let mut v_cv: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("Hdr"), v_Hdr.clone()), (Py::str("t"), v_t.clone())]))?;
    let _ = f__finite(Py::dict(vec![(Py::str("U"), v_U.clone())]))?;
    let _ = f__need({ let t180 = Py::Float(0.01); Py::Bool({ let t181 = v_U.clone(); if py::le(&t180, &t181)? { { let t182 = Py::Float(0.99); if py::le(&t181, &t182)? { true } else { false } } } else { false } }) }, Py::str("the degree of consolidation must lie between 1% and 99%"))?;
    v_Tv = f__Tv_for(v_U.clone())?;
    v_cv = py::truediv(&py::mul(&py::mul(&v_Tv.clone(), &v_Hdr.clone())?, &v_Hdr.clone())?, &v_t.clone())?;
    return Ok(Py::dict(vec![(Py::str("cv"), f__r(v_cv.clone())?), (Py::str("Tv"), f__r(v_Tv.clone())?), (Py::str("cv_m2_yr"), f__r(py::mul(&v_cv.clone(), &c_YEAR()?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_degree_of_consolidation(mut v_cv: Py, mut v_t: Py, mut v_Hdr: Py, mut v_Sc: Py) -> R {
    let mut v_Tv: Py = Py::None;
    let mut v_U: Py = Py::None;
    let mut v_out: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("cv"), v_cv.clone()), (Py::str("t"), v_t.clone()), (Py::str("Hdr"), v_Hdr.clone())]))?;
    v_Tv = py::truediv(&py::mul(&v_cv.clone(), &v_t.clone())?, &py::mul(&v_Hdr.clone(), &v_Hdr.clone())?)?;
    let _ = f__need({ let t553 = Py::Float(0.0001); Py::Bool({ let t554 = v_Tv.clone(); if py::le(&t553, &t554)? { { let t555 = Py::Int(5i64); if py::le(&t554, &t555)? { true } else { false } } } else { false } }) }, Py::str("the time factor must lie between 1e-4 and 5"))?;
    v_U = f__U_fourier(v_Tv.clone())?;
    v_out = Py::dict(vec![(Py::str("Tv"), f__r(v_Tv.clone())?), (Py::str("U"), f__r(v_U.clone())?), (Py::str("U_percent"), f__r(py::mul(&Py::Int(100i64), &v_U.clone())?)?)]);
    if ({ let t565 = v_Sc.clone(); Py::Bool({ let t566 = Py::None; if !py::is(&t565, &t566) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::dict(vec![(Py::str("Sc"), v_Sc.clone())]))?;
        py::setitem(&v_out.clone(), &Py::str("St"), f__r(py::mul(&v_U.clone(), &v_Sc.clone())?)?)?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_design_return_period(mut v_R: Py, mut v_n: Py) -> R {
    let mut v_T: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("R"), v_R.clone())]))?;
    let _ = f__need({ let t571 = Py::Int(0i64); Py::Bool({ let t572 = v_R.clone(); if py::lt(&t571, &t572)? { { let t573 = Py::Int(1i64); if py::lt(&t572, &t573)? { true } else { false } } } else { false } }) }, Py::str("the acceptable risk must be a fraction between 0 and 1"))?;
    let _ = f__need({ let t577 = Py::Bool(matches!(v_n.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t577.truthy() { t577 } else { { let t574 = Py::Int(1i64); Py::Bool({ let t575 = v_n.clone(); if py::le(&t574, &t575)? { { let t576 = Py::Int(1000i64); if py::le(&t575, &t576)? { true } else { false } } } else { false } }) } } }, Py::str("the design life must be a whole number of years, 1 to 1000"))?;
    v_T = py::truediv(&Py::Float(1.0), &py::sub(&Py::Int(1i64), &py::pow(&py::sub(&Py::Int(1i64), &v_R.clone())?, &py::truediv(&Py::Float(1.0), &v_n.clone())?)?)?)?;
    return Ok(Py::dict(vec![(Py::str("T"), f__r(v_T.clone())?), (Py::str("p_annual"), f__r(py::truediv(&Py::Int(1i64), &v_T.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_direct_shear_params(mut v_sigma: Py, mut v_tau: Py) -> R {
    let mut v_icpt: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_slope: Py = Py::None;
    let mut v_t: Py = Py::None;
    v_s = f__vec(Py::str("normal stresses"), v_sigma.clone(), Py::None, Py::Bool(false), Py::Bool(true))?;
    v_t = f__vec(Py::str("shear stresses"), v_tau.clone(), py::len(&v_s.clone())?, Py::Bool(true), Py::Bool(false))?;
    let _ = f__need({ let t599 = py::len(&v_s.clone())?; Py::Bool({ let t600 = Py::Int(2i64); if py::ge(&t599, &t600)? { true } else { false } }) }, Py::str("give at least two tests"))?;
    let t607 = (f__linfit(v_s.clone(), v_t.clone())?).items()?;
    if t607.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_slope = t607[0].clone();
    v_icpt = t607[1].clone();
    let _ = f__need({ let t608 = v_slope.clone(); Py::Bool({ let t609 = Py::Int(0i64); if py::gt(&t608, &t609)? { true } else { false } }) }, Py::str("the shear strength must rise with normal stress"))?;
    return Ok(Py::dict(vec![(Py::str("phi"), f__r(py::math1("degrees", &py::math1("atan", &v_slope.clone())?)?)?), (Py::str("c"), f__r(f__snap(v_icpt.clone(), py::minmax(&[v_t.clone()], true)?)?)?), (Py::str("tan_phi"), f__r(v_slope.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dupuit_recharge(mut v_K: Py, mut v_h1: Py, mut v_h2: Py, mut v_L: Py, mut v_W: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_hmax: Py = Py::None;
    let mut v_hsq: Py = Py::None;
    let mut v_q0: Py = Py::None;
    let mut v_qL: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("K"), v_K.clone()), (Py::str("h1"), v_h1.clone()), (Py::str("h2"), v_h2.clone()), (Py::str("L"), v_L.clone()), (Py::str("W"), v_W.clone())]))?;
    v_c = py::truediv(&py::sub(&py::mul(&v_h1.clone(), &v_h1.clone())?, &py::mul(&v_h2.clone(), &v_h2.clone())?)?, &py::mul(&Py::Int(2i64), &v_L.clone())?)?;
    v_q0 = py::sub(&py::mul(&v_K.clone(), &v_c.clone())?, &py::truediv(&py::mul(&v_W.clone(), &v_L.clone())?, &Py::Int(2i64))?)?;
    v_qL = py::add(&py::mul(&v_K.clone(), &v_c.clone())?, &py::truediv(&py::mul(&v_W.clone(), &v_L.clone())?, &Py::Int(2i64))?)?;
    v_d = py::sub(&py::truediv(&v_L.clone(), &Py::Int(2i64))?, &py::truediv(&py::mul(&v_K.clone(), &py::sub(&py::mul(&v_h1.clone(), &v_h1.clone())?, &py::mul(&v_h2.clone(), &v_h2.clone())?)?)?, &py::mul(&py::mul(&Py::Int(2i64), &v_W.clone())?, &v_L.clone())?)?)?;
    v_hsq = { let v_K = v_K.clone(); let v_L = v_L.clone(); let v_W = v_W.clone(); let v_c = v_c.clone(); let v_d = v_d.clone(); let v_h1 = v_h1.clone(); let v_h2 = v_h2.clone(); let v_q0 = v_q0.clone(); let v_qL = v_qL.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::add(&py::sub(&py::mul(&v_h1.clone(), &v_h1.clone())?, &py::truediv(&py::mul(&py::sub(&py::mul(&v_h1.clone(), &v_h1.clone())?, &py::mul(&v_h2.clone(), &v_h2.clone())?)?, &v_x.clone())?, &v_L.clone())?)?, &py::mul(&py::mul(&py::truediv(&v_W.clone(), &v_K.clone())?, &py::sub(&v_L.clone(), &v_x.clone())?)?, &v_x.clone())?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_hmax = (if ({ let t627 = Py::Int(0i64); Py::Bool({ let t628 = v_d.clone(); if py::lt(&t627, &t628)? { { let t629 = v_L.clone(); if py::lt(&t628, &t629)? { true } else { false } } } else { false } }) }).truthy() { py::math1("sqrt", &py::call(&v_hsq.clone(), &[v_d.clone()])?)? } else { py::minmax(&[v_h1.clone(), v_h2.clone()], true)? });
    return Ok(Py::dict(vec![(Py::str("q0"), f__r(f__snap(v_q0.clone(), py::add(&py::abs(&py::mul(&v_K.clone(), &v_c.clone())?)?, &py::mul(&v_W.clone(), &v_L.clone())?)?)?)?), (Py::str("qL"), f__r(f__snap(v_qL.clone(), py::add(&py::abs(&py::mul(&v_K.clone(), &v_c.clone())?)?, &py::mul(&v_W.clone(), &v_L.clone())?)?)?)?), (Py::str("x_divide"), f__r(v_d.clone())?), (Py::str("h_max"), f__r(v_hmax.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_earthwork_volume(mut v_areas: Py, mut v_d: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_out: Py = Py::None;
    v_A = f__vec(Py::str("areas"), v_areas.clone(), Py::None, Py::Bool(false), Py::Bool(true))?;
    let _ = f__need({ let t653 = py::len(&v_A.clone())?; Py::Bool({ let t654 = Py::Int(2i64); if py::ge(&t653, &t654)? { true } else { false } }) }, Py::str("give at least two cross-section areas"))?;
    let _ = f__pos(Py::dict(vec![(Py::str("d"), v_d.clone())]))?;
    v_out = Py::dict(vec![(Py::str("V_end_area"), f__r(py::mul(&v_d.clone(), &py::sub(&py::sum(&v_A.clone(), None)?, &py::truediv(&py::add(&py::getitem(&v_A.clone(), &Py::Int(0i64))?, &py::getitem(&v_A.clone(), &py::neg(&Py::Int(1i64))?)?)?, &Py::Int(2i64))?)?)?)?)]);
    if ({ let t670 = py::modulo(&py::len(&v_A.clone())?, &Py::Int(2i64))?; Py::Bool({ let t671 = Py::Int(1i64); if py::eq(&t670, &t671) { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("V_prismoidal"), f__r(py::mul(&py::truediv(&v_d.clone(), &Py::Int(3i64))?, &py::add(&py::add(&py::add(&py::getitem(&v_A.clone(), &Py::Int(0i64))?, &py::getitem(&v_A.clone(), &py::neg(&Py::Int(1i64))?)?)?, &py::mul(&Py::Int(4i64), &py::sum(&py::slice(&v_A.clone(), Some((Py::Int(1i64)).i()?), Some((py::neg(&Py::Int(1i64))?).i()?), Some((Py::Int(2i64)).i()?))?, None)?)?)?, &py::mul(&Py::Int(2i64), &py::sum(&py::slice(&v_A.clone(), Some((Py::Int(2i64)).i()?), Some((py::neg(&Py::Int(1i64))?).i()?), Some((Py::Int(2i64)).i()?))?, None)?)?)?)?)?)?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_effective_stress(mut v_H: Py, mut v_gam: Py, mut v_zw: Py, mut v_z: Py) -> R {
    let mut v_g: Py = Py::None;
    let mut v_gi: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_sig: Py = Py::None;
    let mut v_top: Py = Py::None;
    let mut v_u: Py = Py::None;
    v_H = f__vec(Py::str("thicknesses"), v_H.clone(), Py::None, Py::Bool(true), Py::Bool(false))?;
    v_g = { let mut out = vec![]; for t687 in py::iter(&(f__vec(Py::str("unit weights"), v_gam.clone(), py::len(&v_H.clone())?, Py::Bool(true), Py::Bool(false))?))? { let mut v_x: Py; 
v_x = t687;
out.push(py::mul(&v_x.clone(), &Py::Float(1000.0))?); } Py::list(out) };
    let _ = f__finite(Py::dict(vec![(Py::str("zw"), v_zw.clone()), (Py::str("z"), v_z.clone())]))?;
    let _ = f__need({ let t692 = v_zw.clone(); Py::Bool({ let t693 = Py::Int(0i64); if py::ge(&t692, &t693)? { true } else { false } }) }, Py::str("the water table depth must be zero or positive"))?;
    let _ = f__need({ let t694 = Py::Int(0i64); Py::Bool({ let t695 = v_z.clone(); if py::lt(&t694, &t695)? { { let t696 = py::add(&py::sum(&v_H.clone(), None)?, &Py::Float(1e-12))?; if py::le(&t695, &t696)? { true } else { false } } } else { false } }) }, Py::str("the depth must lie within the layers"))?;
    let t697 = (Py::tuple(vec![Py::Float(0.0), Py::Float(0.0)])).items()?;
    if t697.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_sig = t697[0].clone();
    v_top = t697[1].clone();
    for t698 in py::iter(&(py::zip(&[v_H.clone(), v_g.clone()])?))? {
        let t701 = (t698).items()?;
        if t701.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_h = t701[0].clone();
        v_gi = t701[1].clone();
        if ({ let t699 = v_z.clone(); Py::Bool({ let t700 = v_top.clone(); if py::le(&t699, &t700)? { true } else { false } }) }).truthy() {
            break;
        }
        v_sig = py::iadd(&v_sig, &py::mul(&v_gi.clone(), &py::sub(&py::minmax(&[v_z.clone(), py::add(&v_top.clone(), &v_h.clone())?], false)?, &v_top.clone())?)?)?;
        v_top = py::iadd(&v_top, &v_h.clone())?;
    }
    v_u = py::mul(&c_GAMMA_W()?, &py::minmax(&[Py::Float(0.0), py::sub(&v_z.clone(), &v_zw.clone())?], true)?)?;
    return Ok(Py::dict(vec![(Py::str("sigma"), f__r(v_sig.clone())?), (Py::str("u"), f__r(v_u.clone())?), (Py::str("sigma_eff"), f__r(py::sub(&v_sig.clone(), &v_u.clone())?)?), (Py::str("sigma_eff_kPa"), f__r(py::truediv(&py::sub(&v_sig.clone(), &v_u.clone())?, &Py::Float(1000.0))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_exceedance_risk(mut v_T: Py, mut v_n: Py) -> R {
    let mut v_p: Py = Py::None;
    let mut v_risk: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("T"), v_T.clone())]))?;
    let _ = f__need({ let t756 = v_T.clone(); Py::Bool({ let t757 = Py::Int(1i64); if py::gt(&t756, &t757)? { true } else { false } }) }, Py::str("the return period must exceed 1 year"))?;
    let _ = f__need({ let t761 = Py::Bool(matches!(v_n.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t761.truthy() { t761 } else { { let t758 = Py::Int(1i64); Py::Bool({ let t759 = v_n.clone(); if py::le(&t758, &t759)? { { let t760 = Py::Int(1000i64); if py::le(&t759, &t760)? { true } else { false } } } else { false } }) } } }, Py::str("the design life must be a whole number of years, 1 to 1000"))?;
    v_p = py::truediv(&Py::Float(1.0), &v_T.clone())?;
    v_risk = py::sub(&Py::Int(1i64), &py::pow(&py::sub(&Py::Int(1i64), &v_p.clone())?, &v_n.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("risk"), f__r(v_risk.clone())?), (Py::str("p_annual"), f__r(v_p.clone())?), (Py::str("p_none"), f__r(py::sub(&Py::Int(1i64), &v_risk.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_excess_pore_pressure(mut v_u0: Py, mut v_cv: Py, mut v_t: Py, mut v_Hdr: Py, mut v_z: Py) -> R {
    let mut v_M: Py = Py::None;
    let mut v_Tv: Py = Py::None;
    let mut v_Z: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_term: Py = Py::None;
    let mut v_u: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("u0"), v_u0.clone()), (Py::str("cv"), v_cv.clone()), (Py::str("t"), v_t.clone()), (Py::str("Hdr"), v_Hdr.clone())]))?;
    let _ = f__finite(Py::dict(vec![(Py::str("z"), v_z.clone())]))?;
    let _ = f__need({ let t717 = Py::Int(0i64); Py::Bool({ let t718 = v_z.clone(); if py::le(&t717, &t718)? { { let t719 = py::mul(&Py::Int(2i64), &v_Hdr.clone())?; if py::le(&t718, &t719)? { true } else { false } } } else { false } }) }, Py::str("the depth must lie between 0 and 2 Hdr (the drainage faces)"))?;
    v_Tv = py::truediv(&py::mul(&v_cv.clone(), &v_t.clone())?, &py::mul(&v_Hdr.clone(), &v_Hdr.clone())?)?;
    let _ = f__need({ let t720 = Py::Float(0.001); Py::Bool({ let t721 = v_Tv.clone(); if py::le(&t720, &t721)? { { let t722 = Py::Int(3i64); if py::le(&t721, &t722)? { true } else { false } } } else { false } }) }, Py::str("the time factor must lie between 0.001 and 3"))?;
    v_Z = py::truediv(&v_z.clone(), &v_Hdr.clone())?;
    let t723 = (Py::tuple(vec![Py::Float(0.0), Py::Int(0i64)])).items()?;
    if t723.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s = t723[0].clone();
    v_m = t723[1].clone();
    while (Py::Bool(true)).truthy() {
        v_M = py::truediv(&py::mul(&Py::Float(std::f64::consts::PI), &py::add(&py::mul(&Py::Int(2i64), &v_m.clone())?, &Py::Int(1i64))?)?, &Py::Int(2i64))?;
        v_term = py::mul(&py::mul(&py::truediv(&Py::Int(2i64), &v_M.clone())?, &py::math1("sin", &py::mul(&v_M.clone(), &v_Z.clone())?)?)?, &py::math1("exp", &py::mul(&py::mul(&py::neg(&v_M.clone())?, &v_M.clone())?, &v_Tv.clone())?)?)?;
        v_s = py::iadd(&v_s, &v_term.clone())?;
        if ({ let t724 = py::mul(&py::truediv(&Py::Int(2i64), &v_M.clone())?, &py::math1("exp", &py::mul(&py::mul(&py::neg(&v_M.clone())?, &v_M.clone())?, &v_Tv.clone())?)?)?; Py::Bool({ let t725 = Py::Float(1e-18); if py::lt(&t724, &t725)? { true } else { false } }) }).truthy() {
            break;
        }
        v_m = py::iadd(&v_m, &Py::Int(1i64))?;
    }
    v_u = f__snap(py::mul(&v_u0.clone(), &v_s.clone())?, v_u0.clone())?;
    return Ok(Py::dict(vec![(Py::str("u"), f__r(v_u.clone())?), (Py::str("Uz"), f__r(py::sub(&Py::Int(1i64), &py::truediv(&v_u.clone(), &v_u0.clone())?)?)?), (Py::str("Tv"), f__r(v_Tv.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_falling_head(mut v_d: Py, mut v_D: Py, mut v_L: Py, mut v_h1: Py, mut v_h2: Py, mut v_t: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_K: Py = Py::None;
    let mut v_a: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("d"), v_d.clone()), (Py::str("D"), v_D.clone()), (Py::str("L"), v_L.clone()), (Py::str("h1"), v_h1.clone()), (Py::str("h2"), v_h2.clone()), (Py::str("t"), v_t.clone())]))?;
    let _ = f__need({ let t743 = v_h2.clone(); Py::Bool({ let t744 = v_h1.clone(); if py::lt(&t743, &t744)? { true } else { false } }) }, Py::str("the head must fall: h2 < h1"))?;
    let t745 = (Py::tuple(vec![py::truediv(&py::mul(&py::mul(&Py::Float(std::f64::consts::PI), &v_d.clone())?, &v_d.clone())?, &Py::Int(4i64))?, py::truediv(&py::mul(&py::mul(&Py::Float(std::f64::consts::PI), &v_D.clone())?, &v_D.clone())?, &Py::Int(4i64))?])).items()?;
    if t745.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t745[0].clone();
    v_A = t745[1].clone();
    v_K = py::mul(&py::truediv(&py::mul(&v_a.clone(), &v_L.clone())?, &py::mul(&v_A.clone(), &v_t.clone())?)?, &py::math1("log", &py::truediv(&v_h1.clone(), &v_h2.clone())?)?)?;
    return Ok(Py::dict(vec![(Py::str("K"), f__r(v_K.clone())?), (Py::str("K_cm_s"), f__r(py::mul(&v_K.clone(), &Py::Int(100i64))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_froude_rect(mut v_Q: Py, mut v_b: Py, mut v_y: Py) -> R {
    let mut v_Fr: Py = Py::None;
    let mut v_V: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("Q"), v_Q.clone()), (Py::str("b"), v_b.clone()), (Py::str("y"), v_y.clone())]))?;
    v_V = py::truediv(&v_Q.clone(), &py::mul(&v_b.clone(), &v_y.clone())?)?;
    v_Fr = py::truediv(&v_V.clone(), &py::math1("sqrt", &py::mul(&c_G()?, &v_y.clone())?)?)?;
    return Ok(Py::dict(vec![(Py::str("Fr"), f__r(v_Fr.clone())?), (Py::str("V"), f__r(v_V.clone())?), (Py::str("regime"), f__regime(v_Fr.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_grain_size(mut v_sizes: Py, mut v_passing: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_D10: Py = Py::None;
    let mut v_D30: Py = Py::None;
    let mut v_D60: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_pairs: Py = Py::None;
    v_d = f__vec(Py::str("sizes"), v_sizes.clone(), Py::None, Py::Bool(true), Py::Bool(false))?;
    v_p = f__vec(Py::str("percent passing"), v_passing.clone(), py::len(&v_d.clone())?, Py::Bool(false), Py::Bool(false))?;
    let _ = f__need({ let t839 = py::len(&v_d.clone())?; Py::Bool({ let t840 = Py::Int(2i64); if py::ge(&t839, &t840)? { true } else { false } }) }, Py::str("give at least two sieves"))?;
    v_pairs = py::sorted(&py::zip(&[v_d.clone(), v_p.clone()])?, (Py::Bool(false)).truthy())?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t841 in py::range_iter(&[(py::sub(&py::len(&v_pairs.clone())?, &Py::Int(1i64))?).i()?]) { let mut v_i: Py; 
v_i = t841;
out.push({ let t846 = { let t842 = py::getitem(&py::getitem(&v_pairs.clone(), &v_i.clone())?, &Py::Int(0i64))?; Py::Bool({ let t843 = py::getitem(&py::getitem(&v_pairs.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?, &Py::Int(0i64))?; if py::lt(&t842, &t843)? { true } else { false } }) }; if !t846.truthy() { t846 } else { { let t844 = py::getitem(&py::getitem(&v_pairs.clone(), &v_i.clone())?, &Py::Int(1i64))?; Py::Bool({ let t845 = py::getitem(&py::getitem(&v_pairs.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?, &Py::Int(1i64))?; if py::lt(&t844, &t845)? { true } else { false } }) } } }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("the percent passing must rise strictly with sieve size (no repeated sizes or equal percentages)"))?;
    let _ = f__need({ let t851 = { let t847 = py::getitem(&py::getitem(&v_pairs.clone(), &Py::Int(0i64))?, &Py::Int(1i64))?; Py::Bool({ let t848 = Py::Int(10i64); if py::le(&t847, &t848)? { true } else { false } }) }; if !t851.truthy() { t851 } else { { let t849 = py::getitem(&py::getitem(&v_pairs.clone(), &py::neg(&Py::Int(1i64))?)?, &Py::Int(1i64))?; Py::Bool({ let t850 = Py::Int(60i64); if py::ge(&t849, &t850)? { true } else { false } }) } } }, Py::str("the curve must span 10% to 60% passing"))?;
    let _ = f__need({ let t856 = { let t852 = py::getitem(&py::getitem(&v_pairs.clone(), &Py::Int(0i64))?, &Py::Int(1i64))?; Py::Bool({ let t853 = Py::Int(0i64); if py::ge(&t852, &t853)? { true } else { false } }) }; if !t856.truthy() { t856 } else { { let t854 = py::getitem(&py::getitem(&v_pairs.clone(), &py::neg(&Py::Int(1i64))?)?, &Py::Int(1i64))?; Py::Bool({ let t855 = Py::Int(100i64); if py::le(&t854, &t855)? { true } else { false } }) } } }, Py::str("percentages must lie between 0 and 100"))?;
    v_D = { let v_D = v_D.clone(); let v_d = v_d.clone(); let v_p = v_p.clone(); let v_pairs = v_pairs.clone(); let v_passing = v_passing.clone(); let v_sizes = v_sizes.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_P: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_d1: Py = Py::None;
            let mut v_d2: Py = Py::None;
            let mut v_p1: Py = Py::None;
            let mut v_p2: Py = Py::None;
            for t857 in py::iter(&(py::zip(&[v_pairs.clone(), py::slice(&v_pairs.clone(), Some((Py::Int(1i64)).i()?), None, None)?])?))? {
                let t861 = (t857).items()?;
                if t861.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                let t862 = (t861[0].clone()).items()?;
                if t862.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                v_d1 = t862[0].clone();
                v_p1 = t862[1].clone();
                let t863 = (t861[1].clone()).items()?;
                if t863.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                v_d2 = t863[0].clone();
                v_p2 = t863[1].clone();
                if ({ let t858 = v_p1.clone(); Py::Bool({ let t859 = v_P.clone(); if py::le(&t858, &t859)? { { let t860 = v_p2.clone(); if py::le(&t859, &t860)? { true } else { false } } } else { false } }) }).truthy() {
                    return Ok(py::pow(&Py::Int(10i64), &py::add(&py::math1("log10", &v_d1.clone())?, &py::mul(&py::truediv(&py::sub(&v_P.clone(), &v_p1.clone())?, &py::sub(&v_p2.clone(), &v_p1.clone())?)?, &py::sub(&py::math1("log10", &v_d2.clone())?, &py::math1("log10", &v_d1.clone())?)?)?)?)?);
                }
            }
            return Err(py::value_error(py::to_str(&Py::str("percentage outside the curve"))));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    let t864 = (Py::tuple(vec![py::call(&v_D.clone(), &[Py::Int(10i64)])?, py::call(&v_D.clone(), &[Py::Int(30i64)])?, py::call(&v_D.clone(), &[Py::Int(60i64)])?])).items()?;
    if t864.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_D10 = t864[0].clone();
    v_D30 = t864[1].clone();
    v_D60 = t864[2].clone();
    return Ok(Py::dict(vec![(Py::str("D10"), f__r(v_D10.clone())?), (Py::str("D30"), f__r(v_D30.clone())?), (Py::str("D60"), f__r(v_D60.clone())?), (Py::str("Cu"), f__r(py::truediv(&v_D60.clone(), &v_D10.clone())?)?), (Py::str("Cc"), f__r(py::truediv(&py::mul(&v_D30.clone(), &v_D30.clone())?, &py::mul(&v_D10.clone(), &v_D60.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_green_ampt(mut v_K: Py, mut v_psi: Py, mut v_dtheta: Py, mut v_t: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_pd: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("K"), v_K.clone()), (Py::str("psi"), v_psi.clone()), (Py::str("dtheta"), v_dtheta.clone()), (Py::str("t"), v_t.clone())]))?;
    let _ = f__need({ let t880 = v_dtheta.clone(); Py::Bool({ let t881 = Py::Int(1i64); if py::lt(&t880, &t881)? { true } else { false } }) }, Py::str("the moisture deficit is a fraction below 1"))?;
    v_pd = py::mul(&v_psi.clone(), &v_dtheta.clone())?;
    v_g = { let v_K = v_K.clone(); let v_dtheta = v_dtheta.clone(); let v_pd = v_pd.clone(); let v_psi = v_psi.clone(); let v_t = v_t.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_F: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::sub(&v_F.clone(), &py::mul(&v_pd.clone(), &py::math1("log1p", &py::truediv(&v_F.clone(), &v_pd.clone())?)?)?)?, &py::mul(&v_K.clone(), &v_t.clone())?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_F = f__bisect_root(v_g.clone(), Py::Float(0.0), py::add(&py::add(&py::mul(&v_K.clone(), &v_t.clone())?, &py::mul(&Py::Int(2i64), &py::math1("sqrt", &py::mul(&py::mul(&py::mul(&Py::Int(2i64), &v_K.clone())?, &v_pd.clone())?, &v_t.clone())?)?)?)?, &v_pd.clone())?, Py::Int(300i64))?;
    v_f = py::mul(&v_K.clone(), &py::add(&Py::Int(1i64), &py::truediv(&v_pd.clone(), &v_F.clone())?)?)?;
    return Ok(Py::dict(vec![(Py::str("F"), f__r(v_F.clone())?), (Py::str("f"), f__r(v_f.clone())?), (Py::str("F_mm"), f__r(py::mul(&v_F.clone(), &Py::Float(1000.0))?)?), (Py::str("f_mm_h"), f__r(py::mul(&v_f.clone(), &Py::Float(3600000.0))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_gumbel_flood(mut v_peaks: Py, mut v_T: Py) -> R {
    let mut v_K: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_x = f__vec(Py::str("peaks"), v_peaks.clone(), Py::None, Py::Bool(true), Py::Bool(false))?;
    let _ = f__need({ let t925 = py::len(&v_x.clone())?; Py::Bool({ let t926 = Py::Int(5i64); if py::ge(&t925, &t926)? { true } else { false } }) }, Py::str("give at least five annual maxima"))?;
    let _ = f__finite(Py::dict(vec![(Py::str("T"), v_T.clone())]))?;
    let _ = f__need({ let t931 = v_T.clone(); Py::Bool({ let t932 = Py::Int(1i64); if py::gt(&t931, &t932)? { true } else { false } }) }, Py::str("the return period must exceed 1 year"))?;
    let t934 = (f__mean_sd(v_x.clone())?).items()?;
    if t934.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t934[0].clone();
    v_s = t934[1].clone();
    let _ = f__need({ let t935 = v_s.clone(); Py::Bool({ let t936 = Py::Int(0i64); if py::gt(&t935, &t936)? { true } else { false } }) }, Py::str("the annual maxima must not all be equal"))?;
    v_K = py::mul(&py::truediv(&py::neg(&py::math1("sqrt", &Py::Int(6i64))?)?, &Py::Float(std::f64::consts::PI))?, &py::add(&c_EULER_GAMMA()?, &py::math1("log", &py::math1("log", &py::truediv(&v_T.clone(), &py::sub(&v_T.clone(), &Py::Int(1i64))?)?)?)?)?)?;
    return Ok(Py::dict(vec![(Py::str("Q_T"), f__r(py::add(&v_m.clone(), &py::mul(&v_K.clone(), &v_s.clone())?)?)?), (Py::str("K_T"), f__r(v_K.clone())?), (Py::str("mean"), f__r(v_m.clone())?), (Py::str("std"), f__r(v_s.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_gvf_length(mut v_Q: Py, mut v_b: Py, mut v_S0: Py, mut v_n: Py, mut v_y1: Py, mut v_y2: Py) -> R {
    let mut v_L: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_dxdy: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_kind: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_nm: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_qf: Py = Py::None;
    let mut v_yc: Py = Py::None;
    let mut v_yn: Py = Py::None;
    let mut v_zone: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("Q"), v_Q.clone()), (Py::str("b"), v_b.clone()), (Py::str("y1"), v_y1.clone()), (Py::str("y2"), v_y2.clone())]))?;
    let _ = f__ok_mann(v_S0.clone(), v_n.clone())?;
    let _ = f__need({ let t782 = py::abs(&py::sub(&v_y1.clone(), &v_y2.clone())?)?; Py::Bool({ let t783 = py::mul(&Py::Float(1e-06), &py::minmax(&[v_y1.clone(), v_y2.clone()], true)?)?; if py::gt(&t782, &t783)? { true } else { false } }) }, Py::str("the two depths must differ"))?;
    v_q = py::truediv(&v_Q.clone(), &v_b.clone())?;
    v_yc = py::pow(&py::truediv(&py::mul(&v_q.clone(), &v_q.clone())?, &c_G()?)?, &py::truediv(&Py::Float(1.0), &Py::Float(3.0))?)?;
    v_qf = { let v_Q = v_Q.clone(); let v_S0 = v_S0.clone(); let v_b = v_b.clone(); let v_n = v_n.clone(); let v_q = v_q.clone(); let v_y1 = v_y1.clone(); let v_y2 = v_y2.clone(); let v_yc = v_yc.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_y: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::mul(&py::mul(&f__manning_v(py::truediv(&py::mul(&v_b.clone(), &v_y.clone())?, &py::add(&v_b.clone(), &py::mul(&Py::Int(2i64), &v_y.clone())?)?)?, v_S0.clone(), v_n.clone())?, &v_b.clone())?, &v_y.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_yn = f__bisect_root({ let v_Q = v_Q.clone(); let v_S0 = v_S0.clone(); let v_b = v_b.clone(); let v_n = v_n.clone(); let v_q = v_q.clone(); let v_qf = v_qf.clone(); let v_y1 = v_y1.clone(); let v_y2 = v_y2.clone(); let v_yc = v_yc.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_y: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::call(&v_qf.clone(), &[v_y.clone()])?, &v_Q.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, Py::Float(0.0), Py::Float(10000.0), Py::Int(300i64))?;
    let t806 = (Py::tuple(vec![py::minmax(&[v_y1.clone(), v_y2.clone()], false)?, py::minmax(&[v_y1.clone(), v_y2.clone()], true)?])).items()?;
    if t806.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t806[0].clone();
    v_hi = t806[1].clone();
    for t807 in py::iter(&(Py::tuple(vec![Py::tuple(vec![v_yn.clone(), Py::str("normal")]), Py::tuple(vec![v_yc.clone(), Py::str("critical")])])))? {
        let t811 = (t807).items()?;
        if t811.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_c = t811[0].clone();
        v_nm = t811[1].clone();
        let _ = f__need(py::not(&{ let t808 = py::mul(&v_lo.clone(), &Py::Float(0.98))?; Py::Bool({ let t809 = v_c.clone(); if py::lt(&t808, &t809)? { { let t810 = py::mul(&v_hi.clone(), &Py::Float(1.02))?; if py::lt(&t809, &t810)? { true } else { false } } } else { false } }) }), Py::str(&["the depths must stay on one side of (and 2% away from) the ".to_string(), py::format(&v_nm.clone(), "")?, " depth".to_string()].concat()))?;
    }
    v_zone = (if ({ let t812 = v_lo.clone(); Py::Bool({ let t813 = py::minmax(&[v_yn.clone(), v_yc.clone()], true)?; if py::gt(&t812, &t813)? { true } else { false } }) }).truthy() { Py::Int(1i64) } else { (if ({ let t814 = v_hi.clone(); Py::Bool({ let t815 = py::minmax(&[v_yn.clone(), v_yc.clone()], false)?; if py::lt(&t814, &t815)? { true } else { false } }) }).truthy() { Py::Int(3i64) } else { Py::Int(2i64) }) });
    v_kind = (if ({ let t816 = v_yn.clone(); Py::Bool({ let t817 = v_yc.clone(); if py::gt(&t816, &t817)? { true } else { false } }) }).truthy() { Py::str("M") } else { Py::str("S") });
    v_dxdy = { let v_Q = v_Q.clone(); let v_S0 = v_S0.clone(); let v_b = v_b.clone(); let v_c = v_c.clone(); let v_dxdy = v_dxdy.clone(); let v_hi = v_hi.clone(); let v_kind = v_kind.clone(); let v_lo = v_lo.clone(); let v_n = v_n.clone(); let v_nm = v_nm.clone(); let v_q = v_q.clone(); let v_qf = v_qf.clone(); let v_y1 = v_y1.clone(); let v_y2 = v_y2.clone(); let v_yc = v_yc.clone(); let v_yn = v_yn.clone(); let v_zone = v_zone.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_y: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_A: Py = Py::None;
            let mut v_Fr2: Py = Py::None;
            let mut v_R: Py = Py::None;
            let mut v_Sf: Py = Py::None;
            v_A = py::mul(&v_b.clone(), &v_y.clone())?;
            v_R = py::truediv(&v_A.clone(), &py::add(&v_b.clone(), &py::mul(&Py::Int(2i64), &v_y.clone())?)?)?;
            v_Sf = py::truediv(&py::pow(&py::truediv(&py::mul(&v_n.clone(), &v_Q.clone())?, &v_A.clone())?, &Py::Int(2i64))?, &py::pow(&v_R.clone(), &py::truediv(&Py::Float(4.0), &Py::Float(3.0))?)?)?;
            v_Fr2 = py::truediv(&py::mul(&v_Q.clone(), &v_Q.clone())?, &py::mul(&py::mul(&py::mul(&c_G()?, &v_A.clone())?, &v_A.clone())?, &v_y.clone())?)?;
            return Ok(py::truediv(&py::sub(&Py::Int(1i64), &v_Fr2.clone())?, &py::sub(&v_S0.clone(), &v_Sf.clone())?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(15i64))?;
        v_L = py::float(&ax_engine::pymp::call("quad", &[{ let v_Q = v_Q.clone(); let v_S0 = v_S0.clone(); let v_b = v_b.clone(); let v_c = v_c.clone(); let v_dxdy = v_dxdy.clone(); let v_hi = v_hi.clone(); let v_kind = v_kind.clone(); let v_lo = v_lo.clone(); let v_n = v_n.clone(); let v_nm = v_nm.clone(); let v_q = v_q.clone(); let v_qf = v_qf.clone(); let v_y1 = v_y1.clone(); let v_y2 = v_y2.clone(); let v_yc = v_yc.clone(); let v_yn = v_yn.clone(); let v_zone = v_zone.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_y: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::call(&v_dxdy.clone(), &[py::float(&v_y.clone())?])?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, Py::list(vec![v_y1.clone(), v_y2.clone()])], &[])?)?;
    }
    return Ok(Py::dict(vec![(Py::str("L"), f__r(v_L.clone())?), (Py::str("yn"), f__r(v_yn.clone())?), (Py::str("yc"), f__r(v_yc.clone())?), (Py::str("profile"), Py::str(&[py::format(&v_kind.clone(), "")?, py::format(&v_zone.clone(), "")?].concat()))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_hardy_cross(mut v_pipes: Py, mut v_L: Py, mut v_D: Py, mut v_C: Py, mut v_demands: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_J: Py = Py::None;
    let mut v_N: Py = Py::None;
    let mut v_Q: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_dem: Py = Py::None;
    let mut v_den: Py = Py::None;
    let mut v_dq: Py = Py::None;
    let mut v_edges: Py = Py::None;
    let mut v_heads: Py = Py::None;
    let mut v_it: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_ks: Py = Py::None;
    let mut v_loops: Py = Py::None;
    let mut v_lp: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_num: Py = Py::None;
    let mut v_order: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_parent: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_scale: Py = Py::None;
    let mut v_sub: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let mut v_x: Py = Py::None;
    let t993 = (f__network_setup(v_pipes.clone(), v_L.clone(), v_D.clone(), v_C.clone(), v_demands.clone())?).items()?;
    if t993.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_edges = t993[0].clone();
    v_ks = t993[1].clone();
    v_dem = t993[2].clone();
    v_N = t993[3].clone();
    let t1020 = (f__tree_and_loops(v_edges.clone(), v_N.clone())?).items()?;
    if t1020.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_parent = t1020[0].clone();
    v_order = t1020[1].clone();
    v_loops = t1020[2].clone();
    v_m = py::len(&v_edges.clone())?;
    v_Q = py::mul(&Py::list(vec![Py::Float(0.0)]), &v_m.clone())?;
    v_sub = { let mut out = vec![]; for t1021 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_N.clone(), &Py::Int(1i64))?).i()?]) { let mut v_x: Py; 
v_x = t1021;
out.push((v_x.clone(), py::getitem(&v_dem.clone(), &py::sub(&v_x.clone(), &Py::Int(1i64))?)?)); } Py::dict(out) };
    for t1022 in py::iter(&(py::reversed(&py::slice(&v_order.clone(), Some((Py::Int(1i64)).i()?), None, None)?)?))? {
        v_x = t1022;
        let t1023 = (py::getitem(&v_parent.clone(), &v_x.clone())?).items()?;
        if t1023.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_p = t1023[0].clone();
        v_j = t1023[1].clone();
        v_s = t1023[2].clone();
        py::setitem(&v_Q.clone(), &v_j.clone(), py::mul(&v_s.clone(), &py::getitem(&v_sub.clone(), &v_x.clone())?)?)?;
        let t1024 = v_sub.clone(); let t1025 = v_p.clone();
        py::setitem(&t1024, &t1025, py::iadd(&py::getitem(&t1024, &t1025)?, &py::getitem(&v_sub.clone(), &v_x.clone())?)?)?;
    }
    v_scale = py::sum(&v_dem.clone(), None)?;
    for t1026 in py::range_iter(&[(v_m.clone()).i()?]) {
        v_j = t1026;
        if ({ let t1027 = py::getitem(&v_Q.clone(), &v_j.clone())?; Py::Bool({ let t1028 = Py::Float(0.0); if py::eq(&t1027, &t1028) { true } else { false } }) }).truthy() {
            py::setitem(&v_Q.clone(), &v_j.clone(), Py::Float(0.0))?;
        }
    }
    v_it = Py::Int(0i64);
    for t1029 in py::range_iter(&[(Py::Int(1i64)).i()?, (Py::Int(2001i64)).i()?]) {
        v_it = t1029;
        v_worst = Py::Float(0.0);
        for t1030 in py::iter(&(v_loops.clone()))? {
            v_lp = t1030;
            v_num = py::sum(&{ let mut out = vec![]; for t1031 in py::iter(&(py::method(&v_lp.clone(), "items", &[])?))? { let mut v_c: Py; let mut v_j: Py; 
let t1032 = (t1031).items()?;
if t1032.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_j = t1032[0].clone();
v_c = t1032[1].clone();
out.push(py::mul(&v_c.clone(), &f__hw_loss(py::getitem(&v_ks.clone(), &v_j.clone())?, py::getitem(&v_Q.clone(), &v_j.clone())?)?)?); } Py::list(out) }, None)?;
            v_den = py::sum(&{ let mut out = vec![]; for t1033 in py::iter(&(v_lp.clone()))? { let mut v_j: Py; 
v_j = t1033;
out.push(py::mul(&py::mul(&Py::Float(1.852), &py::getitem(&v_ks.clone(), &v_j.clone())?)?, &py::pow(&py::minmax(&[py::abs(&py::getitem(&v_Q.clone(), &v_j.clone())?)?, py::mul(&Py::Float(1e-12), &v_scale.clone())?], true)?, &Py::Float(0.852))?)?); } Py::list(out) }, None)?;
            v_dq = py::truediv(&py::neg(&v_num.clone())?, &v_den.clone())?;
            for t1034 in py::iter(&(py::method(&v_lp.clone(), "items", &[])?))? {
                let t1037 = (t1034).items()?;
                if t1037.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                v_j = t1037[0].clone();
                v_c = t1037[1].clone();
                let t1035 = v_Q.clone(); let t1036 = v_j.clone();
                py::setitem(&t1035, &t1036, py::iadd(&py::getitem(&t1035, &t1036)?, &py::mul(&v_c.clone(), &v_dq.clone())?)?)?;
            }
            v_worst = py::minmax(&[v_worst.clone(), py::abs(&v_dq.clone())?], true)?;
        }
        if ({ let t1038 = v_worst.clone(); Py::Bool({ let t1039 = py::mul(&Py::Float(1e-06), &v_scale.clone())?; if py::lt(&t1038, &t1039)? { true } else { false } }) }).truthy() {
            break;
        }
    }
    for t1040 in py::range_iter(&[(Py::Int(60i64)).i()?]) {
        v__ = t1040;
        if (py::not(&v_loops.clone())).truthy() {
            break;
        }
        v_F = { let mut out = vec![]; for t1041 in py::iter(&(v_loops.clone()))? { let mut v_lp: Py; 
v_lp = t1041;
out.push(py::sum(&{ let mut out = vec![]; for t1042 in py::iter(&(py::method(&v_lp.clone(), "items", &[])?))? { let mut v_c: Py; let mut v_j: Py; 
let t1043 = (t1042).items()?;
if t1043.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_j = t1043[0].clone();
v_c = t1043[1].clone();
out.push(py::mul(&v_c.clone(), &f__hw_loss(py::getitem(&v_ks.clone(), &v_j.clone())?, py::getitem(&v_Q.clone(), &v_j.clone())?)?)?); } Py::list(out) }, None)?); } Py::list(out) };
        v_J = { let mut out = vec![]; for t1044 in py::iter(&(v_loops.clone()))? { let mut v_lp1: Py; 
v_lp1 = t1044;
out.push({ let mut out = vec![]; for t1045 in py::iter(&(v_loops.clone()))? { let mut v_lp2: Py; 
v_lp2 = t1045;
out.push(py::sum(&{ let mut out = vec![]; for t1046 in py::iter(&(py::method(&v_lp1.clone(), "items", &[])?))? { let mut v_c1: Py; let mut v_j: Py; 
let t1047 = (t1046).items()?;
if t1047.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_j = t1047[0].clone();
v_c1 = t1047[1].clone();
out.push(py::mul(&py::mul(&py::mul(&py::mul(&v_c1.clone(), &py::method(&v_lp2.clone(), "get", &[v_j.clone(), Py::Int(0i64)])?)?, &Py::Float(1.852))?, &py::getitem(&v_ks.clone(), &v_j.clone())?)?, &py::pow(&py::abs(&py::getitem(&v_Q.clone(), &v_j.clone())?)?, &Py::Float(0.852))?)?); } Py::list(out) }, None)?); } Py::list(out) }); } Py::list(out) };
        v_dq = f__solve_linear(v_J.clone(), { let mut out = vec![]; for t1060 in py::iter(&(v_F.clone()))? { let mut v_f: Py; 
v_f = t1060;
out.push(py::neg(&v_f.clone())?); } Py::list(out) })?;
        for t1061 in py::iter(&(py::zip(&[v_loops.clone(), v_dq.clone()])?))? {
            let t1066 = (t1061).items()?;
            if t1066.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_lp = t1066[0].clone();
            v_d = t1066[1].clone();
            for t1062 in py::iter(&(py::method(&v_lp.clone(), "items", &[])?))? {
                let t1065 = (t1062).items()?;
                if t1065.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                v_j = t1065[0].clone();
                v_c = t1065[1].clone();
                let t1063 = v_Q.clone(); let t1064 = v_j.clone();
                py::setitem(&t1063, &t1064, py::iadd(&py::getitem(&t1063, &t1064)?, &py::mul(&v_c.clone(), &v_d.clone())?)?)?;
            }
        }
        if ({ let t1067 = py::minmax(&[{ let mut out = vec![]; for t1069 in py::iter(&(v_dq.clone()))? { let mut v_d: Py; 
v_d = t1069;
out.push(py::abs(&v_d.clone())?); } Py::list(out) }], true)?; Py::Bool({ let t1068 = py::mul(&Py::Float(1e-15), &v_scale.clone())?; if py::le(&t1067, &t1068)? { true } else { false } }) }).truthy() {
            break;
        }
    }
    v_heads = Py::dict(vec![(Py::Int(1i64), Py::Float(0.0))]);
    for t1070 in py::iter(&(py::slice(&v_order.clone(), Some((Py::Int(1i64)).i()?), None, None)?))? {
        v_x = t1070;
        let t1071 = (py::getitem(&v_parent.clone(), &v_x.clone())?).items()?;
        if t1071.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_p = t1071[0].clone();
        v_j = t1071[1].clone();
        v_s = t1071[2].clone();
        py::setitem(&v_heads.clone(), &v_x.clone(), py::sub(&py::getitem(&v_heads.clone(), &v_p.clone())?, &py::mul(&v_s.clone(), &f__hw_loss(py::getitem(&v_ks.clone(), &v_j.clone())?, py::getitem(&v_Q.clone(), &v_j.clone())?)?)?)?)?;
    }
    v_Q = { let mut out = vec![]; for t1072 in py::iter(&(v_Q.clone()))? { let mut v_q: Py; 
v_q = t1072;
out.push(f__snap(v_q.clone(), v_scale.clone())?); } Py::list(out) };
    return Ok(Py::dict(vec![(Py::str("flows"), f__r(v_Q.clone())?), (Py::str("head_loss"), f__r({ let mut out = vec![]; for t1081 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_N.clone(), &Py::Int(1i64))?).i()?]) { let mut v_x: Py; 
v_x = t1081;
out.push(f__snap(py::neg(&py::getitem(&v_heads.clone(), &v_x.clone())?)?, Py::Float(1.0))?); } Py::list(out) })?), (Py::str("loops"), py::len(&v_loops.clone())?), (Py::str("iterations"), v_it.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_hazen_williams_flow(mut v_hf: Py, mut v_D: Py, mut v_L: Py, mut v_C: Py) -> R {
    let mut v_Q: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("hf"), v_hf.clone())]))?;
    let _ = f__ok_pipe(v_L.clone(), v_D.clone(), v_C.clone())?;
    v_Q = py::pow(&py::truediv(&v_hf.clone(), &f__hw_k(v_L.clone(), v_D.clone(), v_C.clone())?)?, &py::truediv(&Py::Int(1i64), &Py::Float(1.852))?)?;
    return Ok(Py::dict(vec![(Py::str("Q"), f__r(v_Q.clone())?), (Py::str("V"), f__r(py::truediv(&v_Q.clone(), &py::truediv(&py::mul(&py::mul(&Py::Float(std::f64::consts::PI), &v_D.clone())?, &v_D.clone())?, &Py::Int(4i64))?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_hazen_williams_loss(mut v_Q: Py, mut v_D: Py, mut v_L: Py, mut v_C: Py) -> R {
    let mut v_hf: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("Q"), v_Q.clone())]))?;
    let _ = f__ok_pipe(v_L.clone(), v_D.clone(), v_C.clone())?;
    v_hf = py::mul(&f__hw_k(v_L.clone(), v_D.clone(), v_C.clone())?, &py::pow(&v_Q.clone(), &Py::Float(1.852))?)?;
    return Ok(Py::dict(vec![(Py::str("hf"), f__r(v_hf.clone())?), (Py::str("V"), f__r(py::truediv(&v_Q.clone(), &py::truediv(&py::mul(&py::mul(&Py::Float(std::f64::consts::PI), &v_D.clone())?, &v_D.clone())?, &Py::Int(4i64))?)?)?), (Py::str("Sf"), f__r(py::truediv(&v_hf.clone(), &v_L.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_horizontal_curve(mut v_R: Py, mut v_Delta: Py, mut v_PI: Py) -> R {
    let mut v_L: Py = Py::None;
    let mut v_T: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_out: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("R"), v_R.clone())]))?;
    let _ = f__finite(Py::dict(vec![(Py::str("Delta"), v_Delta.clone()), (Py::str("PI"), v_PI.clone())]))?;
    let _ = f__need({ let t1131 = Py::Int(0i64); Py::Bool({ let t1132 = v_Delta.clone(); if py::lt(&t1131, &t1132)? { { let t1133 = Py::Int(180i64); if py::lt(&t1132, &t1133)? { true } else { false } } } else { false } }) }, Py::str("the deflection angle must lie between 0 and 180 degrees"))?;
    v_d = py::math1("radians", &v_Delta.clone())?;
    v_T = py::mul(&v_R.clone(), &py::math1("tan", &py::truediv(&v_d.clone(), &Py::Int(2i64))?)?)?;
    v_L = py::mul(&v_R.clone(), &v_d.clone())?;
    v_out = Py::dict(vec![(Py::str("T"), f__r(v_T.clone())?), (Py::str("L"), f__r(v_L.clone())?), (Py::str("LC"), f__r(py::mul(&py::mul(&Py::Int(2i64), &v_R.clone())?, &py::math1("sin", &py::truediv(&v_d.clone(), &Py::Int(2i64))?)?)?)?), (Py::str("E"), f__r(py::mul(&v_R.clone(), &py::sub(&py::truediv(&Py::Int(1i64), &py::math1("cos", &py::truediv(&v_d.clone(), &Py::Int(2i64))?)?)?, &Py::Int(1i64))?)?)?), (Py::str("M"), f__r(py::mul(&v_R.clone(), &py::sub(&Py::Int(1i64), &py::math1("cos", &py::truediv(&v_d.clone(), &Py::Int(2i64))?)?)?)?)?), (Py::str("PC"), f__r(py::sub(&v_PI.clone(), &v_T.clone())?)?), (Py::str("PT"), f__r(py::add(&py::sub(&v_PI.clone(), &v_T.clone())?, &v_L.clone())?)?)]);
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_horton(mut v_f0: Py, mut v_fc: Py, mut v_k: Py, mut v_t: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_f: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("f0"), v_f0.clone()), (Py::str("k"), v_k.clone())]))?;
    let _ = f__finite(Py::dict(vec![(Py::str("fc"), v_fc.clone()), (Py::str("t"), v_t.clone())]))?;
    let _ = f__need({ let t1149 = Py::Int(0i64); Py::Bool({ let t1150 = v_fc.clone(); if py::le(&t1149, &t1150)? { { let t1151 = v_f0.clone(); if py::lt(&t1150, &t1151)? { true } else { false } } } else { false } }) }, Py::str("need 0 <= fc < f0"))?;
    let _ = f__need({ let t1152 = v_t.clone(); Py::Bool({ let t1153 = Py::Int(0i64); if py::ge(&t1152, &t1153)? { true } else { false } }) }, Py::str("the time must be zero or positive"))?;
    v_e = py::math1("exp", &py::mul(&py::neg(&v_k.clone())?, &v_t.clone())?)?;
    v_f = py::add(&v_fc.clone(), &py::mul(&py::sub(&v_f0.clone(), &v_fc.clone())?, &v_e.clone())?)?;
    v_F = py::add(&py::mul(&v_fc.clone(), &v_t.clone())?, &py::mul(&py::truediv(&py::sub(&v_f0.clone(), &v_fc.clone())?, &v_k.clone())?, &py::sub(&Py::Int(1i64), &v_e.clone())?)?)?;
    return Ok(Py::dict(vec![(Py::str("f"), f__r(v_f.clone())?), (Py::str("F"), f__r(v_F.clone())?), (Py::str("f_mm_h"), f__r(py::mul(&v_f.clone(), &Py::Float(3600000.0))?)?), (Py::str("F_mm"), f__r(py::mul(&v_F.clone(), &Py::Float(1000.0))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_hydraulic_jump(mut v_Q: Py, mut v_b: Py, mut v_y1: Py) -> R {
    let mut v_Fr1: Py = Py::None;
    let mut v_dE: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_y2: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("Q"), v_Q.clone()), (Py::str("b"), v_b.clone()), (Py::str("y1"), v_y1.clone())]))?;
    v_q = py::truediv(&v_Q.clone(), &v_b.clone())?;
    v_Fr1 = py::truediv(&v_q.clone(), &py::mul(&v_y1.clone(), &py::math1("sqrt", &py::mul(&c_G()?, &v_y1.clone())?)?)?)?;
    let _ = f__need({ let t1169 = v_Fr1.clone(); Py::Bool({ let t1170 = Py::Float(1.0); if py::gt(&t1169, &t1170)? { true } else { false } }) }, Py::str("the upstream flow must be supercritical (Fr1 > 1) for a hydraulic jump"))?;
    v_y2 = py::mul(&py::truediv(&v_y1.clone(), &Py::Int(2i64))?, &py::sub(&py::math1("sqrt", &py::add(&Py::Int(1i64), &py::mul(&py::mul(&Py::Int(8i64), &v_Fr1.clone())?, &v_Fr1.clone())?)?)?, &Py::Int(1i64))?)?;
    v_dE = py::truediv(&py::pow(&py::sub(&v_y2.clone(), &v_y1.clone())?, &Py::Int(3i64))?, &py::mul(&py::mul(&Py::Int(4i64), &v_y1.clone())?, &v_y2.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("y2"), f__r(v_y2.clone())?), (Py::str("Fr1"), f__r(v_Fr1.clone())?), (Py::str("Fr2"), f__r(py::truediv(&v_q.clone(), &py::mul(&v_y2.clone(), &py::math1("sqrt", &py::mul(&c_G()?, &v_y2.clone())?)?)?)?)?), (Py::str("dE"), f__r(v_dE.clone())?), (Py::str("power_loss"), f__r(py::mul(&py::mul(&c_GAMMA_W()?, &v_Q.clone())?, &v_dE.clone())?)?), (Py::str("ratio"), f__r(py::truediv(&v_y2.clone(), &v_y1.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_infinite_slope(mut v_c: Py, mut v_phi: Py, mut v_gam: Py, mut v_z: Py, mut v_beta: Py, mut v_m: Py) -> R {
    let mut v_FS: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_cb: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_sb: Py = Py::None;
    let mut v_sig_eff: Py = Py::None;
    let mut v_tau: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("c"), v_c.clone()), (Py::str("phi"), v_phi.clone()), (Py::str("beta"), v_beta.clone()), (Py::str("m"), v_m.clone())]))?;
    let _ = f__pos(Py::dict(vec![(Py::str("gam"), v_gam.clone()), (Py::str("z"), v_z.clone())]))?;
    let _ = f__need({ let t1186 = v_c.clone(); Py::Bool({ let t1187 = Py::Int(0i64); if py::ge(&t1186, &t1187)? { true } else { false } }) }, Py::str("the cohesion must be zero or positive"))?;
    let _ = f__need({ let t1188 = Py::Int(0i64); Py::Bool({ let t1189 = v_phi.clone(); if py::le(&t1188, &t1189)? { { let t1190 = Py::Int(60i64); if py::lt(&t1189, &t1190)? { true } else { false } } } else { false } }) }, Py::str("the friction angle must lie in [0, 60) degrees"))?;
    let _ = f__need({ let t1191 = Py::Int(1i64); Py::Bool({ let t1192 = v_beta.clone(); if py::le(&t1191, &t1192)? { { let t1193 = Py::Int(80i64); if py::le(&t1192, &t1193)? { true } else { false } } } else { false } }) }, Py::str("the slope angle must lie between 1 and 80 degrees"))?;
    let _ = f__need({ let t1194 = Py::Int(0i64); Py::Bool({ let t1195 = v_m.clone(); if py::le(&t1194, &t1195)? { { let t1196 = Py::Int(1i64); if py::le(&t1195, &t1196)? { true } else { false } } } else { false } }) }, Py::str("the water-table ratio m must lie between 0 (dry) and 1 (at the surface)"))?;
    let _ = f__need({ let t1201 = { let t1197 = v_m.clone(); Py::Bool({ let t1198 = Py::Int(0i64); if py::eq(&t1197, &t1198) { true } else { false } }) }; if t1201.truthy() { t1201 } else { { let t1199 = v_gam.clone(); Py::Bool({ let t1200 = c_GAMMA_W()?; if py::gt(&t1199, &t1200)? { true } else { false } }) } } }, Py::str("a submerged slope needs a unit weight above that of water"))?;
    let t1202 = (Py::tuple(vec![py::math1("radians", &v_beta.clone())?, py::math1("radians", &v_phi.clone())?])).items()?;
    if t1202.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_b = t1202[0].clone();
    v_p = t1202[1].clone();
    let t1203 = (Py::tuple(vec![py::math1("cos", &v_b.clone())?, py::math1("sin", &v_b.clone())?])).items()?;
    if t1203.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_cb = t1203[0].clone();
    v_sb = t1203[1].clone();
    v_tau = py::mul(&py::mul(&py::mul(&v_gam.clone(), &v_z.clone())?, &v_sb.clone())?, &v_cb.clone())?;
    v_sig_eff = py::mul(&py::mul(&py::mul(&py::sub(&v_gam.clone(), &py::mul(&v_m.clone(), &c_GAMMA_W()?)?)?, &v_z.clone())?, &v_cb.clone())?, &v_cb.clone())?;
    v_FS = py::truediv(&py::add(&v_c.clone(), &py::mul(&v_sig_eff.clone(), &py::math1("tan", &v_p.clone())?)?)?, &v_tau.clone())?;
    return Ok(Py::dict(vec![(Py::str("FS"), f__r(v_FS.clone())?), (Py::str("tau"), f__r(v_tau.clone())?), (Py::str("sigma_eff"), f__r(v_sig_eff.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_kge_metrics(mut v_obs: Py, mut v_sim: Py) -> R {
    let mut v_alpha: Py = Py::None;
    let mut v_beta: Py = Py::None;
    let mut v_mo: Py = Py::None;
    let mut v_ms: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_o: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_so: Py = Py::None;
    let mut v_ss: Py = Py::None;
    let t1246 = (f__pair(v_obs.clone(), v_sim.clone())?).items()?;
    if t1246.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_o = t1246[0].clone();
    v_s = t1246[1].clone();
    v_n = py::len(&v_o.clone())?;
    let t1247 = (Py::tuple(vec![py::truediv(&py::sum(&v_o.clone(), None)?, &v_n.clone())?, py::truediv(&py::sum(&v_s.clone(), None)?, &v_n.clone())?])).items()?;
    if t1247.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mo = t1247[0].clone();
    v_ms = t1247[1].clone();
    v_so = py::math1("sqrt", &py::truediv(&py::sum(&{ let mut out = vec![]; for t1248 in py::iter(&(v_o.clone()))? { let mut v_v: Py; 
v_v = t1248;
out.push(py::pow(&py::sub(&v_v.clone(), &v_mo.clone())?, &Py::Int(2i64))?); } Py::list(out) }, None)?, &v_n.clone())?)?;
    v_ss = py::math1("sqrt", &py::truediv(&py::sum(&{ let mut out = vec![]; for t1249 in py::iter(&(v_s.clone()))? { let mut v_v: Py; 
v_v = t1249;
out.push(py::pow(&py::sub(&v_v.clone(), &v_ms.clone())?, &Py::Int(2i64))?); } Py::list(out) }, None)?, &v_n.clone())?)?;
    let _ = f__need({ let t1254 = { let t1250 = v_so.clone(); Py::Bool({ let t1251 = Py::Int(0i64); if py::gt(&t1250, &t1251)? { true } else { false } }) }; if !t1254.truthy() { t1254 } else { { let t1252 = v_ss.clone(); Py::Bool({ let t1253 = Py::Int(0i64); if py::gt(&t1252, &t1253)? { true } else { false } }) } } }, Py::str("both series must vary"))?;
    let _ = f__need({ let t1255 = py::sum(&v_o.clone(), None)?; Py::Bool({ let t1256 = Py::Int(0i64); if py::ne(&t1255, &t1256) { true } else { false } }) }, Py::str("the observed total must not be zero"))?;
    v_r = py::truediv(&py::sum(&{ let mut out = vec![]; for t1257 in py::iter(&(py::zip(&[v_o.clone(), v_s.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t1258 = (t1257).items()?;
if t1258.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t1258[0].clone();
v_b = t1258[1].clone();
out.push(py::mul(&py::sub(&v_a.clone(), &v_mo.clone())?, &py::sub(&v_b.clone(), &v_ms.clone())?)?); } Py::list(out) }, None)?, &py::mul(&py::mul(&v_n.clone(), &v_so.clone())?, &v_ss.clone())?)?;
    let t1259 = (Py::tuple(vec![py::truediv(&v_ss.clone(), &v_so.clone())?, py::truediv(&py::sum(&v_s.clone(), None)?, &py::sum(&v_o.clone(), None)?)?])).items()?;
    if t1259.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_alpha = t1259[0].clone();
    v_beta = t1259[1].clone();
    return Ok(Py::dict(vec![(Py::str("KGE"), f__r(py::sub(&Py::Int(1i64), &py::math1("sqrt", &py::add(&py::add(&py::pow(&py::sub(&v_r.clone(), &Py::Int(1i64))?, &Py::Int(2i64))?, &py::pow(&py::sub(&v_alpha.clone(), &Py::Int(1i64))?, &Py::Int(2i64))?)?, &py::pow(&py::sub(&v_beta.clone(), &Py::Int(1i64))?, &Py::Int(2i64))?)?)?)?)?), (Py::str("r"), f__r(v_r.clone())?), (Py::str("alpha"), f__r(v_alpha.clone())?), (Py::str("beta"), f__r(v_beta.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_kirpich(mut v_L: Py, mut v_S: Py) -> R {
    let mut v_tc: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("L"), v_L.clone()), (Py::str("S"), v_S.clone())]))?;
    let _ = f__need({ let t1219 = v_S.clone(); Py::Bool({ let t1220 = Py::Int(1i64); if py::le(&t1219, &t1220)? { true } else { false } }) }, Py::str("the slope must be at most 1 (m/m)"))?;
    let _ = f__need({ let t1221 = v_L.clone(); Py::Bool({ let t1222 = Py::Float(100000.0); if py::le(&t1221, &t1222)? { true } else { false } }) }, Py::str("the flow length must be at most 100 km"))?;
    v_tc = py::mul(&py::mul(&Py::Float(0.0078), &py::pow(&py::truediv(&v_L.clone(), &Py::Float(0.3048))?, &Py::Float(0.77))?)?, &py::pow(&v_S.clone(), &py::neg(&Py::Float(0.385))?)?)?;
    return Ok(Py::dict(vec![(Py::str("tc"), f__r(py::mul(&v_tc.clone(), &Py::Int(60i64))?)?), (Py::str("tc_min"), f__r(v_tc.clone())?), (Py::str("tc_h"), f__r(py::truediv(&v_tc.clone(), &Py::Int(60i64))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_layered_darcy(mut v_H: Py, mut v_K: Py, mut v_A: Py, mut v_dh: Py) -> R {
    let mut v_Kc: Py = Py::None;
    let mut v_Kh: Py = Py::None;
    let mut v_Kv: Py = Py::None;
    let mut v_Ltot: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_heads: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_ki: Py = Py::None;
    let mut v_q: Py = Py::None;
    v_H = f__vec(Py::str("thicknesses"), v_H.clone(), Py::None, Py::Bool(true), Py::Bool(false))?;
    v_Kc = f__vec(Py::str("conductivities"), v_K.clone(), py::len(&v_H.clone())?, Py::Bool(true), Py::Bool(false))?;
    let _ = f__pos(Py::dict(vec![(Py::str("A"), v_A.clone())]))?;
    let _ = f__finite(Py::dict(vec![(Py::str("dh"), v_dh.clone())]))?;
    let _ = f__need({ let t526 = v_dh.clone(); Py::Bool({ let t527 = Py::Int(0i64); if py::gt(&t526, &t527)? { true } else { false } }) }, Py::str("the head difference must be positive"))?;
    v_Ltot = py::sum(&v_H.clone(), None)?;
    v_Kv = py::truediv(&v_Ltot.clone(), &py::sum(&{ let mut out = vec![]; for t528 in py::iter(&(py::zip(&[v_H.clone(), v_Kc.clone()])?))? { let mut v_h: Py; let mut v_k: Py; 
let t529 = (t528).items()?;
if t529.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_h = t529[0].clone();
v_k = t529[1].clone();
out.push(py::truediv(&v_h.clone(), &v_k.clone())?); } Py::list(out) }, None)?)?;
    v_Kh = py::truediv(&py::sum(&{ let mut out = vec![]; for t530 in py::iter(&(py::zip(&[v_H.clone(), v_Kc.clone()])?))? { let mut v_h: Py; let mut v_k: Py; 
let t531 = (t530).items()?;
if t531.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_h = t531[0].clone();
v_k = t531[1].clone();
out.push(py::mul(&v_h.clone(), &v_k.clone())?); } Py::list(out) }, None)?, &v_Ltot.clone())?;
    v_q = py::truediv(&py::mul(&v_Kv.clone(), &v_dh.clone())?, &v_Ltot.clone())?;
    let t532 = (Py::tuple(vec![Py::list(vec![]), v_dh.clone()])).items()?;
    if t532.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_heads = t532[0].clone();
    v_h = t532[1].clone();
    for t533 in py::iter(&(py::zip(&[v_H.clone(), v_Kc.clone()])?))? {
        let t536 = (t533).items()?;
        if t536.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_hi = t536[0].clone();
        v_ki = t536[1].clone();
        v_h = py::sub(&v_h, &py::truediv(&py::mul(&v_q.clone(), &v_hi.clone())?, &v_ki.clone())?)?;
        let _ = py::method(&v_heads.clone(), "append", &[f__snap(v_h.clone(), v_dh.clone())?])?;
    }
    return Ok(Py::dict(vec![(Py::str("Q"), f__r(py::mul(&v_q.clone(), &v_A.clone())?)?), (Py::str("q"), f__r(v_q.clone())?), (Py::str("Kv"), f__r(v_Kv.clone())?), (Py::str("Kh"), f__r(v_Kh.clone())?), (Py::str("heads"), f__r({ let t543 = py::slice(&v_heads.clone(), None, Some((py::neg(&Py::Int(1i64))?).i()?), None)?; if t543.truthy() { t543 } else { Py::list(vec![Py::Float(0.0)]) } })?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_levelling_rise_fall(mut v_readings: Py, mut v_sights: Py, mut v_RL0: Py) -> R {
    let mut v_diff: Py = Py::None;
    let mut v_fall: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_prev: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_rise: Py = Py::None;
    let mut v_rl: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_sbs: Py = Py::None;
    let mut v_sc: Py = Py::None;
    let mut v_sfs: Py = Py::None;
    v_r = f__vec(Py::str("readings"), v_readings.clone(), Py::None, Py::Bool(false), Py::Bool(true))?;
    v_s = (if (Py::Bool(matches!(v_sights.clone(), Py::Str(_)))).truthy() { f__sights(v_sights.clone())? } else { Py::list((v_sights.clone()).items()?) });
    let _ = f__need({ let t1286 = py::len(&v_s.clone())?; Py::Bool({ let t1287 = py::len(&v_r.clone())?; if py::eq(&t1286, &t1287) { true } else { false } }) }, Py::str("give one sight type per reading"))?;
    let _ = f__need({ let t1292 = { let t1288 = py::getitem(&v_s.clone(), &Py::Int(0i64))?; Py::Bool({ let t1289 = Py::str("BS"); if py::eq(&t1288, &t1289) { true } else { false } }) }; if !t1292.truthy() { t1292 } else { { let t1290 = py::getitem(&v_s.clone(), &py::neg(&Py::Int(1i64))?)?; Py::Bool({ let t1291 = Py::str("FS"); if py::eq(&t1290, &t1291) { true } else { false } }) } } }, Py::str("the first reading is a BS and the last an FS"))?;
    for t1293 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::len(&v_s.clone())?).i()?]) {
        v_i = t1293;
        let _ = f__need(py::not(&{ let t1298 = { let t1294 = py::getitem(&v_s.clone(), &v_i.clone())?; Py::Bool({ let t1295 = Py::str("BS"); if py::eq(&t1294, &t1295) { true } else { false } }) }; if !t1298.truthy() { t1298 } else { { let t1296 = py::getitem(&v_s.clone(), &py::sub(&v_i.clone(), &Py::Int(1i64))?)?; Py::Bool({ let t1297 = Py::str("FS"); if py::ne(&t1296, &t1297) { true } else { false } }) } } }), Py::str("a BS (after the first) must follow an FS at the same change point"))?;
        let _ = f__need(py::not(&{ let t1303 = { let t1299 = py::getitem(&v_s.clone(), &v_i.clone())?; Py::Bool({ let t1300 = Py::str("FS"); if py::eq(&t1299, &t1300) { true } else { false } }) }; if !t1303.truthy() { t1303 } else { { let t1301 = py::getitem(&v_s.clone(), &py::sub(&v_i.clone(), &Py::Int(1i64))?)?; Py::Bool({ let t1302 = Py::str("FS"); if py::eq(&t1301, &t1302) { true } else { false } }) } } }), Py::str("two FS readings in a row"))?;
    }
    let _ = f__finite(Py::dict(vec![(Py::str("RL0"), v_RL0.clone())]))?;
    v_rl = Py::list(vec![v_RL0.clone()]);
    let t1308 = Py::Float(0.0);
    v_rise = t1308.clone();
    v_fall = t1308.clone();
    v_prev = py::getitem(&v_r.clone(), &Py::Int(0i64))?;
    for t1309 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::len(&v_r.clone())?).i()?]) {
        v_i = t1309;
        if ({ let t1310 = py::getitem(&v_s.clone(), &v_i.clone())?; Py::Bool({ let t1311 = Py::str("BS"); if py::eq(&t1310, &t1311) { true } else { false } }) }).truthy() {
            v_prev = py::getitem(&v_r.clone(), &v_i.clone())?;
            continue;
        }
        v_diff = py::sub(&v_prev.clone(), &py::getitem(&v_r.clone(), &v_i.clone())?)?;
        if ({ let t1312 = v_diff.clone(); Py::Bool({ let t1313 = Py::Int(0i64); if py::ge(&t1312, &t1313)? { true } else { false } }) }).truthy() {
            v_rise = py::iadd(&v_rise, &v_diff.clone())?;
        } else {
            v_fall = py::sub(&v_fall, &v_diff.clone())?;
        }
        let _ = py::method(&v_rl.clone(), "append", &[py::add(&py::getitem(&v_rl.clone(), &py::neg(&Py::Int(1i64))?)?, &v_diff.clone())?])?;
        v_prev = py::getitem(&v_r.clone(), &v_i.clone())?;
    }
    v_sbs = py::sum(&{ let mut out = vec![]; for t1314 in py::iter(&(py::zip(&[v_r.clone(), v_s.clone()])?))? { let mut v_t: Py; let mut v_x: Py; 
let t1315 = (t1314).items()?;
if t1315.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t1315[0].clone();
v_t = t1315[1].clone();
if ({ let t1316 = v_t.clone(); Py::Bool({ let t1317 = Py::str("BS"); if py::eq(&t1316, &t1317) { true } else { false } }) }).truthy() { out.push(v_x.clone()); }} Py::list(out) }, None)?;
    v_sfs = py::sum(&{ let mut out = vec![]; for t1318 in py::iter(&(py::zip(&[v_r.clone(), v_s.clone()])?))? { let mut v_t: Py; let mut v_x: Py; 
let t1319 = (t1318).items()?;
if t1319.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t1319[0].clone();
v_t = t1319[1].clone();
if ({ let t1320 = v_t.clone(); Py::Bool({ let t1321 = Py::str("FS"); if py::eq(&t1320, &t1321) { true } else { false } }) }).truthy() { out.push(v_x.clone()); }} Py::list(out) }, None)?;
    v_sc = py::minmax(&[v_r.clone()], true)?;
    return Ok(Py::dict(vec![(Py::str("RL"), f__r(v_rl.clone())?), (Py::str("rise"), f__r(v_rise.clone())?), (Py::str("fall"), f__r(v_fall.clone())?), (Py::str("last_RL"), f__r(py::getitem(&v_rl.clone(), &py::neg(&Py::Int(1i64))?)?)?), (Py::str("check"), f__r(f__snap(py::sub(&v_sbs.clone(), &v_sfs.clone())?, v_sc.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_liquid_limit(mut v_blows: Py, mut v_w: Py, mut v_PL: Py) -> R {
    let mut v_LL: Py = Py::None;
    let mut v_N: Py = Py::None;
    let mut v_icpt: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_slope: Py = Py::None;
    let mut v_wc: Py = Py::None;
    v_N = f__vec(Py::str("blows"), v_blows.clone(), Py::None, Py::Bool(true), Py::Bool(false))?;
    v_wc = f__vec(Py::str("water contents"), v_w.clone(), py::len(&v_N.clone())?, Py::Bool(true), Py::Bool(false))?;
    let _ = f__need({ let t1345 = py::len(&v_N.clone())?; Py::Bool({ let t1346 = Py::Int(2i64); if py::ge(&t1345, &t1346)? { true } else { false } }) }, Py::str("give at least two cup tests"))?;
    let t1354 = (f__linfit({ let mut out = vec![]; for t1353 in py::iter(&(v_N.clone()))? { let mut v_v: Py; 
v_v = t1353;
out.push(py::math1("log10", &v_v.clone())?); } Py::list(out) }, v_wc.clone())?).items()?;
    if t1354.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_slope = t1354[0].clone();
    v_icpt = t1354[1].clone();
    let _ = f__need({ let t1355 = v_slope.clone(); Py::Bool({ let t1356 = Py::Int(0i64); if py::lt(&t1355, &t1356)? { true } else { false } }) }, Py::str("the water content must fall as the number of blows rises"))?;
    v_LL = py::add(&v_icpt.clone(), &py::mul(&v_slope.clone(), &py::math1("log10", &Py::Int(25i64))?)?)?;
    v_out = Py::dict(vec![(Py::str("LL"), f__r(v_LL.clone())?), (Py::str("flow_index"), f__r(py::neg(&v_slope.clone())?)?)]);
    if ({ let t1363 = v_PL.clone(); Py::Bool({ let t1364 = Py::None; if !py::is(&t1363, &t1364) { true } else { false } }) }).truthy() {
        let _ = f__finite(Py::dict(vec![(Py::str("PL"), v_PL.clone())]))?;
        let _ = f__need({ let t1369 = Py::Int(0i64); Py::Bool({ let t1370 = v_PL.clone(); if py::le(&t1369, &t1370)? { { let t1371 = v_LL.clone(); if py::lt(&t1370, &t1371)? { true } else { false } } } else { false } }) }, Py::str("the plastic limit must lie below the liquid limit"))?;
        py::setitem(&v_out.clone(), &Py::str("PI"), f__r(py::sub(&v_LL.clone(), &v_PL.clone())?)?)?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_log_pearson3(mut v_peaks: Py, mut v_T: Py) -> R {
    let mut v_G: Py = Py::None;
    let mut v_K: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    v_x = f__vec(Py::str("peaks"), v_peaks.clone(), Py::None, Py::Bool(true), Py::Bool(false))?;
    let _ = f__need({ let t1387 = py::len(&v_x.clone())?; Py::Bool({ let t1388 = Py::Int(10i64); if py::ge(&t1387, &t1388)? { true } else { false } }) }, Py::str("give at least ten annual maxima"))?;
    let _ = f__finite(Py::dict(vec![(Py::str("T"), v_T.clone())]))?;
    let _ = f__need({ let t1393 = v_T.clone(); Py::Bool({ let t1394 = Py::Float(1.0001); if py::gt(&t1393, &t1394)? { true } else { false } }) }, Py::str("the return period must exceed 1 year"))?;
    v_y = { let mut out = vec![]; for t1395 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t1395;
out.push(py::math1("log10", &v_v.clone())?); } Py::list(out) };
    v_n = py::len(&v_y.clone())?;
    let t1397 = (f__mean_sd(v_y.clone())?).items()?;
    if t1397.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t1397[0].clone();
    v_s = t1397[1].clone();
    let _ = f__need({ let t1398 = v_s.clone(); Py::Bool({ let t1399 = Py::Int(0i64); if py::gt(&t1398, &t1399)? { true } else { false } }) }, Py::str("the annual maxima must not all be equal"))?;
    v_G = py::truediv(&py::mul(&v_n.clone(), &py::sum(&{ let mut out = vec![]; for t1400 in py::iter(&(v_y.clone()))? { let mut v_v: Py; 
v_v = t1400;
out.push(py::pow(&py::sub(&v_v.clone(), &v_m.clone())?, &Py::Int(3i64))?); } Py::list(out) }, None)?)?, &py::mul(&py::mul(&py::sub(&v_n.clone(), &Py::Int(1i64))?, &py::sub(&v_n.clone(), &Py::Int(2i64))?)?, &py::pow(&v_s.clone(), &Py::Int(3i64))?)?)?;
    v_p = py::sub(&Py::Int(1i64), &py::truediv(&Py::Int(1i64), &v_T.clone())?)?;
    if ({ let t1401 = py::abs(&v_G.clone())?; Py::Bool({ let t1402 = Py::Float(1.6e-05); if py::lt(&t1401, &t1402)? { true } else { false } }) }).truthy() {
        v_K = py::normal_dist("inv_cdf", &Py::Float(0.0), &Py::Float(1.0), &[v_p.clone()])?;
    } else {
        v_a = py::truediv(&Py::Int(4i64), &py::mul(&v_G.clone(), &v_G.clone())?)?;
        v_K = (if ({ let t1403 = v_G.clone(); Py::Bool({ let t1404 = Py::Int(0i64); if py::gt(&t1403, &t1404)? { true } else { false } }) }).truthy() { py::truediv(&py::mul(&py::sub(&f__gamma_ppf(v_a.clone(), v_p.clone())?, &v_a.clone())?, &v_G.clone())?, &Py::Int(2i64))? } else { py::truediv(&py::mul(&py::sub(&f__gamma_ppf(v_a.clone(), py::sub(&Py::Int(1i64), &v_p.clone())?)?, &v_a.clone())?, &v_G.clone())?, &Py::Int(2i64))? });
    }
    return Ok(Py::dict(vec![(Py::str("Q_T"), f__r(py::pow(&Py::Int(10i64), &py::add(&v_m.clone(), &py::mul(&v_K.clone(), &v_s.clone())?)?)?)?), (Py::str("K_T"), f__r(v_K.clone())?), (Py::str("mean_log"), f__r(v_m.clone())?), (Py::str("std_log"), f__r(v_s.clone())?), (Py::str("skew"), f__r(v_G.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_manning_from_chezy(mut v_C: Py, mut v_R: Py) -> R {
    let mut v_n: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("C"), v_C.clone()), (Py::str("R"), v_R.clone())]))?;
    v_n = py::truediv(&py::pow(&v_R.clone(), &py::truediv(&Py::Float(1.0), &Py::Float(6.0))?)?, &v_C.clone())?;
    let _ = f__need({ let t1449 = Py::Float(0.005); Py::Bool({ let t1450 = v_n.clone(); if py::le(&t1449, &t1450)? { { let t1451 = Py::Float(0.2); if py::le(&t1450, &t1451)? { true } else { false } } } else { false } }) }, Py::str("the equivalent Manning n falls outside 0.005 to 0.2"))?;
    return Ok(Py::dict(vec![(Py::str("n"), f__r(v_n.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_manning_pipe(mut v_D: Py, mut v_y: Py, mut v_S: Py, mut v_n: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_Qf: Py = Py::None;
    let mut v_out: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("D"), v_D.clone()), (Py::str("y"), v_y.clone())]))?;
    let _ = f__need({ let t1467 = v_y.clone(); Py::Bool({ let t1468 = v_D.clone(); if py::le(&t1467, &t1468)? { true } else { false } }) }, Py::str("the depth cannot exceed the diameter"))?;
    let _ = f__ok_mann(v_S.clone(), v_n.clone())?;
    if ({ let t1474 = v_y.clone(); Py::Bool({ let t1475 = v_D.clone(); if py::eq(&t1474, &t1475) { true } else { false } }) }).truthy() {
        let t1476 = (Py::tuple(vec![py::truediv(&py::mul(&py::mul(&Py::Float(std::f64::consts::PI), &v_D.clone())?, &v_D.clone())?, &Py::Int(4i64))?, py::mul(&Py::Float(std::f64::consts::PI), &v_D.clone())?])).items()?;
        if t1476.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_A = t1476[0].clone();
        v_P = t1476[1].clone();
    } else {
        let t1477 = (py::slice(&f__circ(v_D.clone(), v_y.clone())?, None, Some((Py::Int(2i64)).i()?), None)?).items()?;
        if t1477.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_A = t1477[0].clone();
        v_P = t1477[1].clone();
    }
    v_out = f__mann_out(v_A.clone(), v_P.clone(), v_S.clone(), v_n.clone())?;
    v_Qf = py::mul(&py::truediv(&py::mul(&py::mul(&Py::Float(std::f64::consts::PI), &v_D.clone())?, &v_D.clone())?, &Py::Int(4i64))?, &f__manning_v(py::truediv(&v_D.clone(), &Py::Int(4i64))?, v_S.clone(), v_n.clone())?)?;
    let _ = { let t1484 = v_out.clone(); py::setitem(&t1484, &Py::str("Q_full"), f__r(v_Qf.clone())?)?; py::setitem(&t1484, &Py::str("ratio"), f__r(py::truediv(&py::getitem(&v_out.clone(), &Py::str("Q"))?, &v_Qf.clone())?)?)?; Py::None };
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_manning_rect(mut v_b: Py, mut v_y: Py, mut v_S: Py, mut v_n: Py) -> R {
    let _ = f__pos(Py::dict(vec![(Py::str("b"), v_b.clone()), (Py::str("y"), v_y.clone())]))?;
    let _ = f__ok_mann(v_S.clone(), v_n.clone())?;
    return Ok({ let mut v: Vec<Py> = vec![]; v.extend((py::slice(&f__rect(v_b.clone(), v_y.clone())?, None, Some((Py::Int(2i64)).i()?), None)?).items()?); v.push(v_S.clone()); v.push(v_n.clone()); let mut d: Vec<(Py, Py)> = vec![];  let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "A") { Some(x) => x, None => { return Err(PyErr::Type("_mann_out() missing argument A".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "P") { Some(x) => x, None => { return Err(PyErr::Type("_mann_out() missing argument P".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "S") { Some(x) => x, None => { return Err(PyErr::Type("_mann_out() missing argument S".into())) } } }; let a3 = if 3 < v.len() { v[3].clone() } else { match py::kw_take(&mut d, "n") { Some(x) => x, None => { return Err(PyErr::Type("_mann_out() missing argument n".into())) } } }; if v.len() > 4 { return Err(PyErr::Type("_mann_out() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("_mann_out() got an unexpected keyword argument".into())) } f__mann_out(a0, a1, a2, a3)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_manning_trap(mut v_b: Py, mut v_y: Py, mut v_z: Py, mut v_S: Py, mut v_n: Py) -> R {
    let _ = f__pos(Py::dict(vec![(Py::str("y"), v_y.clone())]))?;
    let _ = f__finite(Py::dict(vec![(Py::str("b"), v_b.clone()), (Py::str("z"), v_z.clone())]))?;
    let _ = f__need({ let t1521 = { let t1514 = v_b.clone(); Py::Bool({ let t1515 = Py::Int(0i64); if py::ge(&t1514, &t1515)? { true } else { false } }) }; if !t1521.truthy() { t1521 } else { { let t1520 = { let t1516 = v_z.clone(); Py::Bool({ let t1517 = Py::Int(0i64); if py::ge(&t1516, &t1517)? { true } else { false } }) }; if !t1520.truthy() { t1520 } else { { let t1518 = py::add(&v_b.clone(), &v_z.clone())?; Py::Bool({ let t1519 = Py::Int(0i64); if py::gt(&t1518, &t1519)? { true } else { false } }) } } } } }, Py::str("need b >= 0, z >= 0 and not both zero"))?;
    let _ = f__ok_mann(v_S.clone(), v_n.clone())?;
    return Ok({ let mut v: Vec<Py> = vec![]; v.extend((py::slice(&f__trap(v_b.clone(), v_y.clone(), v_z.clone())?, None, Some((Py::Int(2i64)).i()?), None)?).items()?); v.push(v_S.clone()); v.push(v_n.clone()); let mut d: Vec<(Py, Py)> = vec![];  let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "A") { Some(x) => x, None => { return Err(PyErr::Type("_mann_out() missing argument A".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "P") { Some(x) => x, None => { return Err(PyErr::Type("_mann_out() missing argument P".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "S") { Some(x) => x, None => { return Err(PyErr::Type("_mann_out() missing argument S".into())) } } }; let a3 = if 3 < v.len() { v[3].clone() } else { match py::kw_take(&mut d, "n") { Some(x) => x, None => { return Err(PyErr::Type("_mann_out() missing argument n".into())) } } }; if v.len() > 4 { return Err(PyErr::Type("_mann_out() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("_mann_out() got an unexpected keyword argument".into())) } f__mann_out(a0, a1, a2, a3)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_manning_velocity(mut v_R: Py, mut v_S: Py, mut v_n: Py) -> R {
    let _ = f__pos(Py::dict(vec![(Py::str("R"), v_R.clone())]))?;
    let _ = f__ok_mann(v_S.clone(), v_n.clone())?;
    return Ok(Py::dict(vec![(Py::str("V"), f__r(f__manning_v(v_R.clone(), v_S.clone(), v_n.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mohr_coulomb_failure(mut v_s3: Py, mut v_c: Py, mut v_phi: Py) -> R {
    let mut v_ctr: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_rad: Py = Py::None;
    let mut v_s1: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("s3"), v_s3.clone()), (Py::str("c"), v_c.clone()), (Py::str("phi"), v_phi.clone())]))?;
    let _ = f__need({ let t1561 = { let t1557 = v_s3.clone(); Py::Bool({ let t1558 = Py::Int(0i64); if py::ge(&t1557, &t1558)? { true } else { false } }) }; if !t1561.truthy() { t1561 } else { { let t1559 = v_c.clone(); Py::Bool({ let t1560 = Py::Int(0i64); if py::ge(&t1559, &t1560)? { true } else { false } }) } } }, Py::str("s3 and c must be zero or positive"))?;
    let _ = f__need({ let t1562 = Py::Int(0i64); Py::Bool({ let t1563 = v_phi.clone(); if py::lt(&t1562, &t1563)? { { let t1564 = Py::Int(60i64); if py::lt(&t1563, &t1564)? { true } else { false } } } else { false } }) }, Py::str("the friction angle must lie between 0 and 60 degrees"))?;
    v_p = py::math1("radians", &v_phi.clone())?;
    v_k = py::math1("tan", &py::add(&py::truediv(&Py::Float(std::f64::consts::PI), &Py::Int(4i64))?, &py::truediv(&v_p.clone(), &Py::Int(2i64))?)?)?;
    v_s1 = py::add(&py::mul(&py::mul(&v_s3.clone(), &v_k.clone())?, &v_k.clone())?, &py::mul(&py::mul(&Py::Int(2i64), &v_c.clone())?, &v_k.clone())?)?;
    let t1565 = (Py::tuple(vec![py::truediv(&py::add(&v_s1.clone(), &v_s3.clone())?, &Py::Int(2i64))?, py::truediv(&py::sub(&v_s1.clone(), &v_s3.clone())?, &Py::Int(2i64))?])).items()?;
    if t1565.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ctr = t1565[0].clone();
    v_rad = t1565[1].clone();
    return Ok(Py::dict(vec![(Py::str("s1"), f__r(v_s1.clone())?), (Py::str("sigma_f"), f__r(py::sub(&v_ctr.clone(), &py::mul(&v_rad.clone(), &py::math1("sin", &v_p.clone())?)?)?)?), (Py::str("tau_f"), f__r(py::mul(&v_rad.clone(), &py::math1("cos", &v_p.clone())?)?)?), (Py::str("theta"), f__r(py::add(&Py::Int(45i64), &py::truediv(&v_phi.clone(), &Py::Int(2i64))?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_muskingum(mut v_inflow: Py, mut v_K: Py, mut v_X: Py, mut v_dt: Py, mut v_O1: Py) -> R {
    let mut v_C0: Py = Py::None;
    let mut v_C1: Py = Py::None;
    let mut v_C2: Py = Py::None;
    let mut v_I: Py = Py::None;
    let mut v_O: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_j: Py = Py::None;
    v_I = f__vec(Py::str("inflow"), v_inflow.clone(), Py::None, Py::Bool(false), Py::Bool(true))?;
    let _ = f__pos(Py::dict(vec![(Py::str("K"), v_K.clone()), (Py::str("dt"), v_dt.clone())]))?;
    let _ = f__finite(Py::dict(vec![(Py::str("X"), v_X.clone())]))?;
    let _ = f__need({ let t1596 = Py::Int(0i64); Py::Bool({ let t1597 = v_X.clone(); if py::le(&t1596, &t1597)? { { let t1598 = Py::Float(0.5); if py::le(&t1597, &t1598)? { true } else { false } } } else { false } }) }, Py::str("the weighting factor X must lie between 0 and 0.5"))?;
    let _ = f__need({ let t1599 = py::len(&v_I.clone())?; Py::Bool({ let t1600 = Py::Int(2i64); if py::ge(&t1599, &t1600)? { true } else { false } }) }, Py::str("give at least two inflow ordinates"))?;
    v_d = py::add(&py::mul(&py::mul(&Py::Int(2i64), &v_K.clone())?, &py::sub(&Py::Int(1i64), &v_X.clone())?)?, &v_dt.clone())?;
    let t1601 = (Py::tuple(vec![py::truediv(&py::sub(&v_dt.clone(), &py::mul(&py::mul(&Py::Int(2i64), &v_K.clone())?, &v_X.clone())?)?, &v_d.clone())?, py::truediv(&py::add(&v_dt.clone(), &py::mul(&py::mul(&Py::Int(2i64), &v_K.clone())?, &v_X.clone())?)?, &v_d.clone())?, py::truediv(&py::sub(&py::mul(&py::mul(&Py::Int(2i64), &v_K.clone())?, &py::sub(&Py::Int(1i64), &v_X.clone())?)?, &v_dt.clone())?, &v_d.clone())?])).items()?;
    if t1601.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_C0 = t1601[0].clone();
    v_C1 = t1601[1].clone();
    v_C2 = t1601[2].clone();
    let _ = f__need({ let t1602 = py::minmax(&[v_C0.clone(), v_C1.clone(), v_C2.clone()], false)?; Py::Bool({ let t1603 = py::neg(&Py::Float(1e-12))?; if py::ge(&t1602, &t1603)? { true } else { false } }) }, Py::str("a routing coefficient is negative: choose 2 K X <= dt <= 2 K (1 - X)"))?;
    v_O = Py::list(vec![(if ({ let t1604 = v_O1.clone(); Py::Bool({ let t1605 = Py::None; if py::is(&t1604, &t1605) { true } else { false } }) }).truthy() { py::getitem(&v_I.clone(), &Py::Int(0i64))? } else { py::float(&v_O1.clone())? })]);
    for t1606 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::len(&v_I.clone())?).i()?]) {
        v_j = t1606;
        let _ = py::method(&v_O.clone(), "append", &[py::add(&py::add(&py::mul(&v_C0.clone(), &py::getitem(&v_I.clone(), &v_j.clone())?)?, &py::mul(&v_C1.clone(), &py::getitem(&v_I.clone(), &py::sub(&v_j.clone(), &Py::Int(1i64))?)?)?)?, &py::mul(&v_C2.clone(), &py::getitem(&v_O.clone(), &py::neg(&Py::Int(1i64))?)?)?)?])?;
    }
    return Ok(Py::dict(vec![(Py::str("outflow"), f__r(v_O.clone())?), (Py::str("C0"), f__r(v_C0.clone())?), (Py::str("C1"), f__r(v_C1.clone())?), (Py::str("C2"), f__r(v_C2.clone())?), (Py::str("peak_outflow"), f__r(py::minmax(&[v_O.clone()], true)?)?), (Py::str("attenuation"), f__r(py::sub(&py::minmax(&[v_I.clone()], true)?, &py::minmax(&[v_O.clone()], true)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_normal_depth_rect(mut v_Q: Py, mut v_b: Py, mut v_S: Py, mut v_n: Py) -> R {
    let mut v_V: Py = Py::None;
    let mut v_qf: Py = Py::None;
    let mut v_y: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("Q"), v_Q.clone()), (Py::str("b"), v_b.clone())]))?;
    let _ = f__ok_mann(v_S.clone(), v_n.clone())?;
    v_qf = { let v_Q = v_Q.clone(); let v_S = v_S.clone(); let v_b = v_b.clone(); let v_n = v_n.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_y: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok((if ({ let t1660 = v_y.clone(); Py::Bool({ let t1661 = Py::Int(0i64); if py::gt(&t1660, &t1661)? { true } else { false } }) }).truthy() { py::mul(&py::mul(&f__manning_v(py::truediv(&py::mul(&v_b.clone(), &v_y.clone())?, &py::add(&v_b.clone(), &py::mul(&Py::Int(2i64), &v_y.clone())?)?)?, v_S.clone(), v_n.clone())?, &v_b.clone())?, &v_y.clone())? } else { Py::Float(0.0) }));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_y = f__normal_depth(v_qf.clone(), v_Q.clone(), Py::Float(10000.0))?;
    v_V = py::truediv(&v_Q.clone(), &py::mul(&v_b.clone(), &v_y.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("yn"), f__r(v_y.clone())?), (Py::str("V"), f__r(v_V.clone())?), (Py::str("Fr"), f__r(py::truediv(&v_V.clone(), &py::math1("sqrt", &py::mul(&c_G()?, &v_y.clone())?)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_normal_depth_trap(mut v_Q: Py, mut v_b: Py, mut v_z: Py, mut v_S: Py, mut v_n: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_T: Py = Py::None;
    let mut v_V: Py = Py::None;
    let mut v__P: Py = Py::None;
    let mut v_qf: Py = Py::None;
    let mut v_y: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("Q"), v_Q.clone())]))?;
    let _ = f__finite(Py::dict(vec![(Py::str("b"), v_b.clone()), (Py::str("z"), v_z.clone())]))?;
    let _ = f__need({ let t1722 = { let t1715 = v_b.clone(); Py::Bool({ let t1716 = Py::Int(0i64); if py::ge(&t1715, &t1716)? { true } else { false } }) }; if !t1722.truthy() { t1722 } else { { let t1721 = { let t1717 = v_z.clone(); Py::Bool({ let t1718 = Py::Int(0i64); if py::ge(&t1717, &t1718)? { true } else { false } }) }; if !t1721.truthy() { t1721 } else { { let t1719 = py::add(&v_b.clone(), &v_z.clone())?; Py::Bool({ let t1720 = Py::Int(0i64); if py::gt(&t1719, &t1720)? { true } else { false } }) } } } } }, Py::str("need b >= 0, z >= 0 and not both zero"))?;
    let _ = f__ok_mann(v_S.clone(), v_n.clone())?;
    v_qf = { let v_Q = v_Q.clone(); let v_S = v_S.clone(); let v_b = v_b.clone(); let v_n = v_n.clone(); let v_qf = v_qf.clone(); let v_z = v_z.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_y: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_A: Py = Py::None;
            let mut v_P: Py = Py::None;
            let mut v__T: Py = Py::None;
            if ({ let t1728 = v_y.clone(); Py::Bool({ let t1729 = Py::Int(0i64); if py::le(&t1728, &t1729)? { true } else { false } }) }).truthy() {
                return Ok(Py::Float(0.0));
            }
            let t1730 = (f__trap(v_b.clone(), v_y.clone(), v_z.clone())?).items()?;
            if t1730.len() != 3 { return Err(PyErr::Value("unpack".into())); }
            v_A = t1730[0].clone();
            v_P = t1730[1].clone();
            v__T = t1730[2].clone();
            return Ok(py::mul(&f__manning_v(py::truediv(&v_A.clone(), &v_P.clone())?, v_S.clone(), v_n.clone())?, &v_A.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_y = f__normal_depth(v_qf.clone(), v_Q.clone(), Py::Float(10000.0))?;
    let t1755 = (f__trap(v_b.clone(), v_y.clone(), v_z.clone())?).items()?;
    if t1755.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t1755[0].clone();
    v__P = t1755[1].clone();
    v_T = t1755[2].clone();
    v_V = py::truediv(&v_Q.clone(), &v_A.clone())?;
    return Ok(Py::dict(vec![(Py::str("yn"), f__r(v_y.clone())?), (Py::str("V"), f__r(v_V.clone())?), (Py::str("A"), f__r(v_A.clone())?), (Py::str("Fr"), f__r(py::truediv(&v_V.clone(), &py::math1("sqrt", &py::truediv(&py::mul(&c_G()?, &v_A.clone())?, &v_T.clone())?)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_nse_metrics(mut v_obs: Py, mut v_sim: Py) -> R {
    let mut v_den: Py = Py::None;
    let mut v_mo: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_o: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_sse: Py = Py::None;
    let t1630 = (f__pair(v_obs.clone(), v_sim.clone())?).items()?;
    if t1630.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_o = t1630[0].clone();
    v_s = t1630[1].clone();
    v_n = py::len(&v_o.clone())?;
    v_mo = py::truediv(&py::sum(&v_o.clone(), None)?, &v_n.clone())?;
    v_den = py::sum(&{ let mut out = vec![]; for t1631 in py::iter(&(v_o.clone()))? { let mut v_v: Py; 
v_v = t1631;
out.push(py::pow(&py::sub(&v_v.clone(), &v_mo.clone())?, &Py::Int(2i64))?); } Py::list(out) }, None)?;
    let _ = f__need({ let t1632 = v_den.clone(); Py::Bool({ let t1633 = Py::Int(0i64); if py::gt(&t1632, &t1633)? { true } else { false } }) }, Py::str("the observed series must vary"))?;
    let _ = f__need({ let t1634 = py::sum(&v_o.clone(), None)?; Py::Bool({ let t1635 = Py::Int(0i64); if py::ne(&t1634, &t1635) { true } else { false } }) }, Py::str("the observed total must not be zero"))?;
    v_sse = py::sum(&{ let mut out = vec![]; for t1636 in py::iter(&(py::zip(&[v_o.clone(), v_s.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t1637 = (t1636).items()?;
if t1637.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t1637[0].clone();
v_b = t1637[1].clone();
out.push(py::pow(&py::sub(&v_a.clone(), &v_b.clone())?, &Py::Int(2i64))?); } Py::list(out) }, None)?;
    return Ok(Py::dict(vec![(Py::str("NSE"), f__r(py::sub(&Py::Int(1i64), &py::truediv(&v_sse.clone(), &v_den.clone())?)?)?), (Py::str("RMSE"), f__r(py::math1("sqrt", &py::truediv(&v_sse.clone(), &v_n.clone())?)?)?), (Py::str("PBIAS"), f__r(py::truediv(&py::mul(&Py::Int(100i64), &py::sum(&{ let mut out = vec![]; for t1644 in py::iter(&(py::zip(&[v_o.clone(), v_s.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t1645 = (t1644).items()?;
if t1645.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t1645[0].clone();
v_b = t1645[1].clone();
out.push(py::sub(&v_a.clone(), &v_b.clone())?); } Py::list(out) }, None)?)?, &py::sum(&v_o.clone(), None)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_offsets_area(mut v_offsets: Py, mut v_d: Py) -> R {
    let mut v_out: Py = Py::None;
    let mut v_y: Py = Py::None;
    v_y = f__vec(Py::str("offsets"), v_offsets.clone(), Py::None, Py::Bool(false), Py::Bool(true))?;
    let _ = f__need({ let t61 = py::len(&v_y.clone())?; Py::Bool({ let t62 = Py::Int(2i64); if py::ge(&t61, &t62)? { true } else { false } }) }, Py::str("give at least two offsets"))?;
    let _ = f__pos(Py::dict(vec![(Py::str("d"), v_d.clone())]))?;
    v_out = Py::dict(vec![(Py::str("A_trapezoidal"), f__r(py::mul(&v_d.clone(), &py::sub(&py::sum(&v_y.clone(), None)?, &py::truediv(&py::add(&py::getitem(&v_y.clone(), &Py::Int(0i64))?, &py::getitem(&v_y.clone(), &py::neg(&Py::Int(1i64))?)?)?, &Py::Int(2i64))?)?)?)?)]);
    if ({ let t78 = py::modulo(&py::len(&v_y.clone())?, &Py::Int(2i64))?; Py::Bool({ let t79 = Py::Int(1i64); if py::eq(&t78, &t79) { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("A_simpson"), f__r(py::mul(&py::truediv(&v_d.clone(), &Py::Int(3i64))?, &py::add(&py::add(&py::add(&py::getitem(&v_y.clone(), &Py::Int(0i64))?, &py::getitem(&v_y.clone(), &py::neg(&Py::Int(1i64))?)?)?, &py::mul(&Py::Int(4i64), &py::sum(&py::slice(&v_y.clone(), Some((Py::Int(1i64)).i()?), Some((py::neg(&Py::Int(1i64))?).i()?), Some((Py::Int(2i64)).i()?))?, None)?)?)?, &py::mul(&Py::Int(2i64), &py::sum(&py::slice(&v_y.clone(), Some((Py::Int(2i64)).i()?), Some((py::neg(&Py::Int(1i64))?).i()?), Some((Py::Int(2i64)).i()?))?, None)?)?)?)?)?)?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_orifice_discharge(mut v_Cd: Py, mut v_d: Py, mut v_h: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("Cd"), v_Cd.clone()), (Py::str("d"), v_d.clone()), (Py::str("h"), v_h.clone())]))?;
    let _ = f__need({ let t1771 = v_Cd.clone(); Py::Bool({ let t1772 = Py::Float(1.0); if py::le(&t1771, &t1772)? { true } else { false } }) }, Py::str("the discharge coefficient must not exceed 1"))?;
    v_a = py::truediv(&py::mul(&py::mul(&Py::Float(std::f64::consts::PI), &v_d.clone())?, &v_d.clone())?, &Py::Int(4i64))?;
    v_v = py::math1("sqrt", &py::mul(&py::mul(&Py::Int(2i64), &c_G()?)?, &v_h.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("Q"), f__r(py::mul(&py::mul(&v_Cd.clone(), &v_a.clone())?, &v_v.clone())?)?), (Py::str("V_ideal"), f__r(v_v.clone())?), (Py::str("area"), f__r(v_a.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_parallel_pipes(mut v_Q: Py, mut v_L: Py, mut v_D: Py, mut v_C: Py) -> R {
    let mut v_flows: Py = Py::None;
    let mut v_hf: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_ks: Py = Py::None;
    let mut v_n: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("Q"), v_Q.clone())]))?;
    v_n = py::len(&v_L.clone())?;
    let _ = f__need({ let t1788 = v_n.clone(); Py::Bool({ let t1789 = Py::Int(2i64); if py::ge(&t1788, &t1789)? { true } else { false } }) }, Py::str("give at least two pipes"))?;
    v_L = f__vec(Py::str("L"), v_L.clone(), Py::None, Py::Bool(true), Py::Bool(false))?;
    v_D = { let mut out = vec![]; for t1805 in py::iter(&(f__vec(Py::str("D"), v_D.clone(), v_n.clone(), Py::Bool(true), Py::Bool(false))?))? { let mut v_d: Py; 
v_d = t1805;
out.push(py::truediv(&v_d.clone(), &Py::Int(1000i64))?); } Py::list(out) };
    v_C = f__vec(Py::str("C"), v_C.clone(), v_n.clone(), Py::Bool(true), Py::Bool(false))?;
    for t1806 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t1806;
        let _ = f__ok_pipe(py::getitem(&v_L.clone(), &v_i.clone())?, py::getitem(&v_D.clone(), &v_i.clone())?, py::getitem(&v_C.clone(), &v_i.clone())?)?;
    }
    v_ks = { let mut out = vec![]; for t1812 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t1812;
out.push(f__hw_k(py::getitem(&v_L.clone(), &v_i.clone())?, py::getitem(&v_D.clone(), &v_i.clone())?, py::getitem(&v_C.clone(), &v_i.clone())?)?); } Py::list(out) };
    v_hf = py::pow(&py::truediv(&v_Q.clone(), &py::sum(&{ let mut out = vec![]; for t1813 in py::iter(&(v_ks.clone()))? { let mut v_k: Py; 
v_k = t1813;
out.push(py::pow(&v_k.clone(), &py::truediv(&py::neg(&Py::Int(1i64))?, &Py::Float(1.852))?)?); } Py::list(out) }, None)?)?, &Py::Float(1.852))?;
    v_flows = { let mut out = vec![]; for t1814 in py::iter(&(v_ks.clone()))? { let mut v_k: Py; 
v_k = t1814;
out.push(py::pow(&py::truediv(&v_hf.clone(), &v_k.clone())?, &py::truediv(&Py::Int(1i64), &Py::Float(1.852))?)?); } Py::list(out) };
    return Ok(Py::dict(vec![(Py::str("hf"), f__r(v_hf.clone())?), (Py::str("flows"), f__r(v_flows.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_phase_from_lab(mut v_M: Py, mut v_Md: Py, mut v_V: Py, mut v_Gs: Py) -> R {
    let mut v_S: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_rho: Py = Py::None;
    let mut v_rhod: Py = Py::None;
    let mut v_w: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("M"), v_M.clone()), (Py::str("Md"), v_Md.clone()), (Py::str("V"), v_V.clone())]))?;
    let _ = f__finite(Py::dict(vec![(Py::str("Gs"), v_Gs.clone())]))?;
    let _ = f__need({ let t2131 = v_M.clone(); Py::Bool({ let t2132 = v_Md.clone(); if py::ge(&t2131, &t2132)? { true } else { false } }) }, Py::str("the wet mass cannot be less than the dry mass"))?;
    let _ = f__need({ let t2133 = Py::Float(2.0); Py::Bool({ let t2134 = v_Gs.clone(); if py::le(&t2133, &t2134)? { { let t2135 = Py::Float(3.0); if py::le(&t2134, &t2135)? { true } else { false } } } else { false } }) }, Py::str("the specific gravity of solids must lie between 2.0 and 3.0"))?;
    v_w = py::truediv(&py::sub(&v_M.clone(), &v_Md.clone())?, &v_Md.clone())?;
    let t2136 = (Py::tuple(vec![py::truediv(&v_M.clone(), &v_V.clone())?, py::truediv(&v_Md.clone(), &v_V.clone())?])).items()?;
    if t2136.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_rho = t2136[0].clone();
    v_rhod = t2136[1].clone();
    v_e = py::sub(&py::truediv(&py::mul(&v_Gs.clone(), &c_RHO_W()?)?, &v_rhod.clone())?, &Py::Int(1i64))?;
    let _ = f__need({ let t2137 = v_e.clone(); Py::Bool({ let t2138 = Py::Int(0i64); if py::gt(&t2137, &t2138)? { true } else { false } }) }, Py::str("the dry density exceeds that of the solids: check the masses and volume"))?;
    v_S = py::truediv(&py::mul(&v_w.clone(), &v_Gs.clone())?, &v_e.clone())?;
    let _ = f__need({ let t2139 = v_S.clone(); Py::Bool({ let t2140 = py::add(&Py::Int(1i64), &Py::Float(1e-09))?; if py::le(&t2139, &t2140)? { true } else { false } }) }, Py::str("the data imply more than full saturation: check the masses and volume"))?;
    return Ok(Py::dict(vec![(Py::str("w"), f__r(v_w.clone())?), (Py::str("rho"), f__r(v_rho.clone())?), (Py::str("rho_d"), f__r(v_rhod.clone())?), (Py::str("e"), f__r(v_e.clone())?), (Py::str("n"), f__r(py::truediv(&v_e.clone(), &py::add(&Py::Int(1i64), &v_e.clone())?)?)?), (Py::str("S"), f__r(v_S.clone())?), (Py::str("gamma"), f__r(py::mul(&v_rho.clone(), &c_G()?)?)?), (Py::str("gamma_d"), f__r(py::mul(&v_rhod.clone(), &c_G()?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_phase_from_void_ratio(mut v_Gs: Py, mut v_e: Py, mut v_w: Py) -> R {
    let mut v_S: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_gd: Py = Py::None;
    let mut v_gsat: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("Gs"), v_Gs.clone()), (Py::str("e"), v_e.clone()), (Py::str("w"), v_w.clone())]))?;
    let _ = f__need({ let t2151 = Py::Float(2.0); Py::Bool({ let t2152 = v_Gs.clone(); if py::le(&t2151, &t2152)? { { let t2153 = Py::Float(3.0); if py::le(&t2152, &t2153)? { true } else { false } } } else { false } }) }, Py::str("the specific gravity of solids must lie between 2.0 and 3.0"))?;
    let _ = f__need({ let t2154 = Py::Int(0i64); Py::Bool({ let t2155 = v_e.clone(); if py::lt(&t2154, &t2155)? { { let t2156 = Py::Int(4i64); if py::le(&t2155, &t2156)? { true } else { false } } } else { false } }) }, Py::str("the void ratio must lie between 0 and 4"))?;
    let _ = f__need({ let t2157 = v_w.clone(); Py::Bool({ let t2158 = Py::Int(0i64); if py::ge(&t2157, &t2158)? { true } else { false } }) }, Py::str("the water content must be zero or positive"))?;
    v_S = py::truediv(&py::mul(&v_w.clone(), &v_Gs.clone())?, &v_e.clone())?;
    let _ = f__need({ let t2159 = v_S.clone(); Py::Bool({ let t2160 = py::add(&Py::Int(1i64), &Py::Float(1e-12))?; if py::le(&t2159, &t2160)? { true } else { false } }) }, Py::str("w Gs exceeds e: the soil would be more than saturated"))?;
    v_gd = py::truediv(&py::mul(&v_Gs.clone(), &c_GAMMA_W()?)?, &py::add(&Py::Int(1i64), &v_e.clone())?)?;
    v_g = py::truediv(&py::mul(&py::mul(&v_Gs.clone(), &c_GAMMA_W()?)?, &py::add(&Py::Int(1i64), &v_w.clone())?)?, &py::add(&Py::Int(1i64), &v_e.clone())?)?;
    v_gsat = py::truediv(&py::mul(&py::add(&v_Gs.clone(), &v_e.clone())?, &c_GAMMA_W()?)?, &py::add(&Py::Int(1i64), &v_e.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("n"), f__r(py::truediv(&v_e.clone(), &py::add(&Py::Int(1i64), &v_e.clone())?)?)?), (Py::str("S"), f__r(py::minmax(&[v_S.clone(), Py::Float(1.0)], false)?)?), (Py::str("gamma_d"), f__r(v_gd.clone())?), (Py::str("gamma"), f__r(v_g.clone())?), (Py::str("gamma_sat"), f__r(v_gsat.clone())?), (Py::str("gamma_sub"), f__r(py::sub(&v_gsat.clone(), &c_GAMMA_W()?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_phi_index(mut v_rain: Py, mut v_dt: Py, mut v_R: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_cand: Py = Py::None;
    let mut v_excess: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_order: Py = Py::None;
    let mut v_phi: Py = Py::None;
    v_P = f__vec(Py::str("rainfall"), v_rain.clone(), Py::None, Py::Bool(false), Py::Bool(true))?;
    let _ = f__pos(Py::dict(vec![(Py::str("dt"), v_dt.clone())]))?;
    let _ = f__finite(Py::dict(vec![(Py::str("R"), v_R.clone())]))?;
    let _ = f__need({ let t1845 = Py::Int(0i64); Py::Bool({ let t1846 = v_R.clone(); if py::lt(&t1845, &t1846)? { { let t1847 = py::sum(&v_P.clone(), None)?; if py::lt(&t1846, &t1847)? { true } else { false } } } else { false } }) }, Py::str("the runoff depth must be positive and less than the total rainfall"))?;
    v_order = py::sorted(&v_P.clone(), (Py::Bool(true)).truthy())?;
    v_phi = Py::None;
    for t1848 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&py::len(&v_order.clone())?, &Py::Int(1i64))?).i()?]) {
        v_k = t1848;
        v_cand = py::truediv(&py::sub(&py::sum(&py::slice(&v_order.clone(), None, Some((v_k.clone()).i()?), None)?, None)?, &v_R.clone())?, &v_k.clone())?;
        if ({ let t1855 = { let t1849 = v_cand.clone(); Py::Bool({ let t1850 = py::sub(&(if ({ let t1851 = v_k.clone(); Py::Bool({ let t1852 = py::len(&v_order.clone())?; if py::lt(&t1851, &t1852)? { true } else { false } }) }).truthy() { py::getitem(&v_order.clone(), &v_k.clone())? } else { Py::Float(0.0) }), &py::mul(&Py::Float(1e-15), &py::getitem(&v_order.clone(), &Py::Int(0i64))?)?)?; if py::ge(&t1849, &t1850)? { true } else { false } }) }; if !t1855.truthy() { t1855 } else { { let t1853 = v_cand.clone(); Py::Bool({ let t1854 = py::getitem(&v_order.clone(), &py::sub(&v_k.clone(), &Py::Int(1i64))?)?; if py::le(&t1853, &t1854)? { true } else { false } }) } } }).truthy() {
            v_phi = v_cand.clone();
            break;
        }
    }
    let _ = f__need({ let t1856 = v_phi.clone(); Py::Bool({ let t1857 = Py::None; if !py::is(&t1856, &t1857) { true } else { false } }) }, Py::str("no phi-index fits the data"))?;
    v_excess = { let mut out = vec![]; for t1858 in py::iter(&(v_P.clone()))? { let mut v_p: Py; 
v_p = t1858;
out.push(py::minmax(&[py::sub(&v_p.clone(), &v_phi.clone())?, Py::Float(0.0)], true)?); } Py::list(out) };
    v_excess = { let mut out = vec![]; for t1859 in py::iter(&(v_excess.clone()))? { let mut v_e: Py; 
v_e = t1859;
out.push(f__snap(v_e.clone(), py::getitem(&v_order.clone(), &Py::Int(0i64))?)?); } Py::list(out) };
    return Ok(Py::dict(vec![(Py::str("phi"), f__r(py::truediv(&v_phi.clone(), &py::truediv(&v_dt.clone(), &Py::Int(3600i64))?)?)?), (Py::str("excess"), f__r(v_excess.clone())?), (Py::str("contributing"), py::sum(&{ let mut out = vec![]; for t1868 in py::iter(&(v_excess.clone()))? { let mut v_e: Py; 
v_e = t1868;
if ({ let t1869 = v_e.clone(); Py::Bool({ let t1870 = Py::Int(0i64); if py::gt(&t1869, &t1870)? { true } else { false } }) }).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_proctor(mut v_w: Py, mut v_rho_d: Py, mut v_Gs: Py) -> R {
    let mut v_S: Py = Py::None;
    let mut v_c0: Py = Py::None;
    let mut v_c1: Py = Py::None;
    let mut v_c2: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_wopt: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    v_x = f__vec(Py::str("water contents"), v_w.clone(), Py::None, Py::Bool(true), Py::Bool(false))?;
    v_y = f__vec(Py::str("dry densities"), v_rho_d.clone(), py::len(&v_x.clone())?, Py::Bool(true), Py::Bool(false))?;
    let _ = f__need({ let t1907 = py::len(&v_x.clone())?; Py::Bool({ let t1908 = Py::Int(3i64); if py::ge(&t1907, &t1908)? { true } else { false } }) }, Py::str("give at least three compaction points"))?;
    v_S = { let mut out = vec![]; for t1909 in py::range_iter(&[(Py::Int(3i64)).i()?]) { let mut v_i: Py; 
v_i = t1909;
out.push({ let mut out = vec![]; for t1910 in py::range_iter(&[(Py::Int(3i64)).i()?]) { let mut v_j: Py; 
v_j = t1910;
out.push(py::sum(&{ let mut out = vec![]; for t1911 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t1911;
out.push(py::pow(&v_v.clone(), &py::add(&v_i.clone(), &v_j.clone())?)?); } Py::list(out) }, None)?); } Py::list(out) }); } Py::list(out) };
    v_r = { let mut out = vec![]; for t1912 in py::range_iter(&[(Py::Int(3i64)).i()?]) { let mut v_i: Py; 
v_i = t1912;
out.push(py::sum(&{ let mut out = vec![]; for t1913 in py::iter(&(py::zip(&[v_x.clone(), v_y.clone()])?))? { let mut v_v: Py; let mut v_yy: Py; 
let t1914 = (t1913).items()?;
if t1914.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_v = t1914[0].clone();
v_yy = t1914[1].clone();
out.push(py::mul(&v_yy.clone(), &py::pow(&v_v.clone(), &v_i.clone())?)?); } Py::list(out) }, None)?); } Py::list(out) };
    let t1927 = (f__solve_linear(v_S.clone(), v_r.clone())?).items()?;
    if t1927.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_c0 = t1927[0].clone();
    v_c1 = t1927[1].clone();
    v_c2 = t1927[2].clone();
    let _ = f__need({ let t1928 = v_c2.clone(); Py::Bool({ let t1929 = Py::Int(0i64); if py::lt(&t1928, &t1929)? { true } else { false } }) }, Py::str("the points do not form a peak (the fitted parabola opens upward)"))?;
    v_wopt = py::truediv(&py::neg(&v_c1.clone())?, &py::mul(&Py::Int(2i64), &v_c2.clone())?)?;
    let _ = f__need({ let t1930 = py::minmax(&[v_x.clone()], false)?; Py::Bool({ let t1931 = v_wopt.clone(); if py::le(&t1930, &t1931)? { { let t1932 = py::minmax(&[v_x.clone()], true)?; if py::le(&t1931, &t1932)? { true } else { false } } } else { false } }) }, Py::str("the fitted optimum lies outside the tested water contents"))?;
    v_out = Py::dict(vec![(Py::str("w_opt"), f__r(v_wopt.clone())?), (Py::str("rho_dmax"), f__r(py::add(&py::add(&v_c0.clone(), &py::mul(&v_c1.clone(), &v_wopt.clone())?)?, &py::mul(&py::mul(&v_c2.clone(), &v_wopt.clone())?, &v_wopt.clone())?)?)?)]);
    if ({ let t1939 = v_Gs.clone(); Py::Bool({ let t1940 = Py::None; if !py::is(&t1939, &t1940) { true } else { false } }) }).truthy() {
        let _ = f__finite(Py::dict(vec![(Py::str("Gs"), v_Gs.clone())]))?;
        let _ = f__need({ let t1945 = Py::Float(2.0); Py::Bool({ let t1946 = v_Gs.clone(); if py::le(&t1945, &t1946)? { { let t1947 = Py::Float(3.0); if py::le(&t1946, &t1947)? { true } else { false } } } else { false } }) }, Py::str("the specific gravity must lie between 2.0 and 3.0"))?;
        py::setitem(&v_out.clone(), &Py::str("rho_zav"), f__r(py::truediv(&py::mul(&v_Gs.clone(), &Py::Float(1.0))?, &py::add(&Py::Int(1i64), &py::mul(&py::truediv(&v_wopt.clone(), &Py::Int(100i64))?, &v_Gs.clone())?)?)?)?)?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rankine_active(mut v_phi: Py, mut v_gam: Py, mut v_H: Py, mut v_c: Py, mut v_q: Py) -> R {
    let mut v_p: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("phi"), v_phi.clone()), (Py::str("c"), v_c.clone()), (Py::str("q"), v_q.clone())]))?;
    let _ = f__pos(Py::dict(vec![(Py::str("gam"), v_gam.clone()), (Py::str("H"), v_H.clone())]))?;
    let _ = f__need({ let t1957 = Py::Int(10i64); Py::Bool({ let t1958 = v_phi.clone(); if py::le(&t1957, &t1958)? { { let t1959 = Py::Int(50i64); if py::le(&t1958, &t1959)? { true } else { false } } } else { false } }) }, Py::str("the friction angle must lie between 10 and 50 degrees"))?;
    let _ = f__need({ let t1964 = { let t1960 = v_c.clone(); Py::Bool({ let t1961 = Py::Int(0i64); if py::ge(&t1960, &t1961)? { true } else { false } }) }; if !t1964.truthy() { t1964 } else { { let t1962 = v_q.clone(); Py::Bool({ let t1963 = Py::Int(0i64); if py::ge(&t1962, &t1963)? { true } else { false } }) } } }, Py::str("c and q must be zero or positive"))?;
    v_p = py::math1("radians", &v_phi.clone())?;
    return Ok(f__rankine_active_K(v_phi.clone(), v_gam.clone(), v_H.clone(), v_c.clone(), v_q.clone(), py::truediv(&py::sub(&Py::Int(1i64), &py::math1("sin", &v_p.clone())?)?, &py::add(&Py::Int(1i64), &py::math1("sin", &v_p.clone())?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rankine_passive(mut v_phi: Py, mut v_gam: Py, mut v_H: Py, mut v_c: Py, mut v_q: Py) -> R {
    let mut v_Kp: Py = Py::None;
    let mut v_Pp: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_s0: Py = Py::None;
    let mut v_sH: Py = Py::None;
    let mut v_sk: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("phi"), v_phi.clone()), (Py::str("c"), v_c.clone()), (Py::str("q"), v_q.clone())]))?;
    let _ = f__pos(Py::dict(vec![(Py::str("gam"), v_gam.clone()), (Py::str("H"), v_H.clone())]))?;
    let _ = f__need({ let t1986 = Py::Int(10i64); Py::Bool({ let t1987 = v_phi.clone(); if py::le(&t1986, &t1987)? { { let t1988 = Py::Int(50i64); if py::le(&t1987, &t1988)? { true } else { false } } } else { false } }) }, Py::str("the friction angle must lie between 10 and 50 degrees"))?;
    let _ = f__need({ let t1993 = { let t1989 = v_c.clone(); Py::Bool({ let t1990 = Py::Int(0i64); if py::ge(&t1989, &t1990)? { true } else { false } }) }; if !t1993.truthy() { t1993 } else { { let t1991 = v_q.clone(); Py::Bool({ let t1992 = Py::Int(0i64); if py::ge(&t1991, &t1992)? { true } else { false } }) } } }, Py::str("c and q must be zero or positive"))?;
    v_p = py::math1("radians", &v_phi.clone())?;
    v_Kp = py::truediv(&py::add(&Py::Int(1i64), &py::math1("sin", &v_p.clone())?)?, &py::sub(&Py::Int(1i64), &py::math1("sin", &v_p.clone())?)?)?;
    v_sk = py::math1("sqrt", &v_Kp.clone())?;
    let t1994 = (Py::tuple(vec![py::add(&py::mul(&v_Kp.clone(), &v_q.clone())?, &py::mul(&py::mul(&Py::Int(2i64), &v_c.clone())?, &v_sk.clone())?)?, py::add(&py::mul(&v_Kp.clone(), &py::add(&py::mul(&v_gam.clone(), &v_H.clone())?, &v_q.clone())?)?, &py::mul(&py::mul(&Py::Int(2i64), &v_c.clone())?, &v_sk.clone())?)?])).items()?;
    if t1994.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s0 = t1994[0].clone();
    v_sH = t1994[1].clone();
    v_Pp = py::mul(&py::truediv(&py::add(&v_s0.clone(), &v_sH.clone())?, &Py::Int(2i64))?, &v_H.clone())?;
    return Ok(Py::dict(vec![(Py::str("Kp"), f__r(v_Kp.clone())?), (Py::str("sigma_base"), f__r(v_sH.clone())?), (Py::str("Pp"), f__r(v_Pp.clone())?), (Py::str("ybar"), f__r(py::truediv(&py::mul(&v_H.clone(), &py::add(&py::mul(&Py::Int(2i64), &v_s0.clone())?, &v_sH.clone())?)?, &py::mul(&Py::Int(3i64), &py::add(&v_s0.clone(), &v_sH.clone())?)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rational_peak(mut v_C: Py, mut v_i: Py, mut v_A: Py) -> R {
    let mut v_Q: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("C"), v_C.clone())]))?;
    let _ = f__pos(Py::dict(vec![(Py::str("i"), v_i.clone()), (Py::str("A"), v_A.clone())]))?;
    let _ = f__need({ let t2010 = Py::Int(0i64); Py::Bool({ let t2011 = v_C.clone(); if py::lt(&t2010, &t2011)? { { let t2012 = Py::Int(1i64); if py::le(&t2011, &t2012)? { true } else { false } } } else { false } }) }, Py::str("the runoff coefficient must lie in (0, 1]"))?;
    let _ = f__need({ let t2013 = v_A.clone(); Py::Bool({ let t2014 = Py::Float(8000000.0); if py::le(&t2013, &t2014)? { true } else { false } }) }, Py::str("the rational method is for small catchments (at most 800 ha here)"))?;
    v_Q = py::mul(&py::mul(&v_C.clone(), &v_i.clone())?, &v_A.clone())?;
    return Ok(Py::dict(vec![(Py::str("Q"), f__r(v_Q.clone())?), (Py::str("Q_Ls"), f__r(py::mul(&v_Q.clone(), &Py::Float(1000.0))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rect_section(mut v_b: Py, mut v_y: Py) -> R {
    let _ = f__pos(Py::dict(vec![(Py::str("b"), v_b.clone()), (Py::str("y"), v_y.clone())]))?;
    return Ok({ let mut v: Vec<Py> = vec![]; v.extend((f__rect(v_b.clone(), v_y.clone())?).items()?); let mut d: Vec<(Py, Py)> = vec![];  let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "A") { Some(x) => x, None => { return Err(PyErr::Type("_section_out() missing argument A".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "P") { Some(x) => x, None => { return Err(PyErr::Type("_section_out() missing argument P".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "T") { Some(x) => x, None => { return Err(PyErr::Type("_section_out() missing argument T".into())) } } }; if v.len() > 3 { return Err(PyErr::Type("_section_out() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("_section_out() got an unexpected keyword argument".into())) } f__section_out(a0, a1, a2)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rect_weir(mut v_Cd: Py, mut v_b: Py, mut v_H: Py, mut v_contractions: Py) -> R {
    let mut v_Le: Py = Py::None;
    let mut v_Q: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("Cd"), v_Cd.clone()), (Py::str("b"), v_b.clone()), (Py::str("H"), v_H.clone())]))?;
    let _ = f__need({ let t2045 = v_Cd.clone(); Py::Bool({ let t2046 = Py::Float(1.0); if py::le(&t2045, &t2046)? { true } else { false } }) }, Py::str("the discharge coefficient must not exceed 1"))?;
    let _ = f__need({ let t2047 = v_contractions.clone(); Py::Bool({ let t2048 = Py::tuple(vec![Py::Int(0i64), Py::Int(1i64), Py::Int(2i64)]); if py::contains(&t2048, &t2047)? { true } else { false } }) }, Py::str("the number of end contractions is 0, 1 or 2"))?;
    v_Le = py::sub(&v_b.clone(), &py::mul(&py::mul(&Py::Float(0.1), &v_contractions.clone())?, &v_H.clone())?)?;
    let _ = f__need({ let t2049 = v_Le.clone(); Py::Bool({ let t2050 = Py::Int(0i64); if py::gt(&t2049, &t2050)? { true } else { false } }) }, Py::str("the effective crest length b - 0.1 n H must be positive"))?;
    v_Q = py::mul(&py::mul(&py::mul(&py::mul(&py::truediv(&Py::Float(2.0), &Py::Float(3.0))?, &v_Cd.clone())?, &v_Le.clone())?, &py::math1("sqrt", &py::mul(&Py::Int(2i64), &c_G()?)?)?)?, &py::pow(&v_H.clone(), &Py::Float(1.5))?)?;
    return Ok(Py::dict(vec![(Py::str("Q"), f__r(v_Q.clone())?), (Py::str("L_eff"), f__r(v_Le.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rectangle_load(mut v_q: Py, mut v_B: Py, mut v_L: Py, mut v_z: Py, mut v_position: Py) -> R {
    let mut v_I: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("q"), v_q.clone()), (Py::str("B"), v_B.clone()), (Py::str("L"), v_L.clone()), (Py::str("z"), v_z.clone())]))?;
    let _ = f__need({ let t2191 = v_position.clone(); Py::Bool({ let t2192 = Py::tuple(vec![Py::str("corner"), Py::str("centre")]); if py::contains(&t2192, &t2191)? { true } else { false } }) }, Py::str("position is corner or centre"))?;
    if ({ let t2193 = v_position.clone(); Py::Bool({ let t2194 = Py::str("corner"); if py::eq(&t2193, &t2194) { true } else { false } }) }).truthy() {
        v_I = f__newmark_I(py::truediv(&v_B.clone(), &v_z.clone())?, py::truediv(&v_L.clone(), &v_z.clone())?)?;
    } else {
        v_I = py::mul(&Py::Int(4i64), &f__newmark_I(py::truediv(&py::truediv(&v_B.clone(), &Py::Int(2i64))?, &v_z.clone())?, py::truediv(&py::truediv(&v_L.clone(), &Py::Int(2i64))?, &v_z.clone())?)?)?;
    }
    return Ok(Py::dict(vec![(Py::str("dsigma"), f__r(py::mul(&v_q.clone(), &v_I.clone())?)?), (Py::str("I"), f__r(v_I.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_relative_density(mut v_gd: Py, mut v_gd_min: Py, mut v_gd_max: Py) -> R {
    let mut v_Dr: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("gd"), v_gd.clone()), (Py::str("gd_min"), v_gd_min.clone()), (Py::str("gd_max"), v_gd_max.clone())]))?;
    let _ = f__need({ let t2066 = v_gd_min.clone(); Py::Bool({ let t2067 = v_gd_max.clone(); if py::lt(&t2066, &t2067)? { true } else { false } }) }, Py::str("the minimum dry unit weight must be below the maximum"))?;
    let _ = f__need({ let t2068 = v_gd_min.clone(); Py::Bool({ let t2069 = v_gd.clone(); if py::le(&t2068, &t2069)? { { let t2070 = v_gd_max.clone(); if py::le(&t2069, &t2070)? { true } else { false } } } else { false } }) }, Py::str("the dry unit weight must lie between the minimum and the maximum"))?;
    v_Dr = py::truediv(&py::mul(&v_gd_max.clone(), &py::sub(&v_gd.clone(), &v_gd_min.clone())?)?, &py::mul(&v_gd.clone(), &py::sub(&v_gd_max.clone(), &v_gd_min.clone())?)?)?;
    return Ok(Py::dict(vec![(Py::str("Dr"), f__r(v_Dr.clone())?), (Py::str("Dr_percent"), f__r(py::mul(&Py::Int(100i64), &v_Dr.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_scs_runoff(mut v_P: Py, mut v_CN: Py, mut v_lam: Py) -> R {
    let mut v_Ia: Py = Py::None;
    let mut v_Q: Py = Py::None;
    let mut v_S: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("P"), v_P.clone()), (Py::str("CN"), v_CN.clone()), (Py::str("lam"), v_lam.clone())]))?;
    let _ = f__need({ let t2081 = v_P.clone(); Py::Bool({ let t2082 = Py::Int(0i64); if py::ge(&t2081, &t2082)? { true } else { false } }) }, Py::str("the rainfall depth must be zero or positive"))?;
    let _ = f__need({ let t2083 = Py::Int(30i64); Py::Bool({ let t2084 = v_CN.clone(); if py::le(&t2083, &t2084)? { { let t2085 = Py::Int(100i64); if py::le(&t2084, &t2085)? { true } else { false } } } else { false } }) }, Py::str("the curve number must lie between 30 and 100"))?;
    let _ = f__need({ let t2086 = Py::Int(0i64); Py::Bool({ let t2087 = v_lam.clone(); if py::le(&t2086, &t2087)? { { let t2088 = Py::Float(0.3); if py::le(&t2087, &t2088)? { true } else { false } } } else { false } }) }, Py::str("the initial-abstraction ratio must lie between 0 and 0.3"))?;
    v_S = py::sub(&py::truediv(&Py::Float(25400.0), &v_CN.clone())?, &Py::Float(254.0))?;
    v_Ia = py::mul(&v_lam.clone(), &v_S.clone())?;
    v_Q = (if ({ let t2089 = v_P.clone(); Py::Bool({ let t2090 = v_Ia.clone(); if py::gt(&t2089, &t2090)? { true } else { false } }) }).truthy() { py::truediv(&py::pow(&py::sub(&v_P.clone(), &v_Ia.clone())?, &Py::Int(2i64))?, &py::add(&py::sub(&v_P.clone(), &v_Ia.clone())?, &v_S.clone())?)? } else { Py::Float(0.0) });
    return Ok(Py::dict(vec![(Py::str("Q"), f__r(v_Q.clone())?), (Py::str("S"), f__r(v_S.clone())?), (Py::str("Ia"), f__r(v_Ia.clone())?), (Py::str("C"), f__r((if ({ let t2097 = v_P.clone(); Py::Bool({ let t2098 = Py::Int(0i64); if py::gt(&t2097, &t2098)? { true } else { false } }) }).truthy() { py::truediv(&v_Q.clone(), &v_P.clone())? } else { Py::Float(0.0) }))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_secondary_compression(mut v_Ca: Py, mut v_H: Py, mut v_ep: Py, mut v_t1: Py, mut v_t2: Py) -> R {
    let mut v_Cm: Py = Py::None;
    let mut v_S: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("Ca"), v_Ca.clone()), (Py::str("ep"), v_ep.clone())]))?;
    let _ = f__pos(Py::dict(vec![(Py::str("H"), v_H.clone()), (Py::str("t1"), v_t1.clone()), (Py::str("t2"), v_t2.clone())]))?;
    let _ = f__need({ let t2108 = Py::Int(0i64); Py::Bool({ let t2109 = v_Ca.clone(); if py::lt(&t2108, &t2109)? { { let t2110 = Py::Float(0.2); if py::le(&t2109, &t2110)? { true } else { false } } } else { false } }) }, Py::str("the secondary compression index must lie in (0, 0.2]"))?;
    let _ = f__need({ let t2111 = Py::Int(0i64); Py::Bool({ let t2112 = v_ep.clone(); if py::lt(&t2111, &t2112)? { { let t2113 = Py::Int(10i64); if py::le(&t2112, &t2113)? { true } else { false } } } else { false } }) }, Py::str("the void ratio at the end of primary consolidation must lie in (0, 10]"))?;
    let _ = f__need({ let t2114 = v_t2.clone(); Py::Bool({ let t2115 = v_t1.clone(); if py::gt(&t2114, &t2115)? { true } else { false } }) }, Py::str("the later time must exceed the end of primary consolidation"))?;
    v_Cm = py::truediv(&v_Ca.clone(), &py::add(&Py::Int(1i64), &v_ep.clone())?)?;
    v_S = py::mul(&py::mul(&v_Cm.clone(), &v_H.clone())?, &py::math1("log10", &py::truediv(&v_t2.clone(), &v_t1.clone())?)?)?;
    return Ok(Py::dict(vec![(Py::str("Ss"), f__r(v_S.clone())?), (Py::str("Ss_mm"), f__r(py::mul(&v_S.clone(), &Py::Float(1000.0))?)?), (Py::str("C_alpha_mod"), f__r(v_Cm.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_strip_load(mut v_q: Py, mut v_B: Py, mut v_z: Py, mut v_x: Py) -> R {
    let mut v_I: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_g: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("q"), v_q.clone()), (Py::str("B"), v_B.clone()), (Py::str("z"), v_z.clone())]))?;
    let _ = f__finite(Py::dict(vec![(Py::str("x"), v_x.clone())]))?;
    v_b = py::truediv(&v_B.clone(), &Py::Int(2i64))?;
    v_g = { let v_B = v_B.clone(); let v_b = v_b.clone(); let v_q = v_q.clone(); let v_x = v_x.clone(); let v_z = v_z.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_u: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::add(&py::math1("atan", &py::truediv(&v_u.clone(), &v_z.clone())?)?, &py::truediv(&py::mul(&v_u.clone(), &v_z.clone())?, &py::add(&py::mul(&v_u.clone(), &v_u.clone())?, &py::mul(&v_z.clone(), &v_z.clone())?)?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_I = py::truediv(&py::sub(&py::call(&v_g.clone(), &[py::add(&v_x.clone(), &v_b.clone())?])?, &py::call(&v_g.clone(), &[py::sub(&v_x.clone(), &v_b.clone())?])?)?, &Py::Float(std::f64::consts::PI))?;
    return Ok(Py::dict(vec![(Py::str("dsigma"), f__r(py::mul(&v_q.clone(), &v_I.clone())?)?), (Py::str("I"), f__r(v_I.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_terzaghi_bearing(mut v_c: Py, mut v_gam: Py, mut v_B: Py, mut v_Df: Py, mut v_Nc: Py, mut v_Nq: Py, mut v_Ngamma: Py, mut v_shape: Py, mut v_FS: Py) -> R {
    let mut v_q: Py = Py::None;
    let mut v_qu: Py = Py::None;
    let mut v_sc: Py = Py::None;
    let mut v_sg: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("c"), v_c.clone()), (Py::str("Df"), v_Df.clone()), (Py::str("Nc"), v_Nc.clone()), (Py::str("Nq"), v_Nq.clone()), (Py::str("Ngamma"), v_Ngamma.clone())]))?;
    let _ = f__pos(Py::dict(vec![(Py::str("gam"), v_gam.clone()), (Py::str("B"), v_B.clone()), (Py::str("FS"), v_FS.clone())]))?;
    let _ = f__need({ let t2252 = { let t2248 = v_c.clone(); Py::Bool({ let t2249 = Py::Int(0i64); if py::ge(&t2248, &t2249)? { true } else { false } }) }; if !t2252.truthy() { t2252 } else { { let t2250 = v_Df.clone(); Py::Bool({ let t2251 = Py::Int(0i64); if py::ge(&t2250, &t2251)? { true } else { false } }) } } }, Py::str("c and Df must be zero or positive"))?;
    let _ = f__need({ let t2260 = { let t2253 = v_Nc.clone(); Py::Bool({ let t2254 = Py::Float(5.14); if py::ge(&t2253, &t2254)? { true } else { false } }) }; if !t2260.truthy() { t2260 } else { { let t2259 = { let t2255 = v_Nq.clone(); Py::Bool({ let t2256 = Py::Int(1i64); if py::ge(&t2255, &t2256)? { true } else { false } }) }; if !t2259.truthy() { t2259 } else { { let t2257 = v_Ngamma.clone(); Py::Bool({ let t2258 = Py::Int(0i64); if py::ge(&t2257, &t2258)? { true } else { false } }) } } } } }, Py::str("the bearing capacity factors must satisfy Nc >= 5.14, Nq >= 1, Ngamma >= 0"))?;
    let _ = f__need({ let t2261 = v_shape.clone(); Py::Bool({ let t2262 = c__TERZ_SHAPE()?; if py::contains(&t2262, &t2261)? { true } else { false } }) }, Py::str("the footing must be strip, square or circular"))?;
    let t2263 = (py::getitem(&c__TERZ_SHAPE()?, &v_shape.clone())?).items()?;
    if t2263.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_sc = t2263[0].clone();
    v_sg = t2263[1].clone();
    v_q = py::mul(&v_gam.clone(), &v_Df.clone())?;
    v_qu = py::add(&py::add(&py::mul(&py::mul(&v_sc.clone(), &v_c.clone())?, &v_Nc.clone())?, &py::mul(&v_q.clone(), &v_Nq.clone())?)?, &py::mul(&py::mul(&py::mul(&v_sg.clone(), &v_gam.clone())?, &v_B.clone())?, &v_Ngamma.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("qu"), f__r(v_qu.clone())?), (Py::str("qa"), f__r(py::truediv(&v_qu.clone(), &v_FS.clone())?)?), (Py::str("q_net"), f__r(py::sub(&v_qu.clone(), &v_q.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_theis(mut v_Q: Py, mut v_T: Py, mut v_S: Py, mut v_r: Py, mut v_t: Py) -> R {
    let mut v_W: Py = Py::None;
    let mut v_u: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("Q"), v_Q.clone()), (Py::str("T"), v_T.clone()), (Py::str("S"), v_S.clone()), (Py::str("r"), v_r.clone()), (Py::str("t"), v_t.clone())]))?;
    let _ = f__need({ let t2279 = v_S.clone(); Py::Bool({ let t2280 = Py::Int(1i64); if py::lt(&t2279, &t2280)? { true } else { false } }) }, Py::str("the storativity is a fraction below 1"))?;
    v_u = py::truediv(&py::mul(&py::mul(&v_r.clone(), &v_r.clone())?, &v_S.clone())?, &py::mul(&py::mul(&Py::Int(4i64), &v_T.clone())?, &v_t.clone())?)?;
    let _ = f__need({ let t2281 = v_u.clone(); Py::Bool({ let t2282 = Py::Int(50i64); if py::le(&t2281, &t2282)? { true } else { false } }) }, Py::str("u exceeds 50: the drawdown is negligible (below 1e-23 of Q/(4 pi T))"))?;
    v_W = f__E1(v_u.clone())?;
    return Ok(Py::dict(vec![(Py::str("s"), f__r(py::mul(&py::truediv(&v_Q.clone(), &py::mul(&py::mul(&Py::Int(4i64), &Py::Float(std::f64::consts::PI))?, &v_T.clone())?)?, &v_W.clone())?)?), (Py::str("u"), f__r(v_u.clone())?), (Py::str("W"), f__r(v_W.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_thick_layer_settlement(mut v_H: Py, mut v_s_top: Py, mut v_gsub: Py, mut v_ds: Py, mut v_Cc: Py, mut v_e0: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_S: Py = Py::None;
    let mut v_S1: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_integral: Py = Py::None;
    let mut v_smid: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("H"), v_H.clone()), (Py::str("s_top"), v_s_top.clone()), (Py::str("gsub"), v_gsub.clone()), (Py::str("ds"), v_ds.clone()), (Py::str("Cc"), v_Cc.clone()), (Py::str("e0"), v_e0.clone())]))?;
    let t2311 = (Py::tuple(vec![v_s_top.clone(), v_gsub.clone()])).items()?;
    if t2311.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2311[0].clone();
    v_b = t2311[1].clone();
    v_F = { let v_Cc = v_Cc.clone(); let v_H = v_H.clone(); let v_a = v_a.clone(); let v_b = v_b.clone(); let v_ds = v_ds.clone(); let v_e0 = v_e0.clone(); let v_gsub = v_gsub.clone(); let v_s_top = v_s_top.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_u: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::mul(&v_u.clone(), &py::math1("log", &v_u.clone())?)?, &v_u.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_integral = py::truediv(&py::add(&py::sub(&py::sub(&py::call(&v_F.clone(), &[py::add(&py::add(&v_a.clone(), &py::mul(&v_b.clone(), &v_H.clone())?)?, &v_ds.clone())?])?, &py::call(&v_F.clone(), &[py::add(&v_a.clone(), &v_ds.clone())?])?)?, &py::call(&v_F.clone(), &[py::add(&v_a.clone(), &py::mul(&v_b.clone(), &v_H.clone())?)?])?)?, &py::call(&v_F.clone(), &[v_a.clone()])?)?, &v_b.clone())?;
    v_S = py::mul(&py::truediv(&v_Cc.clone(), &py::mul(&py::add(&Py::Int(1i64), &v_e0.clone())?, &py::math1("log", &Py::Int(10i64))?)?)?, &v_integral.clone())?;
    v_smid = py::add(&v_a.clone(), &py::truediv(&py::mul(&v_b.clone(), &v_H.clone())?, &Py::Int(2i64))?)?;
    v_S1 = py::mul(&py::truediv(&py::mul(&v_Cc.clone(), &v_H.clone())?, &py::add(&Py::Int(1i64), &v_e0.clone())?)?, &py::math1("log10", &py::truediv(&py::add(&v_smid.clone(), &v_ds.clone())?, &v_smid.clone())?)?)?;
    return Ok(Py::dict(vec![(Py::str("S"), f__r(v_S.clone())?), (Py::str("S_one_layer"), f__r(v_S1.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_thiem_confined(mut v_Q: Py, mut v_r1: Py, mut v_r2: Py, mut v_s1: Py, mut v_s2: Py, mut v_b: Py) -> R {
    let mut v_T: Py = Py::None;
    let mut v_out: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("Q"), v_Q.clone()), (Py::str("r1"), v_r1.clone()), (Py::str("r2"), v_r2.clone())]))?;
    let _ = f__finite(Py::dict(vec![(Py::str("s1"), v_s1.clone()), (Py::str("s2"), v_s2.clone())]))?;
    let _ = f__need({ let t2327 = v_r2.clone(); Py::Bool({ let t2328 = v_r1.clone(); if py::gt(&t2327, &t2328)? { true } else { false } }) }, Py::str("the second observation well must be farther from the pumped well: r2 > r1"))?;
    let _ = f__need({ let t2329 = v_s1.clone(); Py::Bool({ let t2330 = v_s2.clone(); if py::gt(&t2329, &t2330)? { { let t2331 = Py::Int(0i64); if py::ge(&t2330, &t2331)? { true } else { false } } } else { false } }) }, Py::str("the drawdown must decrease with distance: s1 > s2 >= 0"))?;
    v_T = py::truediv(&py::mul(&v_Q.clone(), &py::math1("log", &py::truediv(&v_r2.clone(), &v_r1.clone())?)?)?, &py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &py::sub(&v_s1.clone(), &v_s2.clone())?)?)?;
    v_out = Py::dict(vec![(Py::str("T"), f__r(v_T.clone())?), (Py::str("T_m2_d"), f__r(py::mul(&v_T.clone(), &Py::Int(86400i64))?)?)]);
    if ({ let t2338 = v_b.clone(); Py::Bool({ let t2339 = Py::None; if !py::is(&t2338, &t2339) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::dict(vec![(Py::str("b"), v_b.clone())]))?;
        py::setitem(&v_out.clone(), &Py::str("K"), f__r(py::truediv(&v_T.clone(), &v_b.clone())?)?)?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_thiem_unconfined(mut v_Q: Py, mut v_r1: Py, mut v_r2: Py, mut v_h1: Py, mut v_h2: Py) -> R {
    let mut v_K: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("Q"), v_Q.clone()), (Py::str("r1"), v_r1.clone()), (Py::str("r2"), v_r2.clone()), (Py::str("h1"), v_h1.clone()), (Py::str("h2"), v_h2.clone())]))?;
    let _ = f__need({ let t2349 = v_r2.clone(); Py::Bool({ let t2350 = v_r1.clone(); if py::gt(&t2349, &t2350)? { true } else { false } }) }, Py::str("need r2 > r1"))?;
    let _ = f__need({ let t2351 = v_h2.clone(); Py::Bool({ let t2352 = v_h1.clone(); if py::gt(&t2351, &t2352)? { true } else { false } }) }, Py::str("the saturated thickness must rise with distance: h2 > h1"))?;
    v_K = py::truediv(&py::mul(&v_Q.clone(), &py::math1("log", &py::truediv(&v_r2.clone(), &v_r1.clone())?)?)?, &py::mul(&Py::Float(std::f64::consts::PI), &py::sub(&py::mul(&v_h2.clone(), &v_h2.clone())?, &py::mul(&v_h1.clone(), &v_h1.clone())?)?)?)?;
    return Ok(Py::dict(vec![(Py::str("K"), f__r(v_K.clone())?), (Py::str("K_m_d"), f__r(py::mul(&v_K.clone(), &Py::Int(86400i64))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_thiessen(mut v_gx: Py, mut v_gy: Py, mut v_rain: Py, mut v_bx: Py, mut v_by: Py) -> R {
    let mut v_area: Py = Py::None;
    let mut v_areas: Py = Py::None;
    let mut v_cell: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_mean: Py = Py::None;
    let mut v_poly: Py = Py::None;
    let mut v_pts: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_xi: Py = Py::None;
    let mut v_xj: Py = Py::None;
    let mut v_yi: Py = Py::None;
    let mut v_yj: Py = Py::None;
    let t2374 = (Py::tuple(vec![f__vec(Py::str("gauge x"), v_gx.clone(), Py::None, Py::Bool(false), Py::Bool(false))?, f__vec(Py::str("gauge y"), v_gy.clone(), py::len(&v_gx.clone())?, Py::Bool(false), Py::Bool(false))?])).items()?;
    if t2374.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_gx = t2374[0].clone();
    v_gy = t2374[1].clone();
    v_rain = f__vec(Py::str("rainfall"), v_rain.clone(), py::len(&v_gx.clone())?, Py::Bool(false), Py::Bool(true))?;
    let t2375 = (Py::tuple(vec![f__vec(Py::str("boundary x"), v_bx.clone(), Py::None, Py::Bool(false), Py::Bool(false))?, f__vec(Py::str("boundary y"), v_by.clone(), py::len(&v_bx.clone())?, Py::Bool(false), Py::Bool(false))?])).items()?;
    if t2375.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_bx = t2375[0].clone();
    v_by = t2375[1].clone();
    let _ = f__need({ let t2376 = py::len(&v_gx.clone())?; Py::Bool({ let t2377 = Py::Int(2i64); if py::ge(&t2376, &t2377)? { true } else { false } }) }, Py::str("give at least two gauges"))?;
    let _ = f__need({ let t2378 = py::len(&v_bx.clone())?; Py::Bool({ let t2379 = Py::Int(3i64); if py::ge(&t2378, &t2379)? { true } else { false } }) }, Py::str("the catchment boundary needs at least three vertices"))?;
    v_pts = Py::list((py::zip(&[v_gx.clone(), v_gy.clone()])?).items()?);
    let _ = f__need({ let t2380 = py::len(&py::set(&v_pts.clone())?)?; Py::Bool({ let t2381 = py::len(&v_pts.clone())?; if py::eq(&t2380, &t2381) { true } else { false } }) }, Py::str("two gauges share a location"))?;
    v_poly = Py::list((py::zip(&[v_bx.clone(), v_by.clone()])?).items()?);
    let _ = f__need(f__is_simple(v_poly.clone())?, Py::str("the catchment boundary must be a simple polygon (no crossing or repeated vertices)"))?;
    v_area = f__shoelace(v_poly.clone())?;
    let _ = f__need({ let t2435 = py::abs(&v_area.clone())?; Py::Bool({ let t2436 = Py::Int(0i64); if py::gt(&t2435, &t2436)? { true } else { false } }) }, Py::str("the boundary encloses no area"))?;
    if ({ let t2437 = v_area.clone(); Py::Bool({ let t2438 = Py::Int(0i64); if py::lt(&t2437, &t2438)? { true } else { false } }) }).truthy() {
        let t2439 = (Py::tuple(vec![py::slice(&v_poly.clone(), None, None, Some((py::neg(&Py::Int(1i64))?).i()?))?, py::neg(&v_area.clone())?])).items()?;
        if t2439.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_poly = t2439[0].clone();
        v_area = t2439[1].clone();
    }
    v_areas = Py::list(vec![]);
    for t2440 in py::iter(&(py::enumerate(&v_pts.clone(), 0)?))? {
        let t2459 = (t2440).items()?;
        if t2459.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_i = t2459[0].clone();
        let t2460 = (t2459[1].clone()).items()?;
        if t2460.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_xi = t2460[0].clone();
        v_yi = t2460[1].clone();
        v_cell = v_poly.clone();
        for t2441 in py::iter(&(py::enumerate(&v_pts.clone(), 0)?))? {
            let t2457 = (t2441).items()?;
            if t2457.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_j = t2457[0].clone();
            let t2458 = (t2457[1].clone()).items()?;
            if t2458.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_xj = t2458[0].clone();
            v_yj = t2458[1].clone();
            if ({ let t2444 = { let t2442 = v_i.clone(); Py::Bool({ let t2443 = v_j.clone(); if py::ne(&t2442, &t2443) { true } else { false } }) }; if !t2444.truthy() { t2444 } else { v_cell.clone() } }).truthy() {
                v_cell = f__clip_halfplane(v_cell.clone(), py::mul(&Py::Int(2i64), &py::sub(&v_xj.clone(), &v_xi.clone())?)?, py::mul(&Py::Int(2i64), &py::sub(&v_yj.clone(), &v_yi.clone())?)?, py::sub(&py::sub(&py::add(&py::mul(&v_xj.clone(), &v_xj.clone())?, &py::mul(&v_yj.clone(), &v_yj.clone())?)?, &py::mul(&v_xi.clone(), &v_xi.clone())?)?, &py::mul(&v_yi.clone(), &v_yi.clone())?)?)?;
            }
        }
        let _ = py::method(&v_areas.clone(), "append", &[py::abs(&f__shoelace(v_cell.clone())?)?])?;
    }
    v_areas = { let mut out = vec![]; for t2461 in py::iter(&(v_areas.clone()))? { let mut v_a: Py; 
v_a = t2461;
out.push(f__snap(v_a.clone(), v_area.clone())?); } Py::list(out) };
    v_w = { let mut out = vec![]; for t2464 in py::iter(&(v_areas.clone()))? { let mut v_a: Py; 
v_a = t2464;
out.push(py::truediv(&v_a.clone(), &v_area.clone())?); } Py::list(out) };
    v_mean = py::sum(&{ let mut out = vec![]; for t2465 in py::iter(&(py::zip(&[v_w.clone(), v_rain.clone()])?))? { let mut v_r: Py; let mut v_wi: Py; 
let t2466 = (t2465).items()?;
if t2466.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_wi = t2466[0].clone();
v_r = t2466[1].clone();
out.push(py::mul(&v_wi.clone(), &v_r.clone())?); } Py::list(out) }, None)?;
    return Ok(Py::dict(vec![(Py::str("mean"), f__r(v_mean.clone())?), (Py::str("areas"), f__r(v_areas.clone())?), (Py::str("weights"), f__r(v_w.clone())?), (Py::str("arithmetic_mean"), f__r(py::truediv(&py::sum(&v_rain.clone(), None)?, &py::len(&v_rain.clone())?)?)?), (Py::str("catchment_area"), f__r(v_area.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_three_reservoirs(mut v_z: Py, mut v_L: Py, mut v_D: Py, mut v_C: Py) -> R {
    let mut v_flows: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_inflow: Py = Py::None;
    let mut v_ks: Py = Py::None;
    let mut v_scale: Py = Py::None;
    v_z = f__vec(Py::str("z"), v_z.clone(), Py::Int(3i64), Py::Bool(false), Py::Bool(false))?;
    v_L = f__vec(Py::str("L"), v_L.clone(), Py::Int(3i64), Py::Bool(true), Py::Bool(false))?;
    v_D = { let mut out = vec![]; for t2488 in py::iter(&(f__vec(Py::str("D"), v_D.clone(), Py::Int(3i64), Py::Bool(true), Py::Bool(false))?))? { let mut v_d: Py; 
v_d = t2488;
out.push(py::truediv(&v_d.clone(), &Py::Int(1000i64))?); } Py::list(out) };
    v_C = f__vec(Py::str("C"), v_C.clone(), Py::Int(3i64), Py::Bool(true), Py::Bool(false))?;
    for t2489 in py::range_iter(&[(Py::Int(3i64)).i()?]) {
        v_i = t2489;
        let _ = f__ok_pipe(py::getitem(&v_L.clone(), &v_i.clone())?, py::getitem(&v_D.clone(), &v_i.clone())?, py::getitem(&v_C.clone(), &v_i.clone())?)?;
    }
    v_ks = { let mut out = vec![]; for t2504 in py::range_iter(&[(Py::Int(3i64)).i()?]) { let mut v_i: Py; 
v_i = t2504;
out.push(f__hw_k(py::getitem(&v_L.clone(), &v_i.clone())?, py::getitem(&v_D.clone(), &v_i.clone())?, py::getitem(&v_C.clone(), &v_i.clone())?)?); } Py::list(out) };
    let _ = f__need({ let t2505 = py::sub(&py::minmax(&[v_z.clone()], true)?, &py::minmax(&[v_z.clone()], false)?)?; Py::Bool({ let t2506 = Py::Float(1e-06); if py::gt(&t2505, &t2506)? { true } else { false } }) }, Py::str("the reservoir levels must differ"))?;
    v_inflow = { let v_C = v_C.clone(); let v_D = v_D.clone(); let v_L = v_L.clone(); let v_i = v_i.clone(); let v_ks = v_ks.clone(); let v_z = v_z.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_h: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sum(&{ let mut out = vec![]; for t2507 in py::range_iter(&[(Py::Int(3i64)).i()?]) { let mut v_i: Py; 
v_i = t2507;
out.push(py::copysign(&py::pow(&py::truediv(&py::abs(&py::sub(&py::getitem(&v_z.clone(), &v_i.clone())?, &v_h.clone())?)?, &py::getitem(&v_ks.clone(), &v_i.clone())?)?, &py::truediv(&Py::Int(1i64), &Py::Float(1.852))?)?, &py::sub(&py::getitem(&v_z.clone(), &v_i.clone())?, &v_h.clone())?)?); } Py::list(out) }, None)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_h = f__bisect_root(v_inflow.clone(), py::minmax(&[v_z.clone()], false)?, py::minmax(&[v_z.clone()], true)?, Py::Int(300i64))?;
    v_flows = { let mut out = vec![]; for t2530 in py::range_iter(&[(Py::Int(3i64)).i()?]) { let mut v_i: Py; 
v_i = t2530;
out.push(py::copysign(&py::pow(&py::truediv(&py::abs(&py::sub(&py::getitem(&v_z.clone(), &v_i.clone())?, &v_h.clone())?)?, &py::getitem(&v_ks.clone(), &v_i.clone())?)?, &py::truediv(&Py::Int(1i64), &Py::Float(1.852))?)?, &py::sub(&py::getitem(&v_z.clone(), &v_i.clone())?, &v_h.clone())?)?); } Py::list(out) };
    v_scale = py::minmax(&[{ let mut out = vec![]; for t2531 in py::iter(&(v_flows.clone()))? { let mut v_f: Py; 
v_f = t2531;
out.push(py::abs(&v_f.clone())?); } Py::list(out) }], true)?;
    return Ok(Py::dict(vec![(Py::str("h_junction"), f__r(v_h.clone())?), (Py::str("flows"), f__r({ let mut out = vec![]; for t2538 in py::iter(&(v_flows.clone()))? { let mut v_f: Py; 
v_f = t2538;
out.push(f__snap(v_f.clone(), v_scale.clone())?); } Py::list(out) })?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_trap_section(mut v_b: Py, mut v_y: Py, mut v_z: Py) -> R {
    let _ = f__pos(Py::dict(vec![(Py::str("y"), v_y.clone())]))?;
    let _ = f__finite(Py::dict(vec![(Py::str("b"), v_b.clone()), (Py::str("z"), v_z.clone())]))?;
    let _ = f__need({ let t2557 = { let t2550 = v_b.clone(); Py::Bool({ let t2551 = Py::Int(0i64); if py::ge(&t2550, &t2551)? { true } else { false } }) }; if !t2557.truthy() { t2557 } else { { let t2556 = { let t2552 = v_z.clone(); Py::Bool({ let t2553 = Py::Int(0i64); if py::ge(&t2552, &t2553)? { true } else { false } }) }; if !t2556.truthy() { t2556 } else { { let t2554 = py::add(&v_b.clone(), &v_z.clone())?; Py::Bool({ let t2555 = Py::Int(0i64); if py::gt(&t2554, &t2555)? { true } else { false } }) } } } } }, Py::str("need b >= 0, z >= 0 and not both zero"))?;
    return Ok({ let mut v: Vec<Py> = vec![]; v.extend((f__trap(v_b.clone(), v_y.clone(), v_z.clone())?).items()?); let mut d: Vec<(Py, Py)> = vec![];  let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "A") { Some(x) => x, None => { return Err(PyErr::Type("_section_out() missing argument A".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "P") { Some(x) => x, None => { return Err(PyErr::Type("_section_out() missing argument P".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "T") { Some(x) => x, None => { return Err(PyErr::Type("_section_out() missing argument T".into())) } } }; if v.len() > 3 { return Err(PyErr::Type("_section_out() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("_section_out() got an unexpected keyword argument".into())) } f__section_out(a0, a1, a2)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_traverse_bowditch(mut v_az: Py, mut v_d: Py, mut v_E0: Py, mut v_N0: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_D: Py = Py::None;
    let mut v_E: Py = Py::None;
    let mut v_N: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_area: Py = Py::None;
    let mut v_dep: Py = Py::None;
    let mut v_eE: Py = Py::None;
    let mut v_eN: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_lat: Py = Py::None;
    let mut v_lin: Py = Py::None;
    let mut v_scale: Py = Py::None;
    v_A = f__vec(Py::str("azimuths"), v_az.clone(), Py::None, Py::Bool(false), Py::Bool(false))?;
    v_D = f__vec(Py::str("distances"), v_d.clone(), py::len(&v_A.clone())?, Py::Bool(true), Py::Bool(false))?;
    let _ = f__need({ let t2579 = py::len(&v_A.clone())?; Py::Bool({ let t2580 = Py::Int(3i64); if py::ge(&t2579, &t2580)? { true } else { false } }) }, Py::str("a closed traverse needs at least three legs"))?;
    let _ = f__finite(Py::dict(vec![(Py::str("E0"), v_E0.clone()), (Py::str("N0"), v_N0.clone())]))?;
    for t2585 in py::iter(&(v_A.clone()))? {
        v_a = t2585;
        let _ = f__need({ let t2586 = Py::Int(0i64); Py::Bool({ let t2587 = v_a.clone(); if py::le(&t2586, &t2587)? { { let t2588 = Py::Int(360i64); if py::lt(&t2587, &t2588)? { true } else { false } } } else { false } }) }, Py::str("azimuths are whole-circle bearings from 0 to 360 degrees"))?;
    }
    v_dep = { let mut out = vec![]; for t2589 in py::iter(&(py::zip(&[v_A.clone(), v_D.clone()])?))? { let mut v_a: Py; let mut v_x: Py; 
let t2590 = (t2589).items()?;
if t2590.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t2590[0].clone();
v_x = t2590[1].clone();
out.push(py::mul(&v_x.clone(), &py::math1("sin", &py::math1("radians", &v_a.clone())?)?)?); } Py::list(out) };
    v_lat = { let mut out = vec![]; for t2591 in py::iter(&(py::zip(&[v_A.clone(), v_D.clone()])?))? { let mut v_a: Py; let mut v_x: Py; 
let t2592 = (t2591).items()?;
if t2592.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t2592[0].clone();
v_x = t2592[1].clone();
out.push(py::mul(&v_x.clone(), &py::math1("cos", &py::math1("radians", &v_a.clone())?)?)?); } Py::list(out) };
    let t2593 = (Py::tuple(vec![py::sum(&v_dep.clone(), None)?, py::sum(&v_lat.clone(), None)?])).items()?;
    if t2593.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_eE = t2593[0].clone();
    v_eN = t2593[1].clone();
    v_P = py::sum(&v_D.clone(), None)?;
    v_lin = py::hypot(&[v_eE.clone(), v_eN.clone()])?;
    let t2594 = (Py::tuple(vec![Py::list(vec![v_E0.clone()]), Py::list(vec![v_N0.clone()])])).items()?;
    if t2594.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_E = t2594[0].clone();
    v_N = t2594[1].clone();
    for t2595 in py::range_iter(&[(py::sub(&py::len(&v_A.clone())?, &Py::Int(1i64))?).i()?]) {
        v_i = t2595;
        let _ = py::method(&v_E.clone(), "append", &[py::sub(&py::add(&py::getitem(&v_E.clone(), &py::neg(&Py::Int(1i64))?)?, &py::getitem(&v_dep.clone(), &v_i.clone())?)?, &py::truediv(&py::mul(&v_eE.clone(), &py::getitem(&v_D.clone(), &v_i.clone())?)?, &v_P.clone())?)?])?;
        let _ = py::method(&v_N.clone(), "append", &[py::sub(&py::add(&py::getitem(&v_N.clone(), &py::neg(&Py::Int(1i64))?)?, &py::getitem(&v_lat.clone(), &v_i.clone())?)?, &py::truediv(&py::mul(&v_eN.clone(), &py::getitem(&v_D.clone(), &v_i.clone())?)?, &v_P.clone())?)?])?;
    }
    v_area = py::abs(&f__shoelace(Py::list((py::zip(&[v_E.clone(), v_N.clone()])?).items()?))?)?;
    v_scale = v_P.clone();
    return Ok(Py::dict(vec![(Py::str("misclosure_E"), f__r(f__snap(v_eE.clone(), v_scale.clone())?)?), (Py::str("misclosure_N"), f__r(f__snap(v_eN.clone(), v_scale.clone())?)?), (Py::str("linear_misclosure"), f__r(f__snap(v_lin.clone(), v_scale.clone())?)?), (Py::str("precision"), f__r((if ({ let t2607 = v_lin.clone(); Py::Bool({ let t2608 = py::mul(&Py::Float(1e-12), &v_scale.clone())?; if py::gt(&t2607, &t2608)? { true } else { false } }) }).truthy() { py::truediv(&v_P.clone(), &v_lin.clone())? } else { Py::Float(0.0) }))?), (Py::str("E"), f__r(v_E.clone())?), (Py::str("N"), f__r(v_N.clone())?), (Py::str("area"), f__r(v_area.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_triaxial_params(mut v_s3: Py, mut v_s1: Py) -> R {
    let mut v_a1: Py = Py::None;
    let mut v_a3: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_icpt: Py = Py::None;
    let mut v_phi: Py = Py::None;
    let mut v_pp: Py = Py::None;
    let mut v_qq: Py = Py::None;
    let mut v_slope: Py = Py::None;
    v_a3 = f__vec(Py::str("s3"), v_s3.clone(), Py::None, Py::Bool(false), Py::Bool(true))?;
    v_a1 = f__vec(Py::str("s1"), v_s1.clone(), py::len(&v_a3.clone())?, Py::Bool(false), Py::Bool(false))?;
    let _ = f__need({ let t2624 = py::len(&v_a3.clone())?; Py::Bool({ let t2625 = Py::Int(2i64); if py::ge(&t2624, &t2625)? { true } else { false } }) }, Py::str("give at least two tests"))?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t2626 in py::iter(&(py::zip(&[v_a1.clone(), v_a3.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t2627 = (t2626).items()?;
if t2627.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t2627[0].clone();
v_y = t2627[1].clone();
out.push({ let t2628 = v_x.clone(); Py::Bool({ let t2629 = v_y.clone(); if py::gt(&t2628, &t2629)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("each s1 must exceed its s3"))?;
    v_pp = { let mut out = vec![]; for t2630 in py::iter(&(py::zip(&[v_a1.clone(), v_a3.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t2631 = (t2630).items()?;
if t2631.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t2631[0].clone();
v_y = t2631[1].clone();
out.push(py::truediv(&py::add(&v_x.clone(), &v_y.clone())?, &Py::Int(2i64))?); } Py::list(out) };
    v_qq = { let mut out = vec![]; for t2632 in py::iter(&(py::zip(&[v_a1.clone(), v_a3.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t2633 = (t2632).items()?;
if t2633.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t2633[0].clone();
v_y = t2633[1].clone();
out.push(py::truediv(&py::sub(&v_x.clone(), &v_y.clone())?, &Py::Int(2i64))?); } Py::list(out) };
    let t2640 = (f__linfit(v_pp.clone(), v_qq.clone())?).items()?;
    if t2640.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_slope = t2640[0].clone();
    v_icpt = t2640[1].clone();
    let _ = f__need({ let t2641 = Py::Int(0i64); Py::Bool({ let t2642 = v_slope.clone(); if py::lt(&t2641, &t2642)? { { let t2643 = Py::Int(1i64); if py::lt(&t2642, &t2643)? { true } else { false } } } else { false } }) }, Py::str("the stress points do not define a friction angle between 0 and 90 degrees"))?;
    v_phi = py::math1("asin", &v_slope.clone())?;
    v_c = py::truediv(&v_icpt.clone(), &py::math1("cos", &v_phi.clone())?)?;
    let _ = f__need({ let t2644 = v_c.clone(); Py::Bool({ let t2645 = py::mul(&py::neg(&Py::Float(1e-09))?, &py::minmax(&[v_qq.clone()], true)?)?; if py::ge(&t2644, &t2645)? { true } else { false } }) }, Py::str("the fitted cohesion is negative: the data do not fit Mohr-Coulomb with c >= 0"))?;
    return Ok(Py::dict(vec![(Py::str("phi"), f__r(py::math1("degrees", &v_phi.clone())?)?), (Py::str("c"), f__r((if ({ let t2652 = py::abs(&v_c.clone())?; Py::Bool({ let t2653 = py::mul(&Py::Float(1e-09), &py::minmax(&[v_qq.clone()], true)?)?; if py::lt(&t2652, &t2653)? { true } else { false } }) }).truthy() { py::minmax(&[v_c.clone(), Py::Float(0.0)], true)? } else { v_c.clone() }))?), (Py::str("alpha"), f__r(py::math1("degrees", &py::math1("atan", &v_slope.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_uh_convolution(mut v_excess: Py, mut v_uh: Py, mut v_baseflow: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_Q: Py = Py::None;
    let mut v_U: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_pk: Py = Py::None;
    let mut v_total: Py = Py::None;
    v_P = f__vec(Py::str("excess"), v_excess.clone(), Py::None, Py::Bool(false), Py::Bool(true))?;
    v_U = f__vec(Py::str("uh"), v_uh.clone(), Py::None, Py::Bool(false), Py::Bool(true))?;
    let _ = f__finite(Py::dict(vec![(Py::str("baseflow"), v_baseflow.clone())]))?;
    let _ = f__need({ let t2673 = v_baseflow.clone(); Py::Bool({ let t2674 = Py::Int(0i64); if py::ge(&t2673, &t2674)? { true } else { false } }) }, Py::str("the base flow must be zero or positive"))?;
    let t2675 = (Py::tuple(vec![py::len(&v_P.clone())?, py::len(&v_U.clone())?])).items()?;
    if t2675.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t2675[0].clone();
    v_m = t2675[1].clone();
    v_Q = { let mut out = vec![]; for t2676 in py::range_iter(&[(py::sub(&py::add(&v_n.clone(), &v_m.clone())?, &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t2676;
out.push(py::sum(&{ let mut out = vec![]; for t2677 in py::range_iter(&[(py::minmax(&[Py::Int(0i64), py::add(&py::sub(&v_k.clone(), &v_m.clone())?, &Py::Int(1i64))?], true)?).i()?, (py::minmax(&[v_n.clone(), py::add(&v_k.clone(), &Py::Int(1i64))?], false)?).i()?]) { let mut v_i: Py; 
v_i = t2677;
out.push(py::mul(&py::getitem(&v_P.clone(), &v_i.clone())?, &py::getitem(&v_U.clone(), &py::sub(&v_k.clone(), &v_i.clone())?)?)?); } Py::list(out) }, None)?); } Py::list(out) };
    v_total = { let mut out = vec![]; for t2678 in py::iter(&(v_Q.clone()))? { let mut v_q: Py; 
v_q = t2678;
out.push(py::add(&v_q.clone(), &v_baseflow.clone())?); } Py::list(out) };
    v_pk = py::minmax_key(&[Py::list((py::range_iter(&[(py::len(&v_total.clone())?).i()?])).into_iter().collect::<Vec<Py>>())], &{ let v_P = v_P.clone(); let v_Q = v_Q.clone(); let v_U = v_U.clone(); let v_baseflow = v_baseflow.clone(); let v_excess = v_excess.clone(); let v_m = v_m.clone(); let v_n = v_n.clone(); let v_total = v_total.clone(); let v_uh = v_uh.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_k: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::getitem(&v_total.clone(), &v_k.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, true)?;
    return Ok(Py::dict(vec![(Py::str("DRO"), f__r(v_Q.clone())?), (Py::str("total"), f__r(v_total.clone())?), (Py::str("peak"), f__r(py::getitem(&v_total.clone(), &v_pk.clone())?)?), (Py::str("peak_period"), py::add(&v_pk.clone(), &Py::Int(1i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_uh_derive(mut v_dro: Py, mut v_excess: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_N: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_Q: Py = Py::None;
    let mut v_fit: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_rhs: Py = Py::None;
    let mut v_rms: Py = Py::None;
    let mut v_scale: Py = Py::None;
    let mut v_u: Py = Py::None;
    v_Q = f__vec(Py::str("dro"), v_dro.clone(), Py::None, Py::Bool(false), Py::Bool(false))?;
    v_P = f__vec(Py::str("excess"), v_excess.clone(), Py::None, Py::Bool(false), Py::Bool(true))?;
    v_n = py::len(&v_P.clone())?;
    v_m = py::add(&py::sub(&py::len(&v_Q.clone())?, &v_n.clone())?, &Py::Int(1i64))?;
    let _ = f__need({ let t2700 = v_m.clone(); Py::Bool({ let t2701 = Py::Int(2i64); if py::ge(&t2700, &t2701)? { true } else { false } }) }, Py::str("the runoff hydrograph must have at least n + 1 ordinates for n excess periods"))?;
    let _ = f__need({ let t2702 = py::sum(&v_P.clone(), None)?; Py::Bool({ let t2703 = Py::Int(0i64); if py::gt(&t2702, &t2703)? { true } else { false } }) }, Py::str("the excess rainfall must not be all zero"))?;
    v_A = { let mut out = vec![]; for t2704 in py::range_iter(&[(py::len(&v_Q.clone())?).i()?]) { let mut v_k: Py; 
v_k = t2704;
out.push({ let mut out = vec![]; for t2705 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_j: Py; 
v_j = t2705;
out.push((if ({ let t2706 = Py::Int(0i64); Py::Bool({ let t2707 = py::sub(&v_k.clone(), &v_j.clone())?; if py::le(&t2706, &t2707)? { { let t2708 = v_n.clone(); if py::lt(&t2707, &t2708)? { true } else { false } } } else { false } }) }).truthy() { py::getitem(&v_P.clone(), &py::sub(&v_k.clone(), &v_j.clone())?)? } else { Py::Float(0.0) })); } Py::list(out) }); } Py::list(out) };
    v_N = { let mut out = vec![]; for t2709 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t2709;
out.push({ let mut out = vec![]; for t2710 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_j: Py; 
v_j = t2710;
out.push(py::sum(&{ let mut out = vec![]; for t2711 in py::range_iter(&[(py::len(&v_Q.clone())?).i()?]) { let mut v_k: Py; 
v_k = t2711;
out.push(py::mul(&py::getitem(&py::getitem(&v_A.clone(), &v_k.clone())?, &v_i.clone())?, &py::getitem(&py::getitem(&v_A.clone(), &v_k.clone())?, &v_j.clone())?)?); } Py::list(out) }, None)?); } Py::list(out) }); } Py::list(out) };
    v_rhs = { let mut out = vec![]; for t2712 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t2712;
out.push(py::sum(&{ let mut out = vec![]; for t2713 in py::range_iter(&[(py::len(&v_Q.clone())?).i()?]) { let mut v_k: Py; 
v_k = t2713;
out.push(py::mul(&py::getitem(&py::getitem(&v_A.clone(), &v_k.clone())?, &v_i.clone())?, &py::getitem(&v_Q.clone(), &v_k.clone())?)?); } Py::list(out) }, None)?); } Py::list(out) };
    v_u = f__solve_linear(v_N.clone(), v_rhs.clone())?;
    v_fit = { let mut out = vec![]; for t2726 in py::range_iter(&[(py::len(&v_Q.clone())?).i()?]) { let mut v_k: Py; 
v_k = t2726;
out.push(py::sum(&{ let mut out = vec![]; for t2727 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_j: Py; 
v_j = t2727;
out.push(py::mul(&py::getitem(&py::getitem(&v_A.clone(), &v_k.clone())?, &v_j.clone())?, &py::getitem(&v_u.clone(), &v_j.clone())?)?); } Py::list(out) }, None)?); } Py::list(out) };
    v_rms = py::math1("sqrt", &py::truediv(&py::sum(&{ let mut out = vec![]; for t2728 in py::range_iter(&[(py::len(&v_Q.clone())?).i()?]) { let mut v_k: Py; 
v_k = t2728;
out.push(py::pow(&py::sub(&py::getitem(&v_fit.clone(), &v_k.clone())?, &py::getitem(&v_Q.clone(), &v_k.clone())?)?, &Py::Int(2i64))?); } Py::list(out) }, None)?, &py::len(&v_Q.clone())?)?)?;
    v_scale = py::minmax(&[{ let mut out = vec![]; for t2729 in py::iter(&(v_Q.clone()))? { let mut v_x: Py; 
v_x = t2729;
out.push(py::abs(&v_x.clone())?); } Py::list(out) }], true)?;
    return Ok(Py::dict(vec![(Py::str("uh"), f__r({ let mut out = vec![]; for t2736 in py::iter(&(v_u.clone()))? { let mut v_x: Py; 
v_x = t2736;
out.push(f__snap(v_x.clone(), v_scale.clone())?); } Py::list(out) })?), (Py::str("rms_residual"), f__r(f__snap(v_rms.clone(), v_scale.clone())?)?), (Py::str("peak"), f__r(py::minmax(&[v_u.clone()], true)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_vertical_curve(mut v_g1: Py, mut v_g2: Py, mut v_L: Py, mut v_PVI: Py, mut v_elev_PVI: Py, mut v_x: Py) -> R {
    let mut v_bvc: Py = Py::None;
    let mut v_e_bvc: Py = Py::None;
    let mut v_elev: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_xt: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("g1"), v_g1.clone()), (Py::str("g2"), v_g2.clone()), (Py::str("PVI"), v_PVI.clone()), (Py::str("elev_PVI"), v_elev_PVI.clone()), (Py::str("x"), v_x.clone())]))?;
    let _ = f__pos(Py::dict(vec![(Py::str("L"), v_L.clone())]))?;
    let _ = f__need({ let t2771 = { let t2767 = py::abs(&v_g1.clone())?; Py::Bool({ let t2768 = Py::Float(0.3); if py::lt(&t2767, &t2768)? { true } else { false } }) }; if !t2771.truthy() { t2771 } else { { let t2769 = py::abs(&v_g2.clone())?; Py::Bool({ let t2770 = Py::Float(0.3); if py::lt(&t2769, &t2770)? { true } else { false } }) } } }, Py::str("grades are fractions (or %) below 30%"))?;
    let _ = f__need({ let t2772 = v_g1.clone(); Py::Bool({ let t2773 = v_g2.clone(); if py::ne(&t2772, &t2773) { true } else { false } }) }, Py::str("the grades must differ"))?;
    v_bvc = py::sub(&v_PVI.clone(), &py::truediv(&v_L.clone(), &Py::Int(2i64))?)?;
    let _ = f__need({ let t2774 = py::sub(&v_bvc.clone(), &Py::Float(1e-09))?; Py::Bool({ let t2775 = v_x.clone(); if py::le(&t2774, &t2775)? { { let t2776 = py::add(&py::add(&v_bvc.clone(), &v_L.clone())?, &Py::Float(1e-09))?; if py::le(&t2775, &t2776)? { true } else { false } } } else { false } }) }, Py::str("the station must lie on the curve (between BVC and EVC)"))?;
    v_e_bvc = py::sub(&v_elev_PVI.clone(), &py::truediv(&py::mul(&v_g1.clone(), &v_L.clone())?, &Py::Int(2i64))?)?;
    v_r = py::truediv(&py::sub(&v_g2.clone(), &v_g1.clone())?, &v_L.clone())?;
    v_elev = { let v_L = v_L.clone(); let v_PVI = v_PVI.clone(); let v_bvc = v_bvc.clone(); let v_e_bvc = v_e_bvc.clone(); let v_elev_PVI = v_elev_PVI.clone(); let v_g1 = v_g1.clone(); let v_g2 = v_g2.clone(); let v_r = v_r.clone(); let v_x = v_x.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_s: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::add(&py::add(&v_e_bvc.clone(), &py::mul(&v_g1.clone(), &py::sub(&v_s.clone(), &v_bvc.clone())?)?)?, &py::truediv(&py::mul(&v_r.clone(), &py::pow(&py::sub(&v_s.clone(), &v_bvc.clone())?, &Py::Int(2i64))?)?, &Py::Int(2i64))?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_xt = py::sub(&v_bvc.clone(), &py::truediv(&v_g1.clone(), &v_r.clone())?)?;
    if (py::not(&{ let t2777 = v_bvc.clone(); Py::Bool({ let t2778 = v_xt.clone(); if py::lt(&t2777, &t2778)? { { let t2779 = py::add(&v_bvc.clone(), &v_L.clone())?; if py::lt(&t2778, &t2779)? { true } else { false } } } else { false } }) })).truthy() {
        v_xt = (if ({ let t2780 = { let t2782 = py::call(&v_elev.clone(), &[v_bvc.clone()])?; Py::Bool({ let t2783 = py::call(&v_elev.clone(), &[py::add(&v_bvc.clone(), &v_L.clone())?])?; if py::gt(&t2782, &t2783)? { true } else { false } }) }; Py::Bool({ let t2781 = { let t2784 = v_g1.clone(); Py::Bool({ let t2785 = v_g2.clone(); if py::gt(&t2784, &t2785)? { true } else { false } }) }; if py::eq(&t2780, &t2781) { true } else { false } }) }).truthy() { v_bvc.clone() } else { py::add(&v_bvc.clone(), &v_L.clone())? });
    }
    return Ok(Py::dict(vec![(Py::str("elev"), f__r(py::call(&v_elev.clone(), &[v_x.clone()])?)?), (Py::str("x_turn"), f__r(v_xt.clone())?), (Py::str("elev_turn"), f__r(py::call(&v_elev.clone(), &[v_xt.clone()])?)?), (Py::str("r"), f__r(py::mul(&v_r.clone(), &Py::Int(100i64))?)?), (Py::str("BVC"), f__r(v_bvc.clone())?), (Py::str("elev_BVC"), f__r(v_e_bvc.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_vnotch_weir(mut v_Cd: Py, mut v_theta: Py, mut v_H: Py) -> R {
    let mut v_Q: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("Cd"), v_Cd.clone()), (Py::str("theta"), v_theta.clone()), (Py::str("H"), v_H.clone())]))?;
    let _ = f__need({ let t2748 = v_Cd.clone(); Py::Bool({ let t2749 = Py::Float(1.0); if py::le(&t2748, &t2749)? { true } else { false } }) }, Py::str("the discharge coefficient must not exceed 1"))?;
    let _ = f__need({ let t2750 = v_theta.clone(); Py::Bool({ let t2751 = Py::Int(180i64); if py::lt(&t2750, &t2751)? { true } else { false } }) }, Py::str("the notch angle must be below 180 degrees"))?;
    v_Q = py::mul(&py::mul(&py::mul(&py::mul(&py::truediv(&Py::Float(8.0), &Py::Float(15.0))?, &v_Cd.clone())?, &py::math1("sqrt", &py::mul(&Py::Int(2i64), &c_G()?)?)?)?, &py::math1("tan", &py::truediv(&py::math1("radians", &v_theta.clone())?, &Py::Int(2i64))?)?)?, &py::pow(&v_H.clone(), &Py::Float(2.5))?)?;
    return Ok(Py::dict(vec![(Py::str("Q"), f__r(v_Q.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_weibull_positions(mut v_peaks: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_xs: Py = Py::None;
    v_x = f__vec(Py::str("peaks"), v_peaks.clone(), Py::None, Py::Bool(false), Py::Bool(false))?;
    v_n = py::len(&v_x.clone())?;
    let _ = f__need({ let t2807 = v_n.clone(); Py::Bool({ let t2808 = Py::Int(2i64); if py::ge(&t2807, &t2808)? { true } else { false } }) }, Py::str("give at least two values"))?;
    v_xs = py::sorted(&v_x.clone(), (Py::Bool(true)).truthy())?;
    return Ok(Py::dict(vec![(Py::str("ranked"), f__r(v_xs.clone())?), (Py::str("T"), f__r({ let mut out = vec![]; for t2815 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) { let mut v_m: Py; 
v_m = t2815;
out.push(py::truediv(&py::add(&v_n.clone(), &Py::Int(1i64))?, &v_m.clone())?); } Py::list(out) })?), (Py::str("P"), f__r({ let mut out = vec![]; for t2816 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) { let mut v_m: Py; 
v_m = t2816;
out.push(py::truediv(&v_m.clone(), &py::add(&v_n.clone(), &Py::Int(1i64))?)?); } Py::list(out) })?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_well_superposition(mut v_wx: Py, mut v_wy: Py, mut v_Qs: Py, mut v_T: Py, mut v_S: Py, mut v_t: Py, mut v_x0: Py, mut v_y0: Py) -> R {
    let mut v_parts: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_tot: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    v_wx = f__vec(Py::str("well x"), v_wx.clone(), Py::None, Py::Bool(false), Py::Bool(false))?;
    v_wy = f__vec(Py::str("well y"), v_wy.clone(), py::len(&v_wx.clone())?, Py::Bool(false), Py::Bool(false))?;
    v_Qs = f__vec(Py::str("pumping rates"), v_Qs.clone(), py::len(&v_wx.clone())?, Py::Bool(false), Py::Bool(false))?;
    let _ = f__pos(Py::dict(vec![(Py::str("T"), v_T.clone()), (Py::str("S"), v_S.clone()), (Py::str("t"), v_t.clone())]))?;
    let _ = f__need({ let t2841 = v_S.clone(); Py::Bool({ let t2842 = Py::Int(1i64); if py::lt(&t2841, &t2842)? { true } else { false } }) }, Py::str("the storativity is a fraction below 1"))?;
    let _ = f__finite(Py::dict(vec![(Py::str("x0"), v_x0.clone()), (Py::str("y0"), v_y0.clone())]))?;
    v_parts = Py::list(vec![]);
    for t2843 in py::iter(&(py::zip(&[v_wx.clone(), v_wy.clone(), v_Qs.clone()])?))? {
        let t2861 = (t2843).items()?;
        if t2861.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_x = t2861[0].clone();
        v_y = t2861[1].clone();
        v_q = t2861[2].clone();
        v_r = py::hypot(&[py::sub(&v_x.clone(), &v_x0.clone())?, py::sub(&v_y.clone(), &v_y0.clone())?])?;
        let _ = f__need({ let t2844 = v_r.clone(); Py::Bool({ let t2845 = Py::Float(0.05); if py::ge(&t2844, &t2845)? { true } else { false } }) }, Py::str("the point is at (or within 5 cm of) a well: give a point away from the wells"))?;
        v_u = py::truediv(&py::mul(&py::mul(&v_r.clone(), &v_r.clone())?, &v_S.clone())?, &py::mul(&py::mul(&Py::Int(4i64), &v_T.clone())?, &v_t.clone())?)?;
        let _ = py::method(&v_parts.clone(), "append", &[(if ({ let t2846 = v_u.clone(); Py::Bool({ let t2847 = Py::Int(50i64); if py::gt(&t2846, &t2847)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { py::mul(&py::truediv(&py::truediv(&v_q.clone(), &Py::Int(86400i64))?, &py::mul(&py::mul(&Py::Int(4i64), &Py::Float(std::f64::consts::PI))?, &v_T.clone())?)?, &f__E1(v_u.clone())?)? })])?;
    }
    v_tot = py::sum(&v_parts.clone(), None)?;
    return Ok(Py::dict(vec![(Py::str("s"), f__r(v_tot.clone())?), (Py::str("parts"), f__r(v_parts.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, non_snake_case)]
fn x_extra1692(v_i: Py, v_o: Py) -> R { Ok(f__rel(py::mul(&py::mul(&f__manning_v(py::truediv(&py::mul(&py::getitem(&v_i.clone(), &Py::str("b"))?, &py::getitem(&v_o.clone(), &Py::str("yn"))?)?, &py::add(&py::getitem(&v_i.clone(), &Py::str("b"))?, &py::mul(&Py::Int(2i64), &py::getitem(&v_o.clone(), &Py::str("yn"))?)?)?)?, py::getitem(&v_i.clone(), &Py::str("S"))?, py::getitem(&v_i.clone(), &Py::str("n"))?)?, &py::getitem(&v_i.clone(), &Py::str("b"))?)?, &py::getitem(&v_o.clone(), &Py::str("yn"))?)?, py::getitem(&v_i.clone(), &Py::str("Q"))?)?) }

#[allow(unused_mut, unused_variables, non_snake_case)]
fn x_extra488(v_i: Py, v_o: Py) -> R { Ok(f__rel(py::mul(&py::pow(&py::getitem(&v_i.clone(), &Py::str("Q"))?, &Py::Int(2i64))?, &py::add(&py::getitem(&v_i.clone(), &Py::str("b"))?, &py::mul(&py::mul(&Py::Int(2i64), &py::getitem(&v_i.clone(), &Py::str("z"))?)?, &py::getitem(&v_o.clone(), &Py::str("yc"))?)?)?)?, py::mul(&c_G()?, &py::pow(&py::getitem(&v_o.clone(), &Py::str("Ac"))?, &Py::Int(3i64))?)?)?) }
