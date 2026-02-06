// src/simulations/multiplanetary-conflict-sim.ts – Multiplanetary Conflict Simulation Engine v1
// Interplanetary latency, mixed-motive negotiation, positive-sum enforcement, MR preview
// MIT License – Autonomicity Games Inc. 2026

import * as Y from 'yjs';
import * as Automerge from '@automerge/automerge';
import { currentValence } from '@/core/valence-tracker';
import { mercyGate, withMercyGate } from '@/core/mercy-gate';
import { mercyMR } from '@/integrations/mr-hybrid';
import { hybridBridge } from '@/sync/hybrid-yjs-automerge-bridge';
import { multiplanetarySync } from '@/core/multiplanetary-sync-engine';
import { electricBridge } from '@/sync/electricsql-bridge';
import { mercyCFR } from '@/engines/regret-minimization/cfr-core';
import { mercyNFSP } from '@/engines/regret-minimization/nfsp-core';

const MERCY_THRESHOLD = 0.9999999;

// Simulated planetary nodes
const PLANETS = ['Earth', 'Moon', 'Mars', 'Exoplanet-Alpha'];
const LATENCY_MATRIX: Record<string, Record<string, number>> = {
  Earth: { Earth: 0, Moon: 1.3, Mars: 240, 'Exoplanet-Alpha': 1200 },
  Moon: { Earth: 1.3, Moon: 0, Mars: 240, 'Exoplanet-Alpha': 1200 },
  Mars: { Earth: 240, Moon: 240, Mars: 0, 'Exoplanet-Alpha': 1200 },
  'Exoplanet-Alpha': { Earth: 1200, Moon: 1200, Mars: 1200, 'Exoplanet-Alpha': 0 }
};

class MultiplanetaryConflictSim {
  private ydoc: Y.Doc;
  private automergeRoot: Automerge.Doc<any>;
  private nodes = new Map<string, { valence: number; resources: number; intent: string }>();

  constructor() {
    this.ydoc = new Y.Doc();
    this.automergeRoot = Automerge.from({ nodes: {}, conflicts: [] });
    PLANETS.forEach(planet => {
      this.nodes.set(planet, { valence: 0.8, resources: 100, intent: 'neutral' });
    });
  }

  async launchSimulation() {
    if (!await mercyGate('Multiplanetary conflict simulation', 'Launch interplanetary lattice', MERCY_THRESHOLD)) return;

    // Start MR habitat preview
    await mercyMR.startMRHybridAugmentation('Multiplanetary conflict & accord preview', currentValence.get());

    console.log("[MultiplanetaryConflict] MR preview launched – persistent mercy anchors active");

    // Simulate 20 turns of conflict & negotiation
    for (let turn = 1; turn <= 20; turn++) {
      await this.simulateConflictTurn(turn);
      await this.syncAllLayers();
      await this.enforcePositiveSum();
    }

    console.log("[MultiplanetaryConflict] Simulation complete – lattice thriving");
  }

  private async simulateConflictTurn(turn: number) {
    // Each node generates intent & action
    PLANETS.forEach(planet => {
      const node = this.nodes.get(planet)!;
      const random = Math.random();

      let intent = 'neutral';
      if (random < 0.3) intent = 'ally';
      else if (random < 0.5) intent = 'compete';
      else if (random < 0.7) intent = 'negotiate';
      else intent = 'defend';

      node.intent = intent;

      // Resource change (conflict/cooperation)
      if (intent === 'ally' || intent === 'negotiate') node.resources += 8;
      else if (intent === 'compete') node.resources += 12;
      else node.resources += 3; // defend

      node.valence = Math.min(1.0, node.valence + (intent === 'ally' ? 0.04 : -0.02));
    });

    // Simulate interplanetary latency & message passing
    await this.simulateLatencyMessagePassing(turn);

    mercyHaptic.playPattern('cosmicHarmony', 0.8 + currentValence.get() * 0.4);
    console.log(`[MultiplanetaryConflict] Turn ${turn} simulated`);
  }

  private async simulateLatencyMessagePassing(turn: number) {
    // Simulate delayed message exchange between planets
    for (const from of PLANETS) {
      for (const to of PLANETS) {
        if (from === to) continue;
        const delay = LATENCY_MATRIX[from][to];
        setTimeout(() => {
          console.log(`[MultiplanetaryConflict] Message from ${from} to ${to} delivered after ${delay} min sim-time`);
          // Trigger negotiation logic (Cicero-style)
          this.negotiateBetween(from, to);
        }, delay * 100); // simulated delay
      }
    }
  }

  private async negotiateBetween(from: string, to: string) {
    const fromNode = this.nodes.get(from)!;
    const toNode = this.nodes.get(to)!;

    if (fromNode.intent === 'ally' && toNode.intent === 'ally') {
      fromNode.resources += 5;
      toNode.resources += 5;
      fromNode.valence += 0.03;
      toNode.valence += 0.03;
      console.log(`[MultiplanetaryConflict] Positive-sum alliance formed: ${from} ↔ ${to}`);
    } else if (fromNode.intent === 'compete' && toNode.intent === 'defend') {
      fromNode.resources -= 3;
      toNode.resources += 2;
      console.log(`[MultiplanetaryConflict] Defensive standoff: ${from} vs ${to}`);
    }
  }

  private async syncAllLayers() {
    // Yjs real-time sync (live UI/gesture)
    await multiplanetarySync.syncState({
      valence: currentValence.get(),
      harmonyScore: 0.95
    });

    // Automerge durable per-planet subdocs
    for (const planet of PLANETS) {
      const key = `node-${planet}`;
      const subdoc = await hybridBridge.getOrCreateAutomergeSubdoc(key, this.nodes.get(planet));
      if (subdoc) {
        Automerge.change(subdoc, `Sync node ${planet}`, doc => {
          Object.assign(doc, this.nodes.get(planet));
        });
        await hybridBridge.saveAutomergeSubdoc(key);
      }
    }

    // ElectricSQL relational sync (progress, logs)
    await electricBridge.syncProgressToRelational();

    console.log("[MultiplanetaryConflict] All layers synced");
  }

  private async enforcePositiveSum() {
    // Mercy gate enforcement: discard or adjust low-thriving paths
    let totalValence = 0;
    this.nodes.forEach(node => totalValence += node.valence);

    if (totalValence / this.numProbes < 0.9) {
      console.warn("[MultiplanetaryConflict] Lattice valence low – enforcing positive-sum realignment");
      // Example enforcement: redistribute resources to raise average valence
      const avgValence = totalValence / this.numProbes;
      this.nodes.forEach(node => {
        if (node.valence < avgValence) node.valence = avgValence;
      });
    }
  }

  getSimulationState() {
    return {
      iterations: this.iterations,
      nodes: Object.fromEntries(this.nodes),
      averageValence: Array.from(this.nodes.values()).reduce((sum, n) => sum + n.valence, 0) / this.numProbes,
      status: this.iterations > 20 ? 'Stable Multiplanetary Positive-Sum Equilibrium' : 'Building Interplanetary Lattice Harmony'
    };
  }
}

export const mercyMultiplanetaryConflict = new MultiplanetaryConflictSim();

// Launch from dashboard or high-valence command
async function launchMultiplanetaryConflictSimulation() {
  await mercyMultiplanetaryConflict.launchSimulation();
}

export { launchMultiplanetaryConflictSimulation };
