#![deny(clippy::all)]

use napi::bindgen_prelude::*;
use napi_derive::napi;

// Re-export anylist_rs types for internal use
use anylist_rs::{
    AnyListClient as RsClient, Category as RsCategory, CategoryGroup as RsCategoryGroup,
    FavouriteItem as RsFavouriteItem, FavouritesList as RsFavouritesList,
    ICalendarInfo as RsICalendarInfo, Ingredient as RsIngredient, List as RsList,
    ListItem as RsListItem, MealPlanEvent as RsMealPlanEvent, Recipe as RsRecipe, RecipeBuilder,
    RecipeCollection as RsRecipeCollection, SavedTokens as RsSavedTokens, Store as RsStore,
    StoreFilter as RsStoreFilter,
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

/// A category for organizing list items
#[napi(object)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
    pub sort_index: i32,
}

impl From<&RsCategory> for Category {
    fn from(category: &RsCategory) -> Self {
        Category {
            id: category.id().to_string(),
            name: category.name().to_string(),
            icon: category.icon().map(|s| s.to_string()),
            sort_index: category.sort_index(),
        }
    }
}

/// A group of categories
#[napi(object)]
pub struct CategoryGroup {
    pub id: String,
    pub name: String,
    pub categories: Vec<Category>,
}

impl From<&RsCategoryGroup> for CategoryGroup {
    fn from(group: &RsCategoryGroup) -> Self {
        CategoryGroup {
            id: group.id().to_string(),
            name: group.name().to_string(),
            categories: group.categories().iter().map(Category::from).collect(),
        }
    }
}

/// A store for organizing where to buy items
#[napi(object)]
pub struct Store {
    pub id: String,
    pub name: String,
    pub sort_index: i32,
}

impl From<&RsStore> for Store {
    fn from(store: &RsStore) -> Self {
        Store {
            id: store.id().to_string(),
            name: store.name().to_string(),
            sort_index: store.sort_index(),
        }
    }
}

/// A filter for stores
#[napi(object)]
pub struct StoreFilter {
    pub id: String,
    pub name: String,
    pub store_ids: Vec<String>,
}

impl From<&RsStoreFilter> for StoreFilter {
    fn from(filter: &RsStoreFilter) -> Self {
        StoreFilter {
            id: filter.id().to_string(),
            name: filter.name().to_string(),
            store_ids: filter.store_ids().iter().map(|s| s.to_string()).collect(),
        }
    }
}

/// A favourite item (starter list item)
#[napi(object)]
pub struct FavouriteItem {
    pub id: String,
    pub list_id: String,
    pub name: String,
    pub quantity: Option<String>,
    pub details: Option<String>,
    pub category: Option<String>,
}

impl From<&RsFavouriteItem> for FavouriteItem {
    fn from(item: &RsFavouriteItem) -> Self {
        FavouriteItem {
            id: item.id().to_string(),
            list_id: item.list_id().to_string(),
            name: item.name().to_string(),
            quantity: item.quantity().map(|s| s.to_string()),
            details: item.details().map(|s| s.to_string()),
            category: item.category().map(|s| s.to_string()),
        }
    }
}

/// A list of favourite items (starter list)
#[napi(object)]
pub struct FavouritesList {
    pub id: String,
    pub name: String,
    pub items: Vec<FavouriteItem>,
    pub shopping_list_id: Option<String>,
}

impl From<&RsFavouritesList> for FavouritesList {
    fn from(list: &RsFavouritesList) -> Self {
        FavouritesList {
            id: list.id().to_string(),
            name: list.name().to_string(),
            items: list.items().iter().map(FavouriteItem::from).collect(),
            shopping_list_id: list.shopping_list_id().map(|s| s.to_string()),
        }
    }
}

/// A meal plan event
#[napi(object)]
pub struct MealPlanEvent {
    pub id: String,
    pub date: String,
    pub title: Option<String>,
    pub recipe_id: Option<String>,
    pub label_id: Option<String>,
    pub details: Option<String>,
}

impl From<&RsMealPlanEvent> for MealPlanEvent {
    fn from(event: &RsMealPlanEvent) -> Self {
        MealPlanEvent {
            id: event.id().to_string(),
            date: event.date().to_string(),
            title: event.title().map(|s| s.to_string()),
            recipe_id: event.recipe_id().map(|s| s.to_string()),
            label_id: event.label_id().map(|s| s.to_string()),
            details: event.details().map(|s| s.to_string()),
        }
    }
}

/// iCalendar sync information
#[napi(object)]
pub struct ICalendarInfo {
    pub enabled: bool,
    pub url: Option<String>,
    pub token: Option<String>,
}

impl From<&RsICalendarInfo> for ICalendarInfo {
    fn from(info: &RsICalendarInfo) -> Self {
        ICalendarInfo {
            enabled: info.enabled(),
            url: info.url().map(|s| s.to_string()),
            token: info.token().map(|s| s.to_string()),
        }
    }
}

/// A collection of recipes
#[napi(object)]
pub struct RecipeCollection {
    pub id: String,
    pub name: String,
    pub recipe_ids: Vec<String>,
}

impl From<&RsRecipeCollection> for RecipeCollection {
    fn from(collection: &RsRecipeCollection) -> Self {
        RecipeCollection {
            id: collection.id().to_string(),
            name: collection.name().to_string(),
            recipe_ids: collection
                .recipe_ids()
                .iter()
                .map(|s| s.to_string())
                .collect(),
        }
    }
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

    // ==================== Category Methods ====================

    /// Create a new category in a list
    #[napi]
    pub async fn create_category(
        &self,
        list_id: String,
        category_group_id: String,
        name: String,
    ) -> Result<Category> {
        let category = self
            .inner
            .create_category(&list_id, &category_group_id, &name)
            .await
            .map_err(to_napi_error)?;

        Ok(Category::from(&category))
    }

    /// Delete a category from a list
    #[napi]
    pub async fn delete_category(&self, list_id: String, category_id: String) -> Result<()> {
        self.inner
            .delete_category(&list_id, &category_id)
            .await
            .map_err(to_napi_error)?;

        Ok(())
    }

    /// Rename a category
    #[napi]
    pub async fn rename_category(
        &self,
        list_id: String,
        category_group_id: String,
        category_id: String,
        new_name: String,
    ) -> Result<()> {
        self.inner
            .rename_category(&list_id, &category_group_id, &category_id, &new_name)
            .await
            .map_err(to_napi_error)?;

        Ok(())
    }

    // ==================== Store Methods ====================

    /// Get all stores for a list
    #[napi]
    pub async fn get_stores_for_list(&self, list_id: String) -> Result<Vec<Store>> {
        let stores = self
            .inner
            .get_stores_for_list(&list_id)
            .await
            .map_err(to_napi_error)?;

        Ok(stores.iter().map(Store::from).collect())
    }

    /// Create a new store for a list
    #[napi]
    pub async fn create_store(&self, list_id: String, name: String) -> Result<Store> {
        let store = self
            .inner
            .create_store(&list_id, &name)
            .await
            .map_err(to_napi_error)?;

        Ok(Store::from(&store))
    }

    /// Update a store's name
    #[napi]
    pub async fn update_store(
        &self,
        list_id: String,
        store_id: String,
        new_name: String,
    ) -> Result<()> {
        self.inner
            .update_store(&list_id, &store_id, &new_name)
            .await
            .map_err(to_napi_error)?;

        Ok(())
    }

    /// Get store filters for a list
    #[napi]
    pub async fn get_store_filters_for_list(&self, list_id: String) -> Result<Vec<StoreFilter>> {
        let filters = self
            .inner
            .get_store_filters_for_list(&list_id)
            .await
            .map_err(to_napi_error)?;

        Ok(filters.iter().map(StoreFilter::from).collect())
    }

    /// Delete a store from a list
    #[napi]
    pub async fn delete_store(&self, list_id: String, store_id: String) -> Result<()> {
        self.inner
            .delete_store(&list_id, &store_id)
            .await
            .map_err(to_napi_error)?;

        Ok(())
    }

    // ==================== Favourites Methods ====================

    /// Get all favourite items across all lists
    #[napi]
    pub async fn get_favourites(&self) -> Result<Vec<FavouriteItem>> {
        let favourites = self.inner.get_favourites().await.map_err(to_napi_error)?;

        Ok(favourites.iter().map(FavouriteItem::from).collect())
    }

    /// Get all favourites lists (starter lists)
    #[napi]
    pub async fn get_favourites_lists(&self) -> Result<Vec<FavouritesList>> {
        let lists = self
            .inner
            .get_favourites_lists()
            .await
            .map_err(to_napi_error)?;

        Ok(lists.iter().map(FavouritesList::from).collect())
    }

    /// Get favourites for a specific shopping list
    #[napi]
    pub async fn get_favourites_for_list(
        &self,
        shopping_list_id: String,
    ) -> Result<FavouritesList> {
        let list = self
            .inner
            .get_favourites_for_list(&shopping_list_id)
            .await
            .map_err(to_napi_error)?;

        Ok(FavouritesList::from(&list))
    }

    /// Add a favourite item to the default list
    #[napi]
    pub async fn add_favourite(
        &self,
        name: String,
        category: Option<String>,
    ) -> Result<FavouriteItem> {
        let item = self
            .inner
            .add_favourite(&name, category.as_deref())
            .await
            .map_err(to_napi_error)?;

        Ok(FavouriteItem::from(&item))
    }

    /// Add a favourite item to a specific list
    #[napi]
    pub async fn add_favourite_to_list(
        &self,
        list_id: String,
        name: String,
        category: Option<String>,
    ) -> Result<FavouriteItem> {
        let item = self
            .inner
            .add_favourite_to_list(&list_id, &name, category.as_deref())
            .await
            .map_err(to_napi_error)?;

        Ok(FavouriteItem::from(&item))
    }

    /// Remove a favourite item from a list
    #[napi]
    pub async fn remove_favourite(&self, list_id: String, item_id: String) -> Result<()> {
        self.inner
            .remove_favourite(&list_id, &item_id)
            .await
            .map_err(to_napi_error)?;

        Ok(())
    }

    /// Add a favourite item to a shopping list
    #[napi]
    pub async fn add_favourite_to_shopping_list(
        &self,
        favourite_list_id: String,
        favourite_id: String,
        shopping_list_id: String,
    ) -> Result<ListItem> {
        // First get the favourite item
        let favourites_list = self
            .inner
            .get_favourites_for_list(&favourite_list_id)
            .await
            .map_err(to_napi_error)?;

        let favourite = favourites_list
            .items()
            .iter()
            .find(|f| f.id() == favourite_id)
            .ok_or_else(|| Error::new(Status::GenericFailure, "Favourite item not found"))?;

        let item = self
            .inner
            .add_favourite_to_shopping_list(favourite, &shopping_list_id)
            .await
            .map_err(to_napi_error)?;

        Ok(ListItem::from(&item))
    }

    // ==================== Meal Planning Methods ====================

    /// Get meal plan events for a date range
    #[napi]
    pub async fn get_meal_plan_events(
        &self,
        start_date: String,
        end_date: String,
    ) -> Result<Vec<MealPlanEvent>> {
        let events = self
            .inner
            .get_meal_plan_events(&start_date, &end_date)
            .await
            .map_err(to_napi_error)?;

        Ok(events.iter().map(MealPlanEvent::from).collect())
    }

    /// Create a meal plan event
    #[napi]
    pub async fn create_meal_plan_event(
        &self,
        calendar_id: String,
        date: String,
        recipe_id: Option<String>,
        title: Option<String>,
        label_id: Option<String>,
    ) -> Result<MealPlanEvent> {
        let event = self
            .inner
            .create_meal_plan_event(
                &calendar_id,
                &date,
                recipe_id.as_deref(),
                title.as_deref(),
                label_id.as_deref(),
            )
            .await
            .map_err(to_napi_error)?;

        Ok(MealPlanEvent::from(&event))
    }

    /// Update a meal plan event
    #[napi]
    pub async fn update_meal_plan_event(
        &self,
        calendar_id: String,
        event_id: String,
        date: String,
        recipe_id: Option<String>,
        title: Option<String>,
        label_id: Option<String>,
    ) -> Result<()> {
        self.inner
            .update_meal_plan_event(
                &calendar_id,
                &event_id,
                &date,
                recipe_id.as_deref(),
                title.as_deref(),
                label_id.as_deref(),
            )
            .await
            .map_err(to_napi_error)?;

        Ok(())
    }

    /// Delete a meal plan event
    #[napi]
    pub async fn delete_meal_plan_event(
        &self,
        calendar_id: String,
        event_id: String,
    ) -> Result<()> {
        self.inner
            .delete_meal_plan_event(&calendar_id, &event_id)
            .await
            .map_err(to_napi_error)?;

        Ok(())
    }

    /// Add meal plan ingredients to a shopping list
    #[napi]
    pub async fn add_meal_plan_ingredients_to_list(
        &self,
        list_id: String,
        start_date: String,
        end_date: String,
    ) -> Result<()> {
        self.inner
            .add_meal_plan_ingredients_to_list(&list_id, &start_date, &end_date)
            .await
            .map_err(to_napi_error)?;

        Ok(())
    }

    // ==================== iCalendar Methods ====================

    /// Enable iCalendar sync and get the URL
    #[napi]
    pub async fn enable_icalendar(&self) -> Result<ICalendarInfo> {
        let info = self.inner.enable_icalendar().await.map_err(to_napi_error)?;

        Ok(ICalendarInfo::from(&info))
    }

    /// Disable iCalendar sync
    #[napi]
    pub async fn disable_icalendar(&self) -> Result<()> {
        self.inner
            .disable_icalendar()
            .await
            .map_err(to_napi_error)?;

        Ok(())
    }

    /// Get the iCalendar URL if enabled
    #[napi]
    pub async fn get_icalendar_url(&self) -> Result<Option<String>> {
        let url = self
            .inner
            .get_icalendar_url()
            .await
            .map_err(to_napi_error)?;

        Ok(url)
    }

    // ==================== Recipe Collection Methods ====================

    /// Get all recipe collections
    #[napi]
    pub async fn get_recipe_collections(&self) -> Result<Vec<RecipeCollection>> {
        let collections = self
            .inner
            .get_recipe_collections()
            .await
            .map_err(to_napi_error)?;

        Ok(collections.iter().map(RecipeCollection::from).collect())
    }

    /// Create a new recipe collection
    #[napi]
    pub async fn create_recipe_collection(&self, name: String) -> Result<RecipeCollection> {
        let collection = self
            .inner
            .create_recipe_collection(&name)
            .await
            .map_err(to_napi_error)?;

        Ok(RecipeCollection::from(&collection))
    }

    /// Delete a recipe collection
    #[napi]
    pub async fn delete_recipe_collection(&self, collection_id: String) -> Result<()> {
        self.inner
            .delete_recipe_collection(&collection_id)
            .await
            .map_err(to_napi_error)?;

        Ok(())
    }

    /// Add a recipe to a collection
    #[napi]
    pub async fn add_recipe_to_collection(
        &self,
        collection_id: String,
        recipe_id: String,
    ) -> Result<()> {
        self.inner
            .add_recipe_to_collection(&collection_id, &recipe_id)
            .await
            .map_err(to_napi_error)?;

        Ok(())
    }

    /// Remove a recipe from a collection
    #[napi]
    pub async fn remove_recipe_from_collection(
        &self,
        collection_id: String,
        recipe_id: String,
    ) -> Result<()> {
        self.inner
            .remove_recipe_from_collection(&collection_id, &recipe_id)
            .await
            .map_err(to_napi_error)?;

        Ok(())
    }
}
