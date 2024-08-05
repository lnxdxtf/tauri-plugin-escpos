<template>
  <div class="container">
    <button @click="init()">INIT </button>
    <button @click="start()">START </button>
    <button @click="check_state()">STATE </button>

    <div v-if="state">
      <p>{{ state }}</p>
    </div>

  </div>
</template>

<script lang="ts">
import { invoke } from '@tauri-apps/api/core';
export default {
  

  data() {
    return {
      state: {},
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
    }
  }
}


</script>