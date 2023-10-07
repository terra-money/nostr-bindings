#[cfg(not(feature = "library"))]
pub mod contract;
pub mod definitions;
mod error;
mod execute;
pub mod msg;
mod query;
mod state;
#[cfg(test)]
mod tests;
