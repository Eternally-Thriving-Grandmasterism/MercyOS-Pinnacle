import SwiftUI
import Vosk

struct MercyLedgerView: View {
    // ... states

    private var voskRecognizer: VoskRecognizer?

    private let modelMap = [
        "en-US": "vosk-model-small-en-us-0.15",
        "fr-FR": "vosk-model-small-fr-0.22",
        "es-ES": "vosk-model-small-es-0.42",
        // Add more
    ]

    var body: some View {
        // UI same

        Button(isListening ? "Listening (Vosk Offline)" : "Vosk Offline Voice Oracle") {
            if isListening {
                voskRecognizer?.stop()
                isListening = false
            } else {
                guard let modelPath = Bundle.main.path(forResource: modelMap[selectedLanguage], ofType: nil) else { return }
                let model = VoskModel(path: modelPath)
                voskRecognizer = VoskRecognizer(model: model, sampleRate: 16000)
                voskRecognizer?.setPartialResults(true)
                // Start microphone + feed audio buffers
                isListening = true
                voskRecognizer?.resultHandler = { result in
                    let text = result.text
                    // Oracle query + commit + speak
                }
            }
        }
    }
}
