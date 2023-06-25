#!/bin/bash

set -eux

cargo build && cargo test
