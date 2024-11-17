pub const KILOBYTE: u64 = 1024;
pub const MEGABYTE: u64 = KILOBYTE * KILOBYTE;
pub const GIGABYTE: u64 = MEGABYTE * KILOBYTE;

pub enum DataSize {
    Kilobyte,
    Megabyte,
    Gigabyte,
}

impl DataSize {
    pub fn str(&self) -> &str {
        match *self {
            DataSize::Kilobyte => "KB",
            DataSize::Megabyte => "MB",
            DataSize::Gigabyte => "GB",
        }
    }

    pub fn size_to_value(&self) -> u64 {
        match *self {
            DataSize::Kilobyte => KILOBYTE,
            DataSize::Megabyte => MEGABYTE,
            DataSize::Gigabyte => GIGABYTE,
        }
    }
}
