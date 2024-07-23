<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';



async function status_permissions() {
  let permissions_ok = await invoke('plugin:escpos|permissions_ok');
  alert(`permissions ok: ${permissions_ok}`)
}
async function request_permissions() {
  await invoke('plugin:escpos|request_permissions');
}
async function scan_devices_ble(): Promise<string[]> {
  let devices:string[] = []

  return devices
}

</script>

<template>
  <div class="container">
    <button @click="status_permissions()">STATUS </button>
    <button @click="request_permissions()">REQUEST</button>
    <button @click="scan_devices_ble()">SCAN</button>
  </div>
</template>
