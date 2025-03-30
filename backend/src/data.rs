use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
pub enum Category {
    #[serde(rename = "0_0")]
    All,
    #[serde(rename = "1_0")]
    Anime,
    #[serde(rename = "1_1")]
    AnimeAmv,
    #[serde(rename = "1_2")]
    AnimeEnglish,
    #[serde(rename = "1_3")]
    AnimeNonEnglish,
    #[serde(rename = "1_4")]
    AnimeRaw,
    #[serde(rename = "2_0")]
    Audio,
    #[serde(rename = "2_1")]
    AudioLossless,
    #[serde(rename = "2_2")]
    AudioLossy,
    #[serde(rename = "3_0")]
    Literature,
    #[serde(rename = "3_1")]
    LiteratureEnglish,
    #[serde(rename = "3_2")]
    LiteratureNonEnglish,
    #[serde(rename = "3_3")]
    LiteratureRaw,
    #[serde(rename = "4_0")]
    LiveAction,
    #[serde(rename = "4_1")]
    LiveActionEnglish,
    #[serde(rename = "4_2")]
    LiveActionIdol,
    #[serde(rename = "4_3")]
    LiveActionNonEnglish,
    #[serde(rename = "4_4")]
    LiveActionRaw,
    #[serde(rename = "5_0")]
    Pictures,
    #[serde(rename = "5_1")]
    PicturesGraphics,
    #[serde(rename = "5_2")]
    PicturesPhotos,
    #[serde(rename = "6_0")]
    Software,
    #[serde(rename = "6_1")]
    SoftwareApps,
    #[serde(rename = "6_2")]
    SoftwareGames,
}

impl FromStr for Category {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0_0" => Ok(Category::All),
            "1_0" => Ok(Category::Anime),
            "1_1" => Ok(Category::AnimeAmv),
            "1_2" => Ok(Category::AnimeEnglish),
            "1_3" => Ok(Category::AnimeNonEnglish),
            "1_4" => Ok(Category::AnimeRaw),
            "2_0" => Ok(Category::Audio),
            "2_1" => Ok(Category::AudioLossless),
            "2_2" => Ok(Category::AudioLossy),
            "3_0" => Ok(Category::Literature),
            "3_1" => Ok(Category::LiteratureEnglish),
            "3_2" => Ok(Category::LiteratureNonEnglish),
            "3_3" => Ok(Category::LiteratureRaw),
            "4_0" => Ok(Category::LiveAction),
            "4_1" => Ok(Category::LiveActionEnglish),
            "4_2" => Ok(Category::LiveActionIdol),
            "4_3" => Ok(Category::LiveActionNonEnglish),
            "4_4" => Ok(Category::LiveActionRaw),
            "5_0" => Ok(Category::Pictures),
            "5_1" => Ok(Category::PicturesGraphics),
            "5_2" => Ok(Category::PicturesPhotos),
            "6_0" => Ok(Category::Software),
            "6_1" => Ok(Category::SoftwareApps),
            "6_2" => Ok(Category::SoftwareGames),
            _ => Err(anyhow::anyhow!(format!("invalid category: {}", s))),
        }
    }
}

impl rusqlite::types::ToSql for Category {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        let s = match self {
            Category::All => "0_0",
            Category::Anime => "1_0",
            Category::AnimeAmv => "1_1",
            Category::AnimeEnglish => "1_2",
            Category::AnimeNonEnglish => "1_3",
            Category::AnimeRaw => "1_4",
            Category::Audio => "2_0",
            Category::AudioLossless => "2_1",
            Category::AudioLossy => "2_2",
            Category::Literature => "3_0",
            Category::LiteratureEnglish => "3_1",
            Category::LiteratureNonEnglish => "3_2",
            Category::LiteratureRaw => "3_3",
            Category::LiveAction => "4_0",
            Category::LiveActionEnglish => "4_1",
            Category::LiveActionIdol => "4_2",
            Category::LiveActionNonEnglish => "4_3",
            Category::LiveActionRaw => "4_4",
            Category::Pictures => "5_0",
            Category::PicturesGraphics => "5_1",
            Category::PicturesPhotos => "5_2",
            Category::Software => "6_0",
            Category::SoftwareApps => "6_1",
            Category::SoftwareGames => "6_2",
        };
        Ok(rusqlite::types::ToSqlOutput::Owned(s.to_string().into()))
    }
}

impl rusqlite::types::FromSql for Category {
    fn column_result(value: rusqlite::types::ValueRef) -> rusqlite::types::FromSqlResult<Self> {
        String::column_result(value).and_then(|s| {
            Category::from_str(&s).map_err(|_| rusqlite::types::FromSqlError::InvalidType)
        })
    }
}

impl ToString for Category {
    fn to_string(&self) -> String {
        match self {
            Category::All => "0_0".to_string(),
            Category::Anime => "1_0".to_string(),
            Category::AnimeAmv => "1_1".to_string(),
            Category::AnimeEnglish => "1_2".to_string(),
            Category::AnimeNonEnglish => "1_3".to_string(),
            Category::AnimeRaw => "1_4".to_string(),
            Category::Audio => "2_0".to_string(),
            Category::AudioLossless => "2_1".to_string(),
            Category::AudioLossy => "2_2".to_string(),
            Category::Literature => "3_0".to_string(),
            Category::LiteratureEnglish => "3_1".to_string(),
            Category::LiteratureNonEnglish => "3_2".to_string(),
            Category::LiteratureRaw => "3_3".to_string(),
            Category::LiveAction => "4_0".to_string(),
            Category::LiveActionEnglish => "4_1".to_string(),
            Category::LiveActionIdol => "4_2".to_string(),
            Category::LiveActionNonEnglish => "4_3".to_string(),
            Category::LiveActionRaw => "4_4".to_string(),
            Category::Pictures => "5_0".to_string(),
            Category::PicturesGraphics => "5_1".to_string(),
            Category::PicturesPhotos => "5_2".to_string(),
            Category::Software => "6_0".to_string(),
            Category::SoftwareApps => "6_1".to_string(),
            Category::SoftwareGames => "6_2".to_string(),
        }
    }
}

impl Category {
    pub fn english(&self) -> String {
        match self {
            Category::All => "All".to_string(),
            Category::Anime => "Anime".to_string(),
            Category::AnimeAmv => "Anime - AMV".to_string(),
            Category::AnimeEnglish => "Anime - English".to_string(),
            Category::AnimeNonEnglish => "Anime - Non-English".to_string(),
            Category::AnimeRaw => "Anime - Raw".to_string(),
            Category::Audio => "Audio".to_string(),
            Category::AudioLossless => "Audio - Lossless".to_string(),
            Category::AudioLossy => "Audio - Lossy".to_string(),
            Category::Literature => "Literature".to_string(),
            Category::LiteratureEnglish => "Literature - English".to_string(),
            Category::LiteratureNonEnglish => "Literature - Non-English".to_string(),
            Category::LiteratureRaw => "Literature - Raw".to_string(),
            Category::LiveAction => "Live Action".to_string(),
            Category::LiveActionEnglish => "Live Action - English".to_string(),
            Category::LiveActionIdol => "Live Action - Idol/PV".to_string(),
            Category::LiveActionNonEnglish => "Live Action - Non-English".to_string(),
            Category::LiveActionRaw => "Live Action - Raw".to_string(),
            Category::Pictures => "Pictures".to_string(),
            Category::PicturesGraphics => "Pictures - Graphics".to_string(),
            Category::PicturesPhotos => "Pictures - Photos".to_string(),
            Category::Software => "Software".to_string(),
            Category::SoftwareApps => "Software - Apps".to_string(),
            Category::SoftwareGames => "Software - Games".to_string(),
        }
    }
}


#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub enum Filter {
    #[serde(rename = "0")]
    NoFilter,
    #[serde(rename = "1")]
    NoRemake,
    #[serde(rename = "2")]
    Trusted,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub enum Sort {
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "seeders")]
    Seeders,
    #[serde(rename = "leechers")]
    Leechers,
    #[serde(rename = "downloads")]
    Downloads,
    #[serde(rename = "size")]
    Size,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub enum SortOrder {
    #[serde(rename = "asc")]
    Ascending,
    #[serde(rename = "desc")]
    Descending,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Item {
    pub id: String,
    pub title: String,
    pub link: String,
    pub date: chrono::DateTime<chrono::Utc>,
    pub seeders: usize,
    pub leechers: usize,
    pub downloads: usize,
    pub category: Category,
    pub size: u64,
    pub comments: usize,
    pub trusted: bool,
    pub remake: bool,
    pub download_link: Option<String>,
    pub magnet_link: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct File {
    pub id: String,
    pub name: String,
    pub size: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Comment {
    pub id: String,
    pub user: String,
    pub avatar: Option<String>,
    pub date: chrono::DateTime<chrono::Utc>,
    pub edited: Option<chrono::DateTime<chrono::Utc>>,
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct View {
    pub id: String,
    pub title: String,
    pub link: String,
    pub date: chrono::DateTime<chrono::Utc>,
    pub seeders: usize,
    pub leechers: usize,
    pub downloads: usize,
    pub category: Category,
    pub size: u64,
    pub trusted: bool,
    pub remake: bool,
    pub download_link: String,
    pub magnet_link: String,
    pub description: String,
    pub submitter: String,
    pub info_hash: String,
    pub info_link: Option<String>,
    pub files: Vec<File>,
    pub comments: Vec<Comment>,
}
