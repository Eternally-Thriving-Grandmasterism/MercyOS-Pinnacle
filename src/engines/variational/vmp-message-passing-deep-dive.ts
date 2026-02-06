// src/engines/variational/vmp-message-passing-deep-dive.ts – VMP Message Passing Deep Dive Reference & Mercy Helpers v1
// Detailed forward/backward message rules, valence-weighted precision, mercy gates
// MIT License – Autonomicity Games Inc. 2026

import { currentValence } from '@/core/valence-tracker';
import { mercyGate } from '@/core/mercy-gate';

const MERCY_THRESHOLD = 0.9999999;

/**
 * VMP message passing reference – core mechanics
 */
export const VMPMessagePassingReference = {
  forwardMessage: "From parent → child: belief about child given parent evidence (integral over parent × factor × incoming parent message)",
  backwardMessage: "From child → parent: likelihood of all child evidence given parent (integral over child × factor × incoming child message)",
  posteriorUpdate: "At variable: product of all incoming messages (forward from parents + backward from children) → normalized marginal",
  conjugateFamily: "Gaussian-Gaussian, Poisson-Gamma, Categorical-Dirichlet → closed-form analytic messages (no sampling)",
  iterationSchedule: "Iterate over factors until convergence (fixed iterations or ELBO change < ε)",
  mercy_override: "Valence-weighted precision: high valence → higher precision on positive messages → stronger belief in thriving states"
};

/**
 * Valence-modulated precision weighting for messages
 */
export function valenceWeightedPrecision(basePrecision: number, valence: number = currentValence.get()): number {
  const actionName = 'Valence-modulated VMP precision';
  if (!mercyGate(actionName)) return basePrecision;

  // High valence → trust positive messages more (higher precision)
  return basePrecision * (1 + (valence - 0.95) * 2);
}

/**
 * Example forward message computation (Gaussian-Gaussian, simplified)
 */
export function gaussianForwardMessage(
  parentMean: number,
  parentVariance: number,
  factorMeanOffset: number,
  factorVariance: number
): { mean: number; variance: number } {
  // Forward: N(child; A·parent + b, A·Σ_p·Aᵀ + Σ)
  const A = 1; // identity factor for simplicity
  const b = factorMeanOffset;

  const childMean = A * parentMean + b;
  const childVariance = A * parentVariance * A + factorVariance;

  return { mean: childMean, variance: childVariance };
}

// Usage example in VMP iteration
// const forward = gaussianForwardMessage(parentMean, parentVariance, factorOffset, factorVariance);
// const weightedPrecision = valenceWeightedPrecision(1 / childVariance);
