use async_once::AsyncOnce;
use lazy_static::lazy_static;
use scylla::{Session, SessionBuilder};

lazy_static! {
    static ref SCYLLA_SESSION: AsyncOnce<Session> = AsyncOnce::new(async {
        let session = async {
            SessionBuilder::new()
                .known_node("127.0.0.1:9042")
                .build()
                .await
        }.await.unwrap();

        session
    });
}

pub async fn get_scylla_session() -> &'static Session {
    let sess = SCYLLA_SESSION.get().await;
    sess
}