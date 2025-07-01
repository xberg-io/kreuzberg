# Performance Baseline

This directory contains baseline performance metrics for the Kreuzberg library.

## Files

- `baseline.json` - Performance baseline automatically updated from main branch CI
- This file is used for performance regression detection in PRs

## How it works

1. When code is pushed to `main`, CI runs benchmarks and stores results as `baseline.json`
1. When PRs are opened, CI compares current performance against this baseline
1. If performance degrades beyond threshold (20%), the CI check fails
1. The baseline is automatically updated when new changes are merged to main
