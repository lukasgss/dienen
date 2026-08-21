<div align="center">
    <img height="200" src="./docs/logo/logo.png">
</div>

# dienen

A database CLI that aims to support multiple providers.

dienen is a command-line client for relational databases. It connects to a
database, runs SQL statements against it and prints the results.

The goal is to provide one client that works across different database
engines, so the same commands and output apply whether the database behind it
is PostgreSQL, MySQL or something else.

## Inspiration project

This project is inspired on [pgcli](https://github.com/dbcli/pgcli), but written in Rust instead of Python
