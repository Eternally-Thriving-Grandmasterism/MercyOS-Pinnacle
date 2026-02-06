// src/engines/regret-minimization/deep-nfsp-variants-deep-dive.ts – Deep NFSP Variants Deep Dive Reference & Mercy Helpers v1
// Detailed variant mechanics, valence modulation, positive-sum bias
// MIT License – Autonomicity Games Inc. 2026

import { currentValence } from '@/core/valence-tracker';
import { mercyGate } from '@/core/mercy-gate';

const MERCY_THRESHOLD = 0.9999999;

/**
 * Deep NFSP variants reference – mercy-aligned summary
 */
export const DeepNFSPVariantsReference = {
  originalNFSP: "Tabular fictitious self-play + ε-greedy mixture of RL best-response & supervised average policy",
  deepNFSP: "Replace tabular with deep nets – supervised π̄ on replay, RL Q/policy against frozen π̄",
  nfspPlus: "Regret clipping & normalization → linear convergence",
  realTimeNFSP: "Online updates + sliding window regret → adapt to non-stationary opponents",
  transformerNFSP: "Transformer policy/value nets → better long-horizon modeling & opponent attention",
  hybridCFRReBeL: "NFSP blueprint + CFR-D regret + search → fast equilibrium approximation",
  multiAgentMixedMotive: "Population-based multi-agent + valence-gated positive-sum bias → cooperative cycles",
  mercy_override: "Valence gating before episode → high valence → lower ε (more π̄ play, stability) + positive-sum action bias"
};

/**
 * Valence-modulated ε (exploration rate) – variant-aware
 */
export function valenceModulatedEpsilon(variant: 'deep' | 'realTime' | 'transformer', valence: number = currentValence.get()): number {
  const actionName = `Valence-modulated NFSP epsilon (${variant})`;
  if (!mercyGate(actionName)) return 0.1;

  const baseEpsilon = variant === 'transformer' ? 0.05 : variant === 'realTime' ? 0.15 : 0.1;
  return baseEpsilon * (1 - (valence - 0.95) * 2); // high valence → lower ε
}

/**
 * Positive-sum bias for action utilities (mixed-motive variant)
 */
export function positiveSumBias(actionUtilities: number[], variant: 'multiAgentMixedMotive', valence: number = currentValence.get()): number[] {
  const actionName = `Positive-sum bias (${variant})`;
  if (!mercyGate(actionName)) return actionUtilities;

  const biasFactor = (valence - 0.95) * 2;
  const biased = actionUtilities.slice();

  // Assume index 0 = cooperate/ally, index last = betray/compete
  if (biased.length > 1) {
    biased[0] += biasFactor * 0.5;
    biased[biased.length - 1] -= biasFactor * 0.5;
  }

  return biased;
}

// Usage example in NFSP loop
// const epsilon = valenceModulatedEpsilon('transformer');
// const biasedUtilities = positiveSumBias(rawUtilities, 'multiAgentMixedMotive');
// const action = selectAction(biasedUtilities, epsilon);
