# Programs

> [!NOTE]  
> These programs are written for the free42 emulator and as such may or may not run on the original hp 42s.

There are two file formats used for programs in this folder, `.dm42` and `.free42`.
Free42 files are raw program listings that can be loaded with free42 directly, while the Dm42 files must be run through my transpiler / preprocessor first.
In either case, the `.raw` files for each program will be linked in their respective section.

- [Statistics](#statistics) &mdash; Various statistics functions for use on N×1 matrices
- [Unit converter](#unit-converter) &mdash; A simple unit convertor
- [Physical Constants](#physical-constants) &mdash; Lets you push various physical constants onto the stack
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

## Physical Constants

\<todo>

## Matrix Sorter

\<todo>
