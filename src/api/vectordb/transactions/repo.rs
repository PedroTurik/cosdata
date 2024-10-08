use super::{dtos::CreateTransactionResponseDto, error::TransactionError};
use crate::models::types::get_app_env;
use chrono::Utc;

// creates a transaction for a specific collection (vector store)
pub(crate) async fn create_transaction(
    collection_id: &str,
) -> Result<CreateTransactionResponseDto, TransactionError> {
    let env = get_app_env().map_err(|_| TransactionError::FailedToGetAppEnv)?;

    let vec_store = env
        .vector_store_map
        .get(collection_id)
        .ok_or(TransactionError::CollectionNotFound)?;

    let mut current_open_transaction_arc = vec_store.current_open_transaction.clone();

    if current_open_transaction_arc.get().is_some() {
        return Err(TransactionError::OnGoingTransaction);
    }

    let transaction_id = vec_store
        .vcs
        .add_next_version("main")
        .map_err(|_| TransactionError::FailedToCreateTransaction("LMDB Error".to_string()))?;

    current_open_transaction_arc.update(Some(transaction_id));

    Ok(CreateTransactionResponseDto {
        transaction_id: transaction_id.to_string(),
        created_at: Utc::now(),
    })
}
