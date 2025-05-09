# Tauri + React + Typescript

This template should help get you started developing with Tauri, React and Typescript in Vite.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Knip - Unused Code Detection

This project uses [Knip](https://github.com/webpro/knip) to detect unused files, exports, and dependencies. To run Knip:

```bash
yarn knip
```

Configuration files:

- `knip.ts` - TypeScript configuration
- `.kniprc.json` - JSON configuration
- `.knipignore` - Files and directories to ignore

Knip helps maintain a cleaner codebase by identifying:

- Unused files
- Unused exports
- Unused dependencies
- Duplicate exports
