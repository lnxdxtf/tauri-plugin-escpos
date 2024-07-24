<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';

async function init() {
  let permissions_ok = await invoke('plugin:escpos|permissions_ok');
  if (!permissions_ok) {
    await invoke('plugin:escpos|request_permissions');
  }

}

async function start(): Promise<void> {
  await invoke('plugin:escpos|start', { conn: 'bluetooth' });
}
async function check_state(): Promise<void> {
  let state = await invoke('plugin:escpos|check_store_state');
  console.log(state)
}

</script>

<template>
  <div class="container">
    <button @click="init()">INIT </button>
    <button @click="start()">START </button>
    <button @click="check_state()">STATE </button>

  </div>
</template>
