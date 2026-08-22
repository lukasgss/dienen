use std::ffi::{CStr, CString};

use rust_decimal::{self, Decimal};
use uuid::Uuid;

use crate::{
    ConnStatusType, ExecStatusType, PGconn, PGresult, PQclear, PQconnectdb, PQerrorMessage, PQexec,
    PQfinish, PQfname, PQftype, PQgetisnull, PQgetvalue, PQnfields, PQntuples, PQresultStatus,
    PQstatus,
    oid::{Oid, oid},
};

#[derive(Debug)]
pub(crate) enum PgType {
    Bool,
    Int2,
    Int4,
    Int8,
    Float4,
    Float8,
    Text,
    Uuid,
    Numeric,
    Unknown(Oid),
}

impl From<Oid> for PgType {
    fn from(value: Oid) -> Self {
        match value.0 {
            oid::BOOL => PgType::Bool,
            oid::INT2 => PgType::Int2,
            oid::INT4 => PgType::Int4,
            oid::INT8 => PgType::Int8,
            oid::FLOAT4 => PgType::Float4,
            oid::FLOAT8 => PgType::Float8,
            oid::TEXT | oid::VARCHAR => PgType::Text,
            oid::UUID => PgType::Uuid,
            oid::NUMERIC => PgType::Numeric,
            other => PgType::Unknown(Oid(other)),
        }
    }
}

pub trait Database {
    fn connect(conn_str: impl Into<String>) -> Result<Box<dyn Database>, DatabaseError>
    where
        Self: Sized + Drop;

    fn execute_query_statement(&self, query: &str) -> Result<QueryResult, DatabaseError>;

    fn connection(&self) -> *mut PGconn;
}

#[derive(Debug)]
pub enum DatabaseError {
    UnableToConnect(String),
    UnableToFetchDatabaseTables(String),
    UnableToExecuteStatement(String),
}

#[derive(Debug)]
pub struct Postgres {
    pub connection: *mut PGconn,
    tables: Vec<String>,
}

impl Postgres {
    #[inline]
    fn parse_boolean(&self, postgres_value: *mut i8) -> bool {
        let byte: u8 = unsafe { *postgres_value } as u8;

        byte == b't'
    }

    #[inline]
    fn parse_int(&self, postgres_value: *mut i8) -> i64 {
        let c_str = unsafe { CStr::from_ptr(postgres_value) };
        let str = c_str.to_str().expect("integer should be valid utf-8");

        str.parse::<i64>().expect("value should be valid integer")
    }

    #[inline]
    fn parse_float(&self, postgres_value: *mut i8) -> f64 {
        let c_str = unsafe { CStr::from_ptr(postgres_value) };
        let str = c_str.to_str().expect("float value should be valid utf-8");

        str.parse::<f64>().expect("value should be valid float")
    }

    #[inline]
    fn parse_text(&self, postgres_value: *mut i8) -> String {
        let c_str = unsafe { CStr::from_ptr(postgres_value) };

        c_str.to_string_lossy().into()
    }

    #[inline]
    fn parse_uuid(&self, postgres_value: *mut i8) -> Uuid {
        let c_str = unsafe { CStr::from_ptr(postgres_value) };

        let str = c_str.to_str().expect("uuid value should be valid utf-8");

        let uuid = Uuid::parse_str(str).expect("uuid value should be valid Uuid");

        uuid
    }

    #[inline]
    fn parse_numeric(&self, postgres_value: *mut i8) -> Decimal {
        let c_str = unsafe { CStr::from_ptr(postgres_value) };

        let str = c_str.to_str().expect("numeric value should be valid utf-8");

        let decimal: Decimal = str.parse().expect("should be valid decimal value");

        decimal
    }
}

impl Drop for Postgres {
    fn drop(&mut self) {
        unsafe { PQfinish(self.connection) };
    }
}

impl Database for Postgres {
    fn connect(conn_str: impl Into<String>) -> Result<Box<dyn Database>, DatabaseError> {
        let conn_str = CString::new(conn_str.into())
            .expect("should be able to create string from conncetion string");

        let connection = unsafe { PQconnectdb(conn_str.as_ptr()) };

        if unsafe { PQstatus(connection) } != ConnStatusType::ConnectionOk {
            let error_message_ptr = unsafe { PQerrorMessage(connection) };

            let error_c_str = unsafe { CStr::from_ptr(error_message_ptr) };

            return Err(DatabaseError::UnableToConnect(
                error_c_str.to_string_lossy().into(),
            ));
        }

        let tables_query = CString::new(
            "select table_name from information_schema.tables where table_schema = 'public';",
        )
        .expect("should be able to create query");

        let tables_result = unsafe { PQexec(connection, tables_query.as_ptr()) };

        if unsafe { PQresultStatus(tables_result) } != ExecStatusType::PgresTuplesOk {
            let error_message_ptr = unsafe { PQerrorMessage(connection) };

            let error_c_str = unsafe { CStr::from_ptr(error_message_ptr) };

            return Err(DatabaseError::UnableToFetchDatabaseTables(
                error_c_str.to_string_lossy().into(),
            ));
        }

        let amount_rows = unsafe { PQntuples(tables_result) };

        let mut tables: Vec<String> = Vec::with_capacity(amount_rows as usize);

        for row_num in 0..amount_rows {
            let value = unsafe { PQgetvalue(tables_result, row_num, 0) };

            if unsafe { PQgetisnull(tables_result, row_num, 0) } == 1 {
                tables.push("<null>".into());
                continue;
            }

            let value = unsafe { CStr::from_ptr(value) };

            tables.push(value.to_string_lossy().to_string());
        }

        Ok(Box::new(Postgres { connection, tables }))
    }

    fn execute_query_statement(&self, query: &str) -> Result<QueryResult, DatabaseError> {
        let formatted_query =
            CString::new(query).expect("should be able to create query from string");

        let query_result = unsafe { PQexec(self.connection(), formatted_query.as_ptr()) };

        if unsafe { PQresultStatus(query_result) } != ExecStatusType::PgresTuplesOk {
            let error_message_ptr = unsafe { PQerrorMessage(self.connection()) };

            let error_c_str = unsafe { CStr::from_ptr(error_message_ptr) };

            return Err(DatabaseError::UnableToExecuteStatement(
                error_c_str.to_string_lossy().into(),
            ));
        }

        let amount_cols = unsafe { PQnfields(query_result) };

        let mut cols = Vec::<ColumnInfo>::with_capacity(amount_cols as usize);

        for col in 0..amount_cols {
            let col_name_ptr = unsafe { PQfname(query_result, col) };
            let col_name = unsafe { CStr::from_ptr(col_name_ptr) }
                .to_string_lossy()
                .into();

            let oid = unsafe { PQftype(query_result, col) };
            let col_info = ColumnInfo {
                pg_type: PgType::from(oid),
                name: col_name,
            };

            cols.push(col_info);
        }

        let amount_rows = unsafe { PQntuples(query_result) };
        let mut rows = Vec::<Vec<Value>>::with_capacity(amount_rows as usize);

        for row in 0..amount_rows {
            let mut current_row = Vec::<Value>::with_capacity(amount_cols as usize);

            for col in 0..amount_cols {
                let value = unsafe { PQgetvalue(query_result, row, col) };

                if unsafe { PQgetisnull(query_result, row, col) } == 1 {
                    current_row.push(Value::Null);
                    continue;
                }

                match &cols[col as usize].pg_type {
                    PgType::Bool => {
                        let parsed_value = self.parse_boolean(value);
                        current_row.push(Value::Bool(parsed_value));
                    }
                    PgType::Int2 | PgType::Int4 | PgType::Int8 => {
                        let parsed_value = self.parse_int(value);
                        current_row.push(Value::Int(parsed_value));
                    }
                    PgType::Float4 | PgType::Float8 => {
                        let parsed_value = self.parse_float(value);
                        current_row.push(Value::Float(parsed_value));
                    }
                    PgType::Text => {
                        let parsed_value = self.parse_text(value);
                        current_row.push(Value::Text(parsed_value));
                    }
                    PgType::Uuid => {
                        let parsed_value = self.parse_uuid(value);
                        current_row.push(Value::Uuid(parsed_value));
                    }
                    PgType::Numeric => {
                        let parsed_value = self.parse_numeric(value);
                        current_row.push(Value::Numeric(parsed_value));
                    }
                    PgType::Unknown(oid) => {
                        println!("unknown oid: {:?}", oid);
                        panic!("unknown types are not supported yet");
                    }
                }
            }

            rows.push(current_row);
        }

        Ok(QueryResult {
            cols,
            rows,
            result: query_result,
        })
    }

    fn connection(&self) -> *mut PGconn {
        self.connection
    }
}

#[derive(Debug)]
pub enum Value {
    Null,
    Text(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Bytes(Vec<u8>),
    Uuid(Uuid),
    Numeric(Decimal),
}

#[derive(Debug)]
pub struct ColumnInfo {
    pg_type: PgType,
    pub name: String,
}

#[derive(Debug)]
pub struct QueryResult {
    pub cols: Vec<ColumnInfo>,
    pub rows: Vec<Vec<Value>>,
    result: *mut PGresult,
}

impl Drop for QueryResult {
    fn drop(&mut self) {
        unsafe { PQclear(self.result) };
    }
}
