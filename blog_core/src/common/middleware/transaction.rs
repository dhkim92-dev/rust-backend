/***
 * Request Scope에 존재하는 단일 트랜잭션을 생성하여 관리한다.
 * tokio task_local을 이용하여, 비동기 트랜잭션에 대한 접근을 layer 상관없이 할 수 있도록 만든다.
 */

/* 
use std::sync::Arc;
use axum::{body::Body, extract::State, http::Request, middleware::Next, response::Response};
use sea_orm::{DatabaseConnection, DatabaseTransaction, TransactionTrait};
use tokio::task_local;
use crate::{common::error::error_code::ErrorCode, di::AppContext};

task_local! {
    static TX_STORAGE: Arc<DatabaseTransaction>;
}


pub async fn transaction_middleware(
    State(db): State<Arc<DatabaseConnection>>,
    req: Request<Body>,
    next: Next
) -> Result<Response, ErrorCode> {
    let tx = db.begin().await
        .map_err(|_| ErrorCode::with_message(ErrorCode::INTERNAL_SERVER_ERROR, "데이터베이스 트랜잭션 생성 실패"))?;

     let tx = Arc::new(tx);

    TX_STORAGE.scope(tx.clone(), async move {
        let response = next.run(req).await;

        if response.status().is_success() {
            TX_STORAGE.with(|tx| async move {
                let tx = Arc::clone(tx);
                tx.commit().await.map_err(|err| ErrorCode::with_message(
                    ErrorCode::INTERNAL_SERVER_ERROR, 
                    "트랜잭션 커밋 실패",
                ))
            }).await?;
        } else {
            TX_STORAGE.with(|tx| async {
                tx.rollback().await.map_err(|err| ErrorCode::with_message(
                    ErrorCode::INTERNAL_SERVER_ERROR, 
                    "트랜잭션 롤백 실패"
                ))
            }).await?;
        }

        Ok(response)
    }).await
}

pub async fn get_transaction() -> Option<DatabaseTransaction> {
    TX_STORAGE.try_with(|tx| tx.clone()).ok()
}
 */
