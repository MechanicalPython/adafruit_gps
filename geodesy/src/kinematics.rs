//! # Gps distance
//! Calculate the distance between two points given the longitude, latitude and altitude.
//!
//! ## Vincenty formula
//! This implimentation of this problem will use Vincenty's formula. While more complex and slower
//! than other methods, it is more accurate. For details see
//!
//! Work out the distance between two points on the sphere. Then, with pythagoras, work out the
//! absolute distance.

use super::Coordinate;

/// # Inverse vincenty
/// Breaks down for antipodal points.
///
/// (Vincenty wiki)[https://en.wikipedia.org/wiki/Vincenty%27s_formulae]
pub fn inverse_vincenty(start: &Coordinate, end: &Coordinate) -> f64 {
    #![allow(non_snake_case, non_upper_case_globals)]

    let max_iter = 200;

    const a: f64 = 6378137_f64;  // length radius at equator
    const f: f64 = 1_f64 / 298.257223563;  // flattening of the ellipsoid
    const b: f64 = (1_f64 - f) * a;  // radius at the poles - 6356752.314245 meters in WGS-84


    let U1: f64 = ((1_f64 - f) * start.latitude.unwrap().to_radians().tan() as f64).atan();  // Reduced latitude (latitude on the auxiliary sphere)
    let U2: f64 = ((1_f64 - f) * end.latitude.unwrap().to_radians().tan() as f64).atan();
    let L: f64 = (end.longitude.unwrap().to_radians() - start.longitude.unwrap().to_radians()) as f64;

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
        Y = &L + (1_f64 - C) * f * sin_alpha * (sigma + C * sin_sigma *
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

/// # Haversine
/// Less accurate than vincenty as it assumes that the earth is a perfect sphere,
/// but less computationally expensive.
///
/// (Haversine wiki) [https://en.wikipedia.org/wiki/Haversine_formula]
pub fn haversine(start: &Coordinate, end: &Coordinate) -> f64 {
    let lat1 = start.latitude.unwrap().to_radians() as f64;
    let lat2 = end.latitude.unwrap().to_radians() as f64;
    let long1 = start.longitude.unwrap().to_radians() as f64;
    let long2 = end.longitude.unwrap().to_radians() as f64;
    let mean_earth_radius = 6371008.8; // https://en.wikipedia.org/wiki/Earth_radius#Global_average_radii
    let havlat = ((lat2 - lat1) / 2_f64).sin().powi(2);
    let havlong = ((long2 - long1) / 2_f64).sin().powi(2);
    let distance = ((havlat + lat1.cos() * lat2.cos() * havlong).sqrt()).asin() * 2_f64 * mean_earth_radius;
    return distance;
}

pub trait DeltaCoordinates {
    fn vincenty(&self) -> Vec<(f64, f64)>;
    fn haversine(&self) -> Vec<(f64, f64)>;
}

impl DeltaCoordinates for Vec<Coordinate> {
    fn vincenty(&self) -> Vec<(f64, f64)> {
        let mut return_vec: Vec<(f64, f64)> = Vec::new();
        for t in 0..self.len() - 1 {
            let time_diff = self.get(t+1).unwrap().utc - self.get(t).unwrap().utc;
            let mut d = inverse_vincenty(self.get(t).unwrap(), self.get(t + 1).unwrap());
            d = (d.powi(2) + (self.get(t).unwrap().altitude.unwrap() - self.get(t + 1).unwrap().altitude.unwrap()).powi(2) as f64).sqrt();
            return_vec.push((time_diff, d))
        }
        return return_vec;
    }
    fn haversine(&self) -> Vec<(f64, f64)> {
        let mut return_vec: Vec<(f64, f64)> = Vec::new();
        for t in 0..self.len() - 1 {
            let time_diff = self.get(t+1).unwrap().utc - self.get(t).unwrap().utc;
            let mut d = haversine(self.get(t).unwrap(), self.get(t + 1).unwrap());
            d = (d.powi(2) + (self.get(t).unwrap().altitude.unwrap() - self.get(t + 1).unwrap().altitude.unwrap()).powi(2) as f64).sqrt();
            return_vec.push((time_diff, d))
        }
        return return_vec;
    }
}

pub trait Kinematics{
    fn distance(&self) -> Vec<f64>;
    fn speed(&self) -> Vec<f64>;
}

impl Kinematics for Vec<(f64, f64)> {
    fn distance(&self) -> Vec<f64>{
        let mut return_vec: Vec<f64> = Vec::new();
        for (_time_diff, distance) in self.iter() {
            return_vec.push(*distance)
        }
        return return_vec;
    }

    fn speed(&self) -> Vec<f64> {
        let mut return_vec: Vec<f64> = Vec::new();
        for (time_diff, distance) in self.iter() {
            return_vec.push(distance/time_diff)
        }
        return return_vec;
    }
}


#[cfg(test)]
mod test_distances {
    use super::{haversine, inverse_vincenty, Coordinate, DeltaCoordinates, Kinematics};

    const SMALL1: Coordinate = Coordinate { utc: (0.0), latitude: Some(51.55814), longitude: Some(0.02955), altitude: Some(0.0) };
    const SMALL2: Coordinate = Coordinate { utc: (0.0), latitude: Some(51.55795), longitude: Some(0.03014), altitude: Some(100.0) };
    const SMALL3: Coordinate = Coordinate { utc: (0.0), latitude: Some(51.55795), longitude: Some(0.03014), altitude: Some(0.0) };

    const LONDON: Coordinate = Coordinate { utc: (0.0), latitude: Some(51.500821), longitude: Some(-0.126670), altitude: Some(0.0) };
    const PARIS: Coordinate = Coordinate { utc: (0.0), latitude: Some(48.858788), longitude: Some(2.293746), altitude: Some(0.0) };
    const SYDNEY: Coordinate = Coordinate { utc: (0.0), latitude: Some(-33.852239), longitude: Some(151.210675), altitude: Some(0.0) };

    #[test]
    fn vincenty_same_point() {
        let cal = inverse_vincenty(&LONDON, &LONDON).round();
        assert_eq!(cal, 0.0)
    }

    #[test]
    fn vincenty_small_no_alt() {
        let locations: Vec<Coordinate> = vec![SMALL1, SMALL3];
        let cal = locations.vincenty().distance().get(0).unwrap().round();
        assert_eq!(cal, 46.0)
    }

    #[test]
    fn vincenty_small() {
        let locations: Vec<Coordinate> = vec![SMALL1, SMALL2];
        let cal = locations.vincenty().distance().get(0).unwrap().round();
        assert_eq!(cal, 110.0)
    }

    #[test]
    fn vincenty_lon_paris() {
        let cal = inverse_vincenty(&LONDON, &PARIS).round();
        assert_eq!(cal, 340916.0)
    }

    #[test]
    fn vincenty_lon_syd() {
        let cal = inverse_vincenty(&LONDON, &SYDNEY).round();
        assert_eq!(cal, 16988330.0)
    }

    #[test]
    fn haversine_same_point() {
        let cal = haversine(&LONDON, &LONDON).round();
        assert_eq!(cal, 0.0)
    }

    #[test]
    fn haversine_small() {
        let locations: Vec<Coordinate> = vec![SMALL1, SMALL2];
        let cal = locations.haversine().distance().get(0).unwrap().round();
        assert_eq!(cal, 110.0)
    }

    #[test]
    fn haversine_small_no_alt() {
        let locations: Vec<Coordinate> = vec![SMALL1, SMALL3];
        let cal = locations.haversine().distance().get(0).unwrap().round();
        assert_eq!(cal, 46.0)
    }

    #[test]
    fn haversine_lon_paris() {
        let cal = haversine(&LONDON, &PARIS).round();
        assert_eq!(cal, 340561.0)
    }

    #[test]
    fn haversine_lon_syd() {
        let cal = haversine(&LONDON, &SYDNEY).round();
        assert_eq!(cal, 16992936.0)
    }

}
