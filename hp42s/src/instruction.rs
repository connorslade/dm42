macro_rules! instruction {
    ($(
        $(#[$attr:meta])*
        $name:ident => $($code:expr),*
    );*) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Instruction {
            $(
                $(#[$attr])*
                $name
            ),*
        }

        impl Instruction {
            pub const ALL: &'static [Instruction] = &[
                $(Instruction::$name),*
            ];

            pub fn to_byte(&self) -> &[u8] {
                match *self {
                    $(Instruction::$name => &[$($code),*],)*
                }
            }
        }
    };
}

instruction! {
    /// Clears the X register.
    ClearX   => 0x77;
    /// Duplicates the X register.
    Enter    => 0x83;
    /// Swaps the X and Y registers.
    SwapXY   => 0x71;
    /// Rotates the X, Y, and Z registers.
    RollDown => 0x75;
    /// Flips the sign of the X register.
    Negate   => 0x54;
    /// Divides Y by X.
    Divide   => 0x43;
    /// Multiplies X and Y.
    Multiply => 0x42;
    /// Subtracts X from Y.
    Subtract => 0x41;
    /// Adds X and Y.
    Add      => 0x40;
    /// Pushes the last value of X used in a calculation onto the stack.
    LastX    => 0x76;
    /// Sine.
    Sin      => 0x59;
    /// Cosine.
    Cos      => 0x5A;
    /// Tangent.
    Tan      => 0x5B;
    /// Arcsine.
    Asin     => 0x5C;
    /// Arccosine.
    Acos     => 0x5D;
    /// Arctangent.
    Atan     => 0x5E;
    /// Logarithm base 10.
    Log      => 0x56;
    /// Calculates 10 to the power of X.
    TenX     => 0x57;
    /// Natural logarithm.
    Ln       => 0x50;
    /// Calculates e to the power of X.
    Ex       => 0x55;
    /// Square root.
    Sqrt     => 0x52;
    /// Square.
    Square   => 0x51;
    /// Reciprocal.
    Recip    => 0x60;
    /// Y to the power of X.
    Exp      => 0x53;
    /// Calculates X percent of Y.
    Percent  => 0x4C;
    /// Pushes pi onto the stack.
    Pi       => 0x72;
    /// Creates a complex number from X and Y.
    /// The real part is X and the imaginary part is Y.
    Complex  => 0xA0, 0x72;
    // TODO: What is this?
    All      => 0xA2, 0x5D;
    // TODO: What is this?
    Null     => 0x00;
    /// Clears the alpha register.
    ClAlpha  => 0x87;
    /// Selects degrees as the angle mode.
    Deg      => 0x80;
    /// Selects radians as the angle mode.
    Rad      => 0x81;
    /// Selects gradian as the angle mode.
    Grad     => 0x82;
    /// Displays complex numbers in rectangular form.
    Rect     => 0xA2, 0x5A;
    /// Displays complex numbers in polar form.
    Polar    => 0xA2, 0x59;
    /// Clears the X register.
    ClX      => 0x74;
    // TODO: What is this?
    RealRes	 =>	0xA2, 0x6B;
    /// Enters into key assignment mode for the custom menu.
    KeyAsn   => 0xA2, 0x63;
    /// Enables local label mode.
    LcLbl    => 0xA2, 0x64;
    /// Selects using periods as the decimal point.
    RdxDot   => 0xA2, 0x5B;
    /// Selects using commas as the decimal point.
    RdxComma => 0xA2, 0x5C;
    /// Clears the statistics registers.
    ClStat   => 0x70;
    /// Clears the whole stack.
    ClStack  => 0x73;
    /// Clears all the numbered registers.
    ClReg    => 0x8A;
    /// Clears all custom menu assignments.
    ClKeys   => 0xA2, 0x62;
    /// Clears the screen.
    ClLcd    => 0xA7, 0x63;
    /// Clears all button assignments in the programmable menu.
    ClMenu   => 0xA2, 0x6D;
    /// Converts a number from radians to degrees.
    ToDeg    => 0x6B;
    /// Converts a number from degrees to radians.
    ToRad    => 0x6A;
    /// Converts a number from minutes-seconds format to decimal hours.
    ToHr     => 0x6D;
    /// Converts a number from decimal hours to minutes-seconds format.
    ToHms    => 0x6C;
    /// Converts a number from polar to rectangular form.
    ToRec    => 0x4E;
    /// Converts a number from rectangular to polar form.
    ToPol    => 0x4F;
    /// Gets the integer part of a number.
    IntPart  => 0x68;
    /// Gets the fractional part of a number.
    FracPart => 0x69;
    /// Rounds the X register to the current number of decimal places.
    Round	 =>	0x6E;
    /// Gets the sign of a number.
    /// Returns 1 if the number is positive, -1 if it is negative, and 0 if it is zero.
    Sign     => 0x7A;
    /// Returns the remainder of Y divided by X.
    Mod      => 0x4B;
    /// Combinations of y items taken x at a time.
    /// Y! / (X! * (Y — X)!)
    Comb     => 0xA0, 0x6F;
    /// Permutations of y items taken x at a time.
    /// Y! / (Y — X)!
    Perm     => 0xA0, 0x70;
    /// Calculates the factorial of X.
    Fact     => 0x62;
    /// Calculates the gamma function of X.
    Gamma    => 0xA0, 0x74;
    ///
    Random   => 0xA0, 0x71;
    Seed     => 0xA0, 0x73;
    Rtn      => 0x85;
    Aview    => 0x7E;
    Prompt   => 0x8E;
    Pse      => 0x89;
    Aip      => 0xA6, 0x31;
    Xtoa     => 0xA6, 0x6F;
    Agraph   => 0xA7, 0x64;
    Pixel    => 0xA7, 0x65;
    Beep     => 0x86;
    Getkey   => 0xA2, 0x6E;
    Menu     => 0xA2, 0x5E
}
