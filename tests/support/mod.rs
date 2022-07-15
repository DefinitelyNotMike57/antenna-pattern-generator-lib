use antenna_pattern_generator_lib::GainIface;
use antenna_pattern_generator_lib::PI;
#[cfg(feature = "blosc")]
use hdf5::filters::blosc_set_nthreads;
use hdf5::{File, Result};
use ndarray::{Array2, Axis};

// Will use then when H5 supports writing complex numbers
// use num::complex::Complex;

/// Utility for writing array patterns to a file
///
/// TODO: change this to write to H5
pub fn write_to_file(
    array: Box<dyn GainIface>,
    frequency: f64,
    theta_spacing: f64,
    phi_spacing: f64,
    file_name: String,
) -> Result<()> {
    let num_theta_samples: usize = (PI / theta_spacing) as usize;
    let num_phi_samples: usize = (2.0 * PI / phi_spacing) as usize;
    println!("{} {}", num_theta_samples, num_phi_samples);

    let mut arr = Array2::zeros((num_phi_samples, num_theta_samples));

    for (phi_idx, mut row) in arr.axis_iter_mut(Axis(0)).enumerate() {
        let phi = phi_idx as f64 * phi_spacing;
        for theta_idx in 0..num_theta_samples {
            let theta = theta_idx as f64 * theta_spacing;
            row[theta_idx] = array.get_gain(frequency, theta, phi).unwrap().norm();
        }
    }

    let file = File::create(file_name)?;
    let group = file.create_group("dir")?;
    #[cfg(feature = "blosc")]
    blosc_set_nthreads(2);
    let builder = group.new_dataset_builder();
    #[cfg(feature = "blosc")]
    let builder = builder.blosc_zstd(9, true);
    let _ds = builder.with_data(&arr).create("gain")?;

    Ok(())
}
