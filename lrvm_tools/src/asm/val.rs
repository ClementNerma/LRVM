use super::Reg;

macro_rules! declare_val {
    ($typename: ident, $num: ident, $inum: ident) => {
        /// Strongly-typed assembly value
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
        pub enum $typename {
            Reg(Reg),
            Lit($num),
        }

        impl $typename {
            /// Create a register value
            pub fn reg(reg: Reg) -> Self {
                Self::Reg(reg)
            }

            /// Create an (unsigned) literal value
            pub fn lit(lit: $num) -> Self {
                Self::Lit(lit)
            }

            /// Create a signed literal value
            pub fn signed_lit(lit: $inum) -> Self {
                Self::Lit(lit as $num)
            }

            /// Check if the value is a register
            pub fn is_reg(self) -> bool {
                match self {
                    Self::Reg(_) => true,
                    Self::Lit(_) => false,
                }
            }

            // Check if the value is a literal
            pub fn is_lit(self) -> bool {
                match self {
                    Self::Reg(_) => false,
                    Self::Lit(_) => true,
                }
            }

            // Get the value as a register code or as a literal (depending on its type)
            pub fn value(self) -> $num {
                match self {
                    Self::Reg(reg) => reg.code().into(),
                    Self::Lit(num) => num,
                }
            }

            /// Convert the value to its LASM representation
            pub fn to_lasm(self) -> String {
                match self {
                    Self::Reg(reg) => reg.name().to_string(),
                    Self::Lit(num) => format!("{:#X}", num),
                }
            }

            /// Convert the value to its LASM representation
            /// Represent literals as signed numbers
            pub fn to_lasm_signed(self) -> String {
                match self {
                    Self::Reg(reg) => reg.name().to_string(),
                    Self::Lit(num) => match num as $inum {
                        num @ $inum::MIN..=-1 => format!("-{:#X}", -num),
                        num @ 0..=$inum::MAX => format!("{:#X}", num),
                    },
                }
            }

            /// Convert the value to its LASM representation, using a custom formatter for literals
            pub fn to_lasm_with(self, formatter: impl FnOnce($num) -> String) -> String {
                match self {
                    Self::Reg(reg) => reg.name().to_string(),
                    Self::Lit(num) => formatter(num),
                }
            }
        }

        impl From<$typename> for $num {
            fn from(reg_or_lit: $typename) -> $num {
                reg_or_lit.value()
            }
        }

        impl From<Reg> for $typename {
            fn from(reg: Reg) -> Self {
                Self::reg(reg)
            }
        }

        impl From<$num> for $typename {
            fn from(lit: $num) -> Self {
                Self::lit(lit)
            }
        }

        impl From<$inum> for $typename {
            fn from(lit: $inum) -> Self {
                Self::signed_lit(lit)
            }
        }
    };
}

declare_val!(RegOrLit1, u8, i8);
declare_val!(RegOrLit2, u16, i16);

impl From<u8> for RegOrLit2 {
    fn from(lit: u8) -> Self {
        Self::lit(lit.into())
    }
}

impl From<i8> for RegOrLit2 {
    fn from(lit: i8) -> Self {
        Self::signed_lit(lit.into())
    }
}
