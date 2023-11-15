use std::fmt::Display;

use crate::{
    datetime_to_timestamp, datetime_to_timestamp_option, ClientInfo, Feed, FeedGroup, FeedInfo,
    FeedItem, FeedTypeServer, FeedUpdateRecord, ProLevel, ProLevelPostgres, UserPurchaseDetail,
    VipStatus,
};
use chrono::Utc;
use sqlx::{postgres::PgRow, FromRow, Row};

impl FromRow<'_, PgRow> for FeedGroup {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            name: row.try_get("name")?,
            description: row.try_get("description")?,
            update_time: datetime_to_timestamp(row.try_get("update_time")?),
            is_deleted: row.try_get("is_deleted")?,
            sync_time: datetime_to_timestamp_option(row.try_get("sync_time")?),
        })
    }
}

impl FromRow<'_, PgRow> for VipStatus {
    fn from_row(row: &'_ PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            user_id: row.try_get("user_id")?,
            is_forever: row.try_get("is_forever")?,
            pro_end_time: datetime_to_timestamp(row.try_get("pro_end_time")?),
        })
    }
}

impl FromRow<'_, PgRow> for UserPurchaseDetail {
    fn from_row(row: &'_ PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            user_id: row.try_get("user_id")?,
            product_id: row.try_get("product_id")?,
            purchase_time: datetime_to_timestamp(row.try_get("purchase_time")?),
            source: row.try_get("source")?,
        })
    }
}

impl FromRow<'_, PgRow> for FeedItem {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            feed_url: row.try_get("feed_url")?,
            is_focus: row.try_get("is_focus")?,
            is_seen: row.try_get("is_seen")?,
            title: row.try_get("title")?,
            cover: row.try_get("cover")?,
            link: row.try_get("link")?,
            publish_time: datetime_to_timestamp_option(row.try_get("publish_time")?),
            authors: row.try_get("authors")?,
            tags: row.try_get("tags")?,
            category: row.try_get("category")?,
            description: row.try_get("description")?,
            summary_algo: row.try_get("summary_algo")?,
            create_time: datetime_to_timestamp(row.try_get("create_time")?),
            md5_string: row.try_get("md5_string")?,
            update_time: datetime_to_timestamp(row.try_get("update_time")?),
            is_deleted: row.try_get("is_deleted")?,
            focus_time: datetime_to_timestamp_option(row.try_get("focus_time")?),
            sync_time: datetime_to_timestamp_option(row.try_get("sync_time")?),
            is_marked: row.try_get("is_marked")?,
            is_archived: row.try_get("is_archived")?,
        })
    }
}

impl FromRow<'_, PgRow> for Feed {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            url: row.try_get("url")?,
            name: row.try_get("name")?,
            description: row.try_get("description")?,
            custom_description: row.try_get("custom_description")?,
            custom_name: row.try_get("custom_name")?,
            update_time: datetime_to_timestamp(row.try_get("update_time")?),
            logo: row.try_get("logo")?,
            custom_logo: row.try_get("custom_logo")?,
            create_time: datetime_to_timestamp(row.try_get("create_time")?),
            feed_type: row.try_get("feed_type")?,
            tags: row.try_get("tags")?,
            is_deleted: row.try_get("is_deleted")?,
            sync_time: datetime_to_timestamp_option(row.try_get("sync_time")?),
            group_name: row.try_get("group_name")?,
            is_followed: row.try_get("is_followed")?,
        })
    }
}

impl Display for ProLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProLevel::Normal => write!(f, "normal"),
            ProLevel::Pro => write!(f, "pro"),
            ProLevel::Spro => write!(f, "spro"),
        }
    }
}

impl Display for FeedTypeServer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FeedTypeServer::Rss => write!(f, "rss"),
            FeedTypeServer::Atom => write!(f, "atom"),
            FeedTypeServer::Unknown => write!(f, "unknown"),
        }
    }
}

impl From<ProLevelPostgres> for ProLevel {
    fn from(pg: ProLevelPostgres) -> Self {
        match pg {
            ProLevelPostgres::Normal => ProLevel::Normal,
            ProLevelPostgres::Pro => ProLevel::Pro,
            ProLevelPostgres::Spro => ProLevel::Spro,
        }
    }
}

impl From<FeedInfo> for Feed {
    fn from(info: FeedInfo) -> Self {
        Self {
            url: info.url,
            name: info.title,
            description: None,
            custom_description: None,
            custom_name: None,
            update_time: datetime_to_timestamp(Utc::now()),
            logo: info.image,
            custom_logo: None,
            create_time: datetime_to_timestamp(Utc::now()),
            feed_type: None,
            tags: None,
            is_deleted: false,
            sync_time: None,
            group_name: None,
            is_followed: Some(false),
        }
    }
}

impl FromRow<'_, PgRow> for FeedUpdateRecord {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            feed_url: row.try_get("feed_url")?,
            update_time: datetime_to_timestamp(row.try_get("update_time")?),
            last_content_hash: row.try_get("last_content_hash")?,
            last_update: datetime_to_timestamp(row.try_get("last_update")?),
            last_item_publish_time: datetime_to_timestamp_option(
                row.try_get("last_item_publish_time")?,
            ),
            sync_time: datetime_to_timestamp_option(row.try_get("sync_time")?),
            is_deleted: row.try_get("is_deleted")?,
            failed_count: row.try_get("failed_count")?,
        })
    }
}

impl FromRow<'_, PgRow> for ClientInfo {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            client_id: row.get("id"),
            client_name: row.get("device_name"),
        })
    }
}
