//! Mathematica.ts 桌面：`mathematica.exe` = Tauri + notebook WebUI + `sxo-dialects` → `athena`.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![deny(missing_docs)]

use serde::Serialize;
use std::sync::atomic::{AtomicU64, Ordering};
use sxo_dialects::{Dialect, SxoFrontend};
use tauri::{AppHandle, WebviewUrl, WebviewWindow, WebviewWindowBuilder};

/// Monotonic id for runtime notebook windows (`notebook-1`, …).
static NOTEBOOK_WINDOW_SEQ: AtomicU64 = AtomicU64::new(1);

/// Structured cell output for the notebook UI.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct EvalPayload {
    /// `text` | `svg` | `html` | `image` | `error`
    kind: &'static str,
    /// Payload body (Wolfram text, SVG markup, HTML, or data-URL).
    body: String,
}

fn classify_body(body: String) -> EvalPayload {
    let trimmed = body.trim();
    if trimmed.starts_with("<svg") || trimmed.starts_with("<?xml") && trimmed.contains("<svg") {
        return EvalPayload { kind: "svg", body };
    }
    if trimmed.starts_with("data:image/") {
        return EvalPayload { kind: "image", body };
    }
    if trimmed.starts_with('<') && (trimmed.contains("</") || trimmed.ends_with("/>")) {
        return EvalPayload { kind: "html", body };
    }
    EvalPayload { kind: "text", body }
}

fn eval_form_source(source: &str) -> Result<EvalPayload, EvalPayload> {
    let trimmed = source.trim();
    if trimmed.is_empty() {
        return Err(EvalPayload {
            kind: "error",
            body: "MATHEMATICA_FORM_EMPTY: empty form".into(),
        });
    }
    let fe = SxoFrontend::with_dialect(Dialect::Mathematica);
    match fe.evaluate_mathematica(trimmed) {
        Ok(term) => {
            let term = fe.simplify_term(&term);
            Ok(classify_body(fe.render_as_wolfram(&term)))
        }
        Err(e) => Err(EvalPayload {
            kind: "error",
            body: e.to_string(),
        }),
    }
}

#[tauri::command]
fn eval_form(source: String) -> Result<EvalPayload, EvalPayload> {
    eval_form_source(&source)
}

/// Open another independent notebook window (same process, fresh UI state).
///
/// Must be `async` on Windows — sync `WebviewWindowBuilder::build` can deadlock WebView2.
#[tauri::command]
async fn new_notebook_window(app: AppHandle) -> Result<String, String> {
    let n = NOTEBOOK_WINDOW_SEQ.fetch_add(1, Ordering::Relaxed);
    let label = format!("notebook-{n}");
    WebviewWindowBuilder::new(&app, &label, WebviewUrl::App("index.html".into()))
        .title(format!("Mathematica — {n}"))
        .inner_size(960.0, 720.0)
        .resizable(true)
        .build()
        .map_err(|e| e.to_string())?;
    Ok(label)
}

/// Close the calling notebook window (File → Close Window).
#[tauri::command]
async fn close_current_window(window: WebviewWindow) -> Result<(), String> {
    window.close().map_err(|e| e.to_string())
}

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if args.first().map(String::as_str) == Some("--eval") {
        let source = args.get(1).cloned().unwrap_or_default();
        match eval_form_source(&source) {
            Ok(out) => {
                println!("{}", out.body);
                return;
            }
            Err(e) => {
                eprintln!("{}", e.body);
                std::process::exit(1);
            }
        }
    }

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            eval_form,
            new_notebook_window,
            close_current_window
        ])
        .run(tauri::generate_context!())
        .expect("error while running mathematica");
}
