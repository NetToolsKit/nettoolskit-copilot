# Skill Definitions

> Canonical reusable skill packs aligned to stable engineering role boundaries.

---

## Purpose

`definitions/skills/` stores reusable specialist capability packs that can be projected into provider runtimes without rewriting their intent each time.

Current root lanes:

- `dev-backend/`
- `dev-frontend/`
- `dev-rust/`
- `test/`
- `security/`
- `docs/`

---

### Architecture

```mermaid
graph TD
    ROOT["definitions/skills/"]
    BACKEND["dev-backend/"]
    FRONTEND["dev-frontend/"]
    RUST["dev-rust/"]
    TEST["test/"]
    SECURITY["security/"]
    DOCS["docs/"]

    ROOT --> BACKEND
    ROOT --> FRONTEND
    ROOT --> RUST
    ROOT --> TEST
    ROOT --> SECURITY
    ROOT --> DOCS
```

---

## Notes

- Skill roots should stay capability-oriented, not provider-oriented.
- Provider-specific packaging belongs under `definitions/providers/`.

---