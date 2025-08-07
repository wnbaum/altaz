# altaz

[![docs](https://img.shields.io/badge/docs-github_pages-4c1)](https://wnbaum.github.io/altaz/altaz/)

## About

`altaz` provides a few simple functions to determine angles for altitude azimuth telescope mounts given equatorial coordinates.

I've designed it with my telescope controller in mind, though it may be useful to others who want to accomplish a similar goal.

`altaz` depends on some correct algorithms from the `astro` crate, though I re-implemented some things that are handled incorrectly. `altaz` also depends on `chrono` for easy time handling.

## Usage

Here is a usage example from my telescope controller to control altitude and azimuth stepper motors on my mount:

```rust
// Current geographic coordinates
let geo_coord: GeoCoords = tracking_params.geo_coord;

// Telescope target equatorial coordinates
let target: EqCoords = tracking_params.target;

// Telescope base equatorial coordinates (to rotate from)
let base: EqCoords = tracking_params.base;

// The time when the telescope was angled to the base coordinates
let base_time: DateTime<Utc> = tracking_params.base_datetime;

// Target alt/az coordinates
let target_coords: AltAzCoords = altaz::apparent_alt_az_at(&target, &geo_coord, Utc::now());

// Base alt/az coordinates
let base_coords: AltAzCoords = altaz::apparent_alt_az_at(&base, &geo_coord, base_time);

// Step offsets for stepper motors
let alt_steps = (target_alt - base_alt)*tracking_params.alt_ratio;
let az_steps = (target_az - base_az)*tracking_params.az_ratio;
```

## Todo

- More and higher quality tests