# Mathematica.ts

桌面产物：**`mathematica.exe`**（`mathematica-tauri` + `mathematica-webui`）。

```sh
pnpm install
pnpm build:desktop
# → target/release/mathematica.exe

pnpm bundle:desktop
# → target/release/bundle/nsis/*.exe（安装包，首次会慢：编 WebView2/Tauri）
```

冒烟（无窗口）：

```sh
./target/release/mathematica.exe --eval "Simplify[Sin[x]^2+Cos[x]^2]"
```
