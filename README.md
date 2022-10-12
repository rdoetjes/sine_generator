*Quick and dirty sine table generator of asm projects*

Creates a sine list botn on screen and in a file that you can import in your 68000 asm

Usage: sine_generator <label_name> <nr_of_steps> <amplitude_of_sine>
-label_name is the label for the table and is also used as the filename
-nr_of_steps is the number of steps that needs to be in this 360 degree sine wave
-amplitude_of_sine is the amplitude of the sine wave. F.i. 10 would give a sine wave with values -10 to 10
It will create a file file called <label_name>.s