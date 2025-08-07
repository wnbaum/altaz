/// Altitude and azimuth in `radians`.
#[derive(Clone, Copy, Debug)]
pub struct AltAzCoords {
	/// Altitude angle in `radians`
	pub alt: f64,
	/// Azimuth angle in `radians`
	pub az: f64,
}

impl AltAzCoords {
	pub fn new(alt: f64, az: f64) -> Self {
		Self { alt, az }
	}
}

/// Geographical coordinates in `radians`.
#[derive(Clone, Copy, Debug)]
pub struct GeoCoords {
	/// Latitude in `radians`
	pub lat: f64,
	/// Longitude in `radians`
	pub long: f64,
}

impl GeoCoords {
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

/// Equatorial coordinates in `radians`.
#[derive(Clone, Copy, Debug)]
pub struct EqCoords {
	/// Right ascension in `radians`
	pub ra: f64,
	/// Declination in `radians`
	pub dec: f64,
}

impl EqCoords {
	pub fn new(ra: f64, dec: f64) -> Self {
		Self { ra, dec }
	}
}