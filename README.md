# Tauri Plugin escpos

This is a plugin to use in ESCPOS Thermal Printers.

This plugin uses the <a href="https://github.com/lnxdxtf/eco_print">eco_print</a> lib.


NOTE: FOR WHILE, JUST SUPPORTS BLE

---

## Android + btleplug(droidplug)

1. **Copy the necessary Java files**:  
   Copy the **nonpolynomial**(btleplug\src\droidplug\java\src\main\java\com\nonpolynomial) directory from the repository to your Android project's `android/src/main/java/com/` directory.

2. **Build the jni-utils-rs crate**:  
   Use the following command to build the crate with Java support:
   ```shell
   cargo build --features=build-java-support --release
   ```
   Copy the JAR file:
   After building, copy the jni-utils-0.1.1-SNAPSHOT.jar file from jni-utils-rs/target/release/java/libs to android/libs.

These steps should help integrate the required components into your Android project.


For reference, you can check the<a href="https://github.com/Dreaming-Codes/tauri-plugin-btleplug/tree/master"> tauri-plugin-btleplug repository</a>.

---

