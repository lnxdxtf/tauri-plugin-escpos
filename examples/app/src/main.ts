import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";

createApp(App).mount("#app");


export type PrinterStore = {
    connection: string;
    connected: boolean;
    devices_ble: Device[];
    devices_usb: string[];
    
}

export type Device ={
    name: string;
    address: string;
    services_ble: string[];
    conn: string
}