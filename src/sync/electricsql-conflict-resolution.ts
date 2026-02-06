// src/sync/electricsql-conflict-resolution.ts – ElectricSQL Conflict Resolution Helpers v1
// Valence-weighted custom resolver, mercy gates, relational sync support
// MIT License – Autonomicity Games Inc. 2026

import { currentValence } from '@/core/valence-tracker';
import { mercyGate } from '@/core/mercy-gate';

const MERCY_THRESHOLD = 0.9999999;

/**
 * Valence-weighted custom resolver for ElectricSQL columns
 * Higher valence change wins in concurrent updates
 */
export function valenceElectricResolver(
  columnName: string,
  localValue: any,
  localValence: number,
  remoteValue: any,
  remoteValence: number
): any {
  const actionName = `ElectricSQL conflict resolver for column: ${columnName}`;
  if (!mercyGate(actionName, columnName)) {
    // Fallback to last-writer-wins (LC timestamp already handled by ElectricSQL)
    return localValue; // client-side preference
  }

  if (localValence > remoteValence + 0.05) {
    console.log(`[MercyElectric] Conflict resolved: local wins (valence ${localValence.toFixed(4)})`);
    return localValue;
  } else if (remoteValence > localValence + 0.05) {
    console.log(`[MercyElectric] Conflict resolved: remote wins (valence ${remoteValence.toFixed(4)})`);
    return remoteValue;
  }

  // Fallback to higher value (or other domain logic)
  return localValence > remoteValence ? localValue : remoteValue;
}

// Example usage in ElectricSQL shape resolvers (pseudo-code in schema)
// CREATE RESOLVER valence_resolver FOR valence AS
// SELECT valenceWeightedElectricResolver('valence', NEW.valence, NEW.valence_source, OLD.valence, OLD.valence_source);
