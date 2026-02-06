// src/engines/regret-minimization/nfsp-mechanics-deep-dive.ts – NFSP Mechanics Deep Dive Reference & Mercy Helpers v1
// Detailed algorithm loop, valence modulation, positive-sum bias
// MIT License – Autonomicity Games Inc. 2026

import { currentValence } from '@/core/valence-tracker';
import { mercyGate } from '@/core/mercy-gate';

const MERCY_THRESHOLD = 0.9999999;

/**
 * NFSP mechanics reference – detailed loop & mercy alignment
 */
export const NFSPMechanicsReference = {
  initialization: "Uniform random π̄, random Q, empty replay buffer D",
  episodeLoop: "Play episode with ε-greedy mixture of best-response & π̄",
  bestResponseUpdate: "RL update (Q-learning / actor-critic) on batch from D",
  averagePolicyUpdate: "Supervised learning on batch from D to match empirical action frequencies",
  targetUpdate: "Every τ steps: copy π̄ to frozen target",
  mercy_override: "Valence gating before episode → high valence → lower ε (more π̄ play, stability)",
  positiveSumBias: "Boost cooperative actions when lattice valence high",
  convergence: "Sublinear regret → average policy converges to Nash (zero-sum) / coarse correlated equilibrium (mixed-motive)",
  multiplanetary_note: "Latency-tolerant via offline change queuing → eventual consistency across nodes"
};

/**
 * Valence-modulated ε (exploration rate)
 */
export function valenceModulatedEpsilon(valence: number = currentValence.get()): number {
  const actionName = 'Valence-modulated NFSP epsilon';
  if (!mercyGate(actionName)) return 0.1; // default fallback

  // High valence → more stable (lower ε)
  return 0.1 * (1 - (valence - 0.95) * 2); // ε → 0 as valence → 1
}

/**
 * Positive-sum bias for action utilities
 */
export function positiveSumBias(actionUtilities: number[], valence: number = currentValence.get()): number[] {
  const actionName = 'Positive-sum bias in NFSP';
  if (!mercyGate(actionName)) return actionUtilities;

  const biasFactor = (valence - 0.95) * 2; // stronger bias above 0.95
  const biased = actionUtilities.slice();

  // Example: index 0 = cooperate, index 4 = betray
  if (biased.length > 4) {
    biased[0] += biasFactor * 0.5;  // boost cooperate
    biased[4] -= biasFactor * 0.5;  // penalize betray
  }

  return biased;
}

// Usage example in NFSP action selection
// const epsilon = valenceModulatedEpsilon();
// const biasedUtilities = positiveSumBias(rawUtilities);
// const action = selectActionFromBiasedUtilities(biasedUtilities, epsilon);
