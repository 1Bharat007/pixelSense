pub mod mock;
pub mod windows;

use crate::display::domain::{DisplayError, DisplayInfo};

pub trait DisplayProvider: Send + Sync {
    fn get_displays(&self) -> Result<Vec<DisplayInfo>, DisplayError>;
}
