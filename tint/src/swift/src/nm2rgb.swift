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

import Foundation

func calcWavelength() {
    let ni: Int = arguments.firstIndex(of: "-n")!
    let n = Double(arguments[(ni+1)])!
    
    let di: Int = arguments.firstIndex(of: "-d")!
    let d = Double(arguments[(di+1)])!
    
    if arguments.contains("-g") {
        let ig: Int = arguments.firstIndex(of: "-g")!
        gamma = Double(arguments[(ig+1)])!
    }
    
    let wavelength = (((1.0/d) * n) - 2.0) * -400.0
    
    print(nm2rgb(wavelength:wavelength, gamma:gamma).joined(separator: " "))
}

func nm2rgb(wavelength: Double, gamma: Double = 0.62) -> Array<String> {
    //print("nm2rgb wavelength:" + String(wavelength) )
    var R = 0.0;
    var G = 0.0;
    var B = 0.0;
    
    if wavelength >= 354.4 && wavelength <= 440.0 {  // vs 380-440
        let attenuation = 0.3 + 0.7 * (wavelength - 380.0) / (440.0 - 380.0);
            //print("attenuation:\(attenuation)");
        let rc = (-(wavelength - 440) / (440 - 380)) * attenuation
        R = pow(Double(rc),gamma)
        B = pow((1.0 * attenuation), gamma);
    } else if wavelength >= 440.0 && wavelength <= 490.0 {
        G = pow(((wavelength - 440.0) / (490.0 - 440.0)), gamma);
        B = 1.0;
    } else if wavelength >= 490.0 && wavelength <= 510.0 {
        G = 1.0;
        B = pow((-(wavelength - 510.0) / (510.0 - 490.0)), gamma);
    } else if wavelength >= 510.0 && wavelength <= 580.0 {
        R = pow(((wavelength - 510.0) / (580.0 - 510.0)), gamma);
        G = 1.0;
    } else if wavelength >= 580.0 && wavelength <= 645.0 {
        R = 1.0;
        G = pow((-(wavelength - 645.0) / (645.0 - 580.0)), gamma);
    } else if wavelength >= 645.0 && wavelength <= 794.5 {
        let attenuation = 0.3 + 0.7 * (750.0 - wavelength) / (750.0 - 645.0);
        R = pow((1.0 * attenuation), gamma);
    }
    
    R=round(R*255)
    G=round(G*255)
    B=round(B*255)
    //print("\(Int(R)) \(Int(G)) \(Int(B)) \(wavelength)")
    return [String(Int(R)), String(Int(G)), String(Int(B)), String(wavelength)]
}
