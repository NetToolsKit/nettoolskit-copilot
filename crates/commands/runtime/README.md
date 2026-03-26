# nettoolskit-runtime

> Runtime command boundary contracts for the PowerShell-to-Rust migration.

## Purpose

This crate locks the owned runtime surfaces before full implementation lands. It is the Rust home for:

- runtime operator commands
- runtime lifecycle hooks
- maintenance operator flows
- git hook install and hygiene flows

## Scope

The current contract covers `54` legacy PowerShell scripts:

- `42` runtime commands
- `4` runtime hooks
- `5` maintenance commands
- `3` git hook commands

## Public API

- `RUNTIME_SURFACE_CONTRACTS`
- `runtime_surface_contract`
- `require_runtime_surface_contract`
- `runtime_surface_script_total`