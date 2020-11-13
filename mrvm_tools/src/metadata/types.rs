use super::DeviceCategory;
use std::fmt;

macro_rules! impl_device_type {
    ($type_name: ident, as $type_enum: ident => { $($dev_name: ident => $dev_code: expr),* }) => {
        #[non_exhaustive]
        #[derive(Copy, Clone, Debug)]
        pub enum $type_enum {
            $($dev_name),*
        }

        impl $type_enum {
            pub fn decode(code: u32) -> Result<Self, ()> {
                match code {
                    $($dev_code => Ok(Self::$dev_name)),*
                    , _ => Err(())
                }
            }

            pub fn code(self) -> u32 {
                match self {
                    $(Self::$dev_name => $dev_code),*
                }
            }

            pub fn wrap(self) -> DeviceCategory {
                DeviceCategory::$type_name(self)
            }

            pub fn encode(self) -> u64 {
                self.wrap().encode()
            }
        }

        impl Into<DeviceCategory> for $type_enum {
            fn into(self) -> DeviceCategory {
                self.wrap()
            }
        }

        impl fmt::Display for $type_enum {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(
                    f,
                    "{}",
                    match self {
                        $(Self::$dev_name => stringify!($dev_name)),*
                    }
                )
            }
        }
    };
}

impl_device_type!(Debug, as DebugType => {
    Basic => 0x0000_0100
});

impl_device_type!(Clock, as ClockType => {
    Realtime => 0x0000_0001
});

impl_device_type!(Display, as DisplayType => {
    Number    => 0x0000_0001,
    Character => 0x0000_0010,
    Buffered  => 0x0000_0100
});

impl_device_type!(Keyboard, as KeyboardType => {
    ReadCharSynchronous => 0x0000_0100,
    ReadLineSynchronous => 0x0000_1000
});

impl_device_type!(Memory, as MemoryType => {
    RAM => 0x0000_0100
});

impl_device_type!(Storage, as StorageType => {
    Readonly   => 0x0000_0100,
    Flash      => 0x0000_0011,
    Persistent => 0x0000_0021
});
