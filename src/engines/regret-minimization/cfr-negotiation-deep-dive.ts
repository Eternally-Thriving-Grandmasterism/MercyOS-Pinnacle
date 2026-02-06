// src/engines/regret-minimization/cfr-negotiation-deep-dive.ts – CFR Negotiation Deep Dive Reference & Mercy Helpers v1
// Detailed mechanics, valence-gated positive-sum bias, mercy gates
// MIT License – Autonomicity Games Inc. 2026

import { currentValence } from '@/core/valence-tracker';
import { mercyGate } from '@/core/mercy-gate';

const MERCY_THRESHOLD = 0.9999999;

/**
 * CFR negotiation mechanics reference – detailed loop & mercy alignment
 */
export const CFRNegotiationMechanics = {
  initialization: "Uniform random strategy σ⁰, regret sum R⁰=0, strategy sum Σ⁰=0",
  iterationLoop: "Sample episode → compute counterfactual values → update regrets → regret matching → accumulate average policy",
  counterfactualRegret: "Rᵗ(a) = counterfactual utility of a − utility of current strategy",
  regretMatchingPlus: "Clip negative regrets to 0, accumulate positive regrets",
  negotiationLayer: "Dialogue policy generates messages conditioned on strategy & intent belief → influences reach probabilities & regrets",
  mercy_override: "Valence gating down-weights betrayal paths → positive-sum equilibria emerge when valence high",
  convergence: "Sublinear regret → average strategy converges to Nash (zero-sum) / coarse correlated equilibrium (mixed-motive)",
  multiplanetary_note: "Latency-tolerant via offline change queuing → eventual consistency across nodes"
};

/**
 * Valence-gated positive-sum bias for CFR action selection
 */
export function valencePositiveSumBias(
  actionUtilities: number[],
  currentStrategy: number[],
  valence: number = currentValence.get()
): number[] {
  const actionName = `Valence-gated positive-sum bias`;
  if (!mercyGate(actionName)) {
    return currentStrategy; // fallback to raw CFR strategy
  }

  // Boost cooperative/positive-sum actions when valence high
  const biasFactor = (valence - 0.95) * 2; // stronger bias above 0.95
  const biasedUtilities = actionUtilities.map((u, i) => {
    // Example: assume index 0 = cooperate, 4 = betray
    if (i === 0) return u + biasFactor * 0.5;
    if (i === 4) return u - biasFactor * 0.5;
    return u;
  });

  // Re-normalize into strategy (softmax-like)
  const maxU = Math.max(...biasedUtilities);
  const expU = biasedUtilities.map(u => Math.exp(u - maxU));
  const sumExp = expU.reduce((a, b) => a + b, 0);
  return expU.map(e => e / sumExp);
}

// Usage example in CFR action selection
// const biasedStrategy = valencePositiveSumBias(actionUtilities, currentStrategy);
