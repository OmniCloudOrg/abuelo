#!/bin/bash

# Clean any previous test artifacts
rm -f test_user_db.db3

# Run the tests with verbose output
cargo test -- --nocapture