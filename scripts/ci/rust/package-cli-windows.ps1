#!/usr/bin/env pwsh
# Package CLI binary as zip archive (Windows)
# Used by: ci-rust.yaml - Package CLI (Windows) step
# Arguments: TARGET (e.g., x86_64-pc-windows-msvc)

param(
    [Parameter(Mandatory=$true)]
    [string]$Target
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

Write-Host "=== Packaging CLI binary for $Target ==="

cd target/$Target/release
Compress-Archive -Path xberg.exe -DestinationPath ../../../xberg-cli-$Target.zip

Write-Host "Packaging complete: xberg-cli-$Target.zip"
