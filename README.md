# ACP adapter for Codex

Use [Codex](https://github.com/openai/codex) from [ACP-compatible](https://agentclientprotocol.com) clients such as [Zed](https://zed.dev)!

This tool implements an ACP adapter around the Codex CLI, supporting:

- Context @-mentions
- Images
- Tool calls (with permission requests)
- Following
- Edit review
- TODO lists
- Slash commands:
  - /review (with optional instructions)
  - /review-branch
  - /review-commit
  - /init
  - /compact
  - /logout
- Client MCP servers
- Auth Methods:
  - ChatGPT subscription (requires paid subscription and doesn't work in remote projects)
  - CODEX_API_KEY
  - OPENAI_API_KEY

Learn more about the [Agent Client Protocol](https://agentclientprotocol.com/).

## How to use

### Zed

The latest version of Zed can already use this adapter out of the box.

To use Codex, open the Agent Panel and click "New Codex Thread" from the `+` button menu in the top-right.

Read the docs on [External Agent](https://zed.dev/docs/ai/external-agents) support.

### Other clients

Or try it with any of the other [ACP compatible clients](https://agentclientprotocol.com/overview/clients)!

#### Installation

Install the adapter from the latest release for your architecture and OS: https://github.com/zed-industries/codex-acp/releases

You can then use `codex-acp` as a regular ACP agent:

```
OPENAI_API_KEY=sk-... codex-acp
```

Or via npm:

```
npx @zed-industries/codex-acp
```

## Android support (fork-specific)

This fork was created to add Android cross-compilation. **It does not currently
work.** After upstream codex rebased onto codex-core with a hard dependency on
`codex-code-mode`, which depends on the `v8` Rust crate, building for Android
is blocked by upstream brokenness in `denoland/rusty_v8` v147.4.0:

- rusty_v8 ships no Android prebuilts (CI matrix excludes Android).
- `V8_FROM_SOURCE=1` cross-compile to `aarch64-linux-android` fails because
  the published `v8` crate's bundled chromium build tree is incomplete
  (missing `build/android/**/*.pydeps` files, then missing
  `third_party/rust/chromium_crates_io/vendor/`).
- The only community fork tracking Android (ThetaDev/rusty_v8) is on V8 130.x,
  which codex-code-mode's API (`PinScope`, `callback_scope!`, new
  `Context::new` signature) is incompatible with.

Tracking: [rusty_v8 #1895](https://github.com/denoland/rusty_v8/issues/1895).

See `.github/workflows/prebuild-v8-android.yml` for the partial-progress
patches discovered in this fork (pydeps stubs, BUILD.gn data_deps removal,
install-sysroot, etc.) — useful as a starting point if upstream gets fixed
or someone wants to take another swing.

In the meantime, run codex-acp on a Linux host and connect to it from
Android over the network; no on-device build needed.

## License

Apache-2.0
