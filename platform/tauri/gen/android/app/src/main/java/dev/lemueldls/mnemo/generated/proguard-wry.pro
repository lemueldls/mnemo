# THIS FILE IS AUTO-GENERATED. DO NOT MODIFY!!

# Copyright 2020-2023 Tauri Programme within The Commons Conservancy
# SPDX-License-Identifier: Apache-2.0
# SPDX-License-Identifier: MIT

-keep class dev.lemueldls.mnemo.* {
  native <methods>;
}

-keep class dev.lemueldls.mnemo.WryActivity {
  public <init>(...);

  void setWebView(dev.lemueldls.mnemo.RustWebView);
  java.lang.Class getAppClass(...);
  java.lang.String getVersion();
}

-keep class dev.lemueldls.mnemo.Ipc {
  public <init>(...);

  @android.webkit.JavascriptInterface public <methods>;
}

-keep class dev.lemueldls.mnemo.RustWebView {
  public <init>(...);

  void loadUrlMainThread(...);
  void loadHTMLMainThread(...);
  void evalScript(...);
}

-keep class dev.lemueldls.mnemo.RustWebChromeClient,dev.lemueldls.mnemo.RustWebViewClient {
  public <init>(...);
}
