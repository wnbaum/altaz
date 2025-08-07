use astro::angle::{deg_frm_dms, deg_frm_hms, hms_frm_deg};
use chrono::{Duration, TimeZone, Utc};

use crate::{
	angle::{get_alt_az_from_eq, hour_angle_from_observer_long},
	apparent_alt_az_at, apparent_alt_az_speeds_at,
	coords::{AltAzCoords, EqCoords, GeoCoords},
	time::{apparent_sidereal_at, mean_sidereal_at},
};

#[test]
fn test_mean_sidereal_at_known_time() {
	let datetime = Utc.with_ymd_and_hms(2025, 08, 07, 15, 18, 18).unwrap();
	let sidereal = mean_sidereal_at(datetime);

	// 12:23:53.8163
	let expected = deg_frm_hms(12, 23, 53.8163).to_radians(); // rad

	let tolerance = 0.0001;
	assert!(
		(sidereal - expected).abs() < tolerance,
		"Expected {:?}, got {:?}",
		hms_frm_deg(expected.to_degrees()),
		hms_frm_deg(sidereal.to_degrees())
	);
}

#[test]
fn test_apparent_sidereal_at_known_time() {
	let datetime = Utc.with_ymd_and_hms(2025, 08, 07, 15, 18, 18).unwrap();
	let sidereal = apparent_sidereal_at(datetime);

	// 12:23:54.0787
	let expected = deg_frm_hms(12, 23, 54.0787).to_radians(); // rad

	let tolerance = 0.0001;
	assert!(
		(sidereal - expected).abs() < tolerance,
		"Expected {:?}, got {:?}",
		hms_frm_deg(expected.to_degrees()),
		hms_frm_deg(sidereal.to_degrees())
	);
}

#[test]
fn test_alt_az_at_known_time() {
	// lat: 40 20 5.57
	// long: -74 37 16.06
	// vega eq:  18h 36m 56s     +38° 47′ 01″
	// time: 08/07/25 15:18:18 (11:18:18 local?)
	// output: alt (dms): -10 06 25		azi (dms): 9 24 50
	// extra calc hr angle: 12h 50m 57.82s

	let eq_coord = EqCoords::new(
		deg_frm_hms(18, 36, 56.0).to_radians(),
		deg_frm_dms(38, 47, 01.0).to_radians(),
	);

	let geo_coord = GeoCoords::from_deg(deg_frm_dms(40, 20, 5.57), deg_frm_dms(-74, 37, 16.06));

	let datetime = Utc.with_ymd_and_hms(2025, 08, 07, 15, 18, 18).unwrap();
	let sidereal = apparent_sidereal_at(datetime);

	let alt_expected = deg_frm_dms(-10, 06, 42.8).to_radians();
	let az_expected = deg_frm_dms(9, 23, 31.1).to_radians();

	let alt_az_expected = AltAzCoords {
		alt: alt_expected,
		az: az_expected,
	};

	let hour_angle_expected = deg_frm_hms(12, 47, 35.49).to_radians();

	let tolerance = 0.01;

	// Check hour angle
	let hour_angle = hour_angle_from_observer_long(sidereal, geo_coord.long, eq_coord.ra);
	assert!(
		(hour_angle - hour_angle_expected).abs() < tolerance,
		"Hour Angle: Expected {:?}, got {:?}",
		hms_frm_deg(hour_angle_expected.to_degrees()),
		hms_frm_deg(hour_angle.to_degrees())
	);

	// Check inner function
	let alt_az = get_alt_az_from_eq(&eq_coord, &geo_coord, sidereal);
	assert!(
		(alt_az.alt - alt_expected).abs() < tolerance
			&& (alt_az.az - az_expected).abs() < tolerance,
		"Inner: Expected {:?}, got {:?}",
		alt_az_expected,
		alt_az
	);

	// Check main function
	let alt_az = apparent_alt_az_at(&eq_coord, &geo_coord, datetime);
	assert!(
		(alt_az.alt - alt_expected).abs() < tolerance
			&& (alt_az.az - az_expected).abs() < tolerance,
		"Main: Expected {:?}, got {:?}",
		alt_az_expected,
		alt_az
	);

	// Check speed function
	let tolerance = 1e-5;
	let alt_az_speed = apparent_alt_az_speeds_at(
		&eq_coord,
		&geo_coord,
		datetime,
		Duration::milliseconds(1000),
	);
	let alt_az_speed_expected = AltAzCoords {
		alt: 8.73e-6,
		az: 5.76e-5,
	};
	assert!(
		(alt_az_speed.alt - alt_az_speed_expected.alt).abs() < tolerance
			&& (alt_az_speed.az - alt_az_speed_expected.az).abs() < tolerance,
		"Speeds: Expected {:?}, got {:?}",
		alt_az_speed_expected,
		alt_az_speed
	);
}
