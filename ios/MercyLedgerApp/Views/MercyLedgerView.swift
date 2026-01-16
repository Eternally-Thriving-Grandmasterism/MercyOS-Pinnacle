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
    @State private var errorMessage: String?

    private let oracle = try! GrokOracle(apiKey: KeychainHelper.get(key: "XAI_API_KEY") ?? "")
    private let speechSynthesizer = AVSpeechSynthesizer()

    private let languages = [
        "en-US": "English (US)",
        "fr-FR": "French",
        "es-ES": "Spanish",
        "de-DE": "German",
        "zh-CN": "Chinese (Simplified)"
    ]

    var body: some View {
        NavigationView {
            List {
                Section("Language") {
                    Picker("Voice Language", selection: $selectedLanguage) {
                        ForEach(Array(languages.keys.sorted()), id: \.self) { code in
                            Text(languages[code] ?? code).tag(code)
                        }
                    }
                }

                // Configuration sections unchanged...

                Section("Voice Mercy Oracle") {
                    Button(isListening ? "Listening… (\(languages[selectedLanguage] ?? ""))" : "Speak in \(languages[selectedLanguage] ?? "")") {
                        toggleVoiceRecognition()
                    }
                    .foregroundColor(isListening ? .red : .blue)
                }

                Section("Mercy Entry History") {
                    ForEach(entries.reversed(), id: \.self) { entry in
                        HStack {
                            Text(entry)
                            Button("Read Aloud") {
                                speak(text: entry, language: selectedLanguage)
                            }
                        }
                        .padding()
                        .background(Color.secondary.opacity(0.2))
                        .cornerRadius(8)
                    }
                }
            }
            .navigationTitle("MercyOS Ledger")
            .alert(item: $errorMessage) { Alert(title: Text("Mercy"), message: Text($0)) }
        }
    }

    private func toggleVoiceRecognition() {
        isListening.toggle()
        if isListening {
            let locale = Locale(identifier: selectedLanguage)
            SpeechRecognizer.shared.start(locale: locale) { transcription in
                guard let prompt = transcription else { return }
                Task {
                    do {
                        let wisdom = try await oracle.ask(prompt: prompt)
                        commitMercy(wisdom)
                        speak(text: wisdom, language: selectedLanguage)
                    } catch {
                        errorMessage = error.localizedDescription
                    }
                }
            }
        } else {
            SpeechRecognizer.shared.stop()
        }
    }

    private func speak(text: String, language: String) {
        let utterance = AVSpeechUtterance(string: text)
        utterance.voice = AVSpeechSynthesisVoice(language: language)
        speechSynthesizer.speak(utterance)
    }

    private func commitMercy(_ text: String) {
        // Same as previous
    }
}

// Updated SpeechRecognizer to accept locale
class SpeechRecognizer: ObservableObject {
    private var speechRecognizer: SFSpeechRecognizer?
    // ... rest with init locale parameter
    func start(locale: Locale, completion: @escaping (String?) -> Void) {
        speechRecognizer = SFSpeechRecognizer(locale: locale)
        // ... same implementation
    }
}                                    } catch {
                                        errorMessage = error.localizedDescription
                                    }
                                }
                            }
                        }
                    }
                    .foregroundColor(isListening ? .red : .blue)
                }

                Section("Mercy Entry History") {
                    ForEach(entries.reversed(), id: \.self) { entry in
                        Text(entry)
                            .padding()
                            .background(Color.secondary.opacity(0.2))
                            .cornerRadius(8)
                    }
                }
            }
            .navigationTitle("MercyOS Ledger")
            .alert(item: $errorMessage) { Alert(title: Text("Mercy"), message: Text($0)) }
        }
    }

    private func commitMercy(_ text: String) {
        // Same as previous
    }
}
