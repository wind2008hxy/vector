//! Kubernetes test framework.
//!
//! The main goal of the design of this test framework is to wire kubernetes
//! components testing through the same tools that are available to the
//! developer as executable commands, rather than using a rust interface to talk
//! to k8s cluster directly.
//! This enables very trivial troubleshooting and allows us to use the same
//! deployemnt mechanisms that we use for prodcution - effectively giving us
//! the opportunity to test e2e - not just the code layer, but also the
//! deployment configuration.

#![warn(
    missing_debug_implementations,
    missing_copy_implementations,
    missing_docs
)]

mod custom_resource_file;
pub mod framework;
pub mod interface;
pub mod test_pod;
pub mod vector;

// Re-export some unit for trivial accessability.

pub use framework::Framework;
pub use interface::Interface;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
