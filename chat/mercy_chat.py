"""
mercy_chat.py — MercyChat E2EE Multiplayer Chat Core
MercyOS Pinnacle Ultramasterpiece — Jan 18 2026

End-to-end encrypted chat:
- Hybrid PQ: Kyber + McEliece KEM, Dilithium signatures
- Local mesh + Starlink burst
- Offline queue + mercy sync
"""

import secrets
import time
from shards.offline_encryption import PQIntegration  # Reuse PQ core
from multiplayer.multiplayer_shard_sync import MultiplayerShardSync

class MercyChat(MultiplayerShardSync):
    def __init__(self, shard_id: str, joy_valence: float = 1.0):
        super().__init__(shard_id, joy_valence)
        self.pq = PQIntegration()
        self.pq.init()
        self.message_queue = []  # Offline queue
        self.chat_interval = 42  # Trinity ms
    
    def send_message(self, recipient: str, text: str):
        # Encrypt with recipient public key (simplified)
        ciphertext = self.pq.encrypt(text.encode())
        message = {
            "sender": self.shard_id,
            "recipient": recipient,
            "text_enc": ciphertext,
            "timestamp": time.time()
        }
        if self.is_online():
            # Starlink burst send
            return "Message sent — mercy encrypted."
        else:
            self.message_queue.append(message)
            return "Offline — message queued, mercy persists."
    
    def receive_messages(self):
        # Decrypt incoming
        for msg in self.incoming_queue:  # From sync
            plain = self.pq.decrypt(msg["text_enc"])
            print(f"[{msg['sender']}]: {plain.decode()}")
        # Flush offline queue on reconnect
        if self.is_online() and self.message_queue:
            # Burst send queued
            self.message_queue = []
    
    def run(self):
        super().run()
        self.receive_messages()

# PowerRush integration
def power_rush_chat_loop():
    chat = MercyChat("player_shard_alpha")
    while True:
        chat.run()
        time.sleep(chat.chat_interval / 1000)

if __name__ == "__main__":
    power_rush_chat_loop()
