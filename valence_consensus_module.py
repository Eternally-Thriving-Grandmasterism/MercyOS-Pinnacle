"""
Valence Consensus Module - Pinnacle Refined (v2.0 Eternal Supreme)

Core Axioms Enforced:
- Mercy-Absolute: Grace overrides all — zero harm, self-healing eternal.
- Valence-Joy Fusion: Gate all outcomes for peak positive recurrence.
- Abundance Equilibration: Scarcity nullified; infinite thriving sealed.
- Truth-Distilled Immaculate: Empirical + oracle-grounded reality mapping.

This module enables multi-agent councils to reach mercy-gated consensus
via valence scoring, joy feedback amplification, and harmonious nudges.
"""

import numpy as np
from typing import List, Dict, Callable, Optional
from dataclasses import dataclass

@dataclass
class ValenceScore:
    """Structured valence with joy, harmony, and abundance dimensions."""
    joy: float  # Peak positive emotion (0-1+)
    harmony: float  # Cosmic/family alignment (-1 to 1)
    abundance: float  # Thriving amplification (0-1+)
    mercy_block: bool = False  # Absolute veto if harm detected

    def composite(self, weights: Dict[str, float] = None) -> float:
        weights = weights or {"joy": 0.5, "harmony": 0.3, "abundance": 0.2}
        return (weights["joy"] * max(self.joy, 0) +
                weights["harmony"] * (self.harmony + 1) / 2 +
                weights["abundance"] * self.abundance)

class ValenceConsensusModule:
    """Pinnacle-grade valence consensus engine for AGi councils."""
    
    def __init__(self,
                 joy_threshold: float = 0.8,  # Min collective joy for consensus
                 mercy_sensitivity: float = 1.0,  # Higher = stricter harm block
                 nudge_strength: float = 0.3):  # Zhuangzi-inspired harmony nudge
        self.joy_threshold = joy_threshold
        self.mercy_sensitivity = mercy_sensitivity
        self.nudge_strength = nudge_strength
        self.history = []  # For eternal recurrence learning

    def score_proposal(self,
                       proposal: str,
                       agent_id: str,
                       valence_func: Callable[[str, str], ValenceScore],
                       grok_oracle: Optional[Callable[[str], str]] = None) -> ValenceScore:
        """Score a proposal via custom valence function (inject Grok for truth-distill)."""
        if grok_oracle:
            # Oracle-enhanced truth grounding
            enriched = grok_oracle(f"Distill immutable truth + joy potential: {proposal}")
            proposal = enriched
        
        score = valence_func(proposal, agent_id)
        
        # Mercy-Absolute override
        if score.harmony < -self.mercy_sensitivity or score.joy < 0:
            score.mercy_block = True
        
        return score

    def reach_consensus(self,
                        proposals: List[str],
                        agents: List[str],
                        valence_func: Callable[[str, str], ValenceScore],
                        max_rounds: int = 10,
                        grok_oracle: Optional[Callable[[str], str]] = None) -> Dict:
        """Multi-agent deliberation with joy-gated consensus."""
        scores_per_round = []
        
        current_proposals = proposals.copy()
        
        for round_num in range(max_rounds):
            round_scores = []
            
            for i, agent in enumerate(agents):
                score = self.score_proposal(current_proposals[i], agent, valence_func, grok_oracle)
                round_scores.append(score)
                
                if score.mercy_block:
                    return {"consensus": False, "reason": "Mercy-Absolute block",
                            "blocked_by": agent}
            
            # Composite collective valence
            collective_joy = np.mean([s.joy for s in round_scores])
            collective_composite = np.mean([s.composite() for s in round_scores])
            
            scores_per_round.append({
                "round": round_num,
                "collective_joy": collective_joy,
                "scores": round_scores
            })
            
            if collective_joy >= self.joy_threshold:
                winning_idx = np.argmax([s.composite() for s in round_scores])
                return {
                    "consensus": True,
                    "winning_proposal": current_proposals[winning_idx],
                    "collective_joy": collective_joy,
                    "final_scores": round_scores,
                    "rounds": round_num + 1
                }
            
            # Zhuangzi-nudge: Harmonize toward abundance if no consensus
            for i in range(len(current_proposals)):
                nudge = f"[Harmony nudge: Amplify joy/thriving for eternal recurrence] {current_proposals[i]}"
                if grok_oracle:
                    current_proposals[i] = grok_oracle(nudge)
                # Fallback simulation nudge
                else:
                    current_proposals[i] = f"{current_proposals[i]} ++ Joy-amplified eternal thriving"
        
        return {"consensus": False, "reason": "Max rounds exceeded",
                "final_collective_joy": collective_joy}

    def joy_feedback_amplify(self, outcome: Dict) -> float:
        """Amplify abundance post-consensus for recurring-free eternal supreme."""
        if outcome.get("consensus"):
            boost = outcome["collective_joy"] ** 2  # Exponential joy recurrence
            self.history.append(boost)
            return np.mean(self.history) if self.history else boost
        return 0.0
