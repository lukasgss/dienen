# libpq Function Reference

libpq is the C client library for connecting to PostgreSQL. Signatures reflect recent PostgreSQL versions — consult https://www.postgresql.org/docs/current/libpq.html for authoritative, version-exact prototypes.

---

## 1. Database Connection Control

| Function               | Signature                                                                                                                                                         | Description                                                                           |
| ---------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------- |
| `PQconnectdb`          | `PGconn *PQconnectdb(const char *conninfo);`                                                                                                                      | Opens a new connection using a connection string. Blocks until connected or failed.   |
| `PQconnectdbParams`    | `PGconn *PQconnectdbParams(const char * const *keywords, const char * const *values, int expand_dbname);`                                                         | Like `PQconnectdb`, but takes parameters as null-terminated arrays.                   |
| `PQsetdbLogin`         | `PGconn *PQsetdbLogin(const char *pghost, const char *pgport, const char *pgoptions, const char *pgtty, const char *dbName, const char *login, const char *pwd);` | Older-style connection function; superseded by `PQconnectdb`.                         |
| `PQsetdb`              | `PGconn *PQsetdb(char *pghost, char *pgport, char *pgoptions, char *pgtty, char *dbName);`                                                                        | Macro wrapper around `PQsetdbLogin` with null user/password.                          |
| `PQconnectStart`       | `PGconn *PQconnectStart(const char *conninfo);`                                                                                                                   | Begins a non-blocking connection attempt from a connection string.                    |
| `PQconnectStartParams` | `PGconn *PQconnectStartParams(const char * const *keywords, const char * const *values, int expand_dbname);`                                                      | Non-blocking connection start using parameter arrays.                                 |
| `PQconnectPoll`        | `PostgresPollingStatusType PQconnectPoll(PGconn *conn);`                                                                                                          | Advances a non-blocking connection attempt; poll until `PGRES_POLLING_OK`/`_FAILED`.  |
| `PQconndefaults`       | `PQconninfoOption *PQconndefaults(void);`                                                                                                                         | Returns an array of default connection option structures.                             |
| `PQconninfoParse`      | `PQconninfoOption *PQconninfoParse(const char *conninfo, char **errmsg);`                                                                                         | Parses a connection string into option structures without connecting.                 |
| `PQconninfo`           | `PQconninfoOption *PQconninfo(PGconn *conn);`                                                                                                                     | Returns the connection options used for an existing connection.                       |
| `PQfinish`             | `void PQfinish(PGconn *conn);`                                                                                                                                    | Closes the connection and frees all associated memory.                                |
| `PQreset`              | `void PQreset(PGconn *conn);`                                                                                                                                     | Resets the connection (closes and reopens with same parameters). Blocking.            |
| `PQresetStart`         | `int PQresetStart(PGconn *conn);`                                                                                                                                 | Begins a non-blocking reset attempt.                                                  |
| `PQresetPoll`          | `PostgresPollingStatusType PQresetPoll(PGconn *conn);`                                                                                                            | Advances a non-blocking reset attempt.                                                |
| `PQpingParams`         | `PGPing PQpingParams(const char * const *keywords, const char * const *values, int expand_dbname);`                                                               | Checks server reachability using parameter arrays, without keeping a connection open. |
| `PQping`               | `PGPing PQping(const char *conninfo);`                                                                                                                            | Same as `PQpingParams`, using a connection string.                                    |
| `PQconninfoFree`       | `void PQconninfoFree(PQconninfoOption *connOptions);`                                                                                                             | Frees structures returned by `PQconndefaults`, `PQconninfoParse`, or `PQconninfo`.    |

---

## 2. Connection Status Functions

| Function                    | Signature                                                                   | Description                                                            |
| --------------------------- | --------------------------------------------------------------------------- | ---------------------------------------------------------------------- |
| `PQdb`                      | `char *PQdb(const PGconn *conn);`                                           | Returns the database name of the connection.                           |
| `PQuser`                    | `char *PQuser(const PGconn *conn);`                                         | Returns the user name used to connect.                                 |
| `PQpass`                    | `char *PQpass(const PGconn *conn);`                                         | Returns the password used (or empty string).                           |
| `PQhost`                    | `char *PQhost(const PGconn *conn);`                                         | Returns the server host name/address.                                  |
| `PQhostaddr`                | `char *PQhostaddr(const PGconn *conn);`                                     | Returns the server's numeric IP address.                               |
| `PQport`                    | `char *PQport(const PGconn *conn);`                                         | Returns the port of the connection.                                    |
| `PQtty`                     | `char *PQtty(const PGconn *conn);`                                          | Returns the debug TTY (obsolete, always empty).                        |
| `PQoptions`                 | `char *PQoptions(const PGconn *conn);`                                      | Returns the command-line options passed in the connection request.     |
| `PQstatus`                  | `ConnStatusType PQstatus(const PGconn *conn);`                              | Returns connection status (`CONNECTION_OK`, `CONNECTION_BAD`, etc.).   |
| `PQtransactionStatus`       | `PGTransactionStatusType PQtransactionStatus(const PGconn *conn);`          | Returns the current transaction status.                                |
| `PQparameterStatus`         | `const char *PQparameterStatus(const PGconn *conn, const char *paramName);` | Returns the current value of a server parameter.                       |
| `PQprotocolVersion`         | `int PQprotocolVersion(const PGconn *conn);`                                | Returns the frontend/backend protocol version in use.                  |
| `PQserverVersion`           | `int PQserverVersion(const PGconn *conn);`                                  | Returns the server's version number as an integer.                     |
| `PQerrorMessage`            | `char *PQerrorMessage(const PGconn *conn);`                                 | Returns the last error message associated with the connection.         |
| `PQsocket`                  | `int PQsocket(const PGconn *conn);`                                         | Returns the file descriptor of the connection socket.                  |
| `PQbackendPID`              | `int PQbackendPID(const PGconn *conn);`                                     | Returns the process ID of the backend server process.                  |
| `PQconnectionNeedsPassword` | `int PQconnectionNeedsPassword(const PGconn *conn);`                        | True if a password was required but none/wrong was supplied.           |
| `PQconnectionUsedPassword`  | `int PQconnectionUsedPassword(const PGconn *conn);`                         | True if the connection authenticated using a password.                 |
| `PQsslInUse`                | `int PQsslInUse(PGconn *conn);`                                             | True if the connection uses SSL/TLS.                                   |
| `PQsslAttribute`            | `const char *PQsslAttribute(PGconn *conn, const char *attribute_name);`     | Returns the value of an SSL-related connection attribute.              |
| `PQsslAttributeNames`       | `const char * const *PQsslAttributeNames(PGconn *conn);`                    | Returns a NULL-terminated array of available SSL attribute names.      |
| `PQsslStruct`               | `void *PQsslStruct(PGconn *conn, const char *struct_name);`                 | Returns a pointer to an underlying SSL implementation struct.          |
| `PQgetssl`                  | `void *PQgetssl(PGconn *conn);`                                             | Returns the SSL structure used (deprecated in favor of `PQsslStruct`). |

---

## 3. Command Execution Functions

| Function             | Signature                                                                                                                                                                                             | Description                                                                                     |
| -------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------- |
| `PQexec`             | `PGresult *PQexec(PGconn *conn, const char *command);`                                                                                                                                                | Sends a SQL command (or several separated by `;`) and blocks for the result.                    |
| `PQexecParams`       | `PGresult *PQexecParams(PGconn *conn, const char *command, int nParams, const Oid *paramTypes, const char * const *paramValues, const int *paramLengths, const int *paramFormats, int resultFormat);` | Parameterized version of `PQexec`; avoids SQL injection, supports binary I/O; one command only. |
| `PQprepare`          | `PGresult *PQprepare(PGconn *conn, const char *stmtName, const char *query, int nParams, const Oid *paramTypes);`                                                                                     | Creates a prepared statement on the server for later execution.                                 |
| `PQexecPrepared`     | `PGresult *PQexecPrepared(PGconn *conn, const char *stmtName, int nParams, const char * const *paramValues, const int *paramLengths, const int *paramFormats, int resultFormat);`                     | Executes a previously prepared statement with given parameters.                                 |
| `PQdescribePrepared` | `PGresult *PQdescribePrepared(PGconn *conn, const char *stmtName);`                                                                                                                                   | Retrieves info about a prepared statement's parameters/columns.                                 |
| `PQdescribePortal`   | `PGresult *PQdescribePortal(PGconn *conn, const char *portalName);`                                                                                                                                   | Retrieves info about a portal (cursor).                                                         |
| `PQclosePrepared`    | `PGresult *PQclosePrepared(PGconn *conn, const char *stmtName);`                                                                                                                                      | Closes (deallocates) a prepared statement.                                                      |
| `PQclosePortal`      | `PGresult *PQclosePortal(PGconn *conn, const char *portalName);`                                                                                                                                      | Closes a portal.                                                                                |
| `PQclear`            | `void PQclear(PGresult *res);`                                                                                                                                                                        | Frees the storage associated with a `PGresult`. Required for every result.                      |
| `PQfreemem`          | `void PQfreemem(void *ptr);`                                                                                                                                                                          | Frees memory allocated by libpq (e.g. by escape functions).                                     |

---

## 4. Retrieving Query Result Information

| Function                      | Signature                                                                                                          | Description                                                                          |
| ----------------------------- | ------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------ |
| `PQresultStatus`              | `ExecStatusType PQresultStatus(const PGresult *res);`                                                              | Returns the result status code (`PGRES_TUPLES_OK`, `PGRES_COMMAND_OK`, etc.).        |
| `PQresStatus`                 | `char *PQresStatus(ExecStatusType status);`                                                                        | Converts a result status code into a human-readable string.                          |
| `PQresultErrorMessage`        | `char *PQresultErrorMessage(const PGresult *res);`                                                                 | Returns the error message associated with a result.                                  |
| `PQresultVerboseErrorMessage` | `char *PQresultVerboseErrorMessage(const PGresult *res, PGVerbosity verbosity, PGContextVisibility show_context);` | Returns a reformattable, detailed error message.                                     |
| `PQresultErrorField`          | `char *PQresultErrorField(const PGresult *res, int fieldcode);`                                                    | Returns an individual field of the error report (SQLSTATE, hint, detail, etc.).      |
| `PQntuples`                   | `int PQntuples(const PGresult *res);`                                                                              | Returns the number of rows in the result.                                            |
| `PQnfields`                   | `int PQnfields(const PGresult *res);`                                                                              | Returns the number of columns in the result.                                         |
| `PQfname`                     | `char *PQfname(const PGresult *res, int column_number);`                                                           | Returns the column name given its index.                                             |
| `PQfnumber`                   | `int PQfnumber(const PGresult *res, const char *column_name);`                                                     | Returns the column index given its name.                                             |
| `PQftable`                    | `Oid PQftable(const PGresult *res, int column_number);`                                                            | Returns the OID of the table a column came from.                                     |
| `PQftablecol`                 | `int PQftablecol(const PGresult *res, int column_number);`                                                         | Returns the column number within its source table.                                   |
| `PQfformat`                   | `int PQfformat(const PGresult *res, int column_number);`                                                           | Returns the format code (text=0, binary=1) of a column.                              |
| `PQftype`                     | `Oid PQftype(const PGresult *res, int column_number);`                                                             | Returns the data type OID of a column.                                               |
| `PQfmod`                      | `int PQfmod(const PGresult *res, int column_number);`                                                              | Returns the type modifier of a column.                                               |
| `PQfsize`                     | `int PQfsize(const PGresult *res, int column_number);`                                                             | Returns the size in bytes of the column's data type (-1 if variable-length).         |
| `PQbinaryTuples`              | `int PQbinaryTuples(const PGresult *res);`                                                                         | True if result contains binary data (deprecated; check per-column with `PQfformat`). |
| `PQgetvalue`                  | `char *PQgetvalue(const PGresult *res, int row_number, int column_number);`                                        | Returns a pointer to a single field's value.                                         |
| `PQgetisnull`                 | `int PQgetisnull(const PGresult *res, int row_number, int column_number);`                                         | True if the field's value is SQL NULL.                                               |
| `PQgetlength`                 | `int PQgetlength(const PGresult *res, int row_number, int column_number);`                                         | Returns the length in bytes of a field's value.                                      |
| `PQnparams`                   | `int PQnparams(const PGresult *res);`                                                                              | Returns the number of parameters expected by a prepared statement.                   |
| `PQparamtype`                 | `Oid PQparamtype(const PGresult *res, int param_number);`                                                          | Returns the data type of a prepared statement's parameter.                           |
| `PQcmdStatus`                 | `char *PQcmdStatus(PGresult *res);`                                                                                | Returns the command status tag (e.g. `"INSERT 0 1"`).                                |
| `PQcmdTuples`                 | `char *PQcmdTuples(PGresult *res);`                                                                                | Returns the number of rows affected by INSERT/UPDATE/DELETE, as a string.            |
| `PQoidValue`                  | `Oid PQoidValue(const PGresult *res);`                                                                             | Returns the OID of the inserted row (deprecated).                                    |
| `PQoidStatus`                 | `char *PQoidStatus(const PGresult *res);`                                                                          | Deprecated equivalent returning OID as a string.                                     |

---

## 5. Escaping Strings and Binary Data for Inclusion in SQL

| Function             | Signature                                                                                                           | Description                                                              |
| -------------------- | ------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------ |
| `PQescapeLiteral`    | `char *PQescapeLiteral(PGconn *conn, const char *str, size_t length);`                                              | Escapes a string as a quoted SQL literal (free result with `PQfreemem`). |
| `PQescapeIdentifier` | `char *PQescapeIdentifier(PGconn *conn, const char *str, size_t length);`                                           | Escapes a string for safe use as an SQL identifier.                      |
| `PQescapeStringConn` | `size_t PQescapeStringConn(PGconn *conn, char *to, const char *from, size_t length, int *error);`                   | Escapes a string for inclusion in an SQL command (connection-aware).     |
| `PQescapeString`     | `size_t PQescapeString(char *to, const char *from, size_t length);`                                                 | Deprecated, connection-unaware version of `PQescapeStringConn`.          |
| `PQescapeByteaConn`  | `unsigned char *PQescapeByteaConn(PGconn *conn, const unsigned char *from, size_t from_length, size_t *to_length);` | Escapes binary data for use as a `bytea` literal.                        |
| `PQescapeBytea`      | `unsigned char *PQescapeBytea(const unsigned char *from, size_t from_length, size_t *to_length);`                   | Deprecated, connection-unaware version.                                  |
| `PQunescapeBytea`    | `unsigned char *PQunescapeBytea(const unsigned char *from, size_t *to_length);`                                     | Converts an escaped `bytea` string back into raw binary data.            |

---

## 6. Asynchronous Command Processing

| Function                 | Signature                                                                                                                                                                                            | Description                                                       |
| ------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------- |
| `PQsendQuery`            | `int PQsendQuery(PGconn *conn, const char *command);`                                                                                                                                                | Submits a command without waiting for the result.                 |
| `PQsendQueryParams`      | `int PQsendQueryParams(PGconn *conn, const char *command, int nParams, const Oid *paramTypes, const char * const *paramValues, const int *paramLengths, const int *paramFormats, int resultFormat);` | Async version of `PQexecParams`.                                  |
| `PQsendPrepare`          | `int PQsendPrepare(PGconn *conn, const char *stmtName, const char *query, int nParams, const Oid *paramTypes);`                                                                                      | Async version of `PQprepare`.                                     |
| `PQsendQueryPrepared`    | `int PQsendQueryPrepared(PGconn *conn, const char *stmtName, int nParams, const char * const *paramValues, const int *paramLengths, const int *paramFormats, int resultFormat);`                     | Async version of `PQexecPrepared`.                                |
| `PQsendDescribePrepared` | `int PQsendDescribePrepared(PGconn *conn, const char *stmtName);`                                                                                                                                    | Async version of `PQdescribePrepared`.                            |
| `PQsendDescribePortal`   | `int PQsendDescribePortal(PGconn *conn, const char *portalName);`                                                                                                                                    | Async version of `PQdescribePortal`.                              |
| `PQsendClosePrepared`    | `int PQsendClosePrepared(PGconn *conn, const char *stmtName);`                                                                                                                                       | Async version of `PQclosePrepared`.                               |
| `PQsendClosePortal`      | `int PQsendClosePortal(PGconn *conn, const char *portalName);`                                                                                                                                       | Async version of `PQclosePortal`.                                 |
| `PQgetResult`            | `PGresult *PQgetResult(PGconn *conn);`                                                                                                                                                               | Retrieves the next result from an async command; call until NULL. |
| `PQconsumeInput`         | `int PQconsumeInput(PGconn *conn);`                                                                                                                                                                  | Consumes any input available from the server without blocking.    |
| `PQisBusy`               | `int PQisBusy(PGconn *conn);`                                                                                                                                                                        | True if a call to `PQgetResult` would block.                      |
| `PQflush`                | `int PQflush(PGconn *conn);`                                                                                                                                                                         | Flushes any queued output data to the server.                     |
| `PQsetSingleRowMode`     | `int PQsetSingleRowMode(PGconn *conn);`                                                                                                                                                              | Switches result delivery to one-row-at-a-time mode.               |
| `PQsetChunkedRowsMode`   | `int PQsetChunkedRowsMode(PGconn *conn, int chunkSize);`                                                                                                                                             | Delivers results in chunks of rows.                               |
| `PQcancelCreate`         | `PGcancelConn *PQcancelCreate(PGconn *conn);`                                                                                                                                                        | Creates a cancel request object (modern cancel API).              |
| `PQcancelBlocking`       | `int PQcancelBlocking(PGcancelConn *cancelConn);`                                                                                                                                                    | Sends a blocking cancel request.                                  |
| `PQcancelStart`          | `int PQcancelStart(PGcancelConn *cancelConn);`                                                                                                                                                       | Begins a non-blocking cancel request.                             |
| `PQcancelPoll`           | `PostgresPollingStatusType PQcancelPoll(PGcancelConn *cancelConn);`                                                                                                                                  | Advances a non-blocking cancel request.                           |
| `PQcancelFinish`         | `void PQcancelFinish(PGcancelConn *cancelConn);`                                                                                                                                                     | Frees a cancel connection object.                                 |
| `PQcancelReset`          | `void PQcancelReset(PGcancelConn *cancelConn);`                                                                                                                                                      | Resets a cancel object for reuse.                                 |
| `PQcancelErrorMessage`   | `char *PQcancelErrorMessage(const PGcancelConn *cancelConn);`                                                                                                                                        | Returns the error message from a failed cancel attempt.           |
| `PQgetCancel`            | `PGcancel *PQgetCancel(PGconn *conn);`                                                                                                                                                               | Deprecated: creates the older-API cancel request structure.       |
| `PQfreeCancel`           | `void PQfreeCancel(PGcancel *cancel);`                                                                                                                                                               | Deprecated: frees a `PGcancel` object.                            |
| `PQcancel`               | `int PQcancel(PGcancel *cancel, char *errbuf, int errbufsize);`                                                                                                                                      | Deprecated: requests cancellation via the older API.              |
| `PQrequestCancel`        | `int PQrequestCancel(PGconn *conn);`                                                                                                                                                                 | Deprecated, unsafe (non-thread-safe) query cancellation.          |

---

## 7. Pipeline Mode

| Function              | Signature                                                | Description                                                                                   |
| --------------------- | -------------------------------------------------------- | --------------------------------------------------------------------------------------------- |
| `PQenterPipelineMode` | `int PQenterPipelineMode(PGconn *conn);`                 | Enters pipeline mode, allowing multiple queries to be queued without waiting for each result. |
| `PQexitPipelineMode`  | `int PQexitPipelineMode(PGconn *conn);`                  | Exits pipeline mode (only allowed when idle).                                                 |
| `PQpipelineSync`      | `int PQpipelineSync(PGconn *conn);`                      | Marks a synchronization point in the pipeline queue.                                          |
| `PQsendFlushRequest`  | `int PQsendFlushRequest(PGconn *conn);`                  | Sends a request for the server to flush its output buffer.                                    |
| `PQpipelineStatus`    | `PGpipelineStatus PQpipelineStatus(const PGconn *conn);` | Returns whether the connection is in pipeline mode.                                           |

---

## 8. Asynchronous Notification (LISTEN/NOTIFY)

| Function     | Signature                             | Description                                                                         |
| ------------ | ------------------------------------- | ----------------------------------------------------------------------------------- |
| `PQnotifies` | `PGnotify *PQnotifies(PGconn *conn);` | Returns the next "notify" event from a `LISTEN`ed channel, or NULL if none pending. |

---

## 9. COPY Command Support

| Function         | Signature                                                          | Description                                                    |
| ---------------- | ------------------------------------------------------------------ | -------------------------------------------------------------- |
| `PQputCopyData`  | `int PQputCopyData(PGconn *conn, const char *buffer, int nbytes);` | Sends data for a `COPY ... FROM STDIN` operation.              |
| `PQputCopyEnd`   | `int PQputCopyEnd(PGconn *conn, const char *errormsg);`            | Signals the end of (or aborts) a `COPY FROM STDIN` operation.  |
| `PQgetCopyData`  | `int PQgetCopyData(PGconn *conn, char **buffer, int async);`       | Retrieves a row of data during `COPY ... TO STDOUT`.           |
| `PQgetline`      | `int PQgetline(PGconn *conn, char *buffer, int length);`           | Deprecated legacy method of reading a COPY data line.          |
| `PQputline`      | `int PQputline(PGconn *conn, const char *string);`                 | Deprecated legacy method of sending a COPY data line.          |
| `PQgetlineAsync` | `int PQgetlineAsync(PGconn *conn, char *buffer, int bufsize);`     | Deprecated async legacy COPY read.                             |
| `PQputnbytes`    | `int PQputnbytes(PGconn *conn, const char *buffer, int nbytes);`   | Deprecated legacy method of sending COPY data of known length. |
| `PQendcopy`      | `int PQendcopy(PGconn *conn);`                                     | Deprecated legacy method of ending a COPY operation.           |

---

## 10. Control Functions

| Function                      | Signature                                                                                          | Description                                                        |
| ----------------------------- | -------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------ |
| `PQclientEncoding`            | `int PQclientEncoding(const PGconn *conn);`                                                        | Returns the client's current character-set encoding.               |
| `PQsetClientEncoding`         | `int PQsetClientEncoding(PGconn *conn, const char *encoding);`                                     | Sets the client encoding for the connection.                       |
| `PQsetErrorVerbosity`         | `PGVerbosity PQsetErrorVerbosity(PGconn *conn, PGVerbosity verbosity);`                            | Controls error message detail level; returns previous setting.     |
| `PQsetErrorContextVisibility` | `PGContextVisibility PQsetErrorContextVisibility(PGconn *conn, PGContextVisibility show_context);` | Controls whether `CONTEXT` info appears in error messages.         |
| `PQtrace`                     | `void PQtrace(PGconn *conn, FILE *stream);`                                                        | Enables logging of client/server message traffic to a file stream. |
| `PQuntrace`                   | `void PQuntrace(PGconn *conn);`                                                                    | Disables tracing started by `PQtrace`.                             |
| `PQsetTraceFlags`             | `void PQsetTraceFlags(PGconn *conn, int flags);`                                                   | Sets options controlling trace output detail.                      |

---

## 11. Miscellaneous Functions

| Function                        | Signature                                                                                                                                 | Description                                                         |
| ------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------- |
| `PQlibVersion`                  | `int PQlibVersion(void);`                                                                                                                 | Returns the version of libpq being used, as an integer.             |
| `PQfreemem`                     | `void PQfreemem(void *ptr);`                                                                                                              | Frees memory returned by libpq functions.                           |
| `PQinitOpenSSL`                 | `void PQinitOpenSSL(int do_ssl, int do_crypto);`                                                                                          | Controls libpq's initialization of OpenSSL.                         |
| `PQinitSSL`                     | `void PQinitSSL(int do_ssl);`                                                                                                             | Older equivalent of `PQinitOpenSSL`.                                |
| `PQisthreadsafe`                | `int PQisthreadsafe(void);`                                                                                                               | Returns whether libpq was built with thread safety.                 |
| `PQmakeEmptyPGresult`           | `PGresult *PQmakeEmptyPGresult(PGconn *conn, ExecStatusType status);`                                                                     | Constructs an empty `PGresult` with a given status.                 |
| `PQcopyResult`                  | `PGresult *PQcopyResult(const PGresult *src, int flags);`                                                                                 | Creates a copy of a `PGresult` object.                              |
| `PQsetResultAttrs`              | `int PQsetResultAttrs(PGresult *res, int numAttributes, PGresAttDesc *attDescs);`                                                         | Sets the attribute (column) descriptions of a `PGresult`.           |
| `PQsetvalue`                    | `int PQsetvalue(PGresult *res, int tup_num, int field_num, char *value, int len);`                                                        | Sets a field's value in a `PGresult`.                               |
| `PQresultAlloc`                 | `void *PQresultAlloc(PGresult *res, size_t nBytes);`                                                                                      | Allocates extra memory tied to the lifetime of a `PGresult`.        |
| `PQresultMemorySize`            | `size_t PQresultMemorySize(const PGresult *res);`                                                                                         | Returns the amount of memory used by a `PGresult`.                  |
| `PQregisterEventProc`           | `int PQregisterEventProc(PGconn *conn, PGEventProc proc, const char *name, void *passThrough);`                                           | Registers an event callback procedure on a connection.              |
| `PQinstanceData`                | `int PQinstanceData(const PGconn *conn, PGEventProc proc);`                                                                               | Gets private data for a registered event procedure on a connection. |
| `PQsetInstanceData`             | `int PQsetInstanceData(PGconn *conn, PGEventProc proc, void *data);`                                                                      | Sets private data for a registered event procedure on a connection. |
| `PQresultInstanceData`          | `void *PQresultInstanceData(const PGresult *res, PGEventProc proc);`                                                                      | Gets private data for a registered event procedure on a result.     |
| `PQresultSetInstanceData`       | `int PQresultSetInstanceData(PGresult *res, PGEventProc proc, void *data);`                                                               | Sets private data for a registered event procedure on a result.     |
| `lo_import`                     | `Oid lo_import(PGconn *conn, const char *filename);`                                                                                      | Imports a Unix file into a large object.                            |
| `lo_export`                     | `int lo_export(PGconn *conn, Oid lobjId, const char *filename);`                                                                          | Exports a large object to a Unix file.                              |
| `lo_create`                     | `Oid lo_create(PGconn *conn, Oid lobjId);`                                                                                                | Creates a new large object.                                         |
| `lo_open`                       | `int lo_open(PGconn *conn, Oid lobjId, int mode);`                                                                                        | Opens a large object for reading/writing, returns a descriptor.     |
| `lo_write`                      | `int lo_write(PGconn *conn, int fd, const char *buf, size_t len);`                                                                        | Writes data to an open large object.                                |
| `lo_read`                       | `int lo_read(PGconn *conn, int fd, char *buf, size_t len);`                                                                               | Reads data from an open large object.                               |
| `lo_lseek` / `lo_lseek64`       | `int lo_lseek(PGconn *conn, int fd, int offset, int whence);` / `pg_int64 lo_lseek64(PGconn *conn, int fd, pg_int64 offset, int whence);` | Seeks within a large object (32-bit / 64-bit offset).               |
| `lo_tell` / `lo_tell64`         | `int lo_tell(PGconn *conn, int fd);` / `pg_int64 lo_tell64(PGconn *conn, int fd);`                                                        | Returns the current seek position in a large object.                |
| `lo_truncate` / `lo_truncate64` | `int lo_truncate(PGconn *conn, int fd, size_t len);` / `int lo_truncate64(PGconn *conn, int fd, pg_int64 len);`                           | Truncates a large object to a given length.                         |
| `lo_close`                      | `int lo_close(PGconn *conn, int fd);`                                                                                                     | Closes an open large object descriptor.                             |
| `lo_unlink`                     | `int lo_unlink(PGconn *conn, Oid lobjId);`                                                                                                | Deletes a large object.                                             |

---

## 12. Environment / Threading Notes

- Most libpq functions are thread-safe as long as no two threads share the same `PGconn` object simultaneously.
- `PQexec`-family calls block; use the `PQsend*` / `PQgetResult` / `PQconsumeInput` / `PQisBusy` family for non-blocking, event-loop-friendly code.
- Every `PGresult*` returned by libpq must eventually be passed to `PQclear()`, and every `PGconn*` must eventually be passed to `PQfinish()`, to avoid leaks.

---

_This reference summarizes the public libpq C API as of recent PostgreSQL versions. Exact signatures, constants, and newer additions (e.g., pipeline mode, chunked mode, `PGcancelConn`) can vary by PostgreSQL version — consult the official docs at https://www.postgresql.org/docs/current/libpq.html for authoritative signatures._
