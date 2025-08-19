use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct PaginationParams {
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_limit")]
    pub limit: u32,
}

fn default_page() -> u32 {
    1
}

fn default_limit() -> u32 {
    20
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            page: default_page(),
            limit: default_limit(),
        }
    }
}

impl PaginationParams {
    pub fn validate(&mut self) {
        // Ensure page is at least 1
        if self.page < 1 {
            self.page = 1;
        }
        
        // Limit the maximum page size
        if self.limit > 100 {
            self.limit = 100;
        }
        
        // Ensure limit is at least 1
        if self.limit < 1 {
            self.limit = 1;
        }
    }
    
    pub fn offset(&self) -> u32 {
        (self.page - 1) * self.limit
    }
    
    pub fn limit_plus_one(&self) -> u32 {
        self.limit + 1
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub pagination: PaginationMeta,
}

#[derive(Debug, Clone, Serialize)]
pub struct PaginationMeta {
    pub page: u32,
    pub limit: u32,
    pub total: Option<u32>,
    pub has_next: bool,
    pub has_prev: bool,
}

impl<T> PaginatedResponse<T> {
    pub fn new(mut data: Vec<T>, params: &PaginationParams, total: Option<u32>) -> Self {
        let has_next = data.len() > params.limit as usize;
        
        // Remove the extra item used to check for next page
        if has_next {
            data.pop();
        }
        
        Self {
            data,
            pagination: PaginationMeta {
                page: params.page,
                limit: params.limit,
                total,
                has_next,
                has_prev: params.page > 1,
            },
        }
    }
}

// Macro for easy pagination query building
#[macro_export]
macro_rules! paginate_query {
    ($query:expr, $params:expr) => {{
        let mut params = $params;
        params.validate();
        
        $query
            .limit(params.limit_plus_one() as i64)
            .offset(params.offset() as i64)
    }};
}
