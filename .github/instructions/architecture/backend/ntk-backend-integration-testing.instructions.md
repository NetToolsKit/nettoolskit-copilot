---
applyTo: "**/{integration,e2e,api,test,testcontainers,spec}*/**/*.{cs,ts,js,json,yml,yaml,config,sql}"
priority: high
---

# Backend Integration And API Testing

Use this instruction for API integration tests, service-to-service validation,
Testcontainers, consumer-contract checks, persistence-backed tests, and backend
test environment control. Keep Rust crate-specific test structure in
`ntk-backend-rust-testing.instructions.md`, frontend/browser E2E in
`ntk-frontend-e2e-testing.instructions.md`, and cross-cutting TDD workflow in
`instructions/process/delivery/ntk-process-tdd-verification.instructions.md`.

## Frameworks

- Prefer xUnit or NUnit for `.NET` integration tests.
- Prefer Testcontainers for realistic database, broker, cache, or dependency
  orchestration when external dependencies are part of the contract.
- Prefer HTTP/application-host test harnesses over mocking the entire request
  pipeline when endpoint behavior is under test.
- Keep Playwright/Cypress out of backend-only scopes unless the browser flow is
  the thing being validated.

## Integration Test Boundaries

- Validate public behavior across API, persistence, messaging, and external
  adapter seams.
- Keep contract, schema, and serialization expectations explicit.
- Verify authn/authz, tenancy, rate limit, error payload, and idempotency
  behavior on critical endpoints.
- Prefer deterministic test fixtures and isolated environment state.

## Test Data And Environment

- Use factories/builders for request payloads and persisted test data.
- Seed only the minimum dataset required by the scenario.
- Reset or recreate mutable state between tests when order independence matters.
- Keep connection strings, tokens, and environment-specific secrets outside the
  repository and test logs.

## API And Contract Coverage

- Validate success, validation failure, authorization failure, and boundary
  conditions for externally visible endpoints.
- Check schema compliance, problem-details payloads, and versioned route
  behavior where applicable.
- Include security headers, CORS, pagination, filtering, and resource ownership
  checks when the endpoint contract depends on them.

## Performance And Reliability In Integration Tests

- Use targeted response-time or resource assertions only on flows that justify
  them.
- Cover timeout, retry, circuit-breaker, and failover behavior where the backend
  contract depends on resilience policy.
- Prefer focused integration coverage over accidental load testing inside the
  normal test suite.

## CI And Diagnostics

- Keep integration tests shardable and environment-aware in CI.
- Publish logs, container diagnostics, and failure artifacts when the scenario
  is hard to reproduce locally.
- Make slow or privileged integration suites explicit in naming and execution
  strategy.