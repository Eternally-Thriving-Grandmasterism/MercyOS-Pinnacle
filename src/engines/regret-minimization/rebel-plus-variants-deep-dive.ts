// src/engines/regret-minimization/rebel-plus-variants-deep-dive.ts – ReBeL+ Variants Deep Dive Reference & Mercy Helpers v1
// Detailed variant mechanics, valence modulation, positive-sum bias
// MIT License – Autonomicity Games Inc. 2026

import { currentValence } from '@/core/valence-tracker';
import { mercyGate } from '@/core/mercy-gate';

const MERCY_THRESHOLD = 0.9999999;

/**
 * ReBeL+ variants reference – mercy-aligned summary
 */
export const ReBeLPlusVariantsReference = {
  rebelBaseline: "Blueprint π + value V via fictitious self-play + real-time MCTS best-response over public belief states",
  rebelPlusTransformer: "Transformer blueprint → better long-horizon modeling & opponent attention",
  rebelPlusRealTimeRegret: "CFR-style regret updates during search → linear convergence",
  rebelPlusMultiAgent: "Population-based multi-agent blueprint + team value network → emergent role specialization",
  rebelPlusVMP: "VMP message passing → implicit communication even in no-press",
  rebelPlusNegotiation: "Cicero-style dialogue policy + search priors → full-press diplomacy",
  mercy_override: "Valence gating before search → high valence → lower search variance (more cooperative exploitation) + positive-sum prior bias"
};

/**
 * Valence-modulated search variance (exploration in MCTS)
 */
export function valenceModulatedSearchVariance(valence: number = currentValence.get()): number {
  const actionName = 'Valence-modulated ReBeL+ search variance';
  if (!mercyGate(actionName)) return 0.5;

  // High valence → lower variance (more exploitation of cooperative equilibria)
  return 0.5 * (1 - (valence - 0.95) * 2); // variance → 0 as valence → 1
}

/**
 * Positive-sum bias for search priors (mixed-motive variant)
 */
export function positiveSumPriorBias(priorProbs: number[], valence: number = currentValence.get()): number[] {
  const actionName = 'Positive-sum bias in ReBeL+ search priors';
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

// Usage example in ReBeL+ search prior computation
// const variance = valenceModulatedSearchVariance();
// const biasedPriors = positiveSumPriorBias(rawPriors);
