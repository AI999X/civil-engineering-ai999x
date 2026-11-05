//! Civil Engineering's own live figures (ax_engine::figkit), drawn for the visitor's inputs from what the model
//! computes: open channels, weirs, pipes and networks in figures_hyd.rs; soils, foundations, groundwater, hydrology
//! and surveying in figures_geo.rs (docs/FIGURE_PARITY.md).

use ax_engine::figkit::Solved;
use ax_engine::figures::Figure;

/// The figures for one solved request (the dispatcher ax_engine calls: Pack::figures).
pub fn live(s: &Solved) -> Vec<Figure> {
    let mut out = crate::figures_hyd::live(s);
    out.extend(crate::figures_geo::live(s));
    out
}
