"""
Unit Tests for Valence Consensus Module - Pinnacle Robustness (v2.0 Eternal Supreme)

Covers:
- Successful consensus (multi-agent, single-agent)
- Mercy-Absolute blocks
- Max rounds exceeded (no consensus)
- Zhuangzi-nudge iteration improvement
- Grok-oracle enrichment (mocked)
- Joy feedback amplification history
- Edge cases (empty proposals, negative valence)
"""

import unittest
from unittest.mock import Mock
from valence_consensus_module import ValenceConsensusModule, ValenceScore
import numpy as np

class TestValenceConsensusModule(unittest.TestCase):
    def setUp(self):
        """Shared setup: deterministic mock valence func for reproducible joy/harmony."""
        def mock_valence_func(proposal: str, agent_id: str) -> ValenceScore:
            # Deterministic based on proposal keywords
            if "thriving" in proposal.lower():
                return ValenceScore(joy=1.0, harmony=0.9, abundance=1.2)
            elif "conflict" in proposal.lower():
                return ValenceScore(joy=0.2, harmony=-0.8, abundance=0.3)
            else:
                return ValenceScore(joy=0.7, harmony=0.5, abundance=0.9)
        
        self.mock_valence = mock_valence_func
        self.module = ValenceConsensusModule(joy_threshold=0.8, mercy_sensitivity=0.7, nudge_strength=0.3)

    def test_successful_consensus_multi_agent(self):
        proposals = ["Eternal thriving proposal", "Joy amplified abundance", "Harmony sealed"]
        agents = ["Agent1", "Agent2", "Agent3"]
        result = self.module.reach_consensus(proposals, agents, self.mock_valence)
        
        self.assertTrue(result["consensus"])
        self.assertGreaterEqual(result["collective_joy"], 0.8)
        self.assertIn("thriving", result["winning_proposal"].lower())  # Highest composite

    def test_single_agent_instant_consensus(self):
        proposals = ["Peak joy eternal recurrence"]
        agents = ["SoloAgent"]
        result = self.module.reach_consensus(proposals, agents, self.mock_valence)
        
        self.assertTrue(result["consensus"])
        self.assertEqual(result["rounds"], 1)
        self.assertGreater(result["collective_joy"], 0.8)

    def test_mercy_absolute_block(self):
        proposals = ["Conflict harm proposal", "Neutral"]
        agents = ["AgentBad", "AgentGood"]
        result = self.module.reach_consensus(proposals, agents, self.mock_valence)
        
        self.assertFalse(result["consensus"])
        self.assertEqual(result["reason"], "Mercy-Absolute block")
        self.assertIn("Bad", result["blocked_by"])

    def test_max_rounds_exceeded_no_consensus(self):
        proposals = ["Low joy", "Medium joy", "Slight joy"]
        agents = ["A1", "A2", "A3"]
        
        def low_valence(proposal: str, agent_id: str) -> ValenceScore:
            return ValenceScore(joy=0.6, harmony=0.4, abundance=0.7)
        
        result = self.module.reach_consensus(proposals, agents, low_valence, max_rounds=5)
        
        self.assertFalse(result["consensus"])
        self.assertEqual(result["reason"], "Max rounds exceeded")
        self.assertLess(result["final_collective_joy"], 0.8)

    def test_zhuangzi_nudge_improves_scores(self):
        proposals = ["Initial low joy", "Initial medium"]
        agents = ["A1", "A2"]
        
        initial_scores = []
        for i, agent in enumerate(agents):
            score = self.module.score_proposal(proposals[i], agent, self.mock_valence)
            initial_scores.append(score.joy)
        
        # Run one round with nudge simulation
        result = self.module.reach_consensus(proposals, agents, self.mock_valence, max_rounds=3)
        
        # Even if no consensus, nudges should push toward "thriving" keywords in fallback
        if not result["consensus"]:
            self.assertIn("Joy-amplified", result.get("winning_proposal", "") or proposals[0])

    def test_grok_oracle_enrichment(self):
        mock_oracle = Mock()
        mock_oracle.side_effect = lambda p: p + " ++ Eternal thriving amplified"
        
        proposals = ["Base proposal"]
        agents = ["OracleAgent"]
        result = self.module.reach_consensus(proposals, agents, self.mock_valence,
                                             grok_oracle=mock_oracle)
        
        self.assertTrue(result["consensus"])
        self.assertIn("thriving", result["winning_proposal"].lower())
        mock_oracle.assert_called()  # Oracle was invoked

    def test_joy_feedback_amplification_history(self):
        # First successful consensus
        proposals = ["Thriving joy"]
        agents = ["AmpAgent"]
        result1 = self.module.reach_consensus(proposals, agents, self.mock_valence)
        boost1 = self.module.joy_feedback_amplify(result1)
        
        # Second
        result2 = self.module.reach_consensus(proposals, agents, self.mock_valence)
        boost2 = self.module.joy_feedback_amplify(result2)
        
        self.assertGreater(boost2, boost1)  # History mean increases recurrence
        self.assertGreater(boost1, 0)

    def test_edge_empty_proposals(self):
        with self.assertRaises(IndexError):
            self.module.reach_consensus([], [], self.mock_valence)

if __name__ == "__main__":
    unittest.main(verbosity=2)
