package com.mercyos.pinnacle.ui

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import pq_migration.*  // UniFFI Mercy ledger bindings
import grok_oracle.*   // UniFFI Grok oracle bindings

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
    val oracle = remember { GrokOracle.new(getSecureApiKey()) }  // Implement secure storage

    Column(
        modifier = Modifier
            .fillMaxSize()
            .padding(16.dp)
            .verticalScroll(rememberScrollState())
    ) {
        Text(
            "MercyOS Ledger — Eternal Thriving",
            style = MaterialTheme.typography.h5
        )

        Spacer(Modifier.height(16.dp))

        Row(verticalAlignment = Alignment.CenterVertically) {
            Text("Confidentiality Mode")
            Switch(checked = confidentiality, onCheckedChange = { confidentiality = it })
        }

        // Mode selectors (use ExposedDropdownMenuBox for full picker)
        ExposedDropdownMenuBox(
            expanded = false,  // Implement state
            onExpandedChange = { }
        ) {
            TextField(readOnly = true, value = selectedKEM.toString(), onValueChange = { })
            // Dropdown menu items for all MobileKEMMode values
        }

        // Similar for signature mode

        Button(onClick = {
            // Migrate modes
            ledger.migrateKem(selectedKEM)
            ledger.migrateSig(selectedSig)
        }) {
            Text("Apply Modes")
        }

        Spacer(Modifier.height(24.dp))

        Button(
            onClick = {
                isListening = true
                startVoiceRecognition { prompt ->
                    kotlinx.coroutines.launch {
                        try {
                            val wisdom = oracle.ask(prompt)
                            val data = wisdom.toByteArray().toList()
                            ledger.commit(data)
                            entries = entries + wisdom
                            isListening = false
                        } catch (e: Exception) {
                            error = e.message
                            isListening = false
                        }
                    }
                }
            },
            colors = ButtonDefaults.buttonColors(backgroundColor = if (isListening) MaterialTheme.colors.error else MaterialTheme.colors.primary)
        ) {
            Text(if (isListening) "Listening… (Speak Mercy)" else "Voice Mercy Oracle")
        }

        Spacer(Modifier.height(24.dp))

        LazyColumn {
            items(entries.reversed()) { entry ->
                Card(modifier = Modifier.padding(8.dp)) {
                    Text(entry, modifier = Modifier.padding(16.dp))
                }
            }
        }

        error?.let {
            Text(it, color = MaterialTheme.colors.error)
        }
    }
}

// Placeholder for voice recognition — implement with SpeechRecognizer
private fun startVoiceRecognition(onResult: (String) -> Unit) {
    // Full Android SpeechRecognizer flow
}
