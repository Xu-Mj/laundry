use serde::{Deserialize, Serialize};
use sqlx::{Sqlite, Transaction};

use crate::error::Result;
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserMembershipLevel {
    pub user_id: i64,  // 用户ID
    pub level_id: i64, // 会员等级ID
}

impl UserMembershipLevel {
    pub fn new(user_id: i64, level_id: i64) -> Self {
        UserMembershipLevel { user_id, level_id }
    }
}

impl UserMembershipLevel {
    // insert
    pub async fn create(&self, tx: &mut Transaction<'_, Sqlite>) -> Result<()> {
        sqlx::query("INSERT INTO user_membership_level (user_id, level_id) VALUES (?, ?)")
            .bind(self.user_id)
            .bind(self.level_id)
            .execute(&mut **tx)
            .await?;
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn update(&self, tx: &mut Transaction<'_, Sqlite>) -> Result<()> {
        sqlx::query("UPDATE user_membership_level SET level_id = ? WHERE user_id = ?")
            .bind(self.level_id)
            .bind(self.user_id)
            .execute(&mut **tx)
            .await?;
        Ok(())
    }
}
