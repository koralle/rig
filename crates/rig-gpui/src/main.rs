//! `rig-gpui` の GUI エントリポイント。
//!
//! GPUI 本体は `gpui-kit` が `gpui-pre-*` のスナップショットとして固定し、`gpui_kit::*`
//! から再エクスポートする。gpui を別途依存に入れると GPUI が二重に入るため、依存は
//! gpui-kit だけにしている。
//!
//! workspace の既定のビルド対象 (`default-members`) は CLI (`crates/rig`) なので、実行は
//! `cargo run -p rig-gpui` で行う。ウィンドウを出すには X11/Wayland または macOS /
//! Windows のデスクトップセッションが必要 (ヘッドレスでは起動できない)。
//!
//! 自動検証は `src/tests.rs` の headless な smoke test 1 本だけ。

use gpui_kit::component::button::*;
use gpui_kit::component::*;
use gpui_kit::*;

/// 起点となる最小のビュー。ボタンの押下回数を表示する。
#[derive(Default)]
struct RigGpuiView {
    clicks: usize,
}

impl Render for RigGpuiView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<'_, Self>) -> impl IntoElement {
        div()
            .v_flex()
            .gap_2()
            .size_full()
            .items_center()
            .justify_center()
            .child("rig-gpui")
            .child(format!("Clicked {} times", self.clicks))
            .child(
                Button::new("click")
                    .primary()
                    .label("Click me")
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.clicks += 1;
                        cx.notify();
                    })),
            )
    }
}

fn main() {
    gpui_kit::application().run(|cx| {
        // GPUI Component の機能を使う前に呼ぶ必要がある
        gpui_kit::init(cx);

        // ダイアログ / シート / 通知 / メニューが動くように `Root` で包んだウィンドウを開く
        if let Err(err) = gpui_kit::open_window(WindowOptions::default(), cx, |_, cx| {
            cx.new(|_| RigGpuiView::default())
        }) {
            eprintln!("ウィンドウを開けませんでした: {err}");
        }
    });
}

#[cfg(test)]
mod tests;
