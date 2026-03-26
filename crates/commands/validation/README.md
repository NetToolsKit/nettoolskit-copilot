# nettoolskit-validation

> Validation command boundary contracts for the PowerShell-to-Rust migration.

## Purpose

This crate locks the owned validation surfaces before full implementation lands. It is the Rust home for:

- repository validation commands
- security gates
- governance automation
- documentation validation
- deploy preflight checks

## Scope

The current contract covers `41` legacy PowerShell scripts:

- `31` validation commands
- `6` security commands
- `2` governance commands
- `1` documentation command
- `1` deploy preflight command

## Public API

- `VALIDATION_SURFACE_CONTRACTS`
- `validation_surface_contract`
- `require_validation_surface_contract`
- `validation_surface_script_total`