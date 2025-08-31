#!/bin/bash
# Guitar Practice App - Quick Development Commands
# Usage: ./x <command>
# 
# Commands:
#   dev, d     - Start both frontend and backend
#   frontend, f - Frontend only
#   backend, b  - Backend only
#   test, t     - Run all tests
#   check, c    - Check all workspaces
#   build       - Build frontend for production

exec cargo run --package xtask -- "$@"
