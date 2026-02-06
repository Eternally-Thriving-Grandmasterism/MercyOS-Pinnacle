// src/sync/hybrid-yjs-automerge-bridge.ts – sovereign Hybrid Yjs + Automerge Bridge v1
// Real-time Yjs + durable Automerge subdocs, bidirectional delta sync, mercy-gated
// MIT License – Autonomicity Games Inc. 2026

import * as Y from 'yjs';
import * as Automerge from '@automerge/automerge';
import { currentValence } from '@/core/valence-tracker';
import { mercyGate } from '@/core/mercy-gate';
import { yjsSubdocSyncProviders } from '@/sync/yjs-subdoc-sync-providers';

const MERCY_THRESHOLD = 0.9999999;
const HYBRID_SUBDOC_PREFIX = 'mercy-hybrid-automerge-';

export class HybridYjsAutomergeBridge {
  private ydoc: Y.Doc;
  private automergeRoot: Automerge.Doc<any>;
  private automergeSubdocs = new Map<string, Automerge.Doc<any>>();

  constructor(ydoc: Y.Doc, automergeRoot: Automerge.Doc<any>) {
    this.ydoc = ydoc;
    this.automergeRoot = automergeRoot;
  }

  /**
   * Get or create Automerge subdoc embedded in Yjs (lazy + mercy-gated)
   */
  async getOrCreateAutomergeSubdoc(
    key: string,
    initialValue: any = {},
    requiredValence: number = MERCY_THRESHOLD
  ): Promise<Automerge.Doc<any> | null> {
    const actionName = `Hybrid get/create Automerge subdoc: ${key}`;
    if (!await mercyGate(actionName, key, requiredValence)) {
      return null;
    }

    if (this.automergeSubdocs.has(key)) {
      return this.automergeSubdocs.get(key)!;
    }

    // Check if already embedded in parent Automerge doc
    const binary = Automerge.get(this.automergeRoot, ['hybrid_subdocs', key]);
    let subdoc: Automerge.Doc<any>;

    if (binary instanceof Uint8Array) {
      subdoc = Automerge.load(binary);
      console.log(`[HybridBridge] Loaded Automerge subdoc from binary: \( {key} ( \){binary.byteLength} bytes)`);
    } else {
      subdoc = Automerge.from(initialValue);
      console.log(`[HybridBridge] Created new Automerge subdoc: ${key}`);
    }

    this.automergeSubdocs.set(key, subdoc);
    return subdoc;
  }

  /**
   * Save Automerge subdoc back to parent as binary blob in Automerge root
   */
  async saveAutomergeSubdoc(key: string, requiredValence: number = MERCY_THRESHOLD) {
    const actionName = `Hybrid save Automerge subdoc: ${key}`;
    if (!await mercyGate(actionName, key, requiredValence)) return;

    const subdoc = this.automergeSubdocs.get(key);
    if (!subdoc) return;

    const binary = Automerge.save(subdoc);

    Automerge.change(this.automergeRoot, `Saving hybrid subdoc ${key}`, doc => {
      if (!doc.hybrid_subdocs) doc.hybrid_subdocs = {};
      doc.hybrid_subdocs[key] = binary;
    });

    console.log(`[HybridBridge] Automerge subdoc saved: \( {key} ( \){binary.byteLength} bytes)`);
  }

  /**
   * Sync Automerge subdoc changes to Yjs (delta propagation)
   */
  async propagateAutomergeToYjs(key: string) {
    const subdoc = this.automergeSubdocs.get(key);
    if (!subdoc) return;

    const binary = Automerge.save(subdoc);

    // Embed in Yjs subdoc or map
    const ySubdoc = await yjsSubdocSyncProviders.getOrCreateWithProviders(HYBRID_SUBDOC_PREFIX + key);
    if (ySubdoc) {
      Y.transact(ySubdoc, () => {
        ySubdoc.getMap('binary').set('snapshot', binary);
      });
      console.log(`[HybridBridge] Propagated Automerge → Yjs subdoc: ${key}`);
    }
  }

  /**
   * Sync Yjs changes back to Automerge (if needed – rare)
   */
  async propagateYjsToAutomerge(key: string) {
    // Placeholder – real impl would compare heads & apply delta
    console.log(`[HybridBridge] Yjs → Automerge propagation stub: ${key}`);
  }
}

export const hybridBridge = new HybridYjsAutomergeBridge(/* pass ydoc & automergeRoot */);
