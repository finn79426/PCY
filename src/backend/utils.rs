use arboard::ImageData;
use base64::engine::general_purpose;
use base64::prelude::*;
use chrono::{DateTime, Local, Utc};
use std::borrow::Cow;

/// Converts a timestamp to a human-readable relative time string.
///
/// This function formats the time based on the difference from `Utc::now()`:
/// - **Within 1 min**: "Just now"
/// - **Within 60 mins**: "X min ago"
/// - **Today**: "HH:MM AM/PM" (e.g., "10:30 AM")
/// - **Yesterday**: "Yesterday"
/// - **Within 7 days**: Day of the week (e.g., "Monday")
/// - **Older**: "YYYY-MM-DD"
///
/// # Example
///
/// ```
/// use crate::backend::clipboard;
///
/// let timestamp = chrono::Utc::now();
/// let humanized_time = clipboard::humanize_time(timestamp);
/// println!("Humanized time: {}", humanized_time); // Output: Humanized time: Just now
/// ```
pub fn humanize_time(timestamp: DateTime<Utc>) -> String {
    let now = Utc::now();
    let diff = now.signed_duration_since(timestamp);

    if diff.num_minutes() < 1 {
        return "Just now".to_string();
    }

    if diff.num_minutes() < 60 {
        return format!("{} min ago", diff.num_minutes());
    }

    let local_ts: DateTime<Local> = DateTime::from(timestamp);
    let local_now = Local::now();

    if local_ts.date_naive() == local_now.date_naive() {
        return local_ts.format("%I:%M %p").to_string();
    }

    if local_ts.date_naive() == (local_now.date_naive() - chrono::Duration::days(1)) {
        return "Yesterday".to_string();
    }

    if diff.num_days() < 7 {
        return local_ts.format("%A").to_string();
    }

    local_ts.format("%Y-%m-%d").to_string()
}

/// Helper function to convert a Base64 string into `arboard::ImageData`.
///
/// This process involves:
/// 1. Decoding the Base64 string into bytes.
/// 2. Loading the image from memory (auto-detecting format like PNG, JPEG).
/// 3. Converting the image to **RGBA8** format (required by system clipboards).
/// 4. Extracting raw pixels and dimensions.
///
/// # Panics
/// This function will **panic** if:
/// - The input string is not valid Base64.
/// - The decoded bytes do not represent a valid image.
pub fn b64_to_img_data(content: &str) -> ImageData<'_> {
    let image_bytes = general_purpose::STANDARD.decode(content).unwrap();
    let dynamic_image = image::load_from_memory(&image_bytes).unwrap();
    let rgba_image = dynamic_image.to_rgba8();
    let (width, height) = rgba_image.dimensions();
    let pixels = rgba_image.into_raw();

    ImageData {
        width: width as usize,
        height: height as usize,
        bytes: Cow::Owned(pixels),
    }
}
