# NetToolsKit CLI — Deep Analysis Report

> **Data:** Julho/2025
> **Escopo:** Análise de TODO o código-fonte em 10 crates do workspace
> **Ferramenta binária:** `ntk`

---

## Sumário Executivo

O `nettoolskit-cli` é uma ferramenta de linha de comando escrita em Rust para geração de código baseada em manifestos DDD, com suporte a templating multi-linguagem (Handlebars), tradução de templates entre linguagens, e uma TUI interativa. O projeto está na **versão 1.0.0** mas apresenta comportamento de **beta tardia**: várias funcionalidades estão implementadas apenas como stubs, há um bloco `unsafe` com **Undefined Behavior** confirmado, incompatibilidades de versão entre dependências, e bugs de resize no terminal. A arquitetura de crates é bem segmentada e segue boas práticas de separação de responsabilidades, mas a maturidade varia enormemente entre crates.

---

## 1. Inventário de Arquivos

### Fontes por crate (arquivos `.rs` em `src/`)

| Crate | Arquivos Fonte | LOC (aprox.) | Responsabilidade |
|---|---|---|---|
| **cli** | 5 | ~690 | Ponto de entrada (Clap), modo interativo, input loop, display, eventos |
| **core** | 11+ | ~800 | Exit codes, feature flags, traits de menu, async utils, file search, path/string utils |
| **ui** | 16 | ~1.550 | Terminal layout, paleta de comandos, menus, box rendering, writer, cores, estilos |
| **otel** | 3 | ~340 | Métricas customizadas, Timer, configuração de tracing |
| **orchestrator** | 6 | ~520 | Executor async, processador de comandos, modelo MainAction |
| **commands** (meta) | 1 | ~10 | Re-exporta help/manifest/translate |
| **help** | 5 | ~250 | Descoberta de manifestos via walkdir |
| **manifest** | ~25+ | ~1.800 | Parser YAML, executor, rendering, apply, check, task generators, 38 model structs, menu |
| **templating** | ~12 | ~950 | Handlebars engine+cache, batch renderer paralelo, resolver 4-estratégias, strategies por linguagem |
| **translate** | 5 | ~370 | Requisição de tradução, handler com pipeline de validação, suporte .NET |

**Total estimado:** ~90+ arquivos fonte, ~7.300 LOC de produção

### Testes por crate

| Crate | Arquivos de Teste | Testes (`#[test]` / `#[tokio::test]`) | Cobertura qualitativa |
|---|---|---|---|
| **cli** | 5 | ~5 | **Muito fraca** — testes smoke apenas (`print_logo()` não paniqueia) |
| **core** | 20+ | ~65 | **Boa** — features, async utils, file search, menu, string/path utils |
| **ui** | 18+ | ~55 | **Média** — writer, box, enum_menu, colors, style; **terminal_tests.rs é vazio** |
| **otel** | 4 | ~16 | **Média** — telemetria, tracing setup, erros |
| **orchestrator** | 6 | ~30 | **Boa** — executor async, processor, main_action, erros |
| **manifest** | 42 | ~60+ | **Boa** — tasks, parsing, integration, workflow, dry_run, handlers |
| **templating** | 30+ | ~50+ | **Boa** — engine, batch, resolver, strategies, error |
| **translate** | 8 | ~13 | **Média** — basic/error/integration translation |

**Total estimado:** ~150+ arquivos de teste, ~300+ test cases

---

## 2. Grafo de Dependências

```
                        ┌─────────┐
                        │   cli   │  (binary: ntk)
                        └────┬────┘
                             │
            ┌────────────────┼─────────────────┐
            │                │                  │
            ▼                ▼                  ▼
     ┌──────────────┐  ┌──────────┐      ┌──────────┐
     │ orchestrator  │  │   otel   │      │   ui     │
     └──────┬───────┘  └────┬─────┘      └────┬─────┘
            │               │                  │
    ┌───────┼───────┐       │                  │
    │       │       │       │                  │
    ▼       ▼       ▼       ▼                  ▼
┌──────┐ ┌──────┐ ┌────────────┐          ┌──────────┐
│ help │ │manif.│ │ translate  │          │   core   │
└──┬───┘ └──┬───┘ └─────┬──────┘         └──────────┘
   │        │            │                     ▲
   │        ▼            │                     │
   │   ┌───────────┐     │     Todos dependem ─┘
   │   │templating │◄────┘
   │   └───────────┘
   │        │
   └────────┘
```

### Dependências externas chave

| Categoria | Crates | Versão |
|---|---|---|
| Runtime | `tokio` | 1.34 (multi-thread, 4 workers) |
| CLI | `clap` + `clap_complete` | 4.4 |
| Terminal | `crossterm` | 0.28 |
| Prompts | `inquire` | 0.9 |
| Cores | `owo-colors` | 3.5 |
| Enums | `strum`/`strum_macros` | 0.25 (workspace) / **0.26 (usado)** |
| Templates | `handlebars` | 4.5 (workspace) / **6.2 (usado)** |
| Serialização | `serde` + `serde_yaml` + `serde_json` | 1.0 |
| Concorrência | `dashmap` | (via templating) |
| Observabilidade | `tracing` + `tracing-subscriber` | 0.1 / 0.3 |
| File system | `walkdir` + `ignore` + `globset` | 2.4 / 0.4 / 0.4 |
| Erros | `thiserror` + `anyhow` | 1.0 |

---

## 3. Avaliação Arquitetural

### 3.1 Padrões identificados

| Padrão | Onde | Qualidade |
|---|---|---|
| **Strategy** | `LanguageStrategy` trait + `LanguageStrategyFactory` (6 linguagens) | Boa — extensível via trait |
| **Builder** | `BoxConfig`, `CommandPaletteBuilder`, `SearchConfig` | Boa — API ergonômica |
| **RAII Guard** | `RawModeGuard` (raw terminal cleanup) | Boa — garante `disable_raw_mode()` |
| **Command dispatch** | `process_command()` com pattern matching em prefixos | Adequada — mas frágil (strings hardcoded) |
| **DDD Task Generation** | Domain/Application/API layers com `RenderTask` | Boa — alinhada com Clean Architecture |
| **Multi-strategy resolution** | `TemplateResolver` com 4 níveis de fallback | Boa — resiliente |
| **Async execution** | `AsyncCommandExecutor` + `JoinSet` batch rendering | Boa — tokio idiomático |

### 3.2 Pontos fortes

1. **Separação de crates bem definida** — cada crate tem responsabilidade clara
2. **Feature flags** defensivos — TUI experimental desabilitada por default, overrides via env vars (`NTK_USE_*`)
3. **Modelo de dados robusto** — 38 model structs cobrindo todo o domínio DDD
4. **Testes abrangentes no core** — features, async utils, file search bem cobertos
5. **Clippy rigoroso** — all/pedantic/nursery/cargo em `warn`, `unsafe_code` em `forbid`
6. **Template engine com cache** — `DashMap` lock-free para templates + paths

### 3.3 Debilidades arquiteturais

1. **Inconsistência de error handling**: `thiserror` (manifest, translate), `anyhow` (help, file-search), manual `impl Display+Error` (templating) — 3 estratégias diferentes
2. **Observabilidade ilusória**: crate `otel` NÃO usa OpenTelemetry SDK real — é um wrapper customizado sobre `HashMap<String, _>` com `Mutex`
3. **Acoplamento via re-exports**: `manifest/src/lib.rs` re-exporta ~20 itens; `manifest/src/models/mod.rs` re-exporta ALL `core::models` para backward compat — cria surface area desnecessária
4. **Comando translate parcialmente implementado**: `process_command()` no orchestrator apenas imprime "Translation stub" — nem chama `handle_translate()`
5. **ManifestAction potencialmente duplicado**: definido em `models/manifest_action.rs` E referenciado em `core/definitions.rs` — risco de divergência

---

## 4. Problemas de Qualidade de Código

### CRÍTICO — Severidade P0

#### 4.1 **Undefined Behavior: unsafe em `TemplateEngine`**
- **Arquivo:** [templating/src/rendering/engine.rs](crates/commands/templating/src/rendering/engine.rs#L148)
- **Problema:** Cast de `Arc::as_ptr()` para `*mut Handlebars` para obter referência mutável
  ```rust
  let handlebars_mut = unsafe {
      let ptr = Arc::as_ptr(&handlebars_clone) as *mut Handlebars;
      &mut *ptr
  };
  ```
- **Por que é UB:** Viola regras de aliasing do Rust — `Arc` garante acesso compartilhado (`&T`), nunca exclusivo (`&mut T`). Múltiplas threads podem acessar simultaneamente via `Arc::clone()`, causando data race.
- **Agravante:** `Cargo.toml` declara `unsafe_code = "forbid"` — este código deveria falhar na compilação. Se compila, o lint está sendo overridden localmente.
- **Fix recomendado:** Usar `Arc<RwLock<Handlebars>>` ou `parking_lot::RwLock` para mutação segura. Alternativamente, se registration é apenas na inicialização, registrar todos os templates antes de compartilhar o `Arc`.

#### 4.2 **Incompatibilidade de versão: handlebars**
- **Workspace declara:** `handlebars = "4.5"`
- **templating usa:** `handlebars = "6.2"` (override local)
- **Impacto:** API breaking changes entre 4.x e 6.x do Handlebars — potencial confusão ao adicionar novos consumers, link errors se outro crate tentar usar a versão do workspace
- **Fix:** Alinhar workspace para `6.2` ou manter override explícito documentado

#### 4.3 **Incompatibilidade de versão: strum**
- **Workspace declara:** `strum = "0.25"`, `strum_macros = "0.25"`
- **core e ui usam:** `0.26`
- **Fix:** Atualizar workspace para `0.26`

### ALTO — Severidade P1

#### 4.4 **Timer double-record bug**
- **Arquivo:** [otel/src/telemetry.rs](crates/otel/src/telemetry.rs)
- **Problema:** `Timer::stop()` chama `record_timing()`. O `Drop` impl TAMBÉM chama `record_timing()`. Se o usuário chama `stop()` e depois o timer sai de escopo, a métrica é gravada 2x.
- **Fix:** Adicionar flag `recorded: bool` no `Timer` para prevenir double-record.

#### 4.5 **`check_file()` é placeholder**
- **Arquivo:** [manifest/src/handlers/check.rs](crates/commands/manifest/src/handlers/check.rs#L42)
- **Status:** Apenas verifica se o arquivo YAML existe — não valida schema, constraints ou consistência
- **Comentário no código:** "Status: Placeholder implementation — to be completed in Phase 2.4"

#### 4.6 **.unwrap() excessivo em terminal.rs**
- **Arquivo:** [ui/src/interaction/terminal.rs](crates/ui/src/interaction/terminal.rs)
- **Contagem:** 14+ chamadas `.lock().unwrap()` em `Mutex` — qualquer panic em thread adjacente envenena o Mutex e causa crash cascata
- **Fix:** Substituir por `.lock().unwrap_or_else(|e| e.into_inner())` ou usar `parking_lot::Mutex` (que não envenena)

#### 4.7 **helpers.rs é placeholder vazio**
- **Arquivo:** [templating/src/core/helpers.rs](crates/commands/templating/src/core/helpers.rs)
- **Conteúdo:** `const PLACEHOLDER: () = ()` — comentário menciona "future custom helpers (e.g., to_kebab, to_snake, to_pascal_case)"
- **Impacto:** Sem helpers customizados, templates precisam trabalhar apenas com Handlebars built-ins

### MÉDIO — Severidade P2

#### 4.8 **Sistema de eventos não utilizado**
- **Arquivo:** [cli/src/events.rs](crates/cli/src/events.rs)
- **Problema:** `CliEvent` enum e `EventSender` implementados mas nunca utilizados — `input.rs` usa polling direto do crossterm
- **Fix:** Remover código morto ou integrar com o input loop

#### 4.9 **Código comentado: ensure_layout_guard()**
- **Arquivo:** [cli/src/lib.rs](crates/cli/src/lib.rs)
- **Problema:** Chamada a `ensure_layout_guard()` comentada com `// BUG-WORKAROUND: ...`
- **Impacto:** Funcionalidade de layout guard desabilitada, possivelmente contribuindo para bugs de resize

#### 4.10 **translate retorna Success para linguagens não implementadas**
- **Arquivo:** [translate/src/handlers/translate.rs](crates/commands/translate/src/handlers/translate.rs)
- **Problema:** Quando target != .NET, retorna `ExitStatus::Success` com mensagem "not yet implemented" — deveria ser `ExitStatus::Error`
- **Impacto:** Scripts de automação não detectam falha

#### 4.11 **artifact.rs incompleto**
- **Arquivo:** [manifest/src/tasks/artifact.rs](crates/commands/manifest/src/tasks/artifact.rs)
- **Problema:** Apenas 3 de ~10+ `ArtifactKind` suportados (ValueObject, Entity, UseCaseCommand) — demais retornam `ManifestError`

#### 4.12 **Convenções de tradução hardcoded**
- **Arquivo:** [translate/src/handlers/translate.rs](crates/commands/translate/src/handlers/translate.rs)
- **Problema:** Mapeamento `{{class_name}}` → `{{ClassName}}` via string replace hardcoded — frágil e não extensível
- **Fix:** Usar o `LanguageStrategyFactory` da crate templating para resolver convenções

#### 4.13 **Deprecated color aliases**
- **Arquivo:** [ui/src/core/colors.rs](crates/ui/src/core/colors.rs#L55-L64)
- **4 aliases deprecated** desde v0.1.0 — considerar remoção na v2.0

#### 4.14 **process_text() é stub**
- **Arquivo:** [orchestrator/src/execution/processor.rs](crates/orchestrator/src/execution/processor.rs)
- **Problema:** Qualquer input que não é comando `/` apenas imprime "to be implemented"

---

## 5. Bugs de Resize no Terminal (UI Crate)

**Arquivo principal:** [ui/src/interaction/terminal.rs](crates/ui/src/interaction/terminal.rs) (~604 linhas)

### Arquitetura do layout

`TerminalLayout` usa um `Arc<Mutex<LayoutState>>` com `LayoutMetrics` (largura, altura, zonas: header, content, footer, prompt). Três `static` globais coordenam estado:
- `ACTIVE_LAYOUT: Mutex<Option<TerminalLayout>>` — layout ativo singleton
- `PENDING_LOGS: Mutex<Vec<String>>` — logs pendentes para flush
- `INTERACTIVE_MODE: AtomicBool` — flag de modo interativo

### Bugs identificados

| # | Bug | Localização | Severidade | Descrição |
|---|---|---|---|---|
| **R1** | Sem save/restore de cursor no `reconfigure()` | `terminal.rs` ~L370-400 | Alta | `reconfigure()` chama `clear_and_redraw()` que move o cursor para (0,0) via `MoveTo(0,0)` mas não salva posição anterior. Se resize ocorre durante input, o cursor pula para o topo e o prompt desaparece. |
| **R2** | `cursor::position()` durante resize | `terminal.rs` ~L288-310 | Alta | Métodos que usam `crossterm::cursor::position()` podem retornar valores stale durante resize rápido. O crossterm lê posição via escape sequence (`ESC[6n`) que pode interferir com output pendente. |
| **R3** | Sem debounce de resize | `cli/src/input.rs` ~L50-60 | Média | Cada `Event::Resize(w, h)` do crossterm chama `reconfigure()` imediatamente. Em resizes rápidos (drag de borda), isso causa N reconfigurações em sequência, visíveis como flicker. Solução: debounce de 50-100ms. |
| **R4** | Escape sequences raw causam cursor jumps | `terminal.rs` ~L470-530 | Média | `draw_header()`, `draw_footer()` usam `format!("\x1b[...m")` (escape sequences raw/literais) em vez de `crossterm::style::SetForegroundColor`. Isso pode conflitar com scroll regions e causar artefatos visuais em terminais que processam lentamente. |
| **R5** | Scroll region silenciosamente resetada | `terminal.rs` ~L377 | Média | Se terminal fica menor que `MIN_HEIGHT` (possivelmente definido), `reconfigure()` pode silenciosamente não configurar scroll region, resultando em output fora da área esperada sem feedback ao usuário. |
| **R6** | `ensure_layout_guard()` desabilitado | `cli/src/lib.rs` | Média | O guarda de layout que preveniria renderização inconsistente está comentado como workaround de bug — efetivamente desabilitando proteção durante resize. |

### Recomendações de fix para resize

1. **Debounce:** Acumular resize events por 80ms antes de chamar `reconfigure()`
2. **Save/Restore cursor:** `SavePosition` antes de redraw, `RestorePosition` depois
3. **Usar crossterm API:** Substituir raw `\x1b[` sequences por `crossterm::style` e `crossterm::cursor` commands
4. **Atomic redraw:** Usar `crossterm::execute!` em batch (buffer) em vez de writes individuais
5. **Reabilitar layout guard:** Investigar e corrigir o bug mencionado no workaround

---

## 6. Gaps de Teste

### Cobertura por camada

| Camada | Testado | Não testado |
|---|---|---|
| **core** | Features, async utils (cancellation + timeout), file search filters, menu traits, string/path utils, exit status, errors | — (cobertura boa) |
| **ui** | Writer, box component, enum_menu, colors, formatting, style, palette, integração | **Terminal resize/reconfigure** (terminal_tests.rs é smoke test vazio), menu interativo |
| **otel** | Telemetry counters/gauges/timings, tracing setup, errors | **Timer double-record scenario**, dump_diagnostics |
| **orchestrator** | Executor async (spawn, cancellation, progress), processor (help/quit/manifest), models (MainAction) | **Translate dispatch**, process_text |
| **cli** | Display (smoke) | **Input loop**, **resize handling**, **RawModeGuard**, **interactive_mode** — 2 testes triviais |
| **manifest** | Tasks (domain/app/api/artifact), parsing, integration, workflow, dry_run, handlers | **check_file()** (é placeholder), **apply real I/O** |
| **templating** | Engine (basic/edge/caching/variables/todo), batch (success/error/edge/concurrent), resolver, strategies, errors | **Unsafe code path** em engine, **helpers** (vazio), concurrent mutation |
| **translate** | Basic translation, error handling, integration | **Non-.NET targets**, convention edge cases |

### Gaps críticos

1. **Zero testes para resize do terminal** — `terminal_tests.rs` contém apenas `fn test_terminal_module_compiles() {}` (corpo vazio)
2. **Zero testes para o input loop** — `input_tests.rs` e `events_tests.rs` existem mas são minimais
3. **Zero testes para o bloco unsafe** do TemplateEngine — data race não seria pega nem por Miri (precisaria de stress test multi-thread)
4. **CLI crate quase sem testes** — 5 arquivos mas apenas ~5 testes triviais (smoke tests)
5. **Timer double-record não testado** — cenário stop() + Drop não aparece nos testes de otel
6. **Nenhum teste de snapshot/golden file** para output de terminal
7. **Nenhum benchmark** na pasta `benchmarks/` (existe no workspace mas está vazia)

---

## 7. Features Enterprise Ausentes

### Funcionalidades incompletas (stubs/placeholders)

| Feature | Status | Arquivo | Impacto |
|---|---|---|---|
| Translate (non-.NET) | Apenas stub com mensagem | translate/handlers | 5 de 6 linguagens não funcionam |
| Check/Validate | Placeholder | manifest/handlers/check.rs | Validação de manifesto não funcional |
| Menu Apply handler | TODO stub | manifest/ui/menu.rs:141 | Apply via menu interativo não funciona |
| Menu Validate handler | TODO stub | manifest/ui/menu.rs:69 | Validação via menu não funciona |
| Menu Render handler | TODO stub | manifest/ui/menu.rs:85 | Rendering via menu não funciona |
| process_text() | Placeholder | orchestrator/processor.rs | Free-text input ignorado |
| Template helpers | Arquivo vazio | templating/core/helpers.rs | Sem custom helpers (to_kebab, etc.) |
| Event system | Não conectado | cli/events.rs | Código morto — CliEvent nunca usado |
| Artifact generation | 3/10+ kinds | manifest/tasks/artifact.rs | Maioria dos artifacts não suportada |

### Features enterprise não implementadas

| Feature | Importância | Notas |
|---|---|---|
| **CI/CD pipeline** | Alta | Nenhum workflow GitHub Actions / Azure DevOps |
| **Cargo audit / security scanning** | Alta | Nenhum `cargo deny` ou `cargo audit` configurado |
| **Code coverage** | Alta | Sem `tarpaulin`, `llvm-cov` ou relatórios de cobertura |
| **Benchmarks** | Média | Pasta `benchmarks/` existe mas vazia — sem `criterion` |
| **Integration tests E2E** | Alta | Nenhum teste que execute o binário `ntk` como processo externo |
| **Logging structured** | Média | Tracing configurado mas sem exportação para backends (Grafana, ELK, OTel collector) |
| **Configuration file** | Média | Sem suporte a `~/.ntk/config.toml` ou `ntk.yaml` para preferências do usuário |
| **Plugin system** | Média | Strategies são hardcoded — sem mecanismo de loadable plugins |
| **Shell completions** | Baixa | `clap_complete` está como dependência mas nenhuma geração de completions implementada |
| **i18n** | Baixa | Mensagens hardcoded em inglês — sem framework de internacionalização |
| **Graceful degradation** | Média | Sem fallback para terminais sem suporte a cores/Unicode |
| **Release pipeline** | Alta | Sem `cargo-dist`, `release-plz` ou equivalente para releases automatizados |

---

## 8. Marcadores TODO / FIXME

### TODOs genuínos confirmados (grep em `crates/**/*.rs`)

| # | Arquivo | Linha | Marcador | Conteúdo |
|---|---|---|---|---|
| 1 | [manifest/src/ui/menu.rs](crates/commands/manifest/src/ui/menu.rs#L69) | 69 | TODO | "Implement actual validation" |
| 2 | [manifest/src/ui/menu.rs](crates/commands/manifest/src/ui/menu.rs#L85) | 85 | TODO | "Implement actual rendering" |
| 3 | [manifest/src/ui/menu.rs](crates/commands/manifest/src/ui/menu.rs#L141) | 141 | TODO | "Call the actual apply handler when available" |
| 4 | [manifest/src/handlers/check.rs](crates/commands/manifest/src/handlers/check.rs#L42) | 42 | TODO | "Full implementation pending" (Phase 2.4) |

### Workarounds ativos

| Arquivo | Tipo | Descrição |
|---|---|---|
| [cli/src/lib.rs](crates/cli/src/lib.rs) | BUG-WORKAROUND | `ensure_layout_guard()` comentado — funcionalidade de layout guard desabilitada |

### Código morto identificado

| Arquivo | Item | Razão |
|---|---|---|
| [cli/src/events.rs](crates/cli/src/events.rs) | `CliEvent`, `EventSender` | Nunca referenciado pelo input loop — sistema de polling direto usado em vez disso |
| [ui/src/core/colors.rs](crates/ui/src/core/colors.rs) | 4 aliases deprecated | `MAIN_PURPLE`, `SECONDARY_PURPLE`, `MAIN_WHITE`, `SECONDARY_GRAY` — deprecated desde v0.1.0 |

---

## Apêndice A — Plano de Ação Priorizado

### P0 — Correções imediatas (segurança/corretude)

1. **Eliminar unsafe em `TemplateEngine`** — Substituir por `Arc<RwLock<Handlebars<'static>>>` ou registrar templates antes de compartilhar
2. **Alinhar versões** — `handlebars = "6.2"` e `strum = "0.26"` no workspace
3. **Corrigir Timer double-record** — Adicionar flag `recorded` no struct

### P1 — Estabilidade (próximo sprint)

4. **Debounce de resize** — Timer de 80ms antes de `reconfigure()`
5. **Save/restore cursor** durante redraw
6. **Substituir `.lock().unwrap()`** por handling seguro (ou `parking_lot`)
7. **Corrigir translate exit code** — Retornar `Error` para linguagens não implementadas
8. **Remover código morto** — `events.rs`, deprecated color aliases

### P2 — Completude funcional (próximo milestone)

9. **Implementar check_file()** completo (Phase 2.4)
10. **Conectar menu handlers** (validate, render, apply)
11. **Implementar template helpers** (to_kebab, to_snake, to_pascal_case)
12. **Expandir artifact support** para todos os `ArtifactKind`
13. **Conectar process_text()** ou remover pathway

### P3 — Enterprise readiness

14. **CI/CD pipeline** com `cargo clippy`, `cargo test`, `cargo audit`
15. **Code coverage** via `cargo llvm-cov`
16. **E2E tests** executando o binário `ntk`
17. **Benchmarks** com `criterion` para template rendering e file search
18. **Unificar error handling** — migrar tudo para `thiserror` ou adotar `miette`
19. **Shell completions** via `clap_complete`
20. **Release automation** via `cargo-dist` ou `release-plz`

---

## Apêndice B — Estatísticas Rápidas

| Métrica | Valor |
|---|---|
| Crates no workspace | 10 |
| Arquivos fonte (src/) | ~90+ |
| Arquivos de teste | ~150+ |
| Test cases | ~300+ |
| LOC produção (estimado) | ~7.300 |
| Blocos `unsafe` | **1** (UB confirmado) |
| TODOs genuínos | 4 |
| Workarounds ativos | 1 |
| Código morto | 2 módulos |
| Feature flags | 5 (nenhum ativo por default) |
| Linguagens de template | 6 (apenas .NET funcional em translate) |
| Modelo structs (DDD) | 38 |
| Dependências externas diretas | ~20 |