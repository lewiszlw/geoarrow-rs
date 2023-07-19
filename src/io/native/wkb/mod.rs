//! Parse WKB arrays
//!
//! Each of the data structures in this module is intended to mirror the [WKB
//! spec](https://portal.ogc.org/files/?artifact_id=25355). Crucially each of these data structures
//! implement geometry access traits for interoperability and each of these data structures should
//! be O(1) access to any given coordinate.

pub mod coord;
pub mod geometry;
pub mod linearring;
pub mod linestring;
pub mod maybe_multi_line_string;
pub mod maybe_multi_point;
pub mod maybe_multipolygon;
pub mod multilinestring;
pub mod multipoint;
pub mod multipolygon;
pub mod point;
pub mod polygon;