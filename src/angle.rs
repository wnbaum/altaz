use std::f64::consts::TAU;

/// ALt and az in radians.
#[derive(Clone, Copy, Debug)]
pub struct AltAz {
	pub alt: f64,
	pub az: f64,
}

/// Geocoords in radians.
#[derive(Clone, Copy, Debug)]
pub struct GeoCoord {
	pub lat: f64,
	pub long: f64,
}

impl GeoCoord {
	pub fn new(lat: f64, long: f64) -> Self {
		GeoCoord { lat, long }
	}
}

#[derive(Clone, Copy, Debug)]
pub struct EqCoord {
	pub ra: f64,
	pub dec: f64,
}

impl EqCoord {
	pub fn new(ra: f64, dec: f64) -> Self {
		EqCoord { ra, dec }
	}
}

impl GeoCoord {
	pub fn from_rad(lat: f64, long: f64) -> Self {
		Self { lat, long }
	}

	pub fn from_deg(lat: f64, long: f64) -> Self {
		Self {
			lat: lat.to_radians(),
			long: long.to_radians(),
		}
	}
}

pub fn get_alt_az_from_eq(eq_coord: &EqCoord, geo_coord: &GeoCoord, sidereal: f64) -> AltAz {
	let hour_angle = hour_angle_from_observer_long(sidereal, geo_coord.long, eq_coord.ra);
	let alt = alt_from_eq(hour_angle, eq_coord.dec, geo_coord.lat);
	let az = az_from_eq(hour_angle, eq_coord.dec, geo_coord.lat, alt);

	let az = az.rem_euclid(TAU);

	AltAz { alt, az }
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
