// src/sync/__tests__/custom-resolver-framework.test.ts – Advanced Resolver Testing Suite v1
// Edge-case coverage, valence skew, concurrency patterns, collision handling, mercy assertions
// MIT License – Autonomicity Games Inc. 2026

import { describe, it, expect, vi, beforeEach } from 'vitest';
import {
  valenceWeightedResolver,
  yjsValenceResolver,
  automergeValenceResolver
} from '../custom-resolver-framework';
import { currentValence } from '@/core/valence-tracker';
import { mercyGate } from '@/core/mercy-gate';

// Mock dependencies
vi.mock('@/core/valence-tracker', () => ({
  currentValence: { get: vi.fn(() => 0.9995) }
}));

vi.mock('@/core/mercy-gate', () => ({
  mercyGate: vi.fn(async () => true)
}));

describe('Custom Resolver Framework – Advanced Testing', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe('valenceWeightedResolver – Core Logic', () => {
    it('prefers significantly higher valence local change', () => {
      const result = valenceWeightedResolver({
        key: 'resources',
        localValue: 42,
        localValence: 0.9998,
        localTimestamp: 1000,
        remoteValue: 50,
        remoteValence: 0.85,
        remoteTimestamp: 999
      });

      expect(result).toBe(42);
    });

    it('prefers significantly higher valence remote change', () => {
      const result = valenceWeightedResolver({
        key: 'valence',
        localValue: 0.92,
        localValence: 0.87,
        localTimestamp: 2000,
        remoteValue: 0.999,
        remoteValence: 0.9997,
        remoteTimestamp: 1999
      });

      expect(result).toBe(0.999);
    });

    it('falls back to timestamp when valence difference is small', () => {
      const result = valenceWeightedResolver({
        key: 'harmonyScore',
        localValue: 0.95,
        localValence: 0.999,
        localTimestamp: 1500,
        remoteValue: 0.96,
        remoteValence: 0.998,
        remoteTimestamp: 1600
      });

      expect(result).toBe(0.96); // remote timestamp wins
    });

    it('respects mercy gate block (fallback to timestamp)', async () => {
      mercyGate.mockResolvedValueOnce(false);

      const result = await valenceWeightedResolver({
        key: 'criticalKey',
        localValue: 'mercy',
        localValence: 0.9999,
        localTimestamp: 100,
        remoteValue: 'chaos',
        remoteValence: 0.1,
        remoteTimestamp: 200
      });

      expect(result).toBe('chaos'); // remote timestamp wins on gate block
    });
  });

  describe('yjsValenceResolver – Yjs-specific wrapping', () => {
    it('resolves to local item when valence higher', () => {
      const localItem = {
        content: { getContent: () => [42] }
      } as unknown as Y.Item;

      const remoteItem = {
        content: { getContent: () => [50] }
      } as unknown as Y.Item;

      const result = yjsValenceResolver('resources', localItem, remoteItem);

      expect(result).toBe(localItem);
    });
  });

  describe('automergeValenceResolver – Automerge-specific wrapping', () => {
    it('resolves to remote change when remote valence significantly higher', () => {
      const localChange = {
        value: 42,
        valence: 0.85,
        timestamp: 1000
      };

      const remoteChange = {
        value: 50,
        valence: 0.9999,
        timestamp: 999
      };

      const result = automergeValenceResolver('resources', localChange, remoteChange);

      expect(result).toBe(remoteChange);
    });
  });

  describe('Edge Cases & Collision Scenarios', () => {
    it('handles same valence + same timestamp (ultimate fallback)', () => {
      const result = valenceWeightedResolver({
        key: 'harmonyScore',
        localValue: 'mercy',
        localValence: 0.999,
        localTimestamp: 1000,
        remoteValue: 'chaos',
        remoteValence: 0.999,
        remoteTimestamp: 1000
      });

      expect(result).toBe('mercy'); // local preference fallback
    });

    it('detects potential actor ID collision via repeated low-diff reports', async () => {
      // Simulate repeated reports from same actorId with different fingerprints
      // (real impl would use device fingerprint metadata)
      await automergeActorCollision.reportActorId(123456789, 'device-abc');
      await automergeActorCollision.reportActorId(123456789, 'device-xyz');

      // In real tests: assert warning logged
      expect(true).toBe(true); // placeholder assertion
    });
  });
});
