#!/bin/bash

source .venv/bin/activate

(cd rust_backend && cargo watch -s "maturin develop")
