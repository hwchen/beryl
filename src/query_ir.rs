//! It's a somewhat typed abstract-ish representation of a sql statement
//! that can be passed to a backend to produce the final sql statement
//! with proper syntax for that db.
//!
//! Created by doing a lookup in the Schema depending on the Query.
//!
//! In some cases, the only difference is mapping a "name" in the interface
//! to a column

mod filter_ir;

use crate::query::{
    LimitQuery,
    SortDirection
};

pub use self::filter_ir::{
    FilterIr,
    Constraint,
    Comparison,
};

#[derive(Debug, Clone)]
pub struct QueryIr {
    pub table: String,
    // headers for formatting are separate from projection cols
    pub projection: Vec<String>,

    pub filters: Vec<FilterIr>,
    pub sort: Option<SortIr>,
    pub limit: Option<LimitQuery>,
//    dimension joins?
}

#[derive(Debug, Clone)]
pub struct SortIr {
    pub direction: SortDirection,
    pub column: String,
}
