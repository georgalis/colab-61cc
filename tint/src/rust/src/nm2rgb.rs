// mod nm2rgb;

/*
This converts a given wavelength of light to an 
approximate rGB color value. The wavelength must be given
in nanometers in the range from 380 nm through 750 nm
(789 THz through 400 THz).

Based on code by Dan Bruton
http://www.physics.sfasu.edu/astro/color/spectra.html
and implementation by Noah Spurrier
http://www.noah.org/wiki/Wavelength_to_rGB_in_Python
http://www.noah.org/wiki/canvas.py
*/

pub fn nm2rgb(wavelength: f32) {
    //nm2rgb::nm2rgb(wavelength);
    let gamma = 0.62;
    let mut r = 0.0;
    let mut g = 0.0;
    let mut b = 0.0;

    if wavelength >= 354.4 && wavelength <= 440.0 {
        let attenuation = 0.3 + 0.7 * (wavelength - 380.0) / (440.0 - 380.0);
        r = f32::powf((-(wavelength - 440.0) / (440.0 - 380.0)) * attenuation, gamma);
        b = f32::powf(1.0 * attenuation, gamma);
    } else if wavelength >= 440.0 && wavelength <= 490.0 {
        g = f32::powf((wavelength - 440.0) / (490.0 - 440.0), gamma);
        b = 1.0;
    } else if wavelength >= 490.0 && wavelength <= 510.0 {
        g = 1.0;
        b = f32::powf(-(wavelength - 510.0) / (510.0 - 490.0), gamma);
    } else if wavelength >= 510.0 && wavelength <= 580.0 {
        r = f32::powf((wavelength - 510.0) / (580.0 - 510.0), gamma);
        g = 1.0;
    } else if wavelength >= 580.0 && wavelength <= 645.0 {
        r = 1.0;
        g = f32::powf(-(wavelength - 645.0) / (645.0 - 580.0), gamma);
    } else if wavelength >= 645.0 && wavelength <= 794.5 {
        let attenuation = 0.3 + 0.7 * (750.0 - wavelength) / (750.0 - 645.0);
        r = f32::powf(1.0 * attenuation, gamma);
    }

    r=(r*255.0).round();
    g=(g*255.0).round();
    b=(b*255.0).round();

    println!("{} {} {} {}", r,g,b,wavelength);
}

