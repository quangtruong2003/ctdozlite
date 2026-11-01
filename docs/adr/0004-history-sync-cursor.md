# ADR-0004: Con trỏ History Sync lấy từ Store

- **Trạng thái:** Đã chấp nhận (Accepted)
- **Ngày:** 2026-11-02
- **Người quyết định:** Maintainer

## Bối cảnh

ADR-0001 mô tả một bảng `history_sync_cursors`, nhưng bảng này không tồn tại trong mã hiện tại. `src/history_sync.rs` lấy ID máy chủ cũ nhất đã lưu bằng `MessageStore::oldest_msg_id` trong `src/store.rs`; mỗi trang được ghi vào Store trước khi chuyển tiếp con trỏ. PR #45 cũng mô tả cách tiếp tục từ tin máy chủ cũ nhất đã lưu.

## Quyết định

History Sync tiếp tục suy ra con trỏ từ tin nhắn máy chủ cũ nhất đã lưu trong Store theo từng thread. Không tạo bảng con trỏ riêng. Quyết định này làm rõ và thay thế riêng phần mô tả lưu con trỏ trong ADR-0001; các quyết định khác của ADR-0001 vẫn có hiệu lực.

## Hệ quả

- Không có trạng thái con trỏ thứ hai cần đồng bộ với các hàng tin nhắn.
- Sau khi ứng dụng dừng hoặc người dùng hủy, lần chạy sau tiếp tục từ dữ liệu đã ghi thành công.
- Vị trí tiếp tục phụ thuộc vào các tin nhắn còn trong Store; xóa dữ liệu lịch sử của thread cũng thay đổi con trỏ được suy ra.

## Các phương án đã cân nhắc

| Phương án | Lý do không chọn |
| :--- | :--- |
| Lưu con trỏ riêng trong `history_sync_cursors` | Lặp lại trạng thái đã suy ra được từ tin nhắn trong Store và có thể lệch với các hàng đã lưu. |

## Bằng chứng

- `src/history_sync.rs` gọi `MessageStore::oldest_msg_id` và ghi từng trang bằng `persist_threads`.
- `src/store.rs` lọc ID máy chủ dạng số khi tìm tin cũ nhất và có kiểm thử cho quy tắc này.
- [PR #45](https://github.com/ctdoteam1110/ctdozlite/pull/45) mô tả tiếp tục từ tin máy chủ cũ nhất đã lưu; kiểm thử với tài khoản Zalo trực tiếp còn pending trong PR.
