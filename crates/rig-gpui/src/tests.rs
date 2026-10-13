//! `rig-gpui` の headless な smoke test。
//!
//! ウィンドウを出せない環境 (CI など) でも「起動 → 描画 → 入力 → 状態変化」が通ることを
//! 見る。ピクセルまでは検証しない (gpui-kit のテスト支援はレイアウト / 状態 / コールバック
//! までしか見ない)。テストは `#[gpui_kit::test]` が用意するヘッドレスな `TestAppContext`
//! 上で動くため、ディスプレイサーバーも GPU ドライバーも要らない。

use gpui_kit::{AppContext as _, TestAppContext, WindowOptions, test::TestWindowExt as _};

use crate::RigGpuiView;

#[gpui_kit::test]
fn window_opens_and_clicking_the_button_updates_the_view(cx: &mut TestAppContext) {
    // 本番の main と同じ初期化を通す
    cx.update(gpui_kit::init);

    let (window, view) = cx
        .update(|cx| {
            gpui_kit::open_window(WindowOptions::default(), cx, |_, cx| {
                cx.new(|_| RigGpuiView::default())
            })
        })
        .expect("ウィンドウを開ける");

    assert_eq!(cx.update(|cx| view.read(cx).clicks), 0);

    // 1 フレーム描画できる (レイアウトとテキスト整形が通る)
    cx.update_window(window, |_, window, cx| {
        window.render_frame(cx);
    })
    .expect("1 フレーム描画できる");

    // ボタンを押すとビューの状態が変わる
    cx.update_window(window, |_, window, cx| {
        window.click("click", cx);
    })
    .expect("ボタンを押せる");

    assert_eq!(cx.update(|cx| view.read(cx).clicks), 1);
}
