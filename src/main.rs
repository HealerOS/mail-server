use mail_server::boot::server::Application;
use mail_server::config::system_config::get_config;
use mail_server::telemetry::{get_subscriber, init_subscriber};
use tracing::info;

#[warn(clippy::all, clippy::pedantic)]
#[tokio::main]
//todo 后续重构优化这里
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("mail-server".to_string(), "info".to_string());
    init_subscriber(subscriber);

    let system_config = get_config().expect("读取系统配置失败");

    info!("系统配置是:{:?}", system_config);
    let application = Application::build(system_config).await?;

    application.run_until_stopped().await?;
    Ok(())
}
