# AI Development Operator Playbook

## Purpose

Operate the `ntk` AI development surfaces locally with predictable profile selection, diagnostics, and fallback behavior before executing live provider requests.

## Scope

This playbook covers:

- built-in AI provider profiles
- `ntk ai doctor`
- Markdown and JSON operator reports
- local vs remote provider guidance
- degraded/fallback troubleshooting

It does not replace deeper provider-integration, token-economy, or multi-agent design documents under `planning/`.

## Built-In Profiles

Use `NTK_AI_PROFILE` to select a stable preset before running AI flows.

| Profile | Goal | Network | Primary characteristics |
| --- | --- | --- | --- |
| `balanced` | day-to-day development | required | cheap ask routing plus stronger reasoning for heavier intents |
| `coding` | deeper coding/planning flows | required | longer timeout budgets and stronger reasoning bias |
| `cheap` | cost reduction | required | keeps all intents on the lightweight model tier |
| `latency` | faster responses | required | short timeout budgets and lightweight model selection |
| `local` | offline-safe workflows | not required | stays on the deterministic local/mock provider chain |

Inspection commands:

```powershell
ntk ai profiles list
ntk ai profiles show
ntk ai profiles show coding
```

Example profile activation:

```powershell
$env:NTK_AI_PROFILE = "coding"
ntk ai profiles show
```

## Fast Bootstrap

### Offline-safe local mode

```powershell
$env:NTK_AI_PROFILE = "local"
ntk ai doctor
```

Expected result:

- profile resolves as `local`
- provider chain stays on `mock`
- runtime status is local-only rather than degraded

### Remote development mode

```powershell
$env:NTK_AI_PROFILE = "balanced"
$env:NTK_AI_PROVIDER_ENDPOINT = "https://api.openai.com/v1/chat/completions"
$env:NTK_AI_PROVIDER_API_KEY = "<secret>"
ntk ai doctor
```

Expected result:

- profile resolves as `balanced`
- provider chain begins with `openai-compatible`
- runtime status is `ready` when auth and endpoint configuration are present

## Doctor Workflow

Run the doctor before debugging request execution or changing provider configuration.

### Human-readable terminal output

```powershell
ntk ai doctor
```

Use this for quick local inspection of:

- active profile
- provider chain
- provider endpoint
- timeout budgets
- model defaults
- auth presence
- fallback readiness

### JSON output

```powershell
ntk ai doctor --json-output
```

Use JSON when:

- wiring local automation
- capturing state in scripts
- comparing provider/profile changes over time

### Markdown report

```powershell
ntk ai doctor --report-path .build/reports/ai-doctor.md
```

Use the Markdown report when:

- attaching diagnostics to an operator handoff
- recording a degraded runtime state before remediation
- keeping a troubleshooting snapshot outside terminal history

## Local vs Remote Guidance

Use `local` when:

- developing offline
- validating CLI orchestration without remote dependencies
- reproducing request-shaping behavior without provider variance

Use `balanced` when:

- doing normal day-to-day repo work
- you want remote execution with conservative defaults

Use `coding` when:

- the task is planning-heavy
- the task is code-generation-heavy
- you can tolerate longer primary timeout budgets

Use `cheap` when:

- testing many short asks
- validating low-cost routing behavior
- you prefer cost ceilings over answer depth

Use `latency` when:

- feedback speed matters more than depth
- you need shorter timeout budgets to surface bad routes quickly

## Fallback and Degraded-State Troubleshooting

### Doctor reports `local_only`

Meaning:

- the active profile is intentionally local
- remote auth is not required for the current profile

Action:

- no remediation needed unless you expected a remote profile

### Doctor reports `degraded`

Common causes:

- remote profile selected without API key
- remote endpoint missing
- provider chain resolves to remote-first but the runtime is not fully configured

Recommended sequence:

1. confirm the active profile with `ntk ai profiles show`
2. confirm the provider endpoint and auth state with `ntk ai doctor`
3. switch to `local` if you need deterministic offline progress
4. restore remote env vars and rerun `ntk ai doctor`

### Doctor reports `ready` but requests still fail

Likely causes:

- provider-side outage or throttling
- timeout budget too short for the chosen profile
- model override mismatch

Recommended sequence:

1. keep the current report as evidence with `--report-path`
2. try `balanced` or `coding` if `latency` is too aggressive
3. try `local` to isolate runtime orchestration from provider behavior
4. review fallback expectations before changing budget or provider env vars

## Recommended Operator Loop

For day-to-day development:

1. select or confirm the profile
2. run `ntk ai doctor`
3. capture a Markdown report when the state is degraded
4. only then run higher-cost AI workflows

Minimal loop:

```powershell
$env:NTK_AI_PROFILE = "balanced"
ntk ai doctor
```

Troubleshooting loop:

```powershell
ntk ai doctor --report-path .build/reports/ai-doctor.md
$env:NTK_AI_PROFILE = "local"
ntk ai doctor
```

## References

- [Repository README](../../README.md)
- [crates/cli/README.md](../../crates/cli/README.md)
- [crates/orchestrator/README.md](../../crates/orchestrator/README.md)
- [planning/active/plan-development-agent-orchestrator-experience.md](../../planning/active/plan-development-agent-orchestrator-experience.md)