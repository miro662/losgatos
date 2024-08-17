//! `core`-only flattened device tree (FDT) parser
//!
//! Designed to be used during early boot. It does not use allocation.
//! However, retrieval of a value has a pessimistic complexity of `O(size ^ nest_lvl)`
//!
//! Conforms to a devicetree specification as described on
//! <https://devicetree-specification.readthedocs.io/en/stable/introduction.html>
//!
pub mod flattened;
pub mod node;
