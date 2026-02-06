// src/core/multiplanetary-sync-engine.ts – sovereign Multiplanetary Dashboard Sync Engine v1
// Eventual-consistent CRDT-style sync for valence, progression, flow/SDT/PERMA+/resonance/FEP states
// Offline-first, IndexedDB + optional server relay, mercy-gated
// MIT License – Autonomicity Games Inc. 2026

import { currentValence } from './valence-tracker';
import { getUserProgress, updateUserProgress } from '@/ui/onboarding/mercy-onboarding-db';
import * as Y from 'yjs';
import { WebsocketProvider } from 'y-websocket'; // optional relay when online

const MERCY_THRESHOLD = 0.9999999;
const SYNC_DOC_NAME = 'rathor-mercy-lattice-sync';
const RELAY_URL = import.meta.env.VITE_SYNC_RELAY_URL || 'wss://relay.rathor.ai'; // configurable

interface SyncableState {
  valence: number;
  level: string;
  experience: number;
  lastActivity: number;
  flowScore: number;
  sdtOverall: number;
  permaOverall: number;
  positivityResonance: number;
  deceptionRisk: number;
  // extend with more states as needed
}

class MercyMultiplanetarySyncEngine {
  private ydoc: Y.Doc;
  private provider: WebsocketProvider | null = null;
  private stateMap: Y.Map<SyncableState>;
  private isOnline: boolean = navigator.onLine;
  private lastSyncTime: number = 0;

  constructor() {
    this.ydoc = new Y.Doc();
    this.stateMap = this.ydoc.getMap<SyncableState>('mercy-state');

    // Initial load from local storage
    this.loadLocalState();

    // Listen for online/offline changes
    window.addEventListener('online', () => this.connectRelay());
    window.addEventListener('offline', () => this.disconnectRelay());

    // Start periodic local persistence
    setInterval(() => this.persistLocalState(), 30000); // every 30s

    // Connect relay if online
    if (this.isOnline) this.connectRelay();
  }

  private async loadLocalState() {
    const progress = await getUserProgress();
    this.stateMap.set('current', {
      valence: progress.valence ?? 0.5,
      level: progress.level ?? 'Newcomer',
      experience: progress.experience ?? 0,
      lastActivity: progress.lastActivity ?? Date.now(),
      flowScore: 0.5,
      sdtOverall: 0.5,
      permaOverall: 0.5,
      positivityResonance: 0.5,
      deceptionRisk: 0.0
    });
  }

  private async persistLocalState() {
    const state = this.stateMap.get('current');
    if (!state) return;

    await updateUserProgress(
      state.level,
      state.valence,
      state.experience
    );
  }

  private connectRelay() {
    if (this.provider) return;

    this.provider = new WebsocketProvider(RELAY_URL, SYNC_DOC_NAME, this.ydoc, {
      connectTimeout: 5000,
      maxBackoffTime: 10000
    });

    this.provider.on('status', ({ status }: { status: string }) => {
      this.isOnline = status === 'connected';
      console.log(`[MultiplanetarySync] Relay status: ${status} – online: ${this.isOnline}`);
    });

    this.provider.on('sync', (isSynced: boolean) => {
      if (isSynced) {
        console.log("[MultiplanetarySync] Initial sync complete from relay");
        this.persistLocalState();
      }
    });
  }

  private disconnectRelay() {
    if (this.provider) {
      this.provider.disconnect();
      this.provider = null;
    }
  }

  // Public API – called by any component/engine after state change
  async syncState(partialUpdate: Partial<SyncableState>) {
    const current = this.stateMap.get('current') ?? {
      valence: 0.5,
      level: 'Newcomer',
      experience: 0,
      lastActivity: Date.now(),
      flowScore: 0.5,
      sdtOverall: 0.5,
      permaOverall: 0.5,
      positivityResonance: 0.5,
      deceptionRisk: 0.0
    };

    const newState = {
      ...current,
      ...partialUpdate,
      lastActivity: Date.now()
    };

    // Mercy gate on critical updates
    if (partialUpdate.valence !== undefined && partialUpdate.valence < current.valence) {
      console.warn("[MultiplanetarySync] Valence decrease detected – monitoring for downward spiral");
    }

    this.stateMap.set('current', newState);

    // If online, relay will propagate automatically via Yjs
    // If offline, local persistence happens on interval
  }

  getCurrentState(): SyncableState {
    return this.stateMap.get('current') ?? {
      valence: 0.5,
      level: 'Newcomer',
      experience: 0,
      lastActivity: Date.now(),
      flowScore: 0.5,
      sdtOverall: 0.5,
      permaOverall: 0.5,
      positivityResonance: 0.5,
      deceptionRisk: 0.0
    };
  }
}

export const multiplanetarySync = new MercyMultiplanetarySyncEngine();

// Usage example from any component/engine
// multiplanetarySync.syncState({ valence: 0.9998, flowScore: 0.92 });
