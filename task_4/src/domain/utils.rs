use axum::async_trait;
use tokio::sync::mpsc::UnboundedSender;
use tokio_util::sync::CancellationToken;

#[async_trait]
pub trait ToChangeNotifier {
    type Output;
    type Err;
    async fn to_change_notifier(
        &self,
        cancellation_token: CancellationToken,
        sender: UnboundedSender<()>,
    ) -> Result<Self::Output, Self::Err>;
}
#[async_trait]
pub trait CloseRepository {
    async fn close(&self);
}
