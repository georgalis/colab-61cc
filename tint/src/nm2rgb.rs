// mod nm2rgb;
use std::collections::HashMap;

/*
This converts a given wavelength of light to an 
approximate RGB color value. The wavelength must be given
in nanometers in the range from 380 nm through 750 nm
(789 THz through 400 THz).

Based on code by Dan Bruton
http://www.physics.sfasu.edu/astro/color/spectra.html
and implementation by Noah Spurrier
http://www.noah.org/wiki/Wavelength_to_RGB_in_Python
http://www.noah.org/wiki/canvas.py
*/

pub fn nm2rgb(rgb: &mut HashMap<&str, f32>, wavelength: f32) {
    //nm2rgb::nm2rgb(wavelength);
    let gamma=0.62;
    let mut R = 0.0;
    let mut G = 0.0;
    let mut B = 0.0;

    if wavelength >= 354.4 && wavelength <= 440.0 {
        let attenuation = 0.3 + 0.7 * (wavelength - 380.0) / (440.0 - 380.0);
        R = f32::powf(((-(wavelength - 440.0) / (440.0 - 380.0)) * attenuation), gamma);
        B = f32::powf((1.0 * attenuation), gamma);
    } else if wavelength >= 440.0 && wavelength <= 490.0 {
        G = f32::powf(((wavelength - 440.0) / (490.0 - 440.0)), gamma);
        B = 1.0;
    } else if wavelength >= 490.0 && wavelength <= 510.0 {
        G = 1.0;
        B = f32::powf((-(wavelength - 510.0) / (510.0 - 490.0)), gamma);
    } else if wavelength >= 510.0 && wavelength <= 580.0 {
        R = f32::powf(((wavelength - 510.0) / (580.0 - 510.0)), gamma);
        G = 1.0;
    } else if wavelength >= 580.0 && wavelength <= 645.0 {
        R = 1.0;
        G = f32::powf((-(wavelength - 645.0) / (645.0 - 580.0)), gamma);
    } else if wavelength >= 645.0 && wavelength <= 794.5 {
        let attenuation = 0.3 + 0.7 * (750.0 - wavelength) / (750.0 - 645.0);
        R = f32::powf((1.0 * attenuation), gamma);
    }

    R=(R*255.0).round();
    G=(G*255.0).round();
    B=(B*255.0).round();

}

