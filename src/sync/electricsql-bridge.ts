// src/sync/electricsql-bridge.ts – sovereign ElectricSQL Bridge v1
// Relational sync (Postgres ↔ SQLite), offline-first, real-time, integrated with Yjs + Automerge
// MIT License – Autonomicity Games Inc. 2026

import { electrify, schema } from 'electric-sql';
import { electrify as electrifySqlite } from 'electric-sql/sqlite';
import { currentValence } from '@/core/valence-tracker';
import { mercyGate } from '@/core/mercy-gate';
import { hybridBridge } from '@/sync/hybrid-yjs-automerge-bridge';
import { multiplanetarySync } from '@/core/multiplanetary-sync-engine';

// ElectricSQL schema (simplified – expand as needed)
const mercySchema = schema({
  users: {
    id: 'TEXT PRIMARY KEY',
    level: 'TEXT NOT NULL',
    valence: 'REAL NOT NULL',
    experience: 'INTEGER NOT NULL',
    lastActivity: 'INTEGER NOT NULL',
    createdAt: 'INTEGER NOT NULL'
  },
  progress_logs: {
    id: 'TEXT PRIMARY KEY',
    userId: 'TEXT NOT NULL REFERENCES users(id)',
    eventType: 'TEXT NOT NULL',
    delta: 'REAL NOT NULL',
    timestamp: 'INTEGER NOT NULL'
  },
  probes: {
    id: 'TEXT PRIMARY KEY',
    resources: 'INTEGER NOT NULL',
    valence: 'REAL NOT NULL',
    habitatId: 'TEXT',
    updatedAt: 'INTEGER NOT NULL'
  }
});

export class ElectricSQLBridge {
  private electric: any = null; // Electric client instance
  private db: any = null;       // SQLite DB instance

  constructor() {
    this.initElectricSQL();
  }

  private async initElectricSQL() {
    if (!await mercyGate('ElectricSQL bridge initialization', 'Multiplanetary relational sync')) return;

    try {
      // Initialize SQLite (local) + Electric sync
      const dbName = 'mercyos-local.db';
      this.db = await electrifySqlite(dbName, mercySchema);

      // Connect to Electric Postgres backend
      this.electric = await electrify(this.db, {
        url: import.meta.env.VITE_ELECTRIC_URL || 'https://electric.rathor.ai',
        auth: { token: 'your-auth-token-here' } // replace with real auth
      });

      console.log("[ElectricSQLBridge] Bridge initialized – relational sync active");
    } catch (e) {
      console.error("[ElectricSQLBridge] Failed to initialize ElectricSQL", e);
    }
  }

  /**
   * Sync valence & progression to relational layer (called after state changes)
   */
  async syncProgressToRelational() {
    if (!this.electric) return;

    const v = currentValence.get();
    const progress = await getUserProgress(); // from onboarding db

    await this.electric.db.users.upsert({
      id: 'current-user',
      level: progress.level,
      valence: v,
      experience: progress.experience,
      lastActivity: Date.now(),
      createdAt: Date.now()
    });

    await this.electric.sync(); // trigger sync to Postgres

    console.log(`[ElectricSQLBridge] Progress synced – valence ${v.toFixed(8)}`);
  }

  /**
   * Sync probe fleet state to relational layer (example)
   */
  async syncProbeToRelational(probeId: string, data: { resources: number; valence: number }) {
    if (!this.electric) return;

    await this.electric.db.probes.upsert({
      id: probeId,
      resources: data.resources,
      valence: data.valence,
      updatedAt: Date.now()
    });

    await this.electric.sync();

    console.log(`[ElectricSQLBridge] Probe ${probeId} synced – resources ${data.resources}`);
  }

  /**
   * Listen for remote changes (reactive)
   */
  setupRemoteChangeListener() {
    if (!this.electric) return;

    this.electric.db.probes.liveMany({}).subscribe((result) => {
      console.log("[ElectricSQLBridge] Remote probe changes detected:", result);
      // Propagate to Yjs or Automerge if needed
    });
  }
}

export const electricBridge = new ElectricSQLBridge();

// Hook into state changes
// Example: after valence update
electricBridge.syncProgressToRelational();

// Example: after probe update
electricBridge.syncProbeToRelational('probe-001', { resources: 42, valence: 0.999 });
