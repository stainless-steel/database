use column::Column;
use database::{self, Database};
use driver::Driver;
use operation::{CreateTable, Operation};
use {Result, Safe};

/// A table.
pub struct Table<T: Driver> {
    name: String,
    columns: Vec<Column>,
    driver: Safe<T>,
}

impl<T: Driver> Table<T> {
    /// Create a table.
    ///
    /// The function has no effect on the database if the table already exists.
    pub fn create(database: &Database<T>, name: &str, columns: &[Column]) -> Result<Self> {
        let driver = database::driver(database);
        let mut operation = CreateTable::new();
        operation.if_not_exists().name(name);
        for &Column { ref name, kind } in columns {
            operation.column(|column| {
                column.name(name);
                column.kind(kind);
            });
        }
        try!(driver.execute(&try!(operation.compile())));
        Ok(Table { name: name.to_string(), columns: columns.to_vec(), driver: driver })
    }

    /// Return the columns.
    #[inline]
    pub fn columns(&self) -> &[Column] {
        &self.columns
    }

    /// Return the name.
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[inline]
pub fn driver<T: Driver>(table: &Table<T>) -> Safe<T> {
    table.driver.clone()
}
