# Rese Roadmap

`rese` is an open-source runtime-first desktop app framework, which includes headless control plane, strong internal tooling, and agent-ready workflows.

## 0.1.0

Goal: ship the first usable `rese` experience as a headless-first VPS control plane.

### 0.0.1

- [x] `chore(repo): scaffold initial rengine monorepo structure with rese`
- [x] `docs(rese): define v0.1.0 scope as a headless-first control plane`
- [ ] `chore(web): initialize shadcn/ui app shell for Rese Control Plane`
- [ ] `feat(layout): add initial dashboard layout with sidebar and topbar`
- [ ] `docs(roadmap): add patch-version milestone plan toward v0.1.0`

### 0.0.2

- [ ] `feat(runtime): create minimal rese runtime entrypoint for headless mode`
- [ ] `feat(server): add basic HTTP server for serving the web UI`
- [ ] `feat(health): expose runtime health endpoint`
- [ ] `feat(status): show runtime availability in the web UI`
- [ ] `refactor(core): separate web app and runtime server boundaries`

### 0.0.3

- [ ] `feat(transport): add initial WebSocket server for live runtime communication`
- [ ] `feat(session): implement basic client connection lifecycle`
- [ ] `feat(ui): add connection state badge for connect and disconnect flow`
- [ ] `feat(logging): log runtime startup and websocket session events`
- [ ] `fix(transport): handle stale socket disconnects without crashing runtime`

### 0.0.4

- [ ] `feat(shell): spawn interactive shell session from runtime`
- [ ] `feat(terminal): add Terminal panel placeholder for interactive sessions`
- [ ] `feat(terminal): stream shell stdout and stderr to the browser`
- [ ] `feat(terminal): forward browser input to the shell session`
- [ ] `fix(shell): close shell process cleanly when session ends`

### 0.0.5

- [ ] `feat(terminal): integrate VSCode-like terminal emulator into control plane`
- [ ] `feat(terminal): support terminal resize synchronization`
- [ ] `feat(terminal): preserve terminal scrollback during active session`
- [ ] `fix(terminal): render interactive shell output correctly`
- [ ] `style(terminal): polish terminal panel for first daily-driver workflow`

### 0.0.6

- [ ] `feat(auth): protect web UI behind initial local access token`
- [ ] `feat(auth): validate websocket sessions against authenticated access state`
- [ ] `feat(security): reject anonymous terminal session creation`
- [ ] `docs(auth): document local dev authentication flow`
- [ ] `fix(auth): show friendly unauthorized state in web UI`

### 0.0.7

- [ ] `feat(streams): add Streams panel for runtime and app output`
- [ ] `feat(streams): stream runtime lifecycle logs into the browser`
- [ ] `feat(streams): support follow-tail behavior for live output`
- [ ] `feat(streams): show source and level badges for streamed events`
- [ ] `style(streams): refine stream panel readability for long-running sessions`

### 0.0.8

- [ ] `feat(streams): add pause and resume controls for live stream mode`
- [ ] `feat(streams): add clear action for non-persistent stream view`
- [ ] `feat(ui): keep Terminal and Streams modes clearly separated`
- [ ] `fix(streams): prevent terminal reconnect logic from corrupting stream state`
- [ ] `docs(ui): document the difference between Terminal mode and Streams mode`

### 0.0.9

- [ ] `feat(info): show host, user, cwd, and runtime identity in the dashboard`
- [ ] `feat(layout): add overview cards for runtime and machine state`
- [ ] `feat(nav): add placeholder routes for files, logs, jobs, and agent panels`
- [ ] `feat(logs): add recent runtime session activity panel`
- [ ] `style(ui): refine control-plane spacing and shadcn consistency`

### 0.0.10

- [ ] `feat(dx): add watch mode for web UI development`
- [ ] `feat(dx): restart runtime server cleanly during active development`
- [ ] `feat(reload): reconnect browser client automatically after runtime restart`
- [ ] `fix(dx): preserve terminal page stability during frontend hot reload`
- [ ] `docs(dev): document VPS-first development workflow for rese`

### 0.0.11

- [ ] `feat(proxy): make runtime safe to expose behind reverse proxy`
- [ ] `feat(config): add environment-based host, port, and auth configuration`
- [ ] `feat(deploy): support running rese runtime as a long-lived server process`
- [ ] `docs(deploy): document domain and reverse-proxy setup for rese control plane`
- [ ] `fix(network): handle forwarded headers and websocket upgrades reliably`

### 0.0.12

- [ ] `refactor(runtime): introduce minimal internal event bus for future surfaces`
- [ ] `refactor(api): prepare transport contracts for future native and agent integrations`
- [ ] `feat(observability): improve structured logs for runtime and terminal lifecycle`
- [ ] `docs(architecture): write v0.1.0 architecture note for control plane foundation`
- [ ] `chore(release): prepare first v0.1.0 release checklist`

### v0.1.0 Definition of Done

- [ ] `feat(release): run rese runtime on a VPS as the primary supported workflow`
- [ ] `feat(release): open the web UI from another device and reach the control plane`
- [ ] `feat(release): use the Terminal panel reliably for real development tasks`
- [ ] `feat(release): use the Streams panel reliably for logs and runtime output`
- [ ] `feat(release): expose a clean enough architecture to grow into future rese runtime features`
- [ ] `docs(release): publish v0.1.0 usage and architecture documentation`

## 1.0.0

- [ ] `feat(platform): expand rese from control plane into a full runtime-first platform`
- [ ] `feat(native): add native desktop surface on top of the same runtime concepts`
- [ ] `feat(libs): add official system libraries and typed capability APIs`
- [ ] `feat(agent): add agent-ready interfaces for future Rse integration`
- [ ] `chore(repo): move rese into its own repository when both rese and rengine are stable`
