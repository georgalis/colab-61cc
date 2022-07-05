use std::env;
mod nm2rgb;

fn help() {
    println!("usage: tint <num> ");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        // no arguments passed
        1 => {
            println!("Try passing a argument.");
        },
        // one argument passed
        2 => {            
            let wavelength = args[1].parse::<f32>().unwrap();
            nm2rgb::nm2rgb(wavelength);
        },
        // all the other cases
        _ => {
            // show a help message
            help();
        }
    }
}
