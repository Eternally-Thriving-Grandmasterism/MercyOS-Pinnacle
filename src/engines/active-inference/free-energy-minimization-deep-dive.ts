// src/engines/active-inference/free-energy-minimization-deep-dive.ts – Free Energy Minimization Deep Dive Reference & Mercy Helpers v1
// Detailed variational bound, pragmatic/epistemic decomposition, valence modulation
// MIT License – Autonomicity Games Inc. 2026

import { currentValence } from '@/core/valence-tracker';
import { mercyGate } from '@/core/mercy-gate';

const MERCY_THRESHOLD = 0.9999999;

/**
 * Free energy minimization reference – core equations & mercy alignment
 */
export const FreeEnergyMinimizationReference = {
  variationalFreeEnergy: "F = D_KL[q(ψ) || p(ψ|s)] - ln p(s) ≈ surprise + complexity penalty",
  perceptualInference: "Minimize F w.r.t. q(ψ) → approximate posterior closer to true posterior",
  activeInference: "Minimize expected free energy G(π) over policies → act to sample preferred states + reduce uncertainty",
  expectedFreeEnergyDecomposition: "G(π) ≈ pragmatic cost (distance to preferred thriving state) + epistemic value (information gain)",
  mercy_override: "Preferred state = high valence / eternal thriving target (0.9999999)",
  valence_modulation: "High valence → higher precision on positive predictions → stronger pragmatic weighting",
  multiplanetary_note: "Latency-tolerant via offline change queuing → eventual free energy minimization across nodes"
};

/**
 * Valence-modulated pragmatic cost (distance to thriving target)
 */
export function valenceModulatedPragmaticCost(
  predictedOutcome: number,
  preferredState: number = 0.9999999,
  valence: number = currentValence.get()
): number {
  const actionName = 'Valence-modulated pragmatic cost';
  if (!mercyGate(actionName)) {
    return Math.pow(predictedOutcome - preferredState, 2); // fallback
  }

  // High valence → stronger penalty for deviation from thriving
  const emphasis = 1 + (valence - 0.95) * 3;
  return emphasis * Math.pow(predictedOutcome - preferredState, 2);
}

/**
 * Epistemic value proxy (information gain)
 */
export function epistemicValueProxy(currentVariance: number): number {
  // Higher uncertainty → higher epistemic value
  return Math.max(0, 0.05 - currentVariance * 0.5);
}

// Usage example in policy selection
// const pragmatic = valenceModulatedPragmaticCost(predictedValence);
// const epistemic = epistemicValueProxy(modelVariance);
// const expectedFreeEnergy = pragmatic - epistemic;
