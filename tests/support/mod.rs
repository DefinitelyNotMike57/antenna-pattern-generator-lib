use ndarray::{array, Array2, Axis};
use std::path::Path;
#[cfg(feature = "blosc")]
use hdf5::filters::blosc_set_nthreads;
use hdf5::{File, Result};
use antenna_pattern_generator_lib::PI;
use antenna_pattern_generator_lib::ArrayIface;
use num::complex::Complex;

/// Utility for writing array patterns to a file
///
/// TODO: change this to write to H5
pub fn write_to_file(
    array: Box<dyn ArrayIface>,
    frequency: f64,
    theta_spacing: f64,
    phi_spacing: f64,
    file_name: String,
) -> Result<()> {
    let num_theta_samples: usize = (PI / theta_spacing) as usize;
    let num_phi_samples: usize = (2.0 * PI / phi_spacing) as usize;
    println!( "{} {}", num_theta_samples, num_phi_samples );

    let path = Path::new(&file_name);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    let mut arr = Array2::zeros((num_phi_samples,num_theta_samples));

    for (phi_idx, mut row) in arr.axis_iter_mut(Axis(0)).enumerate() {
        let phi = phi_idx as f64 * phi_spacing;
        for theta_idx in 0..num_theta_samples {
            let theta = theta_idx as f64 * theta_spacing;
            row[theta_idx] = array.get_gain(frequency, theta, phi).norm();
        }
    }

    let file = File::create( file_name )?;
    let group = file.create_group("dir")?;
    #[cfg(feature = "blosc")]
    blosc_set_nthreads(2);
    let builder = group.new_dataset_builder();
    #[cfg(feature = "blosc")]
    let ds = builder
        .with_data( &arr )
        .create("gain")?;

    Ok(())
}
