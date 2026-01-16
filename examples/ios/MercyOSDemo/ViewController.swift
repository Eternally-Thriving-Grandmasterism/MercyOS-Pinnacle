import UIKit

class ViewController: UIViewController {
    let textView = UITextView()

    override func viewDidLoad() {
        super.viewDidLoad()
        view.backgroundColor = .systemBackground

        textView.frame = view.bounds
        textView.autoresizingMask = [.flexibleWidth, .flexibleHeight]
        textView.font = UIFont.monospacedSystemFont(ofSize: 16, weight: .regular)
        textView.text = "❤️🚀 MercyOS-Pinnacle iOS Demo Live\n\n"
        view.addSubview(textView)

        // Call UniFFI exports
        let pk = mercy_pq_encaps()
        textView.text += "Post-Quantum ML-KEM Encaps: \(pk.count) bytes sealed 🔥\n\n"

        let oracle = mercy_grok_query(query: "AlphaProMegaing eternal thriving harmony")
        textView.text += "Grok Oracle Response:\n\(oracle) ❤️"
    }
}
