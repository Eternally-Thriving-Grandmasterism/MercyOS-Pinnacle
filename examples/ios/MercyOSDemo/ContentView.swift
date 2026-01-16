import SwiftUI
import pq_migration  // Generated UniFFI bindings

struct MercyLedgerView: View {
    @State private var ledger: MercyLedger?
    @State private var entryText = ""
    @State private var latestText = ""
    @State private var confidentiality = true

    var body: some View {
        VStack(spacing: 20) {
            Text("MercyOS Ledger — Eternal Thriving")
                .font(.largeTitle)

            Toggle("Confidentiality Mode", isOn: $confidentiality)
                .padding()

            TextField("Enter mercy message", text: $entryText)
                .textFieldStyle(.roundedBorder)
                .padding()

            Button("Commit Entry") {
                if ledger == nil {
                    ledger = create_mercy_ledger(
                        kem_mode: .HybridKyber,
                        sig_mode: .HybridDilithium,
                        enable_confidentiality: confidentiality
                    )
                }
                if let data = entryText.data(using: .utf8) {
                    ledger?.commit(data: Array(data))
                    entryText = ""
                }
            }

            Button("Read Latest") {
                if let data = ledger?.read_latest() {
                    latestText = String(data: Data(data), encoding: .utf8) ?? "Decrypted mercy"
                }
            }

            Text(latestText)
                .padding()
                .frame(maxWidth: .infinity, alignment: .leading)
                .background(Color.gray.opacity(0.2))
                .cornerRadius(8)

            Spacer()
        }
        .padding()
    }
}
