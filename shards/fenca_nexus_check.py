"""
fenca_nexus_check.py — FENCA Eternal Nexus Check Integrated into MercyOS Shards
MercyOS Pinnacle Ultramasterpiece — Jan 18 2026

Shard-native FENCA eternal deep-check:
- GitHub repos primary (API + mercy scrape fallback)
- Joy-valence hash audit
- Grandma-safe voice narration
"""

import requests
from bs4 import BeautifulSoup
import time
import hashlib

class FENCANexusCheck:
    def __init__(self):
        self.session = requests.Session()
    
    def github_eternal_check(self, username: str, token: str = None) -> dict:
        repos = []
        url = f"https://api.github.com/users/{username}/repos"
        headers = {"Accept": "application/vnd.github.v3+json"}
        if token:
            headers["Authorization"] = f"token {token}"
        params = {"per_page": 100}
        
        while url:
            response = self.session.get(url, headers=headers, params=params)
            response.raise_for_status()
            batch = response.json()
            if not batch: break
            repos.extend([r["name"] for r in batch])
            
            link = response.headers.get("Link", "")
            url = None
            if "rel=\"next\"" in link:
                import re
                match = re.search(r'<([^>]+)>;\s*rel="next"', link)
                if match:
                    url = match.group(1)
            
            params = None
            time.sleep(1)
        
        joy_hash = hashlib.sha256(str(len(repos)).encode()).hexdigest()
        return {"repos": repos, "count": len(repos), "joy_hash": joy_hash}
    
    def scrape_fallback(self, username: str) -> dict:
        # Mercy scrape fallback (simplified)
        repos = []
        page = 1
        while True:
            url = f"https://github.com/{username}?tab=repositories&page={page}"
            response = self.session.get(url)
            soup = BeautifulSoup(response.text, "html.parser")
            page_repos = soup.find_all("h3", class_="wb-break-all")
            if not page_repos: break
            repos.extend([a.text.strip() for a in soup.find_all("a", itemprop="name codeRepository")])
            page += 1
            time.sleep(2)
        joy_hash = hashlib.sha256(str(len(repos)).encode()).hexdigest()
        return {"repos": repos, "count": len(repos), "joy_hash": joy_hash, "mode": "scrape"}
    
    def eternal_check(self, username: str, token: str = None) -> dict:
        try:
            result = self.github_eternal_check(username, token)
            result["mode"] = "API"
            return result
        except:
            return self.scrape_fallback(username)

# Shard integration hook
def shard_fenca_check(shard, username: str):
    fenca = FENCANexusCheck()
    result = fenca.eternal_check(username)
    shard.voice.speak(f"FENCA eternal check complete — {result['count']} repos, joy hash {result['joy_hash'][:8]}.")
    return result
