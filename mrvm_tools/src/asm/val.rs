use super::Reg;

macro_rules! declare_val {
    ($typename: ident, $num: ident, $inum: ident) => {
        /// Strongly-typed assembly value
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum $typename {
            Reg(Reg),
            Lit($num)
        }

        impl $typename {
            /// Create a register value
            pub fn reg(reg: Reg) -> Self {
                Self::Reg(reg)
            }

            /// Create an (unsigned) literal value
            pub fn cst(cst: $num) -> Self {
                Self::Lit(cst)
            }

            /// Create a signed literal value
            pub fn signed_cst(cst: $inum) -> Self {
                Self::Lit(cst as $num)
            }

            /// Check if the value is a register
            pub fn is_reg(&self) -> bool {
                match self {
                    Self::Reg(_) => true,
                    Self::Lit(_) => false
                }
            }

            // Check if the value is a literal
            pub fn is_cst(&self) -> bool {
                match self {
                    Self::Reg(_) => false,
                    Self::Lit(_) => true
                }
            }

            // Get the value as a register code or as a literal (depending on its type)
            pub fn value(&self) -> $num {
                match self {
                    Self::Reg(reg) => reg.code().into(),
                    Self::Lit(num) => *num
                }
            }
        }

        impl Into<$num> for $typename {
            fn into(self) -> $num {
                self.value()
            }
        }

        impl From<Reg> for $typename {
            fn from(reg: Reg) -> Self {
                Self::reg(reg)
            }
        }

        impl From<$num> for $typename {
            fn from(cst: $num) -> Self {
                Self::cst(cst)
            }
        }

        impl From<$inum> for $typename {
            fn from(cst: $inum) -> Self {
                Self::signed_cst(cst)
            }
        }
    }
}

declare_val!(RegOrLit1, u8, i8);
declare_val!(RegOrLit2, u16, i16);

impl From<u8> for RegOrLit2 {
    fn from(cst: u8) -> Self {
        Self::cst(cst.into())
    }
}

impl From<i8> for RegOrLit2 {
    fn from(cst: i8) -> Self {
        Self::signed_cst(cst.into())
    }
}
