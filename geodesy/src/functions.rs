/// # Gps distance
/// Calculate the distance between two points given the longitude, latitude and altitude.
///
/// ## Vincenty formula
/// This implimentation of this problem will use Vincenty's formula. While more complex and slower
/// than other methods, it is more accurate. For details see https://en.wikipedia.org/wiki/Vincenty%27s_formulae
///
/// Work out the distance between two points on the sphere. Then, with pythagoras, work out the
/// absolute distance.

use super::Coordinate;

pub fn inverse_vincenty(start: &Coordinate, end: &Coordinate) -> f64 {
    #![allow(non_snake_case, non_upper_case_globals)]

    let max_iter = 200;

    const a: f64 = 6378137_f64;  // length radius at equator
    const f: f64 = 1_f64 / 298.257223563;  // flattening of the ellipsoid
    const b: f64 = (1_f64 - f) * a;  // radius at the poles - 6356752.314245 meters in WGS-84

    let U1: f64 = ((1_f64 - f) * start.latitude.to_radians().tan()).atan();  // Reduced latitude (latitude on the auxiliary sphere)
    let U2: f64 = ((1_f64 - f) * end.latitude.to_radians().tan()).atan();
    let L: f64 = end.longitude.to_radians() - start.longitude.to_radians();

    let sinU1 = U1.sin();
    let cosU1 = U1.cos();
    let sinU2 = U2.sin();
    let cosU2 = U2.cos();

    let mut Y: f64 = L;  // Should be λ,
    let mut counter = 0;
    let (cos_sq_alpha, sin_sigma, cos_2_sigma_m, cos_sigma, sigma) = loop {
        let sinY = Y.sin();
        let cosY = Y.cos();

        let sin_sigma = ((cosU2 * sinY).powi(2) +
            ((cosU1 * sinU2) - (sinU1 * cosU2 * cosY)).powi(2)
        ).sqrt();

        if sin_sigma == 0_f64 {
            // The points are the same so 0 distance.
            return 0_f64;
        }
        let cos_sigma = sinU1 * sinU2 + cosU1 * cosU2 * cosY;

        let sigma = sin_sigma.atan2(cos_sigma);

        let sin_alpha = (cosU1 * cosU2 * sinY) / (sin_sigma);
        let cos_sq_alpha = 1_f64 - sin_alpha.powi(2);

        let cos_2_sigma_m = cos_sigma - ((2_f64 * sinU1 * sinU2) / cos_sq_alpha);

        let C = f / 16_f64 * cos_sq_alpha * (4_f64 + f * (4_f64 - 3.0 * cos_sq_alpha));

        let prev_Y = Y;
        Y = L + (1_f64 - C) * f * sin_alpha * (sigma + C * sin_sigma *
            (cos_2_sigma_m + C * cos_sigma * (-1_f64 + 2_f64 * cos_2_sigma_m.powi(2))));

        if Y - prev_Y < 1e-12 {
            break (cos_sq_alpha, sin_sigma, cos_2_sigma_m, cos_sigma, sigma);
        } else if counter > max_iter {
            break (cos_sq_alpha, sin_sigma, cos_2_sigma_m, cos_sigma, sigma);
        } else {
            counter += 1;
        }
    };

    let uSq = cos_sq_alpha * ((a.powi(2) - b.powi(2)) / b.powi(2));
    let A = 1_f64 + uSq / 16382_f64 * (4096_f64 + uSq * (-768_f64 + uSq * (320_f64 - 175_f64 * uSq)));
    let B = uSq / 1024_f64 * (256_f64 + uSq * (-128_f64 + uSq * (74_f64 - 47_f64 * uSq)));
    let delta_sigma = B * sin_sigma * (cos_2_sigma_m + B / 4_f64 * (cos_sigma *
        (-1_f64 + 2_f64 * cos_2_sigma_m.powi(2)) - B / 6_f64 * cos_2_sigma_m *
        (-3_f64 + 4_f64 * sin_sigma.powi(2)) * (-3_f64 + 4_f64 * cos_2_sigma_m.powi(2))));

    let s = b * A * (sigma - delta_sigma);

    return s;
}

pub fn average_long_lat(locations: &Vec<Coordinate>) -> Coordinate {
    //! # Latitude and Longitude variation
    //! https://stackoverflow.com/questions/6671183/calculate-the-center-point-of-multiple-latitude-longitude-coordinate-pairs
    //! # Altitude variation
    //! Simple mean of the altitudes.
    //!
    //! Returns a Location structure with each long, lat and altitude being the average. UTC is 0.
    //!
    //! Assume that all points in the Vec<Location> are valid.
    let mut x: f64 = 0.0;
    let mut y: f64 = 0.0;
    let mut z: f64 = 0.0;
    let mut altitude: f64 = 0.0;

    for location in locations.iter() {
        x += location.latitude.cos() * location.longitude.cos();
        y += location.latitude.cos() * location.longitude.sin();
        z += location.latitude.sin();
        altitude += location.altitude;
    }
    x = x / locations.len() as f64;
    y = y / locations.len() as f64;
    z = z / locations.len() as f64;

    let central_long = y.atan2(x);
    let central_sq_rt = (x * x + y * y).sqrt();
    let central_lat = z.atan2(central_sq_rt);

    let average_long = central_long.to_degrees();
    let average_lat = central_lat.to_degrees();
    let average_alt: f64 = altitude / locations.len() as f64;

    return Coordinate {
        latitude: average_lat,
        longitude: average_long,
        altitude: average_alt,
        utc: 0.0,
    };
}

#[cfg(test)]
mod test_inverse_vincenty {
    use super::{inverse_vincenty, Coordinate};

    #[test]
    fn test1() {
        let cal = inverse_vincenty(
            &Coordinate { utc: 0.0, latitude: 51.557319, longitude: 0.029082, altitude: 0.0 },
            &Coordinate { utc: 0.0, latitude: 51.557286, longitude: 0.037998, altitude: 0.0 });
        assert_eq!(cal, 618.3658911130701)
    }

    #[test]
    fn test2() {
        let cal = inverse_vincenty(
            &Coordinate { utc: 0.0, latitude: 42.3541165, longitude: -71.0693514, altitude: 0.0 },
            &Coordinate { utc: 0.0, latitude: 40.7791472, longitude: -73.9680804, altitude: 0.0 });
        assert_eq!(cal, 298027.9968565652)
    }

    #[test]
    fn test3() {
        let cal = inverse_vincenty(
            &Coordinate { utc: 0.0, latitude: 51.557319, longitude: 0.029082, altitude: 0.0 },
            &Coordinate { utc: 0.0, latitude: 51.557319, longitude: 0.029082, altitude: 0.0 });
        assert_eq!(cal, 0_f64)
    }
}
