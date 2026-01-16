@Composable
fun MercyLedgerScreen() {
    var entryText by remember { mutableStateOf("") }
    var latestText by remember { mutableStateOf("") }
    var confidentiality by remember { mutableStateOf(true) }
    val ledger = remember { 
        createMercyLedger(
            MobileKEMMode.HybridKyber,
            MobileSigMode.HybridDilithium,
            confidentiality
        )
    }

    Column(
        modifier = Modifier
            .padding(16.dp)
            .fillMaxSize(),
        verticalArrangement = Arrangement.spacedBy(16.dp)
    ) {
        Text("MercyOS Ledger — Eternal Thriving", style = MaterialTheme.typography.headlineMedium)

        Row(verticalAlignment = Alignment.CenterVertically) {
            Text("Confidentiality Mode")
            Switch(checked = confidentiality, onCheckedChange = { confidentiality = it })
        }

        OutlinedTextField(
            value = entryText,
            onValueChange = { entryText = it },
            label = { Text("Enter mercy message") }
        )

        Button(onClick = {
            val data = entryText.toByteArray()
            ledger.commit(data.toList())
            entryText = ""
        }) {
            Text("Commit Entry")
        }

        Button(onClick = {
            val result = ledger.readLatest()
            latestText = result?.let { String(it.toByteArray()) } ?: "No entry"
        }) {
            Text("Read Latest")
        }

        Text(latestText, modifier = Modifier.fillMaxWidth())
    }
}
