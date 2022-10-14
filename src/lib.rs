//! Spotify private api library
//!
//! Supports adding, removing and moving of spotify folders/playlists
//!
//! # Examples
//! ```no_run
//! use spotify_private_api::Session;
//!
//! #[tokio::main]
//! async fn main() {
//!     let dc = "SP_DC".to_string();
//!     let key = "SP_KEY".to_string();
//!     let user_id = "USER_ID".to_string();
//!
//!     let s = Session::new(&dc, &key, &user_id)
//!         .await
//!         .expect("Failed to create session");
//!
//!     let root_list = s.get_root_list()
//!         .await
//!         .expect("failed to get root list");
//!
//!     let changes = root_list
//!         .new_request()
//!         .add("New Folder", &root_list.generate_folder_uri(), 0, 2)
//!         .build();
//!
//!     s.send_changes(&changes)
//!         .await
//!         .expect("failed to send changes");
//! }
//! ```
//!
//! # How to generate dc and key (valid for 1 year)
//! - Open a new Incognito window in your browser at and [login to spotify](https://accounts.spotify.com/en/login?continue=https:%2F%2Fopen.spotify.com%2F)
//!  - Open Developer Tools in your browser (might require developer menu to be enabled in some browsers)
//!  - In the Network tab, enable "Preserve log"
//!  - Login to Spotify.
//!  - In the Network tab, search/Filter for `password`
//!  - Under cookies for the request save the values for `sp_dc` and `sp_key`.
//!  - Close the window without logging out (Otherwise the cookies are made invalid).
//!
//! # How to get your user id
//! - Click on your account name at the top right corner in the [spotify web player](https://open.spotify.com/)
//! - Choose `Profile`
//! - The last part of the link is your user id, e.g: `https://open.spotify.com/user/{user_id}`

mod api;
mod session;

use std::error;

pub type Session = session::Session;
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;
