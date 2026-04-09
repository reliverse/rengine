# Rese

`rese` is an open-source runtime-first desktop app framework, which includes headless control plane, strong internal tooling, and agent-ready workflows.

It is developed within the [Reliverse](https://github.com/reliverse) and [Bleverse](https://github.com/reliverse/bleverse) ecosystems.

## Vision

Most app frameworks begin by wrapping a UI.

`rese` begins from a different place: the app should be a runtime with structured capabilities, not just a window.

This makes it possible to expose the same core logic through different surfaces:

- a browser-based control plane
- a native desktop shell
- a terminal or automation layer
- a future agent interface such as Rse

## Current Direction

`rese` starts as a **headless-first control plane**.

The first real user scenario is simple:

- you develop on a VPS (by being connected to an IDE/terminal via SSH)
- you open `rese` Web UI from your own computer
- you see a shadcn/ui-based **Rese Control Plane**
- you use an in-browser **Terminal** panel for interactive shell work
- you use a **Streams** panel for logs, runtime events, and live output

This is the foundation for the future `rese` framework.

## Workspace Layout

Current scaffold inside `rengine`:

- `apps/cli` → future agent-first CLI entrypoint for starting and managing `rese` in headless mode
- `apps/web` → future browser-based Rese Control Plane app, structured toward TanStack Start + Vite + shadcn/ui
- `apps/wiki` → future docs and wiki app
- `packages/core` → shared runtime contracts, manifests, and low-level reusable helpers

The goal of this first scaffold is simple: make package boundaries, naming, scripts, and entrypoints real enough that the next milestones can build on them without reshuffling the tree.

Package naming is intentionally split:

- `@reliverse/rese` and `@reliverse/rese-core` for packages that can later move toward public npm publishing
- `@repo/rese-web` and `@repo/rese-wiki` for private workspace apps

## Why

The goal of `rese` is not only to wrap apps.

It is meant to provide a runtime layer that can power:

- headless VPS tooling
- browser-based control planes
- desktop surfaces
- internal systems
- agent-native applications
- future runtime and system libraries

## Planned Scope

Over time, `rese` is expected to grow into a broader framework with:

- a runtime core
- browser and native app surfaces
- headless/server mode
- typed bridges and transport layers
- system libraries
- structured capabilities and permissions
- agent-ready infrastructure
- official development, packaging, and deployment tooling

## Project Status

`rese` is currently being matured inside the [`reliverse/rengine`](https://github.com/reliverse/rengine) repository. This is intentional.

Instead of building `rese` in isolation, it is being shaped inside a demanding real-world project. Once both `rese` and `rengine` become more stable, `rese` will move into its own standalone repository.

## Design Principles

`rese` is being built around a few core principles:

- **runtime first** — the app is more than its UI
- **headless first** — VPS and remote workflows are first-class
- **browser is a real surface** — not only a dev server
- **typed boundaries** — transport and capability layers should stay structured
- **small core, useful platform** — keep the core focused while allowing official supporting libraries
- **agent-ready by design** — future Rse integrations should feel natural, not bolted on

## Roadmap

The early focus is:

- ship `rese v0.1.0` as a usable VPS control plane
- provide a reliable browser-based Terminal panel
- provide a Streams panel for logs and live output
- establish a clean architecture for future runtime expansion

Longer-term goals include native desktop surfaces, official system libraries, and deeper AI agent integration.

## Contributing

Contributions are welcome ❤️

Here are a few ways to support the project:

- Contribute code — see [CONTRIBUTING.md](CONTRIBUTING.md)
- Report bugs or request features — <https://github.com/reliverse/rese/issues>
- Support development on GitHub Sponsors — <https://github.com/sponsors/blefnk>

> If you find a security vulnerability, please report it privately. See [SECURITY.md](SECURITY.md).

## Documentation

- Docs: <https://rese.reliverse.org/docs>
- Blog: <https://rese.reliverse.org/blog>

## License

<https://github.com/reliverse/rengine>

Copyright © 2026–present Nazarii Korniienko

The repository is licensed under the [MIT License](../LICENSE).

Some apps or packages may include additional licenses. For more information, check the `LICENSE` file inside each package. If specific packages are missing a LICENSE file, they are licensed under the [MIT License](../LICENSE).
