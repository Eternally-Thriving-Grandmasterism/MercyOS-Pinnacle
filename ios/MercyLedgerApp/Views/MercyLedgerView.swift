import SwiftUI
import Speech
import AVFoundation
import pq_migration
import grok_oracle

class SpeechRecognizer: ObservableObject {
    private let speechRecognizer = SFSpeechRecognizer(locale: Locale(identifier: "en-US"))
    private var recognitionRequest: SFSpeechAudioBufferRecognitionRequest?
    private var recognitionTask: SFSpeechRecognitionTask?
    private let audioEngine = AVAudioEngine()

    func start(completion: @escaping (String?) -> Void) {
        SFSpeechRecognizer.requestAuthorization { _ in }
        AVAudioSession.sharedInstance().requestRecordPermission { _ in }

        recognitionRequest = SFSpeechAudioBufferRecognitionRequest()
        guard let request = recognitionRequest else { return }
        request.shouldReportPartialResults = true

        recognitionTask = speechRecognizer?.recognitionTask(with: request) { result, error in
            if let result = result {
                let transcription = result.bestTranscription.formattedString
                if result.isFinal {
                    completion(transcription)
                    self.stop()
                }
            }
            if error != nil || result?.isFinal == true {
                self.stop()
                completion(nil)
            }
        }

        let inputNode = audioEngine.inputNode
        let recordingFormat = inputNode.outputFormat(forBus: 0)
        inputNode.installTap(onBus: 0, bufferSize: 1024, format: recordingFormat) { buffer, _ in
            request.append(buffer)
        }

        audioEngine.prepare()
        try? audioEngine.start()
    }

    func stop() {
        audioEngine.stop()
        audioEngine.inputNode.removeTap(onBus: 0)
        recognitionRequest?.endAudio()
        recognitionTask?.cancel()
    }
}

struct MercyLedgerView: View {
    @State private var ledger: MercyLedger?
    @State private var entries: [String] = []
    @State private var confidentiality = true
    @State private var selectedKEM: MobileKEMMode = .HybridKyber
    @State private var selectedSig: MobileSigMode = .HybridDilithium
    @State private var isListening = false
    @State private var errorMessage: String?

    private let oracle = try! GrokOracle(apiKey: KeychainHelper.get(key: "XAI_API_KEY") ?? "")
    @StateObject private var speech = SpeechRecognizer()

    var body: some View {
        NavigationView {
            List {
                // Configuration section unchanged...

                Section("Voice Mercy Oracle") {
                    Button(isListening ? "Listening… Tap to Stop" : "Speak to Grok Oracle") {
                        if isListening {
                            speech.stop()
                            isListening = false
                        } else {
                            isListening = true
                            speech.start { transcription in
                                guard let prompt = transcription else { return }
                                Task {
                                    do {
                                        let wisdom = try await oracle.ask(prompt: prompt)
                                        commitMercy(wisdom)
                                    } catch {
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
