// src/sync/custom-resolver-framework.ts – Custom Resolver Framework v1
// Unified valence-weighted semantic resolution for Yjs, Automerge, ElectricSQL
// mercy-gated, thriving-aligned, intention-preserving
// MIT License – Autonomicity Games Inc. 2026

import { currentValence } from '@/core/valence-tracker';
import { mercyGate } from '@/core/mercy-gate';
import * as Y from 'yjs';
import * as Automerge from '@automerge/automerge';

const MERCY_THRESHOLD = 0.9999999;

/**
 * Unified custom resolver context
 */
interface ResolverContext {
  key: string;                    // field/key being resolved
  localValue: any;
  localValence: number;
  localTimestamp: number | string; // Lamport/clock or LC timestamp
  remoteValue: any;
  remoteValence: number;
  remoteTimestamp: number | string;
  actionName?: string;
}

/**
 * Core valence-weighted semantic resolver
 * Higher valence change wins; fallback to timestamp/clientId
 */
export function valenceWeightedResolver(context: ResolverContext): any {
  const {
    key,
    localValue,
    localValence,
    localTimestamp,
    remoteValue,
    remoteValence,
    remoteTimestamp,
    actionName = `Resolve semantic conflict for key: ${key}`
  } = context;

  if (!mercyGate(actionName, key)) {
    console.warn(`[MercyResolver] Gate blocked – using raw timestamp fallback`);
    // Native fallback (higher timestamp wins)
    return localTimestamp > remoteTimestamp ? localValue : remoteValue;
  }

  // Valence primary tie-breaker
  if (localValence > remoteValence + 0.05) {
    console.log(`[MercyResolver] Valence wins: local (\( {localValence.toFixed(4)}) > remote ( \){remoteValence.toFixed(4)})`);
    return localValue;
  } else if (remoteValence > localValence + 0.05) {
    console.log(`[MercyResolver] Valence wins: remote (\( {remoteValence.toFixed(4)}) > local ( \){localValence.toFixed(4)})`);
    return remoteValue;
  }

  // Secondary tie-breaker: timestamp (native behavior)
  if (localTimestamp > remoteTimestamp) {
    console.log(`[MercyResolver] Timestamp fallback: local wins`);
    return localValue;
  } else if (remoteTimestamp > localTimestamp) {
    console.log(`[MercyResolver] Timestamp fallback: remote wins`);
    return remoteValue;
  }

  // Ultimate fallback: preserve local (client preference)
  console.log(`[MercyResolver] Full tie – preserving local value`);
  return localValue;
}

/**
 * Yjs-specific wrapper (for custom Y.Map item resolution)
 */
export function yjsValenceResolver(
  key: string,
  localItem: Y.Item,
  remoteItem: Y.Item
): Y.Item | null {
  const localVal = localItem.content?.getContent()?.[0];
  const remoteVal = remoteItem.content?.getContent()?.[0];

  const ctx: ResolverContext = {
    key,
    localValue: localVal,
    localValence: currentValence.get(),
    localTimestamp: localItem.id.clock,
    remoteValue: remoteVal,
    remoteValence: currentValence.get(), // proxy – real impl would carry valence metadata
    remoteTimestamp: remoteItem.id.clock
  };

  const resolvedValue = valenceWeightedResolver(ctx);
  if (resolvedValue === localVal) return localItem;
  if (resolvedValue === remoteVal) return remoteItem;

  // Fallback: let Yjs native YATA decide
  return null;
}

/**
 * Automerge-specific wrapper (for map key conflict)
 */
export function automergeValenceResolver(
  key: string,
  localChange: Automerge.Change<any>,
  remoteChange: Automerge.Change<any>
): Automerge.Change<any> | null {
  const ctx: ResolverContext = {
    key,
    localValue: localChange.value,
    localValence: localChange.valence ?? currentValence.get(),
    localTimestamp: localChange.timestamp,
    remoteValue: remoteChange.value,
    remoteValence: remoteChange.valence ?? currentValence.get(),
    remoteTimestamp: remoteChange.timestamp
  };

  const resolved = valenceWeightedResolver(ctx);
  if (resolved === localChange.value) return localChange;
  if (resolved === remoteChange.value) return remoteChange;

  return null; // fallback to native LWW
}

/**
 * ElectricSQL SQL resolver snippet (pseudo-code – paste into schema)
 */
export const electricValenceResolverSQL = `
CREATE RESOLVER valence_resolver FOR valence AS
  SELECT CASE
    WHEN NEW.valence > OLD.valence + 0.05 THEN NEW.valence
    WHEN OLD.valence > NEW.valence + 0.05 THEN OLD.valence
    ELSE GREATEST(NEW.valence, OLD.valence)
  END;
`;

// Usage example in sync handler
// const resolved = valenceWeightedResolver({ key: 'resources', localValue: 42, localValence: 0.999, ... });
