#!/bin/bash
set -e

psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" <<-EOSQL
    create role chirpstack_test with login password 'chirpstack_test';
    create database chirpstack_test with owner chirpstack_test;
EOSQL
