# Tint

This python and shell code use colors to indicate progress on output listings. It is rather slow
and a lot more useful ported to Clang.

The python program nm2rgb.py converts light wavelengths (approx 400nm to 800nm) into RGB colors for display.
In `fn.bash`, the `color_dark` function accepts a numerator (arg1), and denominator (arg2) of a number between
0 and 1 and converts it to terminal color control codes for a dimmed background indicating progress:

* red: beginning
* green: middle
* blue: end

If you have 150 items to list `color_dark $n 150` will set the nth tint of 150 color pallet.
Red has a longer wavelength than blue but the orientation is reversed for aesthetics. The
`color_dark` function also darkens the background colors to work well with light text.

Some shell script to generate this example is in `fn.bash` too, it takes a whole second
to write out 29 colored lines... (!)

<img src="29cps.png" width="30%">

A translation to C for the `color_dark` function and the `nm2rgb.py` python could
make the colors useful, anytime you want to display progress while printing output...


