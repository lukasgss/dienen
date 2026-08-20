use core::fmt;
use std::ffi::{CStr, CString};

use crate::{
    ConnStatusType, PGconn, PQconnectdb, PQexec, PQfinish, PQgetvalue, PQntuples, PQstatus,
};

pub trait Database: fmt::Debug {
    fn connect(conn_str: impl Into<String>) -> Result<Box<dyn Database>, DatabaseError>
    where
        Self: Sized + Drop;
}

#[derive(Debug)]
pub struct Postgres {
    pub connection: *mut PGconn,
    tables: Vec<String>,
}

#[derive(Debug)]
pub enum DatabaseError {
    UnableToConnect,
}

impl Database for Postgres {
    fn connect(conn_str: impl Into<String>) -> Result<Box<dyn Database>, DatabaseError> {
        let conn_str = CString::new(conn_str.into())
            .expect("should be able to create string from conncetion string");

        let connection = unsafe { PQconnectdb(conn_str.as_ptr()) };

        if unsafe { PQstatus(connection) } != ConnStatusType::ConnectionOk {
            return Err(DatabaseError::UnableToConnect);
        }

        let tables_query = CString::new(
            "select table_name from information_schema.tables where table_schema = 'public';",
        )
        .expect("should be able to create query");

        let tables_result = unsafe { PQexec(connection, tables_query.as_ptr()) };

        let amount_rows = unsafe { PQntuples(tables_result) };

        let mut tables: Vec<String> = Vec::with_capacity(amount_rows as usize);

        for i in 0..amount_rows {
            let value = unsafe { PQgetvalue(tables_result, i, 0) };

            let value = unsafe { CStr::from_ptr(value) };

            tables.push(value.to_string_lossy().to_string());
        }

        Ok(Box::new(Postgres { connection, tables }))
    }
}

impl Drop for Postgres {
    fn drop(&mut self) {
        unsafe { PQfinish(self.connection) };
    }
}
