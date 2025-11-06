# Vibe Coder Wails Prototype

This is a prototype of an AI console built with:

- Wails (Go desktop shell)
- React + Vite
- Tailwind CSS + custom CSS variables
- shadcn/ui style tokens (manual variables; components can be added later)
- Lucide icons
- Monaco Editor for response viewing

## Structure

```
wails-app/
  main.go            # Wails bootstrap & backend binding
  go.mod
  wails.json         # Wails config
  frontend/
    index.html
    package.json
    tsconfig.json
    vite.config.ts
    postcss.config.js
    tailwind.config.js
    src/
      index.css      # Tailwind + CSS variables theme
      main.tsx       # Frontend entry
      ui/App.tsx     # Editor-like layout
```

## Running (Prerequisites)

Ensure you have Go (>=1.22), Node (>=18), and Wails installed:

```bash
go install github.com/wailsapp/wails/v2/cmd/wails@latest
```

Install frontend deps:

```bash
cd wails-app/frontend
npm install
```

Build frontend:

```bash
npm run build
```

Run Wails app (from `wails-app`):

```bash
wails dev   # for live reload
# or
wails build # creates a desktop binary
```

## Layout Overview

- Activity bar (icons) on the left
- Sidebar (Explorer sections)
- Tab bar with a single "Chat" tab
- Response pane rendered via Monaco (markdown read-only)
- Prompt input at bottom with Send button
- Status bar with style + theme toggles

## Next Steps

- Add real shadcn/ui components (Button, Input, ScrollArea) via generation or manual port.
- Add dynamic sessions/tabs.
- Integrate actual AI provider calls via backend methods.
- Persist chat history and settings.
- Bundle font (JetBrains Mono) for consistent typography.

## License

MIT (same as parent project).
