#![deny(clippy::all)]

use napi::bindgen_prelude::*;
use napi_derive::napi;

// Re-export anylist_rs types for internal use
use anylist_rs::{
    AnyListClient as RsClient, Ingredient as RsIngredient, List as RsList, ListItem as RsListItem,
    Recipe as RsRecipe, RecipeBuilder, SavedTokens as RsSavedTokens,
};

/// Input for creating a new ingredient
#[napi(object)]
pub struct IngredientInput {
    pub name: String,
    pub quantity: Option<String>,
    pub note: Option<String>,
}

impl From<&IngredientInput> for RsIngredient {
    fn from(input: &IngredientInput) -> Self {
        let mut ingredient = RsIngredient::new(&input.name);
        if let Some(ref qty) = input.quantity {
            ingredient = ingredient.quantity_of(qty);
        }
        if let Some(ref note) = input.note {
            ingredient = ingredient.note_of(note);
        }
        ingredient
    }
}

/// Convert AnyList errors to NAPI errors
fn to_napi_error(err: anylist_rs::AnyListError) -> Error {
    Error::new(Status::GenericFailure, format!("{}", err))
}

/// Saved authentication tokens for resuming sessions
#[napi(object)]
pub struct SavedTokens {
    pub user_id: String,
    pub access_token: String,
    pub refresh_token: String,
    pub is_premium_user: bool,
}

impl From<RsSavedTokens> for SavedTokens {
    fn from(tokens: RsSavedTokens) -> Self {
        SavedTokens {
            user_id: tokens.user_id().to_string(),
            access_token: tokens.access_token().to_string(),
            refresh_token: tokens.refresh_token().to_string(),
            is_premium_user: tokens.is_premium_user(),
        }
    }
}

impl From<SavedTokens> for RsSavedTokens {
    fn from(tokens: SavedTokens) -> Self {
        RsSavedTokens::new(
            tokens.access_token,
            tokens.refresh_token,
            tokens.user_id,
            tokens.is_premium_user,
        )
    }
}

/// A grocery list item
#[napi(object)]
pub struct ListItem {
    pub id: String,
    pub name: String,
    pub checked: bool,
    pub note: String,
    pub quantity: Option<String>,
    pub category: Option<String>,
}

impl From<&RsListItem> for ListItem {
    fn from(item: &RsListItem) -> Self {
        ListItem {
            id: item.id().to_string(),
            name: item.name().to_string(),
            checked: item.is_checked(),
            quantity: item.quantity().map(|s| s.to_string()),
            note: item.details().to_owned(),
            category: item.category().map(|s| s.to_string()),
        }
    }
}

/// A grocery list
#[napi(object)]
pub struct List {
    pub id: String,
    pub name: String,
    pub items: Vec<ListItem>,
}

impl From<&RsList> for List {
    fn from(list: &RsList) -> Self {
        List {
            id: list.id().to_string(),
            name: list.name().to_string(),
            items: list.items().iter().map(ListItem::from).collect(),
        }
    }
}

/// A recipe ingredient
#[napi(object)]
pub struct Ingredient {
    pub name: String,
    pub quantity: Option<String>,
    pub note: Option<String>,
}

impl From<&RsIngredient> for Ingredient {
    fn from(ingredient: &RsIngredient) -> Self {
        Ingredient {
            name: ingredient.name().to_string(),
            quantity: ingredient.quantity().map(|s| s.to_string()),
            note: ingredient.note().map(|s| s.to_string()),
        }
    }
}

/// A recipe
#[napi(object)]
pub struct Recipe {
    pub id: String,
    pub name: String,
    pub ingredients: Vec<Ingredient>,
    pub preparation_steps: Vec<String>,
    pub note: Option<String>,
    pub source_name: Option<String>,
    pub source_url: Option<String>,
    pub servings: Option<String>,
    pub prep_time: Option<i32>,
    pub cook_time: Option<i32>,
    pub rating: Option<i32>,
    pub nutritional_info: Option<String>,
    pub photo_id: Option<String>,
}

/// Options for creating a new recipe
#[napi(object)]
pub struct CreateRecipeOptions {
    /// Recipe name (required)
    pub name: String,
    /// List of ingredients
    pub ingredients: Vec<IngredientInput>,
    /// Preparation/cooking steps
    pub preparation_steps: Vec<String>,
    /// Recipe notes/description
    pub note: Option<String>,
    /// Source name (e.g., "Web", "Cookbook")
    pub source_name: Option<String>,
    /// Source URL
    pub source_url: Option<String>,
    /// Serving size (e.g., "4 servings")
    pub servings: Option<String>,
    /// Prep time in minutes
    pub prep_time: Option<i32>,
    /// Cook time in minutes
    pub cook_time: Option<i32>,
    /// Rating from 1-5
    pub rating: Option<i32>,
    /// Nutritional information
    pub nutritional_info: Option<String>,
    /// Photo ID (from upload_photo)
    pub photo_id: Option<String>,
}

impl From<&RsRecipe> for Recipe {
    fn from(recipe: &RsRecipe) -> Self {
        Recipe {
            id: recipe.id().to_string(),
            name: recipe.name().to_string(),
            ingredients: recipe.ingredients().iter().map(Ingredient::from).collect(),
            preparation_steps: recipe
                .preparation_steps()
                .iter()
                .map(|s| s.to_string())
                .collect(),
            note: recipe.note().map(|s| s.to_string()),
            source_name: recipe.source_name().map(|s| s.to_string()),
            source_url: recipe.source_url().map(|s| s.to_string()),
            servings: recipe.servings().map(|s| s.to_string()),
            prep_time: recipe.prep_time(),
            cook_time: recipe.cook_time(),
            rating: recipe.rating(),
            nutritional_info: recipe.nutritional_info().map(|s| s.to_string()),
            photo_id: recipe.photo_id().map(|s| s.to_string()),
        }
    }
}

/// The main AnyList client for interacting with the API
#[napi]
pub struct AnyListClient {
    inner: RsClient,
}

#[napi]
impl AnyListClient {
    /// Login to AnyList with email and password
    #[napi]
    pub async fn login(email: String, password: String) -> Result<AnyListClient> {
        let client = RsClient::login(&email, &password)
            .await
            .map_err(to_napi_error)?;

        Ok(AnyListClient { inner: client })
    }

    /// Create a client from saved tokens (for resuming sessions)
    #[napi]
    pub fn from_tokens(tokens: SavedTokens) -> Result<AnyListClient> {
        let rs_tokens: RsSavedTokens = tokens.into();
        let client = RsClient::from_tokens(rs_tokens).map_err(to_napi_error)?;

        Ok(AnyListClient { inner: client })
    }

    /// Get the saved tokens for this session
    #[napi]
    pub fn get_tokens(&self) -> Result<SavedTokens> {
        let tokens = self.inner.export_tokens().map_err(to_napi_error)?;
        Ok(tokens.into())
    }

    /// Get all lists
    #[napi]
    pub async fn get_lists(&self) -> Result<Vec<List>> {
        let lists = self.inner.get_lists().await.map_err(to_napi_error)?;

        Ok(lists.iter().map(List::from).collect())
    }

    /// Create a new list
    #[napi]
    pub async fn create_list(&self, name: String) -> Result<List> {
        let list = self.inner.create_list(&name).await.map_err(to_napi_error)?;

        Ok(List::from(&list))
    }

    /// Get a specific list by ID
    #[napi]
    pub async fn get_list_by_id(&self, list_id: String) -> Result<List> {
        let list = self
            .inner
            .get_list_by_id(&list_id)
            .await
            .map_err(to_napi_error)?;

        Ok(List::from(&list))
    }

    /// Get a list by name
    #[napi]
    pub async fn get_list_by_name(&self, name: String) -> Result<List> {
        let list = self
            .inner
            .get_list_by_name(&name)
            .await
            .map_err(to_napi_error)?;

        Ok(List::from(&list))
    }

    /// Rename a list
    #[napi]
    pub async fn rename_list(&self, list_id: String, new_name: String) -> Result<()> {
        self.inner
            .rename_list(&list_id, &new_name)
            .await
            .map_err(to_napi_error)?;

        Ok(())
    }

    /// Add an item to a list
    #[napi]
    pub async fn add_item(&self, list_id: String, name: String) -> Result<ListItem> {
        let item = self
            .inner
            .add_item(&list_id, &name)
            .await
            .map_err(to_napi_error)?;

        Ok(ListItem::from(&item))
    }

    /// Add an item with details to a list
    #[napi]
    pub async fn add_item_with_details(
        &self,
        list_id: String,
        name: String,
        quantity: Option<String>,
        note: Option<String>,
        category: Option<String>,
    ) -> Result<ListItem> {
        let item = self
            .inner
            .add_item_with_details(
                &list_id,
                &name,
                quantity.as_deref(),
                note.as_deref(),
                category.as_deref(),
            )
            .await
            .map_err(to_napi_error)?;

        Ok(ListItem::from(&item))
    }

    /// Delete an item from a list
    #[napi]
    pub async fn delete_item(&self, list_id: String, item_id: String) -> Result<()> {
        self.inner
            .delete_item(&list_id, &item_id)
            .await
            .map_err(to_napi_error)?;

        Ok(())
    }

    /// Cross off (check) an item
    #[napi]
    pub async fn cross_off_item(&self, list_id: String, item_id: String) -> Result<()> {
        self.inner
            .cross_off_item(&list_id, &item_id)
            .await
            .map_err(to_napi_error)?;

        Ok(())
    }

    /// Uncheck an item
    #[napi]
    pub async fn uncheck_item(&self, list_id: String, item_id: String) -> Result<()> {
        self.inner
            .uncheck_item(&list_id, &item_id)
            .await
            .map_err(to_napi_error)?;

        Ok(())
    }

    /// Update an existing item
    #[napi]
    pub async fn update_item(
        &self,
        list_id: String,
        item_id: String,
        name: String,
        quantity: Option<String>,
        note: Option<String>,
        category: Option<String>,
    ) -> Result<()> {
        self.inner
            .update_item(
                &list_id,
                &item_id,
                &name,
                quantity.as_deref(),
                note.as_deref(),
                category.as_deref(),
            )
            .await
            .map_err(to_napi_error)?;

        Ok(())
    }

    /// Delete multiple items at once
    #[napi]
    pub async fn bulk_delete_items(&self, list_id: String, item_ids: Vec<String>) -> Result<()> {
        let item_id_refs: Vec<&str> = item_ids.iter().map(|s| s.as_str()).collect();
        self.inner
            .bulk_delete_items(&list_id, &item_id_refs)
            .await
            .map_err(to_napi_error)?;

        Ok(())
    }

    /// Delete all crossed off (checked) items from a list
    #[napi]
    pub async fn delete_all_crossed_off_items(&self, list_id: String) -> Result<()> {
        self.inner
            .delete_all_crossed_off_items(&list_id)
            .await
            .map_err(to_napi_error)?;

        Ok(())
    }

    /// Get all recipes
    #[napi]
    pub async fn get_recipes(&self) -> Result<Vec<Recipe>> {
        let recipes = self.inner.get_recipes().await.map_err(to_napi_error)?;

        Ok(recipes.iter().map(Recipe::from).collect())
    }

    /// Get a specific recipe by ID
    #[napi]
    pub async fn get_recipe_by_id(&self, recipe_id: String) -> Result<Recipe> {
        let recipe = self
            .inner
            .get_recipe_by_id(&recipe_id)
            .await
            .map_err(to_napi_error)?;

        Ok(Recipe::from(&recipe))
    }

    /// Get a recipe by name
    #[napi]
    pub async fn get_recipe_by_name(&self, name: String) -> Result<Recipe> {
        let recipe = self
            .inner
            .get_recipe_by_name(&name)
            .await
            .map_err(to_napi_error)?;

        Ok(Recipe::from(&recipe))
    }

    /// Create a new recipe with full metadata support
    #[napi]
    pub async fn create_recipe(&self, options: CreateRecipeOptions) -> Result<Recipe> {
        let rs_ingredients: Vec<RsIngredient> =
            options.ingredients.iter().map(RsIngredient::from).collect();

        let mut builder = RecipeBuilder::new(&options.name)
            .ingredients(rs_ingredients)
            .preparation_steps(options.preparation_steps);

        if let Some(note) = options.note {
            builder = builder.note(note);
        }
        if let Some(source_name) = options.source_name {
            builder = builder.source_name(source_name);
        }
        if let Some(source_url) = options.source_url {
            builder = builder.source_url(source_url);
        }
        if let Some(servings) = options.servings {
            builder = builder.servings(servings);
        }
        if let Some(prep_time) = options.prep_time {
            builder = builder.prep_time(prep_time);
        }
        if let Some(cook_time) = options.cook_time {
            builder = builder.cook_time(cook_time);
        }
        if let Some(rating) = options.rating {
            builder = builder.rating(rating);
        }
        if let Some(nutritional_info) = options.nutritional_info {
            builder = builder.nutritional_info(nutritional_info);
        }
        if let Some(photo_id) = options.photo_id {
            builder = builder.photo_id(photo_id);
        }

        let recipe = builder.save(&self.inner).await.map_err(to_napi_error)?;

        Ok(Recipe::from(&recipe))
    }

    /// Add recipe ingredients to a list with optional scale factor
    #[napi]
    pub async fn add_recipe_to_list(
        &self,
        recipe_id: String,
        list_id: String,
        scale_factor: Option<f64>,
    ) -> Result<()> {
        self.inner
            .add_recipe_to_list(&recipe_id, &list_id, scale_factor)
            .await
            .map_err(to_napi_error)?;

        Ok(())
    }

    /// Update an existing recipe
    /// Note: The recipe name cannot be changed (use the existing name in options)
    #[napi]
    pub async fn update_recipe(
        &self,
        recipe_id: String,
        options: CreateRecipeOptions,
    ) -> Result<Recipe> {
        // Fetch the existing recipe to use as base for the builder
        let existing = self
            .inner
            .get_recipe_by_id(&recipe_id)
            .await
            .map_err(to_napi_error)?;

        let rs_ingredients: Vec<RsIngredient> =
            options.ingredients.iter().map(RsIngredient::from).collect();

        // Start from existing recipe (preserves the ID for update)
        let mut builder = RecipeBuilder::from(&existing)
            .ingredients(rs_ingredients)
            .preparation_steps(options.preparation_steps);

        if let Some(note) = options.note {
            builder = builder.note(note);
        }
        if let Some(source_name) = options.source_name {
            builder = builder.source_name(source_name);
        }
        if let Some(source_url) = options.source_url {
            builder = builder.source_url(source_url);
        }
        if let Some(servings) = options.servings {
            builder = builder.servings(servings);
        }
        if let Some(prep_time) = options.prep_time {
            builder = builder.prep_time(prep_time);
        }
        if let Some(cook_time) = options.cook_time {
            builder = builder.cook_time(cook_time);
        }
        if let Some(rating) = options.rating {
            builder = builder.rating(rating);
        }
        if let Some(nutritional_info) = options.nutritional_info {
            builder = builder.nutritional_info(nutritional_info);
        }
        if let Some(photo_id) = options.photo_id {
            builder = builder.photo_id(photo_id);
        }

        let recipe = builder.save(&self.inner).await.map_err(to_napi_error)?;

        Ok(Recipe::from(&recipe))
    }

    /// Delete a recipe
    #[napi]
    pub async fn delete_recipe(&self, recipe_id: String) -> Result<()> {
        self.inner
            .delete_recipe(&recipe_id)
            .await
            .map_err(to_napi_error)?;

        Ok(())
    }

    /// Delete a list
    #[napi]
    pub async fn delete_list(&self, list_id: String) -> Result<()> {
        self.inner
            .delete_list(&list_id)
            .await
            .map_err(to_napi_error)?;

        Ok(())
    }

    /// Upload a photo for use with recipes
    /// Returns the photo ID which can be used with createRecipe
    #[napi]
    pub async fn upload_photo(&self, data: Buffer, filename: String) -> Result<String> {
        let photo_id = self
            .inner
            .upload_photo(data.to_vec(), &filename)
            .await
            .map_err(to_napi_error)?;

        Ok(photo_id)
    }
}
