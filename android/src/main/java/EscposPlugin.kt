package com.plugin.escpos

import android.Manifest
import android.app.Activity
import android.webkit.WebView
import app.tauri.annotation.Permission
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Plugin

@TauriPlugin(
    permissions = [
        Permission(strings = [Manifest.permission.BLUETOOTH], alias = "bluetooth"),
        Permission(strings = [Manifest.permission.BLUETOOTH_SCAN], alias = "bluetoothScan"),
        Permission(strings = [Manifest.permission.BLUETOOTH_CONNECT], alias = "bluetoothConnect"),
        Permission(strings = [Manifest.permission.BLUETOOTH_ADMIN], alias = "bluetoothAdmin"),
    ]
)
class EscposPlugin(private val activity: Activity): Plugin(activity) {}
