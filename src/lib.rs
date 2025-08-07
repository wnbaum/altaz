use chrono::{DateTime, Duration, Utc};

use crate::{
	angle::{AltAz, EqCoord, GeoCoord, get_alt_az_from_eq},
	time::apparent_sidereal_at,
};

#[cfg(test)]
mod tests;

mod angle;
mod time;

pub fn apparent_alt_az_at(
	eq_coord: &EqCoord,
	geo_coord: &GeoCoord,
	datetime: DateTime<Utc>,
) -> AltAz {
	let sidereal = apparent_sidereal_at(datetime);

	get_alt_az_from_eq(eq_coord, geo_coord, sidereal)
}

/// Returns alt/az radians per second. Uses epsilon duration for derivative.
pub fn apparent_alt_az_speeds_at(
	eq_coord: &EqCoord,
	geo_coord: &GeoCoord,
	datetime: DateTime<Utc>,
	epsilon: Duration,
) -> AltAz {
	let eps_sec = epsilon.as_seconds_f64();
	let half_eps = epsilon.checked_div(2).unwrap();

	let sidereal_a = apparent_sidereal_at(datetime - half_eps);
	let sidereal_b = apparent_sidereal_at(datetime + half_eps);

	let alt_az_a = get_alt_az_from_eq(eq_coord, geo_coord, sidereal_a);
	let alt_az_b = get_alt_az_from_eq(eq_coord, geo_coord, sidereal_b);

	AltAz {
		alt: (alt_az_b.alt - alt_az_a.alt) / eps_sec,
		az: (alt_az_b.az - alt_az_a.az) / eps_sec,
	}
}
