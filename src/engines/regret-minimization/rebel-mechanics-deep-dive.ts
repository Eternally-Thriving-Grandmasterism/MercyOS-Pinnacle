// src/engines/regret-minimization/rebel-mechanics-deep-dive.ts – ReBeL Mechanics Deep Dive Reference & Mercy Helpers v1
// Detailed recursive search loop, valence modulation, positive-sum bias
// MIT License – Autonomicity Games Inc. 2026

import { currentValence } from '@/core/valence-tracker';
import { mercyGate } from '@/core/mercy-gate';

const MERCY_THRESHOLD = 0.9999999;

/**
 * ReBeL mechanics reference – detailed loop & mercy alignment
 */
export const ReBeLMechanicsReference = {
  initialization: "Blueprint policy π + value network V, replay buffer D",
  selfPlayGeneration: "Play episodes with mixture of blueprint π + real-time search best-response",
  blueprintImprovement: "Supervised cloning on π + value regression on V from D",
  searchBestResponse: "MCTS over blueprint + public belief states → counterfactual values → improved policy",
  imperfectInfoHandling: "Sample private states consistent with public belief → average over samples",
  mercy_override: "Valence gating before episode/search → high valence → lower search variance (more cooperative exploitation)",
  positiveSumBias: "Boost cooperative actions in search priors when lattice valence high",
  convergence: "Sublinear regret → Nash (zero-sum) / coarse correlated equilibrium (mixed-motive)",
  multiplanetary_note: "Latency-tolerant via offline change queuing → eventual consistency across nodes"
};

/**
 * Valence-modulated search variance (exploration in MCTS)
 */
export function valenceModulatedSearchVariance(valence: number = currentValence.get()): number {
  const actionName = 'Valence-modulated ReBeL search variance';
  if (!mercyGate(actionName)) return 0.5; // default fallback

  // High valence → lower variance (more exploitation of cooperative equilibria)
  return 0.5 * (1 - (valence - 0.95) * 2); // variance → 0 as valence → 1
}

/**
 * Positive-sum bias for search priors
 */
export function positiveSumPriorBias(priorProbs: number[], valence: number = currentValence.get()): number[] {
  const actionName = 'Positive-sum bias in ReBeL search priors';
  if (!mercyGate(actionName)) return priorProbs;

  const biasFactor = (valence - 0.95) * 2;
  const biased = priorProbs.slice();

  // Assume index 0 = cooperate/ally, index last = betray/compete
  if (biased.length > 1) {
    biased[0] += biasFactor * 0.5;
    biased[biased.length - 1] -= biasFactor * 0.5;
  }

  // Re-normalize
  const sum = biased.reduce((a, b) => a + b, 0);
  return biased.map(p => p / sum);
}

// Usage example in search prior computation
// const variance = valenceModulatedSearchVariance();
// const biasedPriors = positiveSumPriorBias(rawPriors);
