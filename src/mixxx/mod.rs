/// The logic that executes the queries, cleans the data and brings
/// it into an easier to handle format.
pub mod aggregator;
/// The full representation of a Mixxx library in a clean and organized structure.
pub mod library;
/// The raw schema structs used when interacting with the database.
pub mod schema;
/// This module contains all SQLite facing logic.
pub mod storage;
