---
applyTo: "**/*.{cs,csproj,sln,slnf,props,targets}"
priority: high
---

# .NET And C# Backend Implementation

Use this instruction for `.NET` and `C#` implementation details only. Clean
architecture and backend platform behavior are defined in:

- `ntk-backend-architecture-core.instructions.md`
- `ntk-backend-architecture-platform.instructions.md`

Do not restate those generic backend rules here unless a `.NET`-specific
constraint changes how they must be implemented.

## Code Organization

- Follow the repository class and interface templates:
  - `.github/templates/dotnet-class-template.cs`
  - `.github/templates/dotnet-interface-template.cs`
- Keep namespaces aligned with folder structure.
- Use small, focused types and methods.
- Avoid god classes, mixed concerns, and deep utility dumping grounds.
- Respect the repository EOF policy from `.editorconfig`.

### Region Layout

Use `#region` only when it improves scanability and the region contains real
members.

- `#region Constants`
- `#region Static Variables`
- `#region Static Properties`
- `#region Variables`
- `#region Protected Properties`
- `#region Public Properties`
- `#region Internal Properties`
- `#region Constructors`
- `#region Public Methods/Operators`
- `#region Protected Methods/Operators`
- `#region Internal Methods/Operators`
- `#region Private Methods/Operators`

Formatting rules:

- no blank line immediately after `#region`
- no blank line immediately before `#endregion`
- exactly one blank line between adjacent regions

## Async And Performance

- Use async/await consistently for I/O.
- Pass `CancellationToken` through async call chains.
- Use `ConfigureAwait(false)` in library code when appropriate.
- Use `Task.Run` only for genuine CPU-bound offloading.
- Consider `ValueTask` only on hot paths with measured value.
- Prefer `StringBuilder`, pooling, `Span<T>`, or `ArrayPool<T>` only when profiling justifies it.

## Error Handling And APIs

- Use structured exceptions and narrow try/catch blocks.
- Return `ProblemDetails` or equivalent structured error contracts for HTTP APIs.
- Log with correlation identifiers and meaningful context.
- Fail fast on invalid inputs and configuration.

## Testing Patterns

### Common Test Rules

- Test names must follow `{What}_{How}_{Result}` in PascalCase.
- One test should validate one behavior.
- Use AAA structure and deterministic assertions.
- Do not add XML summaries to test methods.

### Framework Metadata

- NUnit: use `[TestFixture]`, `[Category(\"...\")]`, and threading metadata when required.
- xUnit: use `[Trait(\"...\", \"...\")]`, `[Collection(\"...\")]`, and `ITestOutputHelper` when output matters.

### Templates And Layout

- Unit tests: `.github/templates/dotnet-unit-test-template.cs`
- Integration tests: `.github/templates/dotnet-integration-test-template.cs`
- File layout:
  - `tests/<Project>.UnitTests/Tests/*Tests.cs`
  - `tests/<Project>.IntegrationTests/Tests/*Tests.cs`

### Test Region Layout

- `#region Nested types`
- `#region Variables`
- `#region Constructors`
- `#region SetUp Methods`
- `#region Test Methods - [MethodUnderTest]`
- `#region Test Methods - [MethodUnderTest] Valid Cases`
- `#region Test Methods - [MethodUnderTest] Invalid Cases`
- `#region Test Methods - [MethodUnderTest] Edge Cases`
- `#region Test Methods - [MethodUnderTest] Exception Cases`
- `#region Private Methods/Operators`

Do not use generic region names like `CanHandle Tests` or `RoundTrip Tests`.

## .NET Data And Messaging

- Prefer Fluent API for EF Core model configuration.
- Use scoped `DbContext` lifetimes.
- Use `AsNoTracking()` for read-only queries when appropriate.
- Keep transactions explicit around write use cases.
- Maintain versioned, reviewable migrations.
- Use MediatR only where handler-based workflows materially improve clarity.
- Keep pipeline behaviors focused on cross-cutting concerns.

## Hosted Services And DI

- Use `BackgroundService` or `IHostedService` intentionally.
- Create scopes explicitly for scoped dependencies inside background workers.
- Register service lifetimes deliberately:
  - `AddScoped` for request/use-case scoped behavior
  - `AddSingleton` for immutable/shared configuration or infrastructure
  - `AddTransient` for stateless lightweight services
- Avoid service locator patterns.

## HTTP, Config, And Security

- Prefer `HttpClientFactory` with typed or named clients.
- Keep retry and timeout behavior explicit and measured.
- Use strongly typed configuration via `IOptions` patterns.
- Validate configuration at startup when failure would be fatal.
- Never hardcode secrets.
- Keep authentication, authorization, CORS, and rate limiting explicit in startup/runtime configuration.

## Logging, Metrics, And Docs

- Use structured logging with stable property names.
- Filter sensitive values from logs and telemetry.
- Expose health endpoints and metrics where the runtime requires them.
- Apply XML documentation to non-test code across all accessibility levels when the repository or project expects it.
- Use `<inheritdoc />` for overrides and interface implementations where appropriate.