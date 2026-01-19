// Get all recipes
const recipes = await client.getRecipes();
console.log(`Found ${recipes.length} recipes`);

// Get a specific recipe
const recipe = await client.getRecipeById("recipe-id-here");
console.log(`Recipe: ${recipe.name}`);
console.log(`Ingredients: ${recipe.ingredients.length}`);

// Add recipe ingredients to a list
await client.addRecipeToList(
  recipe.id,
  groceryList.id,
  1.5, // scale factor (optional)
);
