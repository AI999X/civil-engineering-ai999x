//! Civil Engineering: 15 calculations through the Rust library (the same engine as the desktop app, the
//! command line and the browser). Run from the rust/ folder:
//!     cargo run --release --example readme -p civil-engineering-ai999x
//! Inputs are text, read like a form field ("2 m", "0.5", "x^3 - 3x"); a JSON list stays a list.

use serde_json::{json, Value};

/// One calculation: the request in, its outcome and every value with its unit out.
fn show(request: Value) {
    let answer: Value = serde_json::from_str(&civil_engineering_ai999x::solve_json(&request.to_string())).unwrap();
    println!("{}", answer["outcome"].as_str().unwrap_or(""));
    for v in answer["values"].as_array().into_iter().flatten() {
        let unit = v[2].as_str().unwrap_or("");
        let unit = if unit.is_empty() || unit == "1" { String::new() } else { format!(" {unit}") };
        println!("{} = {}{}", v[0].as_str().unwrap_or(""), v[1], unit);
    }
}

fn main() {
    show(json!({"key": "coulomb_active_thrust", "inputs": {"phi": "30", "delta": "20", "beta": "10", "gam": "18000 N/m^3", "H": "6 m"}}));
    println!();
    show(json!({"key": "darcy_flow_layered_soil", "inputs": {"H": [2], "K": [1e-05], "A": "10 m^2", "dh": "0.5 m"}}));
    println!();
    show(json!({"key": "thiessen_polygon_rainfall", "inputs": {"gx": [2, 8, 5], "gy": [2, 3, 9], "rain": [45, 60, 38], "bx": [0, 10, 10, 0], "by": [0, 0, 10, 10]}}));
    println!();
    show(json!({"key": "broad_crested_weir_discharge", "inputs": {"Cd": "0.95", "b": "5 m", "H": "0.6 m"}}));
    println!();
    show(json!({"key": "parallel_pipes_hazen_williams", "inputs": {"Q": "0.3 m^3/s", "L": [800, 1200], "D": [300, 400], "C": [120, 110]}}));
    println!();
    show(json!({"key": "degree_of_consolidation", "inputs": {"cv": "7.922021954e-08 m^2/s", "t": "31557600 s", "Hdr": "3 m"}}));
    println!();
    show(json!({"key": "area_from_offsets", "inputs": {"offsets": [0, 2.5, 3.5, 5, 4.6, 3.2, 0], "d": "10 m"}}));
    println!();
    show(json!({"key": "coulomb_passive_thrust", "inputs": {"phi": "30", "delta": "15", "beta": "0", "gam": "18000 N/m^3", "H": "2 m"}}));
    println!();
    show(json!({"key": "well_field_drawdown", "inputs": {"wx": [0, 200], "wy": [0, 0], "Qs": [2000, 1500], "T": "0.00462962963 m^2/s", "S": "0.0003", "t": "432000 s", "x0": "100 m", "y0": "50 m"}}));
    println!();
    show(json!({"key": "unit_hydrograph_convolution", "inputs": {"excess": [1.5, 2.5, 1], "uh": [0, 10, 30, 22, 12, 5, 0]}}));
    println!();
    show(json!({"key": "chezy_from_manning", "inputs": {"n": "0.013", "R": "0.75 m"}}));
    println!();
    show(json!({"key": "hazen_williams_head_loss", "inputs": {"Q": "0.05 m^3/s", "D": "0.3 m", "L": "1000 m", "C": "120"}}));
    println!();
    show(json!({"key": "coefficient_of_consolidation", "inputs": {"Hdr": "0.01 m", "t": "750 s", "U": "0.5"}}));
    println!();
    show(json!({"key": "traverse_bowditch_adjustment", "inputs": {"az": [45, 135.01, 225, 315], "d": [100, 100.02, 99.98, 100]}}));
    println!();
    show(json!({"key": "rankine_active_thrust", "inputs": {"phi": "30", "gam": "18000 N/m^3", "H": "6 m"}}));
}
