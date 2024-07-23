import { invoke } from '@tauri-apps/api/core'

export async function request_permissions(): Promise<string| any> {
  await invoke('plugin:escpos|request_permissions');
}
export async function permissions_ok():Promise<boolean | any> {
  await invoke('plugin:escpos|permissions_ok');
}
