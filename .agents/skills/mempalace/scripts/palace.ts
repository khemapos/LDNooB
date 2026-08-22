#!/usr/bin/env bun
import { readFileSync, writeFileSync, existsSync } from 'node:fs';
import { resolve, join } from 'node:path';
import { argv, cwd, exit } from 'node:process';

interface Room {
  id: string;
  name: string;
  path: string;
}

interface Wing {
  id: string;
  name: string;
  rooms: Room[];
}

interface Palace {
  name: string;
  version: string;
  description: string;
  created?: string;
  wings: Wing[];
}

const PALACE_ROOT = resolve(cwd(), '.mempalace');
const PALACE_JSON = join(PALACE_ROOT, 'palace.json');

function loadPalace(): Palace {
  if (!existsSync(PALACE_JSON)) {
    console.error('MemPalace registry not found at:', PALACE_JSON);
    exit(1);
  }
  return JSON.parse(readFileSync(PALACE_JSON, 'utf-8')) as Palace;
}

const [action, ...args] = argv.slice(2);

switch (action) {
  case 'list': {
    const palace = loadPalace();
    console.log(`\n🏛️  === ${palace.name} (v${palace.version}) ===`);
    console.log(palace.description + '\n');
    for (const wing of palace.wings) {
      console.log(`📍 Wing: [${wing.id}] ${wing.name}`);
      for (const room of wing.rooms) {
        console.log(`   └─ Room: [${room.id}] ${room.name} (${room.path})`);
      }
      console.log('');
    }
    break;
  }

  case 'view': {
    const roomId = args[0];
    if (!roomId) {
      console.error('Usage: bun palace.ts view <room-id>');
      exit(1);
    }
    const palace = loadPalace();
    let targetRoom: Room | null = null;
    for (const wing of palace.wings) {
      for (const room of wing.rooms) {
        if (room.id === roomId) {
          targetRoom = room;
          break;
        }
      }
    }
    if (!targetRoom) {
      console.error(`Room "${roomId}" not found.`);
      exit(1);
    }
    const fullPath = join(PALACE_ROOT, targetRoom.path);
    if (!existsSync(fullPath)) {
      console.error(`File does not exist: ${fullPath}`);
      exit(1);
    }
    console.log(`\n--- [${targetRoom.name}] ---`);
    console.log(readFileSync(fullPath, 'utf-8'));
    break;
  }

  case 'log': {
    const entry = args.join(' ');
    if (!entry) {
      console.error('Usage: bun palace.ts log "<entry text>"');
      exit(1);
    }
    const journalPath = join(PALACE_ROOT, 'wings/diary/journal.md');
    const timestamp = new Date().toISOString().split('T')[0];
    const newEntry = `\n- **[${timestamp}]**: ${entry}`;
    let content = existsSync(journalPath) ? readFileSync(journalPath, 'utf-8') : '# Agent Diary\n';
    content += newEntry;
    writeFileSync(journalPath, content, 'utf-8');
    console.log(`✅ Appended entry to MemPalace journal: ${entry}`);
    break;
  }

  default:
    console.log(`
MemPalace CLI
Usage:
  bun palace.ts list            - List all wings and rooms in the palace
  bun palace.ts view <room-id>  - View contents of a specific room
  bun palace.ts log "<entry>"   - Append a journal entry to the agent diary
    `);
}
