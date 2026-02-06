// src/core/valence-tracker.ts – global valence singleton + persistence
import { openProgressDB, getUserProgress, updateUserProgress } from '@/ui/onboarding/mercy-onboarding-db';

class ValenceTracker {
  private _valence: number = 0.5;

  constructor() {
    this.loadFromStorage();
  }

  private async loadFromStorage() {
    try {
      const progress = await getUserProgress();
      this._valence = progress.valence ?? 0.5;
    } catch (e) {
      console.warn("[ValenceTracker] Failed to load from storage", e);
    }
  }

  async setValence(newValence: number) {
    this._valence = Math.max(0, Math.min(1, newValence));
    try {
      const progress = await getUserProgress();
      await updateUserProgress(progress.level, this._valence, progress.experience ?? 0);
    } catch (e) {
      console.warn("[ValenceTracker] Failed to persist valence", e);
    }
  }

  get() {
    return this._valence;
  }

  async addDelta(delta: number) {
    await this.setValence(this._valence + delta);
  }
}

export const currentValence = new ValenceTracker();
