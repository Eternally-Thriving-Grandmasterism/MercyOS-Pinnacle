package com.mercyos.pinnacle.ui

import org.vosk.LibVosk
import org.vosk.Model
import org.vosk.Recognizer
import org.vosk.android.StorageService
import android.media.AudioFormat
import android.media.AudioRecord
import android.media.MediaRecorder
import androidx.compose.runtime.*
import java.io.File

@Composable
fun MercyLedgerScreen() {
    // ... states including selectedLanguage

    var voskModel by remember { mutableStateOf<Model?>(null) }
    var recognizer by remember { mutableStateOf<Recognizer?>(null) }
    var isListening by remember { mutableStateOf(false) }

    val modelMap = mapOf(
        "en-US" to "model-small-en-us",
        "fr-FR" to "model-small-fr",
        "es-ES" to "model-small-es",
        // Add more Vosk small models
    )

    LaunchedEffect(selectedLanguage) {
        val modelPath = modelMap[selectedLanguage] ?: return@LaunchedEffect
        LibVosk.setLogLevel(0)
        StorageService.unpack(context, modelPath, "model") { path: String ->
            voskModel = Model(path)
            recognizer = Recognizer(voskModel, 16000.0f)
        }
    }

    // Voice button
    Button(onClick = {
        if (isListening) {
            // Stop recording
            isListening = false
        } else {
            isListening = true
            // Start AudioRecord thread
            val audioRecord = AudioRecord(MediaRecorder.AudioSource.MIC, 16000, AudioFormat.CHANNEL_IN_MONO, AudioFormat.ENCODING_PCM_16BIT, bufferSize)
            audioRecord.startRecording()
            scope.launch {
                val buffer = ShortArray(1024)
                while (isListening) {
                    val read = audioRecord.read(buffer, 0, buffer.size)
                    if (read > 0) {
                        recognizer?.acceptWaveForm(buffer, read)
                        val partial = recognizer?.partialResult
                        // Update UI with partial if desired
                    }
                }
                audioRecord.stop()
                audioRecord.release()
                val final = recognizer?.result
                val text = final?.text ?: ""
                // Oracle query + commit + TTS
            }
        }
    }) {
        Text(if (isListening) "Listening (Vosk Offline)" else "Vosk Offline Voice Oracle")
    }

    // Rest of UI...
}
