mod base_response;
pub mod persons;
pub use persons::delete::delete_handler;
pub use persons::get::get_handler;
pub use persons::get_single::get_single_handler;
pub use persons::patch::patch_handler;
pub use persons::post::post_handler;
