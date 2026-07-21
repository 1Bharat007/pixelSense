use super::Platform;
use crate::platform::windows::WindowsPlatform;

pub fn create_platform() -> Box<dyn Platform> {
    Box::new(WindowsPlatform::new())
}
