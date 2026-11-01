//! Paged history sync for groups and "Cloud của tôi" (#19, #23).
//!
//! Walks `/api/cm/getoldv2` backwards 50 messages at a time from the oldest message already in
//! `zalo_history.db`, so a second run resumes where the first stopped. One thread at a time with
//! a pause between pages to stay clear of Zalo's rate limits.
//!
//! One-to-one history remains unsupported until a request/response route is verified; see
//! re-notes/09_history_sync_one_to_one.md.

use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use anyhow::Result;

use crate::chat::{self, Thread};
use crate::session::Session;
use crate::store::{self, MessageStore};

const PAGE: u32 = 50;
const PAUSE: Duration = Duration::from_millis(400);

/// Set by the GUI's "Dừng" button; checked between pages.
pub static CANCEL: AtomicBool = AtomicBool::new(false);
/// True while a sync runs, so a second click doesn't start a parallel one.
pub static RUNNING: AtomicBool = AtomicBool::new(false);

/// Fetch older pages for `thread` until the server has nothing older or the user cancels.
/// Each page is committed before it is reported to `on_page_persisted`.
/// SQLite work runs on the blocking pool; no connection is held across an await.
///
/// # Errors
/// Returns an error when the history request, SQLite write, or blocking task fails.
pub async fn sync_thread<F>(
    s: &Session,
    thread: &Thread,
    store_path: &str,
    mut on_page_persisted: F,
) -> Result<usize>
where
    F: FnMut(usize, usize) + Send,
{
    let id = thread.id.clone();
    let path = store_path.to_string();
    let mut cursor = tokio::task::spawn_blocking(move || {
        MessageStore::open(&path).map(|store| store.oldest_msg_id(&id))
    })
    .await??
    .unwrap_or_default();
    let mut added = 0;
    let mut page_number = 0;
    loop {
        if CANCEL.load(Ordering::Relaxed) {
            break;
        }
        let resp = chat::get_cloud_messages(s, &thread.id, &cursor, true, PAGE).await?;
        let mut page = Thread {
            presentation: thread.presentation.clone(),
            unread: thread.unread.clone(),
            id: thread.id.clone(),
            name: thread.name.clone(),
            preview: String::new(),
            time: String::new(),
            avatar: thread.avatar.clone(),
            is_group: thread.is_group,
            messages: Vec::new(),
        };
        chat::merge_group_history(&mut page, &resp);
        if page.messages.is_empty() {
            break; // nothing older on the server
        }
        let next = next_cursor(&page, &cursor);
        let path = store_path.to_string();
        let page_added = tokio::task::spawn_blocking(move || {
            store::persist_threads(
                &mut MessageStore::open(&path)?,
                std::slice::from_ref(&page),
            )
        })
        .await??;
        added += page_added;
        page_number += 1;
        on_page_persisted(page_number, page_added);
        match next {
            Some(next) => cursor = next,
            None => break,
        }
        tokio::time::sleep(PAUSE).await;
    }
    Ok(added)
}

/// The oldest server message id after a merge, or `None` when the page didn't move the cursor
/// back (the server returned only messages we already had); continuing would loop forever.
/// Local echoes (`out_*`) are not valid cursors, hence the digits-only filter.
fn next_cursor(t: &Thread, current: &str) -> Option<String> {
    let oldest = t
        .messages
        .iter()
        .map(|m| m.id.as_str())
        .find(|id| !id.is_empty() && id.bytes().all(|b| b.is_ascii_digit()))?;
    (oldest != current).then(|| oldest.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chat::ChatMessage;

    fn thread_with(ids: &[&str]) -> Thread {
        Thread {
            id: "g1".into(),
            name: "Nhóm".into(),
            preview: String::new(),
            time: String::new(),
            avatar: None,
            is_group: true,
            messages: ids
                .iter()
                .map(|id| ChatMessage {
                    id: (*id).into(),
                    ..Default::default()
                })
                .collect(),
            ..Default::default()
        }
    }

    #[test]
    fn cursor_moves_to_the_oldest_server_message() {
        assert_eq!(
            next_cursor(&thread_with(&["5", "9"]), "9").as_deref(),
            Some("5")
        );
        assert_eq!(
            next_cursor(&thread_with(&["out_1", "7", "9"]), "9").as_deref(),
            Some("7")
        );
    }

    #[test]
    fn cursor_stops_when_page_brought_nothing_older() {
        assert_eq!(next_cursor(&thread_with(&["5", "9"]), "5"), None);
        assert_eq!(next_cursor(&thread_with(&[]), "5"), None);
        assert_eq!(next_cursor(&thread_with(&["", "out_2"]), "5"), None);
    }
}
