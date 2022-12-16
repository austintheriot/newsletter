#[cfg(test)]
extern crate quickcheck;

#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

pub mod configuration;
pub mod routes;
pub mod startup;
pub mod telemetry;

mod domain;

pub use domain::*;
