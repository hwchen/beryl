use failure::Error;
use futures::Future;

use crate::dataframe::DataFrame;
use crate::query_ir::QueryIr;

pub trait Backend {
    /// Takes in a SQL string, outputs a DataFrame, which will go on to be formatted into the
    /// desired query output format.
    fn exec_sql(&self, sql: String) -> Box<Future<Item=DataFrame, Error=Error>>;

    fn box_clone(&self) -> Box<dyn Backend + Send + Sync>;

    /// takes &self, but only required to be able to turn Backend into
    /// a trait object. It's not needed for any of the logic
    fn generate_sql(&self, query_ir: QueryIr) -> Vec<String>;
}

impl Clone for Box<dyn Backend + Send + Sync> {
    fn clone(&self) -> Box<Backend + Send + Sync> {
        self.box_clone()
    }
}
