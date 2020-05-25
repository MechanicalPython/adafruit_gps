/// # Position Accuracy
/// Given a set of coordinates, produce the average longitude and latitude,

use super::Coordinate;

use plotters::prelude::*;

pub trait Position {
    fn average_long_lat(&self) -> Coordinate;
    fn plot_positions(&self);
}

impl Position for Vec<Coordinate> {
    /// # Latitude and Longitude variation
    /// https://stackoverflow.com/questions/6671183/calculate-the-center-point-of-multiple-latitude-longitude-coordinate-pairs
    /// # Altitude variation
    /// Simple mean of the altitudes.
    ///
    /// Returns a Location structure with each long, lat and altitude being the average. UTC is 0.
    ///
    /// Assume that all points in the Vec<Location> are valid.
    fn average_long_lat(&self) -> Coordinate {
        let mut x: f64 = 0.0;
        let mut y: f64 = 0.0;
        let mut z: f64 = 0.0;
        let mut altitude: f64 = 0.0;

        for location in self.iter() {
            x += location.latitude.cos() * location.longitude.cos();
            y += location.latitude.cos() * location.longitude.sin();
            z += location.latitude.sin();
            altitude += location.altitude;
        }
        x = x / self.len() as f64;
        y = y / self.len() as f64;
        z = z / self.len() as f64;

        let central_long = y.atan2(x);
        let central_sq_rt = (x * x + y * y).sqrt();
        let central_lat = z.atan2(central_sq_rt);

        let average_long = central_long.to_degrees();
        let average_lat = central_lat.to_degrees();
        let average_alt: f64 = altitude / self.len() as f64;

        return Coordinate {
            latitude: average_lat,
            longitude: average_long,
            altitude: average_alt,
            utc: 0.0,
        };
    }

    fn plot_positions(&self) {
        // let mut lats  = Vec::new();
        // let mut longs = Vec::new();
        // for coord in self.iter() {
        //     lats.push(coord.latitude);
        //     longs.push(coord.longitude);
        // }
        //
        // let root = BitMapBackend::gif("location.gif", (800, 800), 1000).unwrap().into_drawing_area();
        // for i in coords.iter() {
        //     root.fill(&WHITE).unwrap();
        //     let mut chart = ChartBuilder::on(&root)
        //         .caption("Positions", ("sans-serif", 50))
        //         .build_ranged(lats.max()..lats.min(), longs.max()..longs.min()).unwrap();
        //     chart.draw()
        //
        // }
    }
}

