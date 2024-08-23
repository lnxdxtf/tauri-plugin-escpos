<template>
  <!-- @vue-skip -->
  <div>
    <div class="w-screen h-screen bg-neutral-800 p-4">

      <div class="w-full h-fit grid grid-cols-2 text-white bg-black bg-opacity-75 p-4 rounded-md">
        <div class="w-full text-center col-span-2 border-b">
          State
        </div>
        <div class="flex flex-col gap-2 items-center">
          <span>Connected</span>
          <span class="p-2 rounded-full"
            :class="{ 'bg-red-500': !state.connected, 'bg-green-500': state.connected }"></span>
        </div>
        <div class="flex flex-col gap-2 items-center">
          <span>Type</span>
          <span>{{ state.connection }}</span>
        </div>
      </div>

      <div class="w-full h-fit flex flex-col p-4 bg-black bg-opacity-75 rounded-md mt-20 text-white">
        <div class="w-full flex justify-between items-center border-b pb-6">
          <span>Devices</span>
          <div @click="scan()" class="btn btn-outline btn-primary btn-sm">Scan</div>
        </div>
        <div class="w-full h-[500px] overflow-auto ">
          <template v-if="loading">
            <div class="w-full h-full flex justify-center items-center skeleton">

            </div>
          </template>
          <template v-else>
            <div v-for="device in devices" :key="device.address"
              class="relative w-full flex justify-between items-center border-b border-primary py-2">

              <div class="badge badge-info absolute -top-2 right-0">
                {{ device.conn }}
              </div>

              <div class="flex flex-col">
                <span>{{ device.name }}</span>
                <span>{{ device.address }}</span>
                <span>{{ device.services_ble }}</span>
              </div>

              <div @click="connect(device)" class="btn btn-outline btn-info">Connect</div>
            </div>
          </template>
        </div>


      </div>


    </div>
  </div>
</template>

<script lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { listen } from "@tauri-apps/api/event";
import { PrinterStore } from './main';
export default {


  data() {
    return {
      state: {},
      devices: [],
      loading: false,
    }
  },

  methods: {

    async scan(): Promise<void> {
      this.loading = true;
      await invoke('plugin:escpos|start_scan', { time: 1 });
      this.loading = false;

    },

    async connect(device: { name: string, address: string, services_ble: string[], conn: string }): Promise<void> {
      await invoke('plugin:escpos|connect', { time: 5, device: device });
    },

  },
  async mounted() {
    // If permissions not granted, the backend will request the permissions.
    await invoke('plugin:escpos|request_permissions');
    await invoke('plugin:escpos|start', { conn: 'BLE' });
    // Listen for the state changes
    listen<PrinterStore>('store_state_update', (_ev) => {
      console.log(_ev.payload);
      
      this.state = _ev.payload;
      //@ts-ignore
      this.devices = _ev.payload.devices_ble
    })
    await this.scan();
  }
}


</script>