package com.mercyos.pinnacle.ui

import android.Manifest
import android.content.Intent
import android.content.pm.PackageManager
import android.speech.RecognizerIntent
import android.speech.SpeechRecognizer
import androidx.activity.compose.rememberLauncherForActivityResult
import androidx.activity.result.contract.ActivityResultContracts
import androidx.compose.runtime.*
import androidx.core.content.ContextCompat
import kotlinx.coroutines.launch

@Composable
fun MercyLedgerScreen(viewModel: MercyViewModel = viewModel()) {
    // ... states same

    val context = LocalContext.current
    val scope = rememberCoroutineScope()

    val speechLauncher = rememberLauncherForActivityResult(
        contract = ActivityResultContracts.StartActivityForResult()
    ) { result ->
        val spokenText = result.data?.getStringArrayListExtra(RecognizerIntent.EXTRA_RESULTS)?.firstOrNull()
        spokenText?.let { prompt ->
            scope.launch {
                try {
                    val wisdom = oracle.ask(prompt)
                    viewModel.commitEntry(wisdom)
                } catch (e: Exception) {
                    viewModel.error = e.message
                }
            }
        }
        isListening = false
    }

    val permissionLauncher = rememberLauncherForActivityResult(
        contract = ActivityResultContracts.RequestPermission()
    ) { granted ->
        if (granted) {
            val intent = Intent(RecognizerIntent.ACTION_RECOGNIZE_SPEECH).apply {
                putExtra(RecognizerIntent.EXTRA_LANGUAGE_MODEL, RecognizerIntent.LANGUAGE_MODEL_FREE_FORM)
            }
            speechLauncher.launch(intent)
        }
    }

    // UI with Button
    Button(onClick = {
        isListening = true
        when {
            ContextCompat.checkSelfPermission(context, Manifest.permission.RECORD_AUDIO) == PackageManager.PERMISSION_GRANTED -> {
                val intent = Intent(RecognizerIntent.ACTION_RECOGNIZE_SPEECH).apply {
                    putExtra(RecognizerIntent.EXTRA_LANGUAGE_MODEL, RecognizerIntent.LANGUAGE_MODEL_FREE_FORM)
                }
                speechLauncher.launch(intent)
            }
            else -> permissionLauncher.launch(Manifest.permission.RECORD_AUDIO)
        }
    }) {
        Text(if (isListening) "Listening…" else "Voice Mercy Oracle")
    }

    // Rest of UI unchanged...
}
