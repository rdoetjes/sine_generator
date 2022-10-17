# SINE_GENERATOR

*A quick and dirty sine table generator of asm projects*
I found myself always writing a couple of lines of code from scratch to generate a sine table list for a demo or jump arc, and I figured lets do it ONE LAST TIME and store it so I can reuse this.

This program: creates a sine list both on screen and in a file that you can import in your 68000, 6502 asm
![alt text](https://github.com/rdoetjes/sine_generator/blob/main/image.png?raw=true)

**GUI requirements**
The UI is build with egui and compiles without any additional libraries on Windows and Mac.<br/>
<br/>
On Linux the following libs should be installed.
```shell
sudo apt-get install -y libclang-dev libgtk-3-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libspeechd-dev libxkbcommon-dev libssl-dev
```
On Fedora Rawhide you need to run:
```shell
dnf install clang clang-devel clang-tools-extra speech-dispatcher-devel libxkbcommon-devel pkg-config openssl-devel libxcb-devel
```

**The math**
it's a very straight forward 360 degree (2*pi) sine function.
point = sin(i/nr_steps * (2.0 * pi)) where i is the current iteration going from 0 to nr_steps and nr_steps is the total number of points. This is the devisor for the 2*pi.
So a one period (360 degree) sine is generated.

**output**
As you can see we get a list that is started by our provided label (sine:), then a byte list you tend not to go over 255 in retro assembly (we had 225 lines in most cases) and since this is an offset that you add to the current position you'll be probably fine. Otherwise this code is easily extendable.
And after the sie points we add a sine_end: label so that you can calculate the number of point (bytes) in your assembler by using sine_end-sine.

```asm
sine:
    dc.b 0, 1, 3, 5, 7, 9, 11, 12
    dc.b 14, 16, 17, 19, 20, 21, 23, 24
    dc.b 25, 26, 27, 27, 28, 29, 29, 29
    dc.b 29, 30, 29, 29, 29, 29, 28, 27
    dc.b 27, 26, 25, 24, 23, 21, 20, 19
    dc.b 17, 16, 14, 12, 11, 9, 7, 5
    dc.b 3, 1, 0, -1, -3, -5, -7, -9
    dc.b -11, -12, -14, -16, -17, -19, -20, -21
    dc.b -23, -24, -25, -26, -27, -27, -28, -29
    dc.b -29, -29, -29, -30, -29, -29, -29, -29
    dc.b -28, -27, -27, -26, -25, -24, -23, -21
    dc.b -20, -19, -17, -16, -14, -12, -11, -9
    dc.b -7, -5, -3, -1
sine_end:
```
You can copy the data from the textedit or click on the button which will save the file the asmlabelname.s in the current directory.

**No file error handling**
Because it's quick and dirty there's not real file error handling, everything is unwrapped for quickness sake.