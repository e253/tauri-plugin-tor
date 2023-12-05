mod commands;

use hyper::Uri;
use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub struct TauriTorPluginConfig {
    pub endpoints: Vec<Uri>,
}

pub fn init<R: Runtime>(endpoints: Vec<String>) -> TauriPlugin<R> {
    Builder::<R>::new("tor")
        .invoke_handler(tauri::generate_handler![commands::check])
        .setup(move |a| {
            let endpoints: Vec<Uri> = endpoints
                .iter()
                .map(|s| s.parse::<Uri>().unwrap())
                .inspect(|url| assert!(url.scheme_str() == Some("https")))
                .collect::<Vec<Uri>>();
            a.manage(TauriTorPluginConfig { endpoints });
            Ok(())
        })
        .build()
}
