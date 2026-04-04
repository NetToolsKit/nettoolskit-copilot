---
applyTo: "**/{db,database,data,migrations,sql,prisma}*/**/*.{sql,psql,pgsql,mysql,db,prisma,json,yaml,yml}"
priority: medium
---

# Database Schema and Query Design

Use this instruction for schema design, normalization, query shape, indexing,
transaction semantics, concurrency control, and data-model choices.

Use adjacent `data-security` instructions for other concerns:

- `ntk-data-database-configuration-operations.instructions.md` for connection settings, pooling, failover, backup/restore, and operational database policy
- `ntk-data-orm.instructions.md` for ORM mapping, repository boundaries, projections, and persistence-layer conventions
- `ntk-security-vulnerabilities.instructions.md` for cross-layer security and dependency vulnerability policy

## Schema Design

- Keep table, column, index, and constraint naming deterministic and in English.
- Choose naming conventions that stay consistent across migrations and diagnostics.
- Model keys, uniqueness, and referential integrity explicitly.
- Keep schema changes reviewable and aligned with domain boundaries.

## Normalization and Model Shape

- Apply normalization intentionally and denormalize only with measurable justification.
- Keep write models and read models explicit when they diverge.
- Avoid encoding business ambiguity into weak or overloaded columns.
- Treat partitioning and archive strategy as deliberate workload decisions, not defaults.

## Query Design

- Avoid unbounded scans, accidental cartesian products, and broad `SELECT *` usage on important paths.
- Prefer SARGable predicates and predictable filter patterns.
- Use pagination and seek/keyset patterns for large ordered lists where offset cost is material.
- Keep query shape aligned with actual access patterns instead of generic “one query for everything” designs.

## Index Strategy

- Create indexes for real filter, join, sort, and uniqueness needs.
- Prefer composite index order based on actual selectivity and query patterns.
- Revisit unused or redundant indexes regularly.
- Treat covering indexes and include columns as workload-specific tools, not defaults.

## Transactions and Consistency

- Use explicit transaction boundaries for multi-step writes.
- Keep isolation levels aligned with correctness and contention requirements.
- Avoid long-running transactions that mix interactive latency with batch work.
- Prefer forward-only migration strategy and reentrant scripts where deployment re-entry is possible.

## Concurrency Control

- Choose optimistic or pessimistic coordination intentionally.
- Use row versioning, compare-and-swap, or explicit lock semantics where the domain requires them.
- Design retry behavior with idempotency in mind.
- Keep hot-row and hot-partition risks visible in the schema and workload design.

## Data Types and Storage Semantics

- Use data types that match real precision, scale, range, and collation needs.
- Keep time and timezone semantics explicit.
- Treat large objects, computed columns, and surrogate keys as design decisions with operational impact.
- Keep document/NoSQL modeling aligned with actual access patterns and partition-key constraints.

## Query and Schema Verification

- Validate plans, cardinality assumptions, and dataset-scale behavior on meaningful workloads.
- Test migrations and query regressions against realistic data shape where the risk justifies it.
- Keep runtime configuration, backup/restore, and ORM mapping policy out of this instruction and in the specialized surfaces.