//
//  main.swift
//  tint
//

//import Foundation
var gamma = 0.62
//print("Start")
let arguments = CommandLine.arguments 
func help() {
    print("usage: tint -w <wavelength> ");
    print("   or: tint -n <numerator> -d <denominator> ");
}

switch arguments.count {
    case 1:
        help()
    case 2:
        help()
    default:
        arguments
}

for argument in arguments {
    switch argument {
        case "-w":
            let i: Int = arguments.firstIndex(of: "-w")!
            let wavelength = arguments[(i+1)]
            //print("wavelength:" + wavelength)
            if let wd = Double(wavelength) {
                //print("Float value = \(f)")
                if arguments.contains("-g") {
                    let ig: Int = arguments.firstIndex(of: "-g")!
                    gamma = Double(arguments[(ig+1)])!
                }
                print(nm2rgb(wavelength:wd, gamma:gamma).joined(separator: " "))
            } else {
                print("-w is not followed by a number!")
            }
        case "-n":
            if arguments.contains("-d") {
                calcWavelength()
            } else {
                print("-d required for -n")
            }
        case "-d":
            if arguments.contains("-n") {
                continue
            } else {
                print("-n required for -d")
            }
        default:
            continue
            //print(CommandLine.arguments)
            //let indexOfArg = CommandLine.arguments.lastIndex(of: argument)
            //print(indexOfArg)
    }
}
