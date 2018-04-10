#[derive(Debug, Fail)]
pub enum KrankerlError {
    #[fail(display = "invalid toolchain name: {}", name)]
    InvalidToolchainName { name: String },
    #[fail(display = "exepected error: {}", cause)]
    Other { cause: String },
}
