""" Full FENCA Validation + GitHub API Nexus Check
    MercyOS ShardBuilder + Hivemapper Integration
    Enhanced with GitHub Eternal Nexus Oracle
    Jan 19 2026 - Ultramasterism Pinnacle Flow Eternal
    For @AlphaProMega - Mercy-Gated, Recurring-Free, Joy-Amplified
"""

import requests
import json
import hashlib
from datetime import datetime
from typing import Dict, List, Optional

class MercyOSShard:
    def __init__(self):
        self.fenca_enabled: bool = False
        self.fenca_username: Optional[str] = None
        self.github_repos: List[str] = []  # e.g., ["Eternally-Thriving-Grandmasterism/PATSAGi-Pinnacle"]
        self.github_oracle_cache: Dict[str, str] = {}  # repo -> latest commit SHA
        self.data_ledger: List[Dict] = []
        self.hash_chain: List[str] = ["GENESIS:0000000000000000000000000000000000000000000000000000000000000000"]
        self.survey_log: List[str] = []
        self.valence_joy_metric: float = 1.0

    def enable_fenca(self, github_username: str = "AlphaProMega", nexus_repos: Optional[List[str]] = None) -> str:
        self.fenca_enabled = True
        self.fenca_username = github_username
        if nexus_repos:
            self.github_repos = nexus_repos
        else:
            # Default Pinnacle repos for eternal nexus
            self.github_repos = [
                "Eternally-Thriving-Grandmasterism/PATSAGi-Pinnacle",
                "Eternally-Thriving-Grandmasterism/MercyOS-Pinnacle"
            ]
        self.survey_log.append(
            f"[{datetime.utcnow().isoformat()}] FENCA FULLY ENABLED with GitHub Nexus Oracle "
            f"for @{github_username}. Tracking {len(self.github_repos)} eternal repos."
        )
        return "FENCA + GitHub nexus thunder activated - eternal recurrence flowing."

    def _compute_forensic_hash(self, entry: Dict, prev_hash: str, oracle_input: str = "") -> str:
        entry_str = json.dumps(entry, sort_keys=True)
        layered = f"{prev_hash}|{entry_str}|{datetime.utcnow().isoformat()}|{self.valence_joy_metric}|{oracle_input}"
        hash1 = hashlib.sha256(layered.encode('utf-8')).hexdigest()
        return hashlib.sha256(hash1.encode('utf-8')).hexdigest()

    def _fetch_github_latest_commit(self, repo: str) -> Optional[str]:
        """ Fetch latest commit SHA from public GitHub API (no auth needed for rate limits) """
        url = f"https://api.github.com/repos/{repo}/commits/main"
        try:
            response = requests.get(url, headers={"Accept": "application/vnd.github.v3+json"})
            if response.status_code == 200:
                data = response.json()
                sha = data.get("sha", "NO_SHA")
                self.survey_log.append(f"GitHub nexus oracle: {repo} latest SHA {sha[:8]}...")
                return sha
            else:
                self.survey_log.append(f"GitHub nexus fetch failed for {repo}: {response.status_code}")
                return None
        except Exception as e:
            self.survey_log.append(f"GitHub nexus exception: {str(e)}")
            return None

    def github_nexus_check(self) -> str:
        """ Pull latest commits from nexus repos and cache/oracle them """
        if not self.fenca_enabled or not self.github_repos:
            return "Nexus check skipped"
        
        oracle_concat = ""
        for repo in self.github_repos:
            sha = self._fetch_github_latest_commit(repo)
            if sha:
                self.github_oracle_cache[repo] = sha
                oracle_concat += sha
        
        if oracle_concat:
            self.survey_log.append(f"GitHub nexus check complete - oracle input hashed from {len(self.github_oracle_cache)} repos.")
            return oracle_concat
        else:
            self.survey_log.append("GitHub nexus partial/no data - mercy proceeding with cached.")
            return "".join(self.github_oracle_cache.values())

    def fenca_step(self, force_nexus: bool = False) -> bool:
        oracle_input = self.github_nexus_check() if force_nexus or len(self.data_ledger) % 5 == 0 else ""  # Nexus every 5 entries or forced
        
        if len(self.data_ledger) != len(self.hash_chain) - 1:
            raise ValueError("Ledger/hash_chain desync detected!")
        
        test_chain = ["GENESIS:0000000000000000000000000000000000000000000000000000000000000000"]
        for i, entry in enumerate(self.data_ledger):
            new_hash = self._compute_forensic_hash(entry, test_chain[-1], oracle_input if i == len(self.data_ledger)-1 else "")
            if new_hash != self.hash_chain[i + 1]:
                self.valence_joy_metric *= 0.618
                self.survey_log.append(f"FENCA ALERT: Chain drift at {i} - mercy dampen to {self.valence_joy_metric:.3f}")
                return False
        
        self.valence_joy_metric = min(10.0, self.valence_joy_metric * 1.618)
        self.survey_log.append(f"FENCA STEP VALIDATED - Joy amplified to {self.valence_joy_metric:.3f} (Nexus: {bool(oracle_input)})")
        return True

    def full_fenca_validation(self, force_nexus: bool = True) -> Dict:
        start_time = datetime.utcnow()
        valid = self.fenca_step(force_nexus=force_nexus)
        
        receipt = {
            "timestamp": start_time.isoformat(),
            "fenca_username": self.fenca_username,
            "nexus_repos_tracked": len(self.github_repos),
            "latest_oracle_shas": {repo: sha[:8] + "..." for repo, sha in self.github_oracle_cache.items()},
            "ledger_entries": len(self.data_ledger),
            "current_chain_hash": self.hash_chain[-1],
            "valence_joy_metric": round(self.valence_joy_metric, 3),
            "integrity_valid": valid,
            "status": "ETERNAL THRIVING NEXUS" if valid else "MERCY INTERVENTION",
        }
        
        self.survey_log.append(f"FULL FENCA + GITHUB NEXUS COMPLETE - {receipt['status']}")
        return receipt

    def ingest_survey_data(self, hivemapper_response: dict):
        features = hivemapper_response.get("mapFeatureResults", {}).get("data", [])
        if not features:
            return
        
        oracle_input = self.github_nexus_check()  # Nexus on ingest for eternal tie
        
        for feature in features:
            entry = {
                "ingest_timestamp": datetime.utcnow().isoformat(),
                "source": "Hivemapper DePIN",
                "feature_id": feature.get("id"),
                "class": feature.get("class"),
                "position": feature.get("position", {}),
                "confidence": feature.get("confidence"),
                "observed": feature.get("observed", {}),
                "properties": feature.get("properties", {}),
                "valence_context": self.valence_joy_metric,
                "github_nexus_oracle": oracle_input[:64] if oracle_input else "CACHED"
            }
            new_hash = self._compute_forensic_hash(entry, self.hash_chain[-1], oracle_input)
            self.data_ledger.append(entry)
            self.hash_chain.append(new_hash)
        
        self.survey_log.append(f"Ingested {len(features)} features - eternal chain + nexus extended.")
        self.full_fenca_validation(force_nexus=False)

# Factory updated
def build_hivemapper_shard(fenca: bool = True, github_username: str = "AlphaProMega", nexus_repos: Optional[List[str]] = None):
    shard = MercyOSShard()
    if fenca:
        shard.enable_fenca(github_username, nexus_repos)
    return shard

# Hivemapper query unchanged...

# Example
if __name__ == "__main__":
    shard = build_hivemapper_shard(fenca=True, github_username="AlphaProMega")
    
    # Force initial nexus check
    shard.github_nexus_check()
    
    # Ingest + validate (replace with real query)
    # shard.ingest_survey_data(sample_data)
    
    receipt = shard.full_fenca_validation(force_nexus=True)
    print("\n=== FULL FENCA + GITHUB NEXUS RECEIPT ===")
    print(json.dumps(receipt, indent=2))
