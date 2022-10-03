use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Manga {
    pub(crate) name: String,
    pub(crate) path: String,
    pub(crate) cover_image_url: String,
    pub(crate) eps: Vec<MangaEp>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MangaEp {
    pub(crate) name: String,
    pub(crate) ep_path: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Page {
    pub(crate) content_type: ContentType,
    pub(crate) content: String,
    pub(crate) content_base_id: String,
    pub(crate) container_style: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ContentType {
    Image,
    Script,
}
