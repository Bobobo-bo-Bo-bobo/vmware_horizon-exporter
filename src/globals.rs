use crate::data;

use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    pub static ref DESKTOP_POOLS: Mutex<Vec<data::DesktopPool>> = Mutex::new(Vec::new());
}
