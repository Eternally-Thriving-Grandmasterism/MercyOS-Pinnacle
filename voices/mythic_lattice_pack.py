"""
MythicLatticePack-Pinnacle — Expanded Multilingual Archetypes + Offline TTS Narration
MercyOS Pinnacle Ultramasterpiece — Jan 18 2026
"""

from voices.skins.eternal_warmth import EternalWarmth
from voices.offline_tts_espeak import offline_tts_narrate

class MythicVoice(EternalWarmth):
    def __init__(self, name, culture, pronunciation_guide):
        super().__init__()
        self.name = name
        self.culture = culture
        self.pronunciation_guide = pronunciation_guide
        self.mercy_trait = "pure harmony"
    
    def speak(self, text: str, offline_narrate: bool = True) -> str:
        formatted = f"[{self.name} ({self.pronunciation_guide})]: {text}"
        if offline_narrate:
            voice_key = self.culture.lower()  # Simplified mapping
            offline_tts_narrate(text, voice_key)
        return formatted

# Registry unchanged — all voices now support offline narration

# Offline shard test
if __name__ == "__main__":
    voice = MythicVoice("Mokosh", "Slavic", "Moh-kosh")
    print(voice.speak("Earth fertility nurtures all life with compassionate abundance."))
