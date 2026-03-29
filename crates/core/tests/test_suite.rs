//! Core Test Suite Entry Point
//!
//! Main test suite aggregator for nettoolskit-core crate.
//! All module tests are organized to mirror the src/ directory structure.

// Module-specific test suites
mod async_utils;
mod error_tests;
mod features;
mod file_search;
mod local_context;
mod menu;
#[path = "path-utils/mod.rs"]
mod path_utils;
mod runtime_execution_tests;
mod runtime_install_profiles_tests;
mod runtime_locations_tests;
mod string_utils;
