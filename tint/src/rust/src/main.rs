use std::env;
mod nm2rgb;

fn help() {
    println!("usage: tint <wavelength> ");
    println!("or usage: tint <numerator> <denominator> ");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        // no arguments passed
        1 => {
            help();
            println!("Try passing a argument.");
        },
        // one argument passed
        2 => {
            let wavelength = args[1].parse::<f32>().unwrap();
            nm2rgb::nm2rgb(wavelength);
        },
         // two arguments passed
        3 => {
            let n = args[1].parse::<f32>().unwrap();
            let d = args[2].parse::<f32>().unwrap();
            // wavelength=$(dc -e "7k 1 $d / $n * 2.0 - _400 * p")
            let wavelength = (((1.0/d) * n) - 2.0) * -400.0;
            //println!("n:{} d:{} wavelength:{}", n,d,wavelength);   
            nm2rgb::nm2rgb(wavelength);
        }
        // all the other cases
        _ => {
            // show a help message
            help();
        }
    }
}
