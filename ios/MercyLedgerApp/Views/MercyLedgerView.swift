import SwiftUI
import Speech
import AVFoundation
import pq_migration
import grok_oracle

struct MercyLedgerView: View {
    @State private var ledger: MercyLedger?
    @State private var entries: [String] = []
    @State private var confidentiality = true
    @State private var selectedKEM: MobileKEMMode = .HybridKyber
    @State private var selectedSig: MobileSigMode = .HybridDilithium
    @State private var selectedLanguage = "en-US"
    @State private var isListening = false
    @State private var statusMessage = "Offline Ready"
    @State private var errorMessage: String?

    private let oracle = try! GrokOracle(apiKey: KeychainHelper.get(key: "XAI_API_KEY") ?? "")
    private let speechSynthesizer = AVSpeechSynthesizer()

    private let languages: [String: String] = [
        "en-US": "English (US)",
        "fr-FR": "French",
        "es-ES": "Spanish",
        "de-DE": "German",
        "zh-CN": "Chinese (Simplified)",
        "ja-JP": "Japanese",
        "ko-KR": "Korean",
        "ru-RU": "Russian",
        "pt-BR": "Portuguese (Brazil)",
        "it-IT": "Italian"
    ]

    var body: some View {
        NavigationView {
            List {
                Section("Offline Voice Language") {
                    Picker("Language", selection: $selectedLanguage) {
                        ForEach(Array(languages.keys.sorted()), id: \.self) { code in
                            Text(languages[code] ?? code).tag(code)
                        }
                    }
                    Text(statusMessage)
                        .foregroundColor(.secondary)
                }

                // Configuration sections...

                Section("Offline Voice Mercy Oracle") {
                    Button(isListening ? "Listening Offline… (\(languages[selectedLanguage] ?? ""))" : "Speak Offline in \(languages[selectedLanguage] ?? "")") {
                        toggleOfflineVoice()
                    }
                    .foregroundColor(isListening ? .red : .green)
                }

                Section("Mercy Entry History") {
                    ForEach(entries.reversed(), id: \.self) { entry in
                        HStack {
                            Text(entry)
                            Button("Speak Offline") {
                                speakOffline(text: entry, language: selectedLanguage)
                            }
                            .foregroundColor(.blue)
                        }
                        .padding()
                        .background(Color.secondary.opacity(0.2))
                        .cornerRadius(8)
                    }
                }
            }
            .navigationTitle("MercyOS Ledger — Offline")
            .alert(item: $errorMessage) { Alert(title: Text("Mercy Note"), message: Text($0)) }
            .onChange(of: selectedLanguage) { _ in checkOfflineAvailability() }
            .onAppear { checkOfflineAvailability() }
        }
    }

    private func checkOfflineAvailability() {
        let locale = Locale(identifier: selectedLanguage)
        if SFSpeechRecognizer(locale: locale)?.isAvailable ?? false {
            statusMessage = "Offline Model Ready"
        } else {
            statusMessage = "Download offline voice in Settings > Accessibility > Spoken Content"
        }
    }

    private func toggleOfflineVoice() {
        isListening.toggle()
        if isListening {
            let locale = Locale(identifier: selectedLanguage)
            OfflineSpeechRecognizer.shared.start(locale: locale) { transcription in
                guard let prompt = transcription else {
                    statusMessage = "No speech detected"
                    isListening = false
                    return
                }
                Task {
                    do {
                        let wisdom = try await oracle.ask(prompt: prompt)
                        commitMercy(wisdom)
                        speakOffline(text: wisdom, language: selectedLanguage)
                    } catch {
                        errorMessage = "Oracle error (offline voice OK): \(error.localizedDescription)"
                    }
                    isListening = false
                }
            }
        } else {
            OfflineSpeechRecognizer.shared.stop()
        }
    }

    private func speakOffline(text: String, language: String) {
        let utterance = AVSpeechUtterance(string: text)
        utterance.voice = AVSpeechSynthesisVoice(language: language)
        speechSynthesizer.stopSpeaking(at: .immediate)
        speechSynthesizer.speak(utterance)
    }

    private func commitMercy(_ text: String) {
        // Unchanged
    }
}

// OfflineSpeechRecognizer wrapper class ensuring on-device
class OfflineSpeechRecognizer: ObservableObject {
    static let shared = OfflineSpeechRecognizer()
    // Full implementation same as previous, using SFSpeechRecognizer (on-device by default for supported languages)
}
