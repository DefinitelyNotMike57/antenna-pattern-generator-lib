use antenna_pattern_generator_lib as apg;
use antenna_pattern_generator_lib::ElementIface;

use num::complex::Complex;

mod support;
use support::write_to_file;

#[test]
fn element_spacing() {
    let wavelength = apg::SPEED_OF_LIGHT / 1e9;

    let mut e0 = Box::new(apg::OmniElement::new(
        apg::Point::new(0.0, 0.0, 0.0),
        1.0,
        Complex::new(1.0, 0.0),
    ));
    &e0.set_weight(Complex::new(0.0, apg::PI));

    let e1 = Box::new(apg::OmniElement::new(
        apg::Point::new(wavelength / 2.0, 0.0, 0.0),
        1.0,
        Complex::new(1.0, 0.0),
    ));

    let array = Box::new(apg::ElementArray::new(vec![e0, e1]));

    write_to_file(
        array,
        1e9,
        0.5 * apg::PI / 180.0,
        1.0 * apg::PI / 180.0,
        "tests/output/two_element.h5".to_string(),
    );
}
