use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::sync::Arc;

/// Saved authentication tokens for reuse across sessions
#[napi(object)]
#[derive(Clone)]
pub struct SavedTokens {
    pub access_token: String,
    pub refresh_token: String,
    pub user_id: String,
    pub is_premium_user: bool,
}

impl From<anylist_rs::SavedTokens> for SavedTokens {
    fn from(tokens: anylist_rs::SavedTokens) -> Self {
        Self {
            access_token: tokens.access_token().to_string(),
            refresh_token: tokens.refresh_token().to_string(),
            user_id: tokens.user_id().to_string(),
            is_premium_user: tokens.is_premium_user(),
        }
    }
}

impl From<SavedTokens> for anylist_rs::SavedTokens {
    fn from(tokens: SavedTokens) -> Self {
        anylist_rs::SavedTokens::new(
            tokens.access_token,
            tokens.refresh_token,
            tokens.user_id,
            tokens.is_premium_user,
        )
    }
}

/// A single item in a shopping list
#[napi(object)]
#[derive(Clone)]
pub struct ListItem {
    pub id: String,
    pub list_id: String,
    pub name: String,
    pub details: String,
    pub is_checked: bool,
    pub quantity: Option<String>,
    pub category: Option<String>,
    pub user_id: Option<String>,
    pub product_upc: Option<String>,
}

impl From<&anylist_rs::ListItem> for ListItem {
    fn from(item: &anylist_rs::ListItem) -> Self {
        Self {
            id: item.id().to_string(),
            list_id: item.list_id().to_string(),
            name: item.name().to_string(),
            details: item.details().to_string(),
            is_checked: item.is_checked(),
            quantity: item.quantity().map(|s| s.to_string()),
            category: item.category().map(|s| s.to_string()),
            user_id: item.user_id().map(|s| s.to_string()),
            product_upc: item.product_upc().map(|s| s.to_string()),
        }
    }
}

/// A shopping list containing items
#[napi(object)]
#[derive(Clone)]
pub struct List {
    pub id: String,
    pub name: String,
    pub items: Vec<ListItem>,
}

impl From<&anylist_rs::List> for List {
    fn from(list: &anylist_rs::List) -> Self {
        Self {
            id: list.id().to_string(),
            name: list.name().to_string(),
            items: list.items().iter().map(ListItem::from).collect(),
        }
    }
}

/// AnyList API client for interacting with the AnyList service
#[napi]
pub struct AnyListClient {
    inner: Arc<anylist_rs::AnyListClient>,
}

#[napi]
impl AnyListClient {
    /// Login with email and password
    #[napi(factory)]
    pub async fn login(email: String, password: String) -> Result<AnyListClient> {
        let client = anylist_rs::AnyListClient::login(&email, &password)
            .await
            .map_err(|e| Error::from_reason(format!("Login failed: {}", e)))?;

        Ok(Self {
            inner: Arc::new(client),
        })
    }

    /// Create client from saved tokens
    #[napi(factory)]
    pub fn from_tokens(tokens: SavedTokens) -> Result<AnyListClient> {
        let client = anylist_rs::AnyListClient::from_tokens(tokens.into())
            .map_err(|e| Error::from_reason(format!("Failed to create client from tokens: {}", e)))?;

        Ok(Self {
            inner: Arc::new(client),
        })
    }

    /// Export tokens for later reuse
    #[napi]
    pub fn export_tokens(&self) -> Result<SavedTokens> {
        self.inner
            .export_tokens()
            .map(SavedTokens::from)
            .map_err(|e| Error::from_reason(format!("Failed to export tokens: {}", e)))
    }

    /// Get the user ID
    #[napi]
    pub fn user_id(&self) -> String {
        self.inner.user_id()
    }

    /// Check if user has premium subscription
    #[napi]
    pub fn is_premium_user(&self) -> bool {
        self.inner.is_premium_user()
    }

    /// Get the client identifier
    #[napi]
    pub fn client_identifier(&self) -> String {
        self.inner.client_identifier().to_string()
    }

    /// Get all shopping lists
    #[napi]
    pub async fn get_lists(&self) -> Result<Vec<List>> {
        let lists = self
            .inner
            .get_lists()
            .await
            .map_err(|e| Error::from_reason(format!("Failed to get lists: {}", e)))?;

        Ok(lists.iter().map(List::from).collect())
    }

    /// Create a new shopping list
    #[napi]
    pub async fn create_list(&self, name: String) -> Result<List> {
        let list = self
            .inner
            .create_list(&name)
            .await
            .map_err(|e| Error::from_reason(format!("Failed to create list: {}", e)))?;

        Ok(List::from(&list))
    }

    /// Add an item to a list
    #[napi]
    pub async fn add_item(&self, list_id: String, name: String) -> Result<ListItem> {
        let item = self
            .inner
            .add_item(&list_id, &name)
            .await
            .map_err(|e| Error::from_reason(format!("Failed to add item: {}", e)))?;

        Ok(ListItem::from(&item))
    }

    /// Add an item to a list with additional details
    #[napi]
    pub async fn add_item_with_details(
        &self,
        list_id: String,
        name: String,
        quantity: Option<String>,
        details: Option<String>,
        category: Option<String>,
    ) -> Result<ListItem> {
        let item = self
            .inner
            .add_item_with_details(
                &list_id,
                &name,
                quantity.as_deref(),
                details.as_deref(),
                category.as_deref(),
            )
            .await
            .map_err(|e| Error::from_reason(format!("Failed to add item: {}", e)))?;

        Ok(ListItem::from(&item))
    }
}
