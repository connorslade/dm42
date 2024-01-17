# Programs

> [!NOTE]  
> These programs are written for the free42 emulator and as such may or may not run on the original hp 42s.

> [!WARNING]
> I haven't updated this document with many of my newer programs.
> Ill get to it eventually.
> Maybe.

There are two file formats used for programs in this folder, `.dm42` and `.free42`.
Free42 files are raw program listings that can be loaded with free42 directly, while the Dm42 files must be run through my transpiler / preprocessor first.
In either case, the `.raw` files for each program will be linked in their respective section.
Now on to the programs! (In order of how cool I think they are)

- [Statistics](#statistics) &mdash; Various statistics functions for use on N×1 matrices
- [Fractions](#fractions) &mdash; Operations for working with exact fractions
- [Unit converter](#unit-converter) &mdash; A simple unit convertor
- [Boombox](#boombox) &mdash; Collection of songs you can play through the buzzer
- [Keyboard](#keyboard) &mdash; Play music using your calculator's built in buzzer
- [Physical Constants](#physical-constants) &mdash; Lets you push various physical constants onto the stack
- [Miscellaneous Functions](miscellaneous-functions) &mdash; Miscellaneous mathematical functions missing from the 42s
- [Matrix Sorter](#matrix-sorter) &mdash; Sorts a matrix column, used in the statistics program

## Statistics

![Screenshot](https://github.com/Basicprogrammer10/dm42/assets/50306817/1ccf0d30-071e-4e88-9e2a-8d9a21383bd1)

[[download .raw](https://connorcode.com/files/Misc/dm42/statistics.raw)]
[[source code](statistics.free42)]

Setting flag `05` marks that the data is of a population rather than a sample.
This is currently just used in the standard deviation calculation.

To use this program start by creating an N×1 matrix (1, 1, MATRIX>NEW) and adding your data (MATRIX>EDIT).
The run the "STAT" program (XEQ>STAT), this will open a two page menu with the following options, when run, each function will push its result to the stack and allow you to run another.

- Mean (MEAN)
- Median (MEAD)
- Standard Deviation (STD)
- Inter-Quartile Range (IRQ)
- 5-Var Summery (SUMMR)
- Range (RANGE)
- Outlier Fences (OUTLR)
- Sum (SUM)
- Square Sum (SUM↑2) &mdash; Sums the square of each datapoint
- Toggle Sample (TSAM) &mdash; Toggles flag `05`
- Min (MIN)
- Max (MAX)
- Q1 (Q1)
- Q3 (Q3)

## Fractions

![Screenshot](https://github.com/Basicprogrammer10/dm42/assets/50306817/159d25b5-fc40-45bd-96e1-fbfabcb16856)

[[download .raw](https://connorcode.com/files/Misc/dm42/fractions.raw)]
[[source code](fractions.dm42)]

Lets you work with fractions!
Flag `03` will open the custom menu on exit.
Fractions are stored as rectangular complex numbers with the numerators in the real part and the denominator in the imaginary part.
The program exposes some operators that work with these fractions through custom menus.
Listed below are all of the functions:

- Frac &mdash; Makes a fraction out of $\frac{y}{x}$. Automatically simplifies.
- Addition
- Subtraction
- Multiplication
- Division
- Decimal Conversion &mdash; Replaces the fraction on the stack with its decimal representation.
- Simplification &mdash; Simplifies the fraction. This is preformed automatically after every operation.
- Reciprocal &mdash; Swaps the numerator and denominator.
- Flip Signs &mdash; Flips the sign of the numerator, the denominator should never be negative.
- Preview &mdash; Shows the decimal representation of the fraction without consuming it.

## Unit converter

![Screenshot](https://github.com/Basicprogrammer10/dm42/assets/50306817/63c33a17-5a51-4230-8c15-e49a303e5bdb)

[[download .raw](https://connorcode.com/files/Misc/dm42/convert.raw)]
[[source code](convert.dm42)]

Simple unit converter that just supports converting between predefined unit dimensions (i.e. it does no dimensional analysis, just uses Unit ⇒ Base conversion factors).

Setting flag `03` will open the custom menu when you exit the program.
I use this as I like always having my custom menu open :3.
Setting flag `02` will keep the value you are converting from on the stack, otherwise it will be dropped (although converting back to the unit you are converting from will bring it back).

To use, have the value you want to convert in the X reg, run the program, select the unit dimension you want to convert within, select the unit the value is in, then select the unit you want to convert into.
Clicking multiple units at this point will continue converting the original value into new units.
To convert a new value exit the menu and re-select a dimension and select a new from unit.

Currently the units it supports are as follows:

- **Length**: Meter, Inch, Foot, Yard, Mile, Nautical Mile
- **Mass**: Kilogram, Pound, Tonne
- **Speed**: Meters per second, Kilometers per hour, Miles per hour, Knot
- **Acceleration**: Meters per second squared, Gravity
- **Force**: Newton, Pound force
- **Energy**: Joule, Calorie, Kilowatt hour, Electronvolt
- **Area**: Square meter, Square inch, Square foot, Square yard, Square mile
- **Volume**: Cubic meter, Cubic inch, Cubic foot
- **Time**: Second, Minute, Hour, Day, Julian year
- **Bytes**: Byte, Kilobyte, Megabyte, Gigabyte, Terabyte, Petabyte
- **Pressure**: Pascal, Bar, Atmosphere, Pounds per square inch
- **Power**: Watt, Horsepower

## Boombox

![Screenshot](https://github.com/Basicprogrammer10/dm42/assets/50306817/acf4b74b-888e-44f2-9f78-aaa1f450e907)

[[download .raw](https://connorcode.com/files/Misc/dm42/boombox.raw)]
[[source code](boombox.dm42)]

Uses the internal buzzer to play music!
Setting the `03` flag will open the custom menu on exit.
Currently there are seven songs:

- Say So &ndash; Doja Cat
- Hips Dont Lie &ndash; Shakira
- Died in Your Arms &ndash; Cutting Crew
- WII Channel Music
- A Thousand Miles &ndash; Vanessa Carlton
- Super Mario Bros Theme
- Take On Me &ndash;
- Barbie Girl &ndash; Aqua

## Keyboard

[[download .raw](https://connorcode.com/files/Misc/dm42/keyboard.raw)]
[[source code](keyboard.dm42)]

Lets you play tones with the keypad.
The first ten keys are used with increasing pitch.
$\{\sum+, \frac{1}{x}, \sqrt{x}, \log, \ln, \text{xeq}, \text{sto}, \text{rcl}, \text{R}\downarrow, \sin\}$
The calculator only gives access to these ten tones, each with a length of a quarter second.

## Physical Constants

![Screenshot](https://github.com/Basicprogrammer10/dm42/assets/50306817/3fab1fec-aec5-4330-ba6d-a73329bcd7df)

[[download .raw](https://connorcode.com/files/Misc/dm42/constants.raw)]
[[source code](constant.dm42)]

Simple program to push some constants to the stack.
Setting flag `03` will open the custom menu when you exit the program.
It contains the following constants:

- Light speed
- Planck constant
- Elementary charge
- Avogadro constant
- Cesium hyperfine transition
- Boltzmann constant
- Luminous efficacy

From [Meet the Constants &ndash; NIST](https://www.nist.gov/si-redefinition/meet-constants)

## Miscellaneous Functions

![Screenshot](https://github.com/Basicprogrammer10/dm42/assets/50306817/02398dbe-b400-489d-be61-e879f026d59f)

[[download .raw](https://connorcode.com/files/Misc/dm42/math.raw)]
[[source code](math.dm42)]

Adds various functions missing from the 42s.
Exits to the custom menu when flag `03` is set.
It contains the following functions:

- Greatest Common Factor
- Least Common Multiple
- $\log_y(x)$
- $\sqrt[x]{y}$
- Hypotenuse $\sqrt{x^2 + y^2}$

## Matrix Sorter

![Screenshot](https://github.com/Basicprogrammer10/dm42/assets/50306817/70a4ac2b-5d5e-480a-9675-89575144f016)

[[download .raw](https://connorcode.com/files/Misc/dm42/sort.raw)]
[[source code](sort.dm42)]

This program was originally written for my statistics program (its bundled in the download) but I rewrote it while working on my transpiler / preprocessor.
The only differences are that it throws a custom error if there is not a matrix on X and its slightly less efficient.
It just sorts the first column of an N×1 matrix.
Thats all.
