#SINE_GENERATOR

*A quick and dirty sine table generator of asm projects*
I found myself always writing a couple of lines of code from scratch to generate a sine table list for a demo or jump arc, and I figured lets do it ONE LAST TIME and store it so I can reuse this.

This program: creates a sine list both on screen and in a file that you can import in your 68000 asm

Usage: sine_generator <label_name> <nr_of_steps> <amplitude_of_sine>
-label_name is the label for the table and is also used as the filename
-nr_of_steps is the number of steps that needs to be in this 360 degree sine wave
-amplitude_of_sine is the amplitude of the sine wave. F.i. 10 would give a sine wave with values -10 to 10
It will create a file file called <label_name>.s

**The math**
it's a very straight forward 360 degree (2*pi) sine function.
point = sin(i/nr_steps * (2.0 * pi)) where i is the current iteration going from 0 to nr_steps and nr_steps is the total number of points. This is the devisor for the 2*pi.
So a one period (360 degree) sine is generated.
