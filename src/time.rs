use astro::coords::{EqPoint, GeographPoint, alt_frm_eq, az_frm_eq, hr_angl_frm_observer_long};
use astro::time;
use chrono::{DateTime, Datelike, Timelike, Utc};

pub fn mean_sidereal_now() -> f64 {
	mean_sidereal_at(Utc::now())
}

pub fn apparent_sidereal_now() -> f64 {
	apparent_sidereal_at(Utc::now())
}

fn julian_day_at(datetime: DateTime<Utc>) -> f64 {
	let year = datetime.year() as i16;
	let month = datetime.month() as u8;
	let day = datetime.day() as f64;
	let hr = datetime.hour() as f64;
	let min = datetime.minute() as f64;
	let sec = datetime.second() as f64;
	let nanosecond = datetime.timestamp_subsec_nanos() as f64;

	let total_sec = sec + (nanosecond / 1_000_000_000.0);
	let decimal_day = day + ((hr + (min / 60.0) + (total_sec / 3600.0)) / 24.0);

	let date = time::Date {
		year,
		month,
		decimal_day,
		cal_type: time::CalType::Gregorian,
	};

	let julian_day = time::julian_day(&date);

	julian_day
}

pub fn mean_sidereal_at(datetime: DateTime<Utc>) -> f64 {
	let julian_day = julian_day_at(datetime);



	time::mn_sidr(julian_day)
}

pub fn apparent_sidereal_at(datetime: DateTime<Utc>) -> f64 {
	let julian_day = julian_day_at(datetime);

	let (nut_in_long, nut_in_oblq) = astro::nutation::nutation(julian_day);
	let true_oblq = astro::ecliptic::mn_oblq_IAU(julian_day) + nut_in_oblq;

	let mn_sidr = time::mn_sidr(julian_day);

	time::apprnt_sidr(mn_sidr, nut_in_long, true_oblq)
}