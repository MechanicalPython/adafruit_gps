/// # Geodesy library for all that extra fun post data gathering analysis.
///
/// One to work out the distance between two points.
/// Make a structure of a series of points and implement.
/// At any point you are doing stuff to a series of points.

//todo - size of error for a long lat: 51.0, 1.0 is x m^2 area.
// todo - expected distance error for a given pdop.

pub mod kinematics;
pub mod position;

/// This is the basic coordinate data for a single point in space.
///
/// - UTC is used when calculating speed (relative UTC is needed)
/// - altitude is used when measuring distance and actually calculates euclidian distance between
/// points. If not required just put altitude to 0 and it will not affect calculations.
#[derive(Default, PartialEq, Debug)]
pub struct Coordinate {
    pub utc: f64,
    pub latitude: Option<f32>,
    pub longitude: Option<f32>,
    pub altitude: Option<f32>,
}


