use plotters::prelude::*;

use adafruit_gps::GpsSentence;

use crate::kinematics::{inverse_vincenty};

/// # Position Accuracy
/// Given a set of coordinates, produce the average longitude and latitude,

use super::Coordinate;
use std::fs::File;
use std::io::Write;


pub trait GpsSentenceConverter {
    fn to_coords(&self, include_geoidal_separation: bool) -> Vec<Coordinate>;
}

impl GpsSentenceConverter for Vec<GpsSentence> {
    /// Converts Vec<GpsSentence> to Vec<Coordinate>. Ignores GpsSentence types that have no long
    /// lat data in it. Adds all data it has.
    fn to_coords(&self, include_geoidal_separation: bool) -> Vec<Coordinate> {
        let mut vec_coord = Vec::new();
        for s in self.iter() {
            match s {
                GpsSentence::GGA(sentence) => {
                    let mut gga = Coordinate {
                        utc: sentence.utc,
                        latitude: sentence.lat,
                        longitude: sentence.long,
                        altitude: sentence.msl_alt,
                    };
                    if include_geoidal_separation {
                        gga.altitude = Some(gga.altitude.unwrap() + sentence.geoidal_sep.unwrap());
                    }
                    vec_coord.push(gga);
                }
                GpsSentence::GLL(sentence) => {
                    vec_coord.push(Coordinate {
                        utc: sentence.utc.unwrap_or(0.0),
                        latitude: sentence.latitude,
                        longitude: sentence.longitude,
                        altitude: None,
                    });
                }
                GpsSentence::RMC(sentence) => {
                    vec_coord.push(Coordinate {
                        utc: sentence.utc,
                        latitude: sentence.latitude,
                        longitude: sentence.longitude,
                        altitude: None,
                    });
                }
                _ => {}
            };
        }
        vec_coord
    }
}

pub trait Position {
    fn average_long_lat(&self) -> Coordinate;
    fn pprint(&self);
    fn plot_positions(&self, name: &str);
    fn to_klm(&self, name: &str, description: &str) -> std::io::Result<()>;
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
            x += (location.latitude.unwrap().cos() * location.longitude.unwrap().cos()) as f64;
            y += (location.latitude.unwrap().cos() * location.longitude.unwrap().sin()) as f64;
            z += location.latitude.unwrap().sin() as f64;
            altitude += location.altitude.unwrap() as f64;
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
            latitude: Some(average_lat as f32),
            longitude: Some(average_long as f32),
            altitude: Some(average_alt as f32),
            utc: 0.0,
        };
    }

    fn pprint(&self) {
        for item in self.iter() {
            println!("({:?}, {:?})", item.latitude.unwrap_or_default(), item.longitude.unwrap_or_default())
        }
    }

    /// Lat, Long. But to plot it you want long, lat as longitudes should be along the x axis.
    fn plot_positions(&self, name: &str) {
        let mut positions: Vec<(f32, f32)> = self.into_iter()
            .map(|x| (x.longitude.unwrap(), x.latitude.unwrap())).collect();
        positions.retain(|x| *x != (0.0, 0.0));  // Remove all (0,0) coords.

        let latitudes: Vec<f32> = positions.clone().into_iter().map(|x| x.0).collect();
        let longitudes: Vec<f32> = positions.clone().into_iter().map(|x| x.1).collect();
        let min_long = longitudes.iter().cloned().fold(0. / 0., f32::min);
        let max_long = longitudes.iter().cloned().fold(0. / 0., f32::max);
        let min_lat = latitudes.iter().cloned().fold(0. / 0., f32::min);
        let max_lat = latitudes.iter().cloned().fold(0. / 0., f32::max);

        // x axis is
        let _x_axis = inverse_vincenty(
            &Coordinate { utc: 0.0, latitude: Some(min_lat), longitude: Some(min_long), altitude: Some(0.0) },
            &Coordinate { utc: 0.0, latitude: Some(min_lat), longitude: Some(max_long), altitude: Some(0.0) });
        let _y_axis = inverse_vincenty(
            &Coordinate { utc: 0.0, latitude: Some(min_lat), longitude: Some(min_long), altitude: Some(0.0) },
            &Coordinate { utc: 0.0, latitude: Some(max_lat), longitude: Some(max_long), altitude: Some(0.0) },
        );

        let file_name = format!("images/{}.png", name);
        let root_area = BitMapBackend::new(file_name.as_str(), (600, 600))
            .into_drawing_area();
        root_area.fill(&WHITE).unwrap();

        let mut ctx = ChartBuilder::on(&root_area)
            .set_label_area_size(LabelAreaPosition::Left, 40)
            .set_label_area_size(LabelAreaPosition::Bottom, 40)
            .caption(name, ("Arial", 40))
            .build_ranged(min_lat..max_lat, min_long..max_long)
            .unwrap();

        ctx.configure_mesh().draw().unwrap();

        ctx.draw_series(
            LineSeries::new(
                positions, &BLUE))
            .unwrap();
    }


    fn to_klm(&self, name: &str, description: &str) -> std::io::Result<()>{
        let mut coordinates = String::new();
        for c in self.iter() {
            if c.latitude.is_some() && c.longitude.is_some() && c.altitude.is_some() {
                coordinates.push_str(format!(
                    "{},{},{}
                    ", c.longitude.unwrap(), c.latitude.unwrap(), c.altitude.unwrap()).as_str());
            }
        }

        let mut klm_string = String::new();
        klm_string.push_str(
            format!(
"<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<kml xmlns=\"http://www.opengis.net/kml/2.2\">
  <Document>
    <name>{}</name>
    <description>{}</description>
    <Style id=\"yellowLineGreenPoly\">
      <LineStyle>
        <color>7f00ffff</color>
        <width>4</width>
      </LineStyle>
      <PolyStyle>
        <color>7f00ff00</color>
      </PolyStyle>
    </Style>
    <Placemark>
      <name>Coordinate path</name>
      <description>Transparent green wall with yellow outlines</description>
      <styleUrl>#yellowLineGreenPoly</styleUrl>
      <LineString>
        <extrude>1</extrude>
        <tessellate>1</tessellate>
        <altitudeMode>absolute</altitudeMode>
        <coordinates> {} </coordinates>
      </LineString>
    </Placemark>
  </Document>
</kml>
            ", name, description, coordinates).as_str());
        let mut file = File::create(format!("{}.kml", name))?;
        file.write_all(klm_string.as_bytes())?;
        Ok(())
    }
}

