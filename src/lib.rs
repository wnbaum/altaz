//! # About
//! 
//! `altaz` provides a few simple functions to determine angles for altitude azimuth telescope mounts given equatorial coordinates.
//! 
//! I've designed it with my telescope controller in mind, though it may be useful to others who want to accomplish a similar goal.
//! 
//! `altaz` depends on some correct algorithms from the `astro` crate, though I re-implemented some things that are handled incorrectly. `altaz` also depends on `chrono` for easy time handling.
//! 
//! # Usage
//! 
//! Here is a usage example from my telescope controller to control altitude and azimuth stepper motors on my mount:
//! 
//! ```rust
//! // Current geographic coordinates
//! let geo_coord: GeoCoords = tracking_params.geo_coord;
//! 
//! // Telescope target equatorial coordinates
//! let target: EqCoords = tracking_params.target;
//! 
//! // Telescope base equatorial coordinates (to rotate from)
//! let base: EqCoords = tracking_params.base;
//! 
//! // The time when the telescope was angled to the base coordinates
//! let base_time: DateTime<Utc> = tracking_params.base_datetime;
//! 
//! // Target alt/az coordinates
//! let target_coords: AltAzCoords = altaz::apparent_alt_az_at(&target, &geo_coord, Utc::now());
//! 
//! // Base alt/az coordinates
//! let base_coords: AltAzCoords = altaz::apparent_alt_az_at(&base, &geo_coord, base_time);
//! 
//! // Step offsets for stepper motors
//! let alt_steps = (target_alt - base_alt)*tracking_params.alt_ratio;
//! let az_steps = (target_az - base_az)*tracking_params.az_ratio;
//! ```


use chrono::{DateTime, Duration, Utc};

use crate::{
	angle::get_alt_az_from_eq,
	coords::{AltAzCoords, EqCoords, GeoCoords},
	time::apparent_sidereal_at,
};

#[cfg(test)]
mod tests;

mod angle;
pub mod coords;
mod time;

/// Returns altitude and azimuth angles in `radians`.
///
/// # Examples
///
/// ```
/// // Current geographic coordinates
/// let geo_coord: GeoCoords = tracking_params.geo_coord;
///
/// // Telescope target equatorial coordinates
/// let target: EqCoords = tracking_params.target;
///
/// // Target alt/az coordinates at current time
/// let target_coords: AltAzCoords = altaz::apparent_alt_az_at(
/// 	&target,
/// 	&geo_coord,
/// 	Utc::now()
/// );
/// ```
pub fn apparent_alt_az_at(
	eq_coord: &EqCoords,
	geo_coord: &GeoCoords,
	datetime: DateTime<Utc>,
) -> AltAzCoords {
	let sidereal = apparent_sidereal_at(datetime);

	get_alt_az_from_eq(eq_coord, geo_coord, sidereal)
}

/// Returns altitude and azimuth angle speeds in `rad/s`. Uses epsilon duration for symmetrical derivative based on position function.
pub fn apparent_alt_az_speeds_at(
	eq_coord: &EqCoords,
	geo_coord: &GeoCoords,
	datetime: DateTime<Utc>,
	epsilon: Duration,
) -> AltAzCoords {
	let eps_sec = epsilon.as_seconds_f64();
	let half_eps = epsilon.checked_div(2).unwrap();

	let sidereal_a = apparent_sidereal_at(datetime - half_eps);
	let sidereal_b = apparent_sidereal_at(datetime + half_eps);

	let alt_az_a = get_alt_az_from_eq(eq_coord, geo_coord, sidereal_a);
	let alt_az_b = get_alt_az_from_eq(eq_coord, geo_coord, sidereal_b);

	AltAzCoords {
		alt: (alt_az_b.alt - alt_az_a.alt) / eps_sec,
		az: (alt_az_b.az - alt_az_a.az) / eps_sec,
	}
}
