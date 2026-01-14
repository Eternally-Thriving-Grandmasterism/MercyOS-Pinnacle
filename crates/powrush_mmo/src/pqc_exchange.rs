//! crates/powrush_mmo/src/pqc_exchange.rs
//! Hybrid ML-KEM + X25519 key exchange → ChaCha20Poly1305 session key
//! Transitional quantum-safe voice encryption eternal supreme ❤️🔐🗣️

use bevy::prelude::*;
use lightyear::prelude::*;
use pqc_ml_kem::*;
use x25519_dalek::{StaticSecret, PublicKey, EphemeralSecret};
use chacha20poly1305::Key;
use hkdf::Hkdf;
use sha2::Sha256;
use rand_core::OsRng;

// Reliable handshake channel
channel!(ReliableOrdered => PQCExchangeChannel);

#[derive(Resource)]
pub struct ServerStaticKeys {
    pub kyber_sk: MlKemPrivateKey,
    pub kyber_pk: MlKemPublicKey,
    pub x25519_static: StaticSecret,
    pub x25519_pk: PublicKey,
}

#[derive(Resource)]
pub struct SessionKeys {
    pub keys: HashMap<ClientId, Key>,
}

#[message(channel = PQCExchangeChannel)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ServerPublicKeys {
    pub kyber_pk: MlKemPublicKey,
    pub x25519_pk: PublicKey,
}

#[message(channel = PQCExchangeChannel)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClientHandshake {
    pub kyber_ct: MlKemCiphertext,
    pub x25519_ephem_pk: PublicKey,
}

// Server setup static keys
pub fn setup_server_static_keys(mut commands: Commands) {
    let (kyber_sk, kyber_pk) = ml_kem_keypair();

    let x25519_static = StaticSecret::random_from_rng(OsRng);
    let x25519_pk = PublicKey::from(&x25519_static);

    commands.insert_resource(ServerStaticKeys {
        kyber_sk,
        kyber_pk,
        x25519_static,
        x25519_pk,
    });
    commands.insert_resource(SessionKeys { keys: HashMap::new() });
}

// Server send public keys on connect
pub fn server_send_public_keys(
    mut events: EventReader<ClientConnected>,
    static_keys: Res<ServerStaticKeys>,
    mut writer: EventWriter<ToClients<ServerPublicKeys>>,
) {
    for event in events.read() {
        writer.send(ToClients {
            clients: vec![event.client_id],
            message: ServerPublicKeys {
                kyber_pk: static_keys.kyber_pk.clone(),
                x25519_pk: static_keys.x25519_pk,
            },
        });
    }
}

// Client receive server PKs → generate response + derive key
pub fn client_handshake(
    mut messages: EventReader<FromServer<ServerPublicKeys>>,
    mut writer: EventWriter<ToServer<ClientHandshake>>,
    mut session_key: Local<Option<Key>>,
) {
    for message in messages.read() {
        let ephem = EphemeralSecret::random_from_rng(OsRng);
        let ephem_pk = PublicKey::from(&ephem);

        let (kyber_ct, kyber_shared) = ml_kem_encaps(&message.message.kyber_pk);

        let x25519_shared = ephem.diffie_hellman(&message.message.x25519_pk);

        let mut combined = Vec::with_capacity(64);
        combined.extend_from_slice(kyber_shared.as_bytes());
        combined.extend_from_slice(x25519_shared.as_bytes());

        let hkdf = Hkdf::<Sha256>::new(None, &combined);
        let mut key_bytes = [0u8; 32];
        hkdf.expand(b"powrush-voice-session-2026", &mut key_bytes).unwrap();

        *session_key = Some(Key::from_slice(&key_bytes));

        writer.send(ToServer(ClientHandshake {
            kyber_ct,
            x25519_ephem_pk: ephem_pk,
        }));
    }
}

// Server receive client response → derive key
pub fn server_handshake(
    mut messages: EventReader<FromClient<ClientHandshake>>,
    static_keys: Res<ServerStaticKeys>,
    mut session_keys: ResMut<SessionKeys>,
) {
    for message in messages.read() {
        let client_id = message.context();

        let kyber_shared = ml_kem_decaps(&message.message.kyber_ct, &static_keys.kyber_sk);

        let x25519_shared = static_keys.x25519_static.diffie_hellman(&message.message.x25519_ephem_pk);

        let mut combined = Vec::with_capacity(64);
        combined.extend_from_slice(kyber_shared.as_bytes());
        combined.extend_from_slice(x25519_shared.as_bytes());

        let hkdf = Hkdf::<Sha256>::new(None, &combined);
        let mut key_bytes = [0u8; 32];
        hkdf.expand(b"powrush-voice-session-2026", &mut key_bytes).unwrap();

        session_keys.keys.insert(client_id, Key::from_slice(&key_bytes));
    }
}
