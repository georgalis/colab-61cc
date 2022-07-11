#!/usr/bin/env python

'''
    == A few notes about color ==

    Color   Wavelength(nm) Frequency(THz)
    Red     620-750        484-400
    Orange  590-620        508-484
    Yellow  570-590        526-508
    Green   495-570        606-526
    Blue    450-495        668-606
    Violet  380-450        789-668

    f is frequency (cycles per second)
    l (lambda) is wavelength (meters per cycle)
    e is energy (Joules)
    h (Plank's constant) = 6.6260695729 x 10^-34 Joule*seconds
                         = 6.6260695729 x 10^-34 m^2*kg/seconds
    c = 299792458 meters per second
    f = c/l
    l = c/f
    e = h*f
    e = c*h/l

    List of peak frequency responses for each type of 
    photoreceptor cell in the human eye:
        S cone: 437 nm
        M cone: 533 nm
        L cone: 564 nm
        rod:    550 nm in bright daylight, 498 nm when dark adapted. 
                Rods adapt to low light conditions by becoming more sensitive.
                Peak frequency response shifts to 498 nm.

'''

import sys
#import os
#import traceback
#import optparse
#import time
#import logging

# default gamma=0.8, I see more colors with 0.62
def nm2rgb(wavelength, gamma=0.62):

    '''This converts a given wavelength of light to an 
    approximate RGB color value. The wavelength must be given
    in nanometers in the range from 380 nm through 750 nm
    (789 THz through 400 THz).

    Based on code by Dan Bruton
    http://www.physics.sfasu.edu/astro/color/spectra.html
    and implementation by Noah Spurrier
    http://www.noah.org/wiki/Wavelength_to_RGB_in_Python
    http://www.noah.org/wiki/canvas.py
    '''

# useable 350 - 795, typical 400-800
    wavelength = float(wavelength)
    if wavelength >= 354.4 and wavelength <= 440: # vs 380-440
        attenuation = 0.3 + 0.7 * (wavelength - 380) / (440 - 380)
        r = ((-(wavelength - 440) / (440 - 380)) * attenuation) ** gamma
        G = 0.0
        B = (1.0 * attenuation) ** gamma
    elif wavelength >= 440 and wavelength <= 490:
        r = 0.0
        G = ((wavelength - 440) / (490 - 440)) ** gamma
        B = 1.0
    elif wavelength >= 490 and wavelength <= 510:
        r = 0.0
        G = 1.0
        B = (-(wavelength - 510) / (510 - 490)) ** gamma
    elif wavelength >= 510 and wavelength <= 580:
        r = ((wavelength - 510) / (580 - 510)) ** gamma
        G = 1.0
        B = 0.0
    elif wavelength >= 580 and wavelength <= 645:
        r = 1.0
        G = (-(wavelength - 645) / (645 - 580)) ** gamma
        B = 0.0
    elif wavelength >= 645 and wavelength <= 794.5 : # vs 620-750
        attenuation = 0.3 + 0.7 * (750 - wavelength) / (750 - 645)
        r = (1.0 * attenuation) ** gamma
        G = 0.0
        B = 0.0
    else:
        r = 0.0
        G = 0.0
        B = 0.0
    R *= 255
    G *= 255
    B *= 255
    print ("{R} {G} {B} {w}".format(R=int(R), G=int(G), B=int(B), w=float(wavelength)))
#   print ("{R:02x}{G:02x}{B:02x}".format(R=int(R), G=int(G), B=int(B)))
    return

#nm2rgb(float(sys.argv[1]),float(sys.argv[2]))
nm2rgb(float(sys.argv[1]))

# generate a testing canvas
#def main(options=None, args=None):
#
##    import ppm_dump
##    import png_canvas
#    import canvas
#   if options.ppm:
#       canvas = canvas.ppm_canvas(371, 278)
#       canvas.is_ascii = True
#   else:
#       canvas = canvas.png_canvas(371, 278)
#   for wl in range(380, 751):
#       r, g, b = wavelength_to_rgb(wl)
#       for yy in range(0, 278):
#           canvas.pixel(wl - 380, yy, r, g, b)
#   sys.stdout.write(str(canvas))
#
#if __name__ == '__main__':
#    try:
#        start_time = time.time()
#        parser = optparse.OptionParser(
#            formatter=optparse.TitledHelpFormatter(),
#            usage=globals()['__doc__'],
#            version='1'
#        )
#        parser.add_option(
#            '-v', '--verbose', action='store_true',
#            default=False, help='verbose output'
#        )
#        parser.add_option(
#            '--png', action='store_true',
#            default=True, help='Output as PNG.'
#        )
#        parser.add_option(
#            '--ppm', action='store_true',
#            default=False, help='Output as PPM ASCII (Portable Pixmap).'
#        )
#        (options, args) = parser.parse_args()
#        #if len(args) < 1:
#        #    parser.error ('missing argument')
#        if options.verbose:
#            print(time.asctime())
#        exit_code = main(options, args)
#        if exit_code is None:
#            exit_code = 0
#        if options.verbose:
#            print(time.asctime())
#            print('TOTAL TIME IN MINUTES: %f'
#                  % ((time.time() - start_time) / 60.0))
#        sys.exit(exit_code)
#    except KeyboardInterrupt as e:  # The user pressed Ctrl-C.
#        raise e
#    except SystemExit as e:  # The script called sys.exit() somewhere.
#        raise e
#    except Exception as e:
#        print('ERROR: Unexpected Exception')
#        print(str(e))
#        traceback.print_exc()
#        os._exit(2)
