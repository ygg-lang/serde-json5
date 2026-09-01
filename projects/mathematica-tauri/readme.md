# `mathematica-tauri`

产出 **`mathematica.exe`**。

```sh
pnpm --filter @sxo/mathematica-webui build
cargo build -p mathematica-tauri --release
```

二进制：`target/release/mathematica.exe`  
无窗口求值：`mathematica.exe --eval '…'`
