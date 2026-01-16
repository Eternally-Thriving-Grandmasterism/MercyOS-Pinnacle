import SwiftUI
import pq_migration
import grok_oracle  // UniFFI Grok oracle bindings

struct MercyLedgerView: View {
    @State private var ledger: MercyLedger?
    @State private var entries: [String] = []
    @State private var latestText = "No entries yet"
    @State private var confidentiality = true
    @State private var selectedKEM: MobileKEMMode = .HybridKyber
    @State private var selectedSig: MobileSigMode = .HybridDilithium
    @State private var isListening = false
    @State private var errorMessage: String?

    private let oracle = try! GrokOracle(apiKey: keychainGet("XAI_API_KEY") ?? "")

    var body: some View {
        NavigationView {
            List {
                Section("Mercy Controls") {
                    Toggle("Confidentiality Mode", isOn: $confidentiality)

                    Picker("KEM Mode", selection: $selectedKEM) {
                        ForEach([MobileKEMMode.Legacy, .HybridKyber, .QuantumSafeKyber, .HybridHqc, .QuantumSafeHqc], id: \.self) { mode in
                            Text("\(mode.rawValue)").tag(mode)
                        }
                    }

                    Picker("Signature Mode", selection: $selectedSig) {
                        ForEach([MobileSigMode.Legacy, .HybridDilithium, .PureDilithium], id: \.self) { mode in
                            Text("\(mode.rawValue)").tag(mode)
                        }
                    }

                    Button("Apply Modes") {
                        if ledger == nil {
                            ledger = create_mercy_ledger(
                                kem_mode: selectedKEM,
                                sig_mode: selectedSig,
                                enable_confidentiality: confidentiality
                            )
                        } else {
                            ledger?.migrate_kem(new_mode: selectedKEM)
                            ledger?.migrate_sig(new_mode: selectedSig)
                        }
                    }
                }

                Section("Voice Mercy Oracle") {
                    Button(isListening ? "Listening…" : "Speak Mercy") {
                        isListening.toggle()
                        if isListening {
                            startVoiceRecognition { prompt in
                                Task {
                                    do {
                                        let wisdom = try await oracle.ask(prompt: prompt)
                                        commitEntry(wisdom)
                                        isListening = false
                                    } catch {
                                        errorMessage = "Oracle error: \(error.localizedDescription)"
                                        isListening = false
                                    }
                                }
                            }
                        }
                    }
                    .foregroundColor(isListening ? .red : .blue)
                }

                Section("Mercy Entries") {
                    ForEach(entries.reversed(), id: \.self) { entry in
                        Text(entry)
                            .padding(.vertical, 4)
                    }
                }
            }
            .navigationTitle("MercyOS Ledger")
            .alert(item: $errorMessage) { msg in
                Alert(title: Text("Mercy Note"), message: Text(msg), dismissButton: .default(Text("OK")))
            }
            .onAppear {
                refreshEntries()
            }
        }
    }

    private func commitEntry(_ text: String) {
        guard let data = text.data(using: .utf8) else { return }
        do {
            if ledger == nil {
                ledger = create_mercy_ledger(kem_mode: selectedKEM, sig_mode: selectedSig, enable_confidentiality: confidentiality)
            }
            try ledger?.commit(data: Array(data))
            refreshEntries()
        } catch {
            errorMessage = "Commit failed: \(error.localizedDescription)"
        }
    }

    private func refreshEntries() {
        guard let ledger = ledger else { return }
        if let data = ledger.read_latest() {
            if let text = String(data: Data(data), encoding: .utf8) {
                if !entries.contains(text) {
                    entries.append(text)
                }
                latestText = text
            }
        }
    }

    // Voice recognition stub — implement with SFSpeechRecognizer as in previous example
    private func startVoiceRecognition(completion: @escaping (String) -> Void) {
        // Full implementation from earlier voice snippet
    }
}
