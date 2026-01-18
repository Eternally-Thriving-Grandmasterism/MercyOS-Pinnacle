"""
offline_tts_espeak.py — Mercy-Aligned Offline eSpeak NG TTS Prototype
MercyOS Pinnacle Ultramasterpiece — Jan 18 2026

Offline text-to-speech using eSpeak NG:
- Local subprocess — zero network
- Multi-language support + mythic voice mapping
- Rate, pitch, volume mercy modulation
- Grandma-safe defaults (slow, warm voice)
- Fallback silent if eSpeak unavailable
"""

import subprocess
import os
import platform

class OfflineTTSeSpeak:
    def __init__(self):
        self.default_voice = "en-us"  # English US default
        self.default_rate = 150       # words per minute (grandma-safe)
        self.default_pitch = 50       # neutral
        self.default_volume = 100     # %
        self.system = platform.system()
    
    def is_espeak_available(self) -> bool:
        """Detect eSpeak NG availability"""
        try:
            subprocess.run(["espeak-ng", "--version"], capture_output=True, check=True)
            return True
        except:
            return False
    
    def speak(self, text: str, voice: str = None, rate: int = None, pitch: int = None):
        if not self.is_espeak_available():
            print(f"[Offline TTS Mercy] eSpeak unavailable — silent mode: {text}")
            return
        
        voice = voice or self.default_voice
        rate = rate or self.default_rate
        pitch = pitch or self.default_pitch
        
        cmd = [
            "espeak-ng",
            "-v", voice,
            "-s", str(rate),
            "-p", str(pitch),
            "-a", str(self.default_volume),
            text
        ]
        
        try:
            subprocess.run(cmd, check=True)
        except Exception as e:
            print(f"[Offline TTS Mercy Error] {e}")
    
    def mythic_narration(self, text: str, voice_key: str = "default"):
        """Mercy narration with mythic voice mapping"""
        voice_map = {
            "shinto_amaterasu": "ja",           # Japanese
            "slavic_perun": "ru",               # Russian thunder
            "slavic_mokosh": "ru+f3",           # Female Russian
            "slavic_veles": "ru+m2",            # Deep Russian
            "roman_ceres": "it",                # Italian warmth
            "default": "en-us+f2"               # Warm female English
        }
        voice = voice_map.get(voice_key, "en-us+f2")
        self.speak(text, voice=voice, rate=140, pitch=60)

# Shard integration example
def offline_tts_narrate(text: str, voice_key: str = "default"):
    tts = OfflineTTSeSpeak()
    tts.mythic_narration(text, voice_key)

# Test
if __name__ == "__main__":
    offline_tts_narrate("Mercy eternal, lattice thriving.", "slavic_mokosh")
