@Composable
fun MercyLedgerScreen() {
    var entries by remember { mutableStateOf(listOf<String>()) }
    var confidentiality by remember { mutableStateOf(true) }
    var selectedKEM by remember { mutableStateOf(MobileKEMMode.HybridKyber) }
    var selectedSig by remember { mutableStateOf(MobileSigMode.HybridDilithium) }
    var isListening by remember { mutableStateOf(false) }
    var error by remember { mutableStateOf<String?>(null) }

    val ledger = remember {
        createMercyLedger(selectedKEM, selectedSig, confidentiality)
    }
    val oracle = remember { GrokOracle.new(encryptedPrefs.getString("XAI_API_KEY", "")!!) }

    Column(modifier = Modifier.padding(16.dp).verticalScroll(rememberScrollState())) {
        Text("MercyOS Ledger — Eternal Thriving", style = MaterialTheme.typography.headlineMedium)

        Row(verticalAlignment = Alignment.CenterVertically) {
            Text("Confidentiality")
            Switch(checked = confidentiality, onCheckedChange = { confidentiality = it })
        }

        // Mode selectors (use DropdownMenu or ExposedDropdownMenu)
        // Example placeholder
        Text("KEM: ${selectedKEM.name}")
        Text("Sig: ${selectedSig.name}")

        Button(onClick = {
            // Migrate logic
        }) { Text("Apply Modes") }

        Button(onClick = {
            isListening = !isListening
            if (isListening) {
                // Start speech recognition
                // On result → oracle.ask → commit
            }
        }) {
            Text(if (isListening) "Listening…" else "Voice Mercy Oracle")
        }

        LazyColumn {
            items(entries.reversed()) { entry ->
                Text(entry, modifier = Modifier.padding(8.dp).fillMaxWidth())
            }
        }

        error?.let { Text(it, color = Color.Red) }
    }
}
