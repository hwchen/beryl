use failure::Error;
use futures::{Future, Stream};

use crate::dataframe::DataFrame;


pub trait Backend {
    /// Takes in a SQL string, outputs a DataFrame, which will go on to be formatted into the
    /// desired query output format.
    fn exec_sql(&self, sql: String) -> Box<Future<Item=DataFrame, Error=Error>>;

    fn box_clone(&self) -> Box<dyn Backend + Send + Sync>;

    fn generate_sql(&self) -> String;
}

impl Clone for Box<dyn Backend + Send + Sync> {
    fn clone(&self) -> Box<Backend + Send + Sync> {
        self.box_clone()
    }
}
