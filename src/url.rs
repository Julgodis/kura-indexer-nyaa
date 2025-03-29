use serde::Serialize;

#[derive(Serialize, Debug, Clone, Copy)]
pub enum Category {
    Anime,
    Audio,
    Literature,
    LiveAction,
    Pictures,
    Software,
}

impl Category {
    pub fn as_str(&self) -> &'static str {
        match self {
            Category::Anime => "1_0",
            Category::Audio => "2_0",
            Category::Literature => "3_0",
            Category::LiveAction => "4_0",
            Category::Pictures => "5_0",
            Category::Software => "6_0",
        }
    }
}

impl ToString for Category {
    fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}

#[derive(Serialize, Debug, Clone, Copy)]
pub enum Filter {
    NoFilter = 0,
    NoRemakes = 1,
    TrustedOnly = 2,
}

impl Filter {
    pub fn as_str(&self) -> &'static str {
        match self {
            Filter::NoFilter => "0",
            Filter::NoRemakes => "1",
            Filter::TrustedOnly => "2",
        }
    }
}

#[derive(Serialize, Debug, Clone, Copy)]
pub enum Sort {
    Comments = 1,
    Size = 2,
    Date = 3,
    Seeders = 4,
    Leechers = 5,
    Downloads = 6,
}

impl Sort {
    pub fn as_str(&self) -> &'static str {
        match self {
            Sort::Comments => "1",
            Sort::Size => "2",
            Sort::Date => "3",
            Sort::Seeders => "4",
            Sort::Leechers => "5",
            Sort::Downloads => "6",
        }
    }
}

#[derive(Serialize, Debug, Clone, Copy)]
pub enum Order {
    Ascending = 1,
    Descending = 2,
}

impl Order {
    pub fn as_str(&self) -> &'static str {
        match self {
            Order::Ascending => "1",
            Order::Descending => "2",
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct NyaaUrlBuilder {
    #[serde(skip)]
    base_url: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<String>,
    #[serde(rename = "q")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    query: Option<String>,
    #[serde(rename = "c")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    category: Option<Category>,
    #[serde(rename = "f")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Filter>,
    #[serde(rename = "s")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<Sort>,
    #[serde(rename = "o")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    order: Option<Order>,
}

impl NyaaUrlBuilder {
    pub fn new(base_url: impl AsRef<str>) -> Self {
        Self {
            base_url: base_url.as_ref().to_string(),
            page: None,
            query: None,
            category: None,
            filter: None,
            sort: None,
            order: None,
        }
    }

    pub fn with_page(mut self, page: impl AsRef<str>) -> Self {
        self.page = Some(page.as_ref().to_string());
        self
    }

    pub fn with_query(mut self, query: impl AsRef<str>) -> Self {
        self.query = Some(query.as_ref().to_string());
        self
    }

    pub fn with_category(mut self, category: impl Into<Category>) -> Self {
        self.category = Some(category.into());
        self
    }

    pub fn with_filter(mut self, filter: impl Into<Filter>) -> Self {
        self.filter = Some(filter.into());
        self
    }

    pub fn with_sort(mut self, sort: impl Into<Sort>) -> Self {
        self.sort = Some(sort.into());
        self
    }

    pub fn with_order(mut self, order: impl Into<Order>) -> Self {
        self.order = Some(order.into());
        self
    }

    pub fn build(self) -> String {
        let base_url = self.base_url.clone();
        match (
            &self.page,
            &self.query,
            &self.category,
            &self.filter,
            &self.sort,
            &self.order,
        ) {
            (None, None, None, None, None, None) => self.base_url,
            _ => format!(
                "{}/?{}",
                base_url.trim_end_matches("/"),
                serde_urlencoded::to_string(self)
                    .expect("Failed to serialize URL parameters")
                    .trim_start_matches("?")
            ),
        }
    }
}
