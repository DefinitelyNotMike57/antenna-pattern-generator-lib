//! # Pattern Generator
//!
//! This library provides tools for the user to create standard and custom
//! antenna patterns.
//!
//!

#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

use derive_new::new;
use num::{
    complex::Complex,
    traits::{Float, Num},
};

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/// Speed of Light (m/s)
pub const SPEED_OF_LIGHT: f64 = 299792458.0;

/// An imaginary number
const IMAG: Complex<f64> = Complex::new(0.0, 1.0);

/// Pi
pub const PI: f64 = std::f64::consts::PI;

/// Interface for elements of an array
///
///
pub trait ElementIface {
    /// Returns the gain of this element
    ///
    ///
    fn get_gain(&self, frequency: f64, theta: f64, phi: f64) -> Complex<f64>;
}

/// Associate a position with an element
///
/// This allows a single instance of an element to be used multiple times in
/// an array and if that element has cached it input/output pairs then the same
/// operation won't have to be run hundreds of times for a single call.
struct PositionElement {
    position: Point,
    position_factor: Complex<f64>,
    weight: Complex<f64>,
    element: Box<dyn ElementIface>,
}

/// Obviously get_gain() but also cache the results from the translation function
///
/// The elements are always in the same position relative to each other so caching
/// the translation results just make sense.
impl PositionElement {
    fn get_gain(&self, frequency: f64, theta: f64, phi: f64) -> Complex<f64> {
}

/// Translates element patterns in space
///
/// Antenna patterns are normally created at the phase center of the antenna
/// element. To create an array of elements, each element needs to be shifted
/// to a different position so that their independent patterns can combine into
/// a more focused pattern.
///
#[memoize(Capacity: 1000)]
fn calc_phase(pnt: &Point, frequency: f64, theta: f64, phi: f64) -> Complex<f64> {
    let k = 2.0 * PI * frequency / SPEED_OF_LIGHT;

    let dx = IMAG * k * pnt.x * phi.cos() * theta.sin();
    let dy = IMAG * k * pnt.y * phi.sin() * theta.sin();
    let dz = IMAG * k * pnt.z * theta.cos();

    dx.exp() * dy.exp() * dz.exp()
}

/// An omni-directional element is the most generic type of element
///
///
#[derive(new)]
pub struct OmniElement {
    position: Point,
    gain: f64,
}

impl ElementIface for OmniElement {
    fn get_gain(&self, frequency: f64, theta: f64, phi: f64) -> Complex<f64> {
        calc_phase(&self.position, frequency, theta, phi) * self.gain
    }
}

pub struct PatchElement {
    position: Point,
    // side of patch parallel with feed (meters)
    length: f64,
    // side of patch normal to feed (meters)
    width: f64,
}

impl ElementIface for PatchElement {
    #[memoize(Capacity: 123)]
    fn get_gain(&self, frequency: f64, theta: f64, phi: f64) -> Complex<f64> {
        let k = 2.0 * PI * frequency / SPEED_OF_LIGHT;
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();
        let sin_phi = phi.sin();
        let cos_phi = phi.cos();

        let inside0 = k * self.width * sin_theta * sin_phi / 2.0;
        let value0 = inside0.sin() / inside0;
        let value1 = (k * self.length * sin_theta * cos_phi).cos();
        let value2 = value0 * value1;

        let e_field_theta = value2 * cos_phi;
        let e_field_phi = -value2 * cos_theta * sin_phi;

        Complex::new(
            (e_field_theta.powf(2.0) + e_field_phi.powf(2.0)).powf(0.5),
            0.0,
        )
    }
}

// Reads and interpolates a data table for the antenna pattern with optional
// positional offset

/// A special element that relies on a table of data
///
///
#[derive(new)]
struct DataElement {
    position: Option<Point>,
    data: Vec<Vec<Complex<f64>>>,
}

/// Interface for types of arrays
///
///
pub trait ArrayIface {
    /// Return the gain of a single element from the array
    ///
    ///
    fn get_channel_gain(
        &self,
        channel: usize,
        frequency: f64,
        theta: f64,
        phi: f64,
    ) -> Complex<f64>;

    /// Return the cumulative gain of all elements in array
    ///
    ///
    fn get_gain(&self, frequency: f64, theta: f64, phi: f64) -> Complex<f64>;
}

/// A position in 3D cartesian space
#[derive(new)]
pub struct Point {
    // all values are distance from origin (meters)
    x: f64,
    y: f64,
    z: f64,
}

/// This object represents an antenna array
///
/// Antenna arrays take many shapes, this can handle all of them as long as
/// each element satisfies the ElementIface trait.
#[derive(new)]
pub struct ArrayAntenna {
    elements: Vec<PositionElement>,
}

impl ArrayIface for ArrayAntenna {
    fn get_channel_gain(
        &self,
        channel: usize,
        frequency: f64,
        phi: f64,
        theta: f64,
    ) -> Complex<f64> {
        self.elements[channel].get_gain(frequency, phi, theta)
    }
    fn get_gain(&self, frequency: f64, phi: f64, theta: f64) -> Complex<f64> {
        let gains: Vec<Complex<f64>> = self
            .elements
            .iter()
            .map(|n| n.get_gain(frequency, phi, theta))
            .collect();
        gains.iter().sum()
    }
}

/// Utility for writing array patterns to a file
///
///
pub fn write_to_file(
    array: Box<dyn ArrayIface>,
    frequency: f64,
    theta_spacing: f64,
    phi_spacing: f64,
    file_name: String,
) {
    let num_theta_samples: u64 = (PI / theta_spacing) as u64;
    let num_phi_samples: u64 = (2.0 * PI / phi_spacing) as u64;

    let path = Path::new(&file_name);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    file.write_all(b"0.0");

    for theta_idx in 0..num_theta_samples {
        let theta = theta_idx as f64 * theta_spacing;
        file.write_all(format!(" {:0.2}", theta * 180.0 / PI).as_bytes());
    }

    for phi_idx in 0..num_theta_samples {
        let phi = phi_idx as f64 * phi_spacing;
        file.write_all(format!("\n{:0.2}", phi * 180.0 / PI).as_bytes());
        for theta_idx in 0..num_theta_samples {
            let theta = theta_idx as f64 * theta_spacing;
            let gain = array.get_gain(frequency, theta, phi);
            file.write_all(format!(" {:0.2}", gain.norm()).as_bytes());
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
