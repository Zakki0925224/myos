use crate::println;

pub const OS_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub const OS_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
pub const OS_HOMEPAGE: &str = env!("CARGO_PKG_HOMEPAGE");
pub const OS_NAME: &str = env!("CARGO_PKG_NAME");
pub const OS_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const OS_VERSION_MAJOR: &str = env!("CARGO_PKG_VERSION_MAJOR");
pub const OS_VERSION_MINOR: &str = env!("CARGO_PKG_VERSION_MINOR");
pub const OS_VERSION_PATCH: &str = env!("CARGO_PKG_VERSION_PATCH");
pub const OS_VERSION_PRE: &str = env!("CARGO_PKG_VERSION_PRE");

pub fn print_info()
{
    println!("{}", OS_NAME);
    println!("Version: {}", OS_VERSION);
    println!("Authors: {}", OS_AUTHORS);
    println!("Description: {}", OS_DESCRIPTION);
}