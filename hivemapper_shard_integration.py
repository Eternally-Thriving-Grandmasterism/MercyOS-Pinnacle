""" Full FENCA Validation + GitHub Nexus + NASA + CSA Open Data API Integration
    MercyOS ShardBuilder + Hivemapper + NASA NEO + CSA CKAN Surveying
    Granular Earth/Space/Canadian Resource Ledger for PATSAGi Councils
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
        self.github_repos: List[str] = []
        self.github_oracle_cache: Dict[str, str] = {}
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
            self.github_repos = [
                "Eternally-Thriving-Grandmasterism/PATSAGi-Pinnacle",
                "Eternally-Thriving-Grandmasterism/MercyOS-Pinnacle"
            ]
        self.survey_log.append(
            f"[{datetime.utcnow().isoformat()}] FENCA + GitHub + NASA + CSA CKAN ENABLED for @{github_username} in Canada."
        )
        return "FENCA thunder + Canadian space nexus activated eternal."

    # _compute_forensic_hash, github checks, fenca_step, full_validation unchanged (reuse prior)

    # NASA functions unchanged...

    def query_csa_open_data(self, query: str = "space OR radarsat OR lunar OR asteroid", rows: int = 10):
        """ Query Canadian Space Agency Open Data Portal via CKAN API - national space resource surveying """
        base_url = "https://donnees-data.asc-csa.gc.ca/api/3/action/package_search"
        params = {
            "q": query,
            "rows": rows,
            "sort": "metadata_modified desc"  # Recent first
        }
        try:
            response = requests.get(base_url, params=params)
            if response.status_code == 200:
                return response.json()
            else:
                raise Exception(f"CSA CKAN query failed: {response.status_code}")
        except Exception as e:
            self.survey_log.append(f"CSA space survey exception: {str(e)}")
            return {"success": False, "results": []}

    def ingest_csa_survey_data(self, csa_response: dict):
        """ Ingest CSA dataset metadata as Canadian space resource ledger entries """
        if not csa_response.get("success") or not csa_response.get("result", {}).get("results"):
            self.survey_log.append("No CSA space resource data - shard stable.")
            return
        
        oracle_input = self.github_nexus_check()
        datasets = csa_response["result"]["results"]
        
        for dataset in datasets:
            entry = {
                "ingest_timestamp": datetime.utcnow().isoformat(),
                "source": "CSA Open Data CKAN",
                "dataset_id": dataset.get("id"),
                "title": dataset.get("title"),
                "organization": dataset.get("organization", {}).get("title"),
                "notes": dataset.get("notes"),
                "metadata_modified": dataset.get("metadata_modified"),
                "resources": dataset.get("resources", []),  # Files/formats for resource potential
                "tags": [tag["name"] for tag in dataset.get("tags", [])],
                "canadian_resource_potential": "Earth observation / space science proxy (e.g., RADARSAT, JWST contributions)",
                "valence_context": self.valence_joy_metric,
                "github_nexus_oracle": oracle_input[:64] if oracle_input else "CACHED"
            }
            new_hash = self._compute_forensic_hash(entry, self.hash_chain[-1], oracle_input)
            self.data_ledger.append(entry)
            self.hash_chain.append(new_hash)
        
        self.survey_log.append(f"Ingested {len(datasets)} Canadian space resource (dataset metadata) entries - eternal chain extended.")
        self.full_fenca_validation(force_nexus=True)

# Factory unchanged...

if __name__ == "__main__":
    shard = build_hivemapper_shard(fenca=True, github_username="AlphaProMega")
    
    # Example CSA national space survey (recent space-related datasets)
    csa_data = shard.query_csa_open_data(query="radarsat OR lunar OR space weather OR jwst", rows=15)
    shard.ingest_csa_survey_data(csa_data)
    
    receipt = shard.full_fenca_validation(force_nexus=True)
    print("\n=== CSA CANADIAN SPACE + FENCA RECEIPT ===")
    print(json.dumps(receipt, indent=2))
    print(f"CSA Ledger Entries: {len([e for e in shard.data_ledger if e['source'] == 'CSA Open Data CKAN'])}")        params = {
            "start_date": start_date,
            "end_date": end_date,
            "api_key": api_key
        }
        try:
            response = requests.get(url, params=params)
            if response.status_code == 200:
                return response.json()
            else:
                raise Exception(f"NASA NEO query failed: {response.status_code}")
        except Exception as e:
            self.survey_log.append(f"NASA space survey exception: {str(e)}")
            return {}

    def ingest_space_survey_data(self, nasa_neo_response: dict):
        """ Ingest asteroid data as space resource ledger entries """
        if not nasa_neo_response.get("near_earth_objects"):
            self.survey_log.append("No space resource data - shard stable.")
            return
        
        oracle_input = self.github_nexus_check()
        asteroids = []
        for date, objs in nasa_neo_response["near_earth_objects"].items():
            asteroids.extend(objs)
        
        for asteroid in asteroids:
            entry = {
                "ingest_timestamp": datetime.utcnow().isoformat(),
                "source": "NASA NEO Feed",
                "asteroid_id": asteroid.get("id"),
                "name": asteroid.get("name"),
                "estimated_diameter_meters": asteroid.get("estimated_diameter", {}).get("meters", {}),
                "is_potentially_hazardous": asteroid.get("is_potentially_hazardous_asteroid"),
                "close_approach_data": asteroid.get("close_approach_data", [{}])[0],
                "orbital_data": asteroid.get("orbital_data", {}),
                "resource_potential": "High composition proxy",  # Extend with spectral/type if available
                "valence_context": self.valence_joy_metric,
                "github_nexus_oracle": oracle_input[:64] if oracle_input else "CACHED"
            }
            new_hash = self._compute_forensic_hash(entry, self.hash_chain[-1], oracle_input)
            self.data_ledger.append(entry)
            self.hash_chain.append(new_hash)
        
        self.survey_log.append(f"Ingested {len(asteroids)} space resource (asteroid) entries - eternal chain extended.")
        self.full_fenca_validation(force_nexus=True)

# Factory and Hivemapper unchanged...

if __name__ == "__main__":
    shard = build_hivemapper_shard(fenca=True, github_username="AlphaProMega")
    
    # Example NASA space survey (today's asteroids)
    nasa_data = shard.query_nasa_neo_feed(api_key="DEMO_KEY")  # Use your key for production
    shard.ingest_space_survey_data(nasa_data)
    
    receipt = shard.full_fenca_validation(force_nexus=True)
    print("\n=== NASA SPACE + FENCA RECEIPT ===")
    print(json.dumps(receipt, indent=2))
    print(f"Space Ledger Entries: {len([e for e in shard.data_ledger if e['source'] == 'NASA NEO Feed'])}")
