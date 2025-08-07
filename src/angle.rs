use std::f64::consts::TAU;

use crate::coords::{AltAzCoords, EqCoords, GeoCoords};

pub fn get_alt_az_from_eq(eq_coord: &EqCoords, geo_coord: &GeoCoords, sidereal: f64) -> AltAzCoords {
	let hour_angle = hour_angle_from_observer_long(sidereal, geo_coord.long, eq_coord.ra);
	let alt = alt_from_eq(hour_angle, eq_coord.dec, geo_coord.lat);
	let az = az_from_eq(hour_angle, eq_coord.dec, geo_coord.lat, alt);

	let az = az.rem_euclid(TAU);

	AltAzCoords { alt, az }
}

/// DO NOT USE `astro::coords::hr_angl_frm_observer_long`. This is the correct formula. All in radians.
pub fn hour_angle_from_observer_long(green_sidereal: f64, long: f64, ra: f64) -> f64 {
	let local_sidereal = green_sidereal + long;

	let hour_angle = local_sidereal - ra;

	hour_angle.rem_euclid(TAU)
}

pub fn alt_from_eq(hour_angle: f64, dec: f64, lat: f64) -> f64 {
	let alt = dec.sin() * lat.sin() + dec.cos() * lat.cos() * hour_angle.cos();

	alt.asin()
}

pub fn az_from_eq(hour_angle: f64, dec: f64, lat: f64, alt: f64) -> f64 {
	let y = -(dec.cos() * hour_angle.sin()) / alt.cos();
	let x = (dec.sin() - alt.sin() * lat.sin()) / (alt.cos() * lat.cos());

	y.atan2(x)
}
