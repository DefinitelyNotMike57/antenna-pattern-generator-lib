use antenna_pattern_generator_lib as apg;

use num::complex::Complex;

mod support;
use support::write_to_file;

#[test]
fn element_spacing() {
    let wavelength = apg::SPEED_OF_LIGHT / 1e9;

    let e0 = Box::new(apg::OmniElementBuilder::default().position(apg::PointBuilder::default().build().unwrap())
        .gain(1.0)
        .build()
        .unwrap());
    let e1 =
        Box::new(apg::OmniElementBuilder::default().position(apg::PointBuilder::default().x(wavelength / 2.0).build().unwrap())
            .gain(1.0)
            .weight(Complex::new(0.0, 1.0))
            .build()
            .unwrap());

    let array = Box::new(apg::ElementArray( vec![e0, e1] ) );

    write_to_file(
        array,
        1e9,
        0.5 * apg::PI / 180.0,
        1.0 * apg::PI / 180.0,
        "tests/output/two_element.h5".to_string(),
    );

}
