extern crate libc;

mod oid;
pub mod postgres;

use crate::oid::Oid;
pub use crate::postgres::*;

#[derive(Debug, PartialEq)]
#[repr(C)]
pub enum ConnStatusType {
    ConnectionOk,
    ConnectionBad,
    ConnectionStarted, /* Waiting for connection to be made.  */
    ConnectionMade,    /* Connection OK; waiting to send.     */
    ConnectionAwaitingResponse, /* Waiting for a response from the
                        * postmaster.        */
    ConnectionAuthOk,        /* Received authentication; waiting for
                              * backend startup. */
    ConnectionSetenv,        /* This state is no longer used. */
    ConnectionSslStartup,    /* Performing SSL handshake. */
    ConnectionNeeded,        /* Internal state: connect() needed. */
    ConnectionCheckWritable, /* Checking if session is read-write. */
    ConnectionConsume,       /* Consuming any extra messages. */
    ConnectionGssStartup,    /* Negotiating GSSAPI. */
    ConnectionCheckTarget,   /* Internal state: checking target server
                              * properties. */
    ConnectionCheckStandby, /* Checking if server is in standby mode. */
    ConnectionAllocated,    /* Waiting for connection attempt to be
                             * started.  */
    ConnectionAuthenticating, /* Authentication is in progress with some
                               * external system. */
}

#[derive(Debug, PartialEq)]
#[repr(C)]
pub enum ExecStatusType {
    PgresEmptyQuery,
    PgresCommandOk,
    PgresTuplesOk,
    PgresCopyOut,
    PgresCopyIn,
    PgresBadResponse,
    PgresNonfatalError,
    PgresFatalError,
}

#[repr(C)]
pub struct PGconn {
    _private: [u8; 0],
}

#[repr(C)]
pub struct PGresult {
    _private: [u8; 0],
}

#[link(name = "pq")]
unsafe extern "C" {
    pub fn PQconnectdb(conninfo: *const std::ffi::c_char) -> *mut PGconn;

    pub fn PQfinish(connection: *mut PGconn);

    pub fn PQstatus(connection: *mut PGconn) -> ConnStatusType;

    pub fn PQerrorMessage(connection: *mut PGconn) -> *const std::ffi::c_char;

    pub fn PQexec(connection: *mut PGconn, query: *const std::ffi::c_char) -> *mut PGresult;

    pub fn PQresultStatus(result: *mut PGresult) -> ExecStatusType;

    pub fn PQclear(result: *mut PGresult);

    pub fn PQntuples(result: *const PGresult) -> i32;

    pub fn PQnfields(result: *const PGresult) -> i32;

    pub fn PQftype(result: *mut PGresult, field_num: i32) -> Oid;

    pub fn PQgetisnull(result: *const PGresult, row_number: i32, column_number: i32) -> i32;

    pub fn PQfname(result: *const PGresult, column_number: i32) -> *const std::ffi::c_char;

    pub fn PQdb(conn: *const PGconn) -> *const std::ffi::c_char;

    pub fn PQgetvalue(
        result: *const PGresult,
        row_number: i32,
        column_number: i32,
    ) -> *mut std::ffi::c_char;
}
