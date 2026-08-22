---
name: mempalace
description: >-
  Spatial long-term memory system for storing, retrieving, and updating project knowledge,
  architectural decisions, conventions, and agent diary logs across sessions.
  Use this skill whenever you need to recall project architecture, read past decisions,
  or record new discoveries and milestones into the project's memory palace.
---

# MemPalace Skill

MemPalace organizes long-term project memory hierarchically into **Wings**, **Rooms**, and **Drawers**:

```
.mempalace/
├── palace.json                  # Memory registry and index of wings/rooms
└── wings/
    ├── architecture/            # Technical setup, IPC, conventions, tooling
    ├── decisions/               # Architecture Decision Records (ADRs)
    └── diary/                   # Chronological agent development journal
```

## When to Use This Skill
- **Session Start**: Check `.mempalace/palace.json` to recall established conventions and architectural rules before modifying the project.
- **Making Major Decisions**: Consult or add an ADR to `.mempalace/wings/decisions/`.
- **Completing Milestones**: Append key actions to `.mempalace/wings/diary/journal.md`.

## CLI Operations (Powered by Bun)

To interact with the palace directly from terminal or scripts:

1. **List all Wings & Rooms**:
   ```bash
   bun run .agents/skills/mempalace/scripts/palace.ts list
   ```

2. **View a Room's knowledge**:
   ```bash
   bun run .agents/skills/mempalace/scripts/palace.ts view tauri-backend
   bun run .agents/skills/mempalace/scripts/palace.ts view svelte-frontend
   bun run .agents/skills/mempalace/scripts/palace.ts view tailwind-styling
   bun run .agents/skills/mempalace/scripts/palace.ts view tooling-bun
   ```

3. **Append to Agent Diary**:
   ```bash
   bun run .agents/skills/mempalace/scripts/palace.ts log "Implemented new feature XYZ"
   ```

## Best Practices
- Keep room documents concise and focused.
- Ensure new architectural changes are reflected in their corresponding room documents.
- Always log significant refactorings or new capabilities in the diary.
