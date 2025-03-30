use serde::Serialize;

use crate::data::{Category, Filter, Sort, SortOrder};


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
    order: Option<SortOrder>,
    #[serde(rename = "p")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<usize>,
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
            offset: None,
        }
    }

    pub fn with_page(mut self, page: impl AsRef<str>) -> Self {
        self.page = Some(page.as_ref().to_string());
        self
    }

    pub fn with_page_option(mut self, page: Option<impl AsRef<str>>) -> Self {
        if let Some(page) = page {
            self.page = Some(page.as_ref().to_string());
        }
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

    pub fn with_category_option(mut self, category: Option<impl Into<Category>>) -> Self {
        if let Some(category) = category {
            self.category = Some(category.into());
        }
        self
    }

    pub fn with_filter(mut self, filter: impl Into<Filter>) -> Self {
        self.filter = Some(filter.into());
        self
    }

    pub fn with_filter_option(mut self, filter: Option<impl Into<Filter>>) -> Self {
        if let Some(filter) = filter {
            self.filter = Some(filter.into());
        }
        self
    }

    pub fn with_sort(mut self, sort: impl Into<Sort>) -> Self {
        self.sort = Some(sort.into());
        self
    }

    pub fn with_sort_option(mut self, sort: Option<impl Into<Sort>>) -> Self {
        if let Some(sort) = sort {
            self.sort = Some(sort.into());
        }
        self
    }

    pub fn with_order(mut self, order: impl Into<SortOrder>) -> Self {
        self.order = Some(order.into());
        self
    }

    pub fn with_order_option(mut self, order: Option<impl Into<SortOrder>>) -> Self {
        if let Some(order) = order {
            self.order = Some(order.into());
        }
        self
    }

    pub fn with_offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn with_offset_option(mut self, offset: Option<usize>) -> Self {
        if let Some(offset) = offset {
            self.offset = Some(offset);
        }
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
            &self.offset,
        ) {
            (None, None, None, None, None, None, None) => self.base_url,
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
