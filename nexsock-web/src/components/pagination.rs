use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page<T>
where
    T: Clone + serde::Serialize,
{
    pub page: u32,
    pub page_size: u32,
    pub total: u32,
    pub items: Vec<T>,
}

impl<T> Page<T>
where
    T: Clone + serde::Serialize,
{
    #[allow(dead_code)]
    pub fn new(page: u32, page_size: u32, total: u32, items: Vec<T>) -> Self {
        Self {
            page,
            page_size,
            total,
            items,
        }
    }

    pub fn total_pages(&self) -> u32 {
        if self.total == 0 {
            return 1;
        }
        (self.total as f32 / self.page_size as f32).ceil() as u32
    }

    pub fn next_page(&self) -> Option<u32> {
        if self.page < self.total_pages() {
            Some(self.page + 1)
        } else {
            None
        }
    }

    pub fn prev_page(&self) -> Option<u32> {
        if self.page > 1 {
            Some(self.page - 1)
        } else {
            None
        }
    }
}

/// This struct holds the data for the Tera template context.
/// It contains pre-calculated values for pagination, so the template
/// doesn't need to call methods.
#[derive(Debug, Clone, Serialize)]
pub struct PageContext<T>
where
    T: Clone + serde::Serialize,
{
    pub page: u32,
    pub page_size: u32,
    pub total: u32,
    pub total_pages: u32,
    pub items: Vec<T>,
    pub next_page: Option<u32>,
    pub prev_page: Option<u32>,
}

impl<T> From<Page<T>> for PageContext<T>
where
    T: Clone + serde::Serialize,
{
    /// Converts a Page struct into a PageContext struct, calculating
    /// the pagination fields in the process.
    fn from(page: Page<T>) -> Self {
        Self {
            total_pages: page.total_pages(),
            next_page: page.next_page(),
            prev_page: page.prev_page(),
            page: page.page,
            page_size: page.page_size,
            total: page.total,
            items: page.items,
        }
    }
}
