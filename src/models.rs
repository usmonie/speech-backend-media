pub mod result {

    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
    pub struct GraphicMedia {
        pub id: String,
        pub url: String,
        pub name: String,
        pub uploaded_by_id: String,
        pub size: u64,
        pub created_at: u64,
        pub uploaded_at: u64,
        pub height: u32,
        pub width: u32,

        pub media_type: GraphicMediaType,
    }

    #[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
    pub enum GraphicMediaType {
        Video { repeatable: bool, duration: u64 },
        Image,
    }

    #[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
    pub struct AudioMedia {
        pub id: String,
        pub url: String,
        pub name: String,
        pub uploaded_by_id: String,
        pub size: u64,
        pub created_at: u64,
        pub uploaded_at: u64,
        pub duration: u64,

        pub media_type: AudioMediaType,
    }

    #[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
    pub enum AudioMediaType {
        Audio,
        Music,
    }
}