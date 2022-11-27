pub mod config;
pub mod data_loader;

pub fn in_delta_f64(val1: f64, val2: f64, d: f64) -> bool {
	let diff = val1 - val2;
	diff.abs() < d
}

pub fn in_delta_i64(val1: i64, val2: i64, d: i64) -> bool {
	let diff = val1 - val2;
	diff.abs() < d
}