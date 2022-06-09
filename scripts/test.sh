#!/bin/bash
set -e

chmod +x scripts/test.sh 

cargo test -- --nocapture
