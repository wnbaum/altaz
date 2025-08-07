use chrono::{DateTime, Utc};

use crate::{angle::{get_alt_az_from_eq, AltAz, EqCoord, GeoCoord}, time::apparent_sidereal_at};

#[cfg(test)]
mod tests;

mod time;
mod angle;

pub fn apparent_alt_az_at(eq_coord: &EqCoord, geo_coord: &GeoCoord, datetime: DateTime<Utc>) -> AltAz {
	let sidereal = apparent_sidereal_at(datetime);

	get_alt_az_from_eq(eq_coord, geo_coord, sidereal)
}