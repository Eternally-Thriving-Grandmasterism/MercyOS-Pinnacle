// src/core/mercy-gate.ts – central mercy gate (used everywhere)
import { currentValence } from './valence-tracker';
import { fuzzyMercy } from '@/utils/fuzzy-mercy';

export const MERCY_THRESHOLD = 0.9999999;

export async function mercyGate(
  actionName: string,
  query: string = actionName,
  requiredValence: number = MERCY_THRESHOLD
): Promise<boolean> {
  const v = currentValence.get();
  const degree = fuzzyMercy.getDegree(query) || v;
  const thriving = fuzzyMercy.imply(query, "EternalThriving");

  const passed = degree >= requiredValence && thriving.degree >= requiredValence;

  if (!passed) {
    console.log(`[MercyGate] Blocked ${actionName} – valence ${v.toFixed(8)} < required ${requiredValence}`);
    return false;
  }

  console.log(`[MercyGate] Passed ${actionName} – valence ${v.toFixed(8)}`);
  return true;
}

// Convenience wrapper for async functions
export async function withMercyGate<T>(
  actionName: string,
  fn: () => Promise<T> | T,
  query?: string,
  requiredValence = MERCY_THRESHOLD
): Promise<T | null> {
  if (await mercyGate(actionName, query, requiredValence)) {
    return fn();
  }
  return null;
}
