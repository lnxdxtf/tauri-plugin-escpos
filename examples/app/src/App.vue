<template>
  <div>
    <div class="container">
      <button @click="init()">INIT </button>
      <button @click="start()">START </button>
      <button @click="check_state()">STATE </button>
      <button @click="scan()">SCAN </button>
    </div>

      <div v-if="state">
        <p>{{ state }}</p>
      </div>

    <div v-if="devices.length > 0">
      {{ devices }}
    </div>

  </div>
</template>

<script lang="ts">
import { invoke } from '@tauri-apps/api/core';
export default {


  data() {
    return {
      state: {},
      devices: [],
    }
  },

  methods: {
    async init() {
      let permissions_ok = await invoke('plugin:escpos|permissions_ok');
      if (!permissions_ok) {
        await invoke('plugin:escpos|request_permissions');
      }

    },

    async start(): Promise<void> {
      await invoke('plugin:escpos|start', { conn: 'bluetooth' });
    },

    async check_state(): Promise<void> {
      let state: any = await invoke('plugin:escpos|check_store_state');
      console.log(state);

      this.state = state;
    },

    async scan(): Promise<void> {
      let d = await invoke('plugin:escpos|start_scan');
      console.log(d)
      this.devices = d as any

    }
  }
}


</script>