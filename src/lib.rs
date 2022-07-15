//! # Pattern Generator
//!
//! This library provides tools for the user to create standard and custom
//! antenna patterns.
//!
//!

#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

use derive_new::new;
use num::complex::Complex;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/// Speed of Light (m/s)
pub const SPEED_OF_LIGHT: f64 = 299792458.0;

/// An imaginary number
const I: Complex<f64> = Complex::new(0.0, 1.0);

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

    //fn set_position(&self, position: Point);
    fn set_weight(&mut self, weight: Complex<f64>);
}

/// Translates element patterns in space
///
/// Antenna patterns are normally created at the phase center of the antenna
/// element. To create an array of elements, each element needs to be shifted
/// to a different position so that their independent patterns can combine into
/// a more focused pattern.
///
fn calc_phase(pnt: &Point, frequency: f64, theta: f64, phi: f64) -> Complex<f64> {
    let k = 2.0 * PI * frequency / SPEED_OF_LIGHT;

    let dx = I * k * pnt.x * phi.cos() * theta.sin();
    let dy = I * k * pnt.y * phi.sin() * theta.sin();
    let dz = I * k * pnt.z * theta.cos();

    dx.exp() * dy.exp() * dz.exp()
}

/// An omni-directional element is the most generic type of element
///
/// On initialization, the user can set the position, gain, and weight
/// of this element.
#[derive(new)]
pub struct OmniElement {
    // position of omni in space
    position: Point,
    // Omni elements usually have a gain of 1 (0dBi) but the user can set this manually
    gain: f64,
    // Weight applied to element pattern
    weight: Complex<f64>,
}

/// Satisfy required interface for OmniElement
///
///
impl ElementIface for OmniElement {
    fn get_gain(&self, frequency: f64, theta: f64, phi: f64) -> Complex<f64> {
        calc_phase(&self.position, frequency, theta, phi) * self.gain * self.weight
    }
    fn set_weight(&mut self, weight: Complex<f64>) {
        self.weight = weight;
    }
}

/// A patch is a PCB based antenna that has a hemispherically directional pattern
///
///
pub struct PatchElement {
    // position of patch in space
    position: Point,
    // side of patch parallel with feed (meters)
    length: f64,
    // side of patch normal to feed (meters)
    width: f64,
    // Weight applied to element pattern
    weight: Complex<f64>,
}

/// Canonical formula to calculate gain of patch antenna
///
/// I created a function for this so that all PatchElement instances
/// can benefit from the memoization that is here.
fn patch_gain(length: f64, width: f64, frequency: f64, theta: f64, phi: f64) -> Complex<f64> {
    let k = 2.0 * PI * frequency / SPEED_OF_LIGHT;
    let sin_theta = theta.sin();
    let cos_theta = theta.cos();
    let sin_phi = phi.sin();
    let cos_phi = phi.cos();

    let inside0 = k * width * sin_theta * sin_phi / 2.0;
    let value0 = inside0.sin() / inside0;
    let value1 = (k * length * sin_theta * cos_phi).cos();
    let value2 = value0 * value1;

    let e_field_theta = value2 * cos_phi;
    let e_field_phi = -value2 * cos_theta * sin_phi;

    Complex::new(
        (e_field_theta.powf(2.0) + e_field_phi.powf(2.0)).powf(0.5),
        0.0,
    )
}

/// Satisfy required interface for PatchElement
///
///
impl ElementIface for PatchElement {
    fn get_gain(&self, frequency: f64, theta: f64, phi: f64) -> Complex<f64> {
        patch_gain(self.length, self.width, frequency, theta, phi)
    }

    fn set_weight(&mut self, weight: Complex<f64>) {
        self.weight = weight;
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

/// This object represents an array of elements
///
/// Antenna arrays take many shapes, this can handle all of them as long as
/// each element satisfies the ElementIface trait.
#[derive(new)]
pub struct ElementArray {
    elements: Vec<Box<dyn ElementIface>>,
}

impl ArrayIface for ElementArray {
    fn get_gain(&self, frequency: f64, phi: f64, theta: f64) -> Complex<f64> {
        let gains: Vec<Complex<f64>> = self
            .elements
            .iter()
            .map(|n| n.get_gain(frequency, phi, theta))
            .collect();
        gains.iter().sum()
    }
}
