pub mod result {

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

    impl From<GraphicMedia> for GraphicResponse {
        fn from(graphic_media: GraphicMedia) -> Self {
            let (repeatable, duration) = match graphic_media.media_type {
                GraphicMediaType::Video {
                    repeatable,
                    duration,
                } => (repeatable, duration),
                GraphicMediaType::Image => (false, 0),
            };

            Self {
                id: graphic_media.id,
                uploaded_by_id: graphic_media.uploaded_by_id,
                name: graphic_media.name,
                size: graphic_media.size,
                created_at: graphic_media.created_at,
                upload_at: graphic_media.uploaded_at,
                height: graphic_media.height,
                width: graphic_media.width,
                repeatable,
                duration,
            }
        }
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

    impl From<AudioMedia> for AudioResponse {
        fn from(audio_media: AudioMedia) -> Self {
            Self {
                id: audio_media.id,
                uploaded_by_id: audio_media.uploaded_by_id,
                name: audio_media.name,
                size: 0,
                created_at: 0,
                upload_at: 0,
                repeatable: false,
                duration: 0,
            }
        }
    }
}