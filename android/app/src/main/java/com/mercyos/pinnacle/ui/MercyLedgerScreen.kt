package com.mercyos.pinnacle.ui

import android.speech.RecognizerIntent.EXTRA_PREFER_OFFLINE
// ... imports

@Composable
fun MercyLedgerScreen() {
    // ... states
    var selectedLanguage by remember { mutableStateOf("en-US") }

    val languages = mapOf(
        "en-US" to "English (US)",
        "fr-FR" to "French",
        "es-ES" to "Spanish",
        "de-DE" to "German",
        "zh-CN" to "Chinese",
        "ja-JP" to "Japanese",
        "ko-KR" to "Korean",
        "ru-RU" to "Russian",
        "pt-BR" to "Portuguese",
        "it-IT" to "Italian"
    )

    val tts = remember { TextToSpeech(context) { /* init */ } }

    val intent = Intent(RecognizerIntent.ACTION_RECOGNIZE_SPEECH).apply {
        putExtra(RecognizerIntent.EXTRA_LANGUAGE_MODEL, RecognizerIntent.LANGUAGE_MODEL_FREE_FORM)
        putExtra(RecognizerIntent.EXTRA_LANGUAGE, selectedLanguage)
        putExtra(EXTRA_PREFER_OFFLINE, true)  // Force offline mode
    }

    // In voice button
    Button(onClick = {
        isListening = true
        speechLauncher.launch(intent)
    }) {
        Text(if (isListening) "Listening Offline…" else "Offline Voice Oracle")
    }

    // After recognition result
    spokenText?.let { prompt ->
        // Oracle query same
        // TTS offline
        tts.language = Locale(selectedLanguage.replace("-", "_"))
        tts.speak(wisdom, TextToSpeech.QUEUE_FLUSH, null, null)
    }

    // Language picker UI
    ExposedDropdownMenuBox(/* expanded state */) {
        // Menu items from languages
    }

    // Status message
    Text("Offline mode forced — download languages in Settings > System > Languages & input > On-device speech recognition")
}
