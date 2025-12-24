use crate::models::meals::Meal;
use crate::utils::pexels::get_meal_image;

pub async fn get_seed_meals() -> Vec<Meal> {
    let mut meals = Vec::new();

    // =============== INDONESIAN FOOD ===============
    meals.extend(get_indonesian_meals().await);
    
    // =============== ITALIAN FOOD ===============
    meals.extend(get_italian_meals().await);
    
    // =============== JAPANESE FOOD ===============
    meals.extend(get_japanese_meals().await);
    
    // =============== MEXICAN FOOD ===============
    meals.extend(get_mexican_meals().await);
    
    // =============== DESSERTS INTERNATIONAL ===============
    meals.extend(get_international_desserts().await);
    
    // =============== HEALTHY OPTIONS ===============
    meals.extend(get_healthy_meals().await);

    meals
}

// =============== INDONESIAN SECTION ===============
async fn get_indonesian_meals() -> Vec<Meal> {
    vec![
        create_meal(
            "Nasi Goreng",
            "Main Course",
            Some(400),
            vec![
                "Cooked rice".to_string(),
                "Eggs".to_string(),
                "Chicken breast".to_string(),
                "Shrimp".to_string(),
                "Carrots".to_string(),
                "Green peas".to_string(),
                "Spring onions".to_string(),
                "Garlic".to_string(),
                "Shallots".to_string(),
                "Sweet soy sauce".to_string(),
                "Fish sauce".to_string(),
                "Vegetable oil".to_string(),
                "Salt and pepper".to_string(),
            ],
            "Indonesian fried rice with eggs, chicken, shrimp, and vegetables, seasoned with sweet soy sauce.",
        ).await,
        
        create_meal(
            "Rendang",
            "Main Course",
            Some(450),
            vec![
                "Beef chuck".to_string(),
                "Coconut milk".to_string(),
                "Lemongrass".to_string(),
                "Galangal".to_string(),
                "Turmeric leaves".to_string(),
                "Kaffir lime leaves".to_string(),
                "Candlenuts".to_string(),
                "Garlic".to_string(),
                "Shallots".to_string(),
                "Chilies".to_string(),
                "Ginger".to_string(),
                "Coriander".to_string(),
                "Cumin".to_string(),
                "Turmeric powder".to_string(),
                "Tamarind".to_string(),
                "Palm sugar".to_string(),
                "Salt".to_string(),
            ],
            "Spicy slow-cooked beef curry from West Sumatra, cooked in coconut milk and rich spices until tender.",
        ).await,
        
        create_meal(
            "Sate Ayam",
            "Appetizer",
            Some(250),
            vec![
                "Chicken thighs".to_string(),
                "Bamboo skewers".to_string(),
                "Sweet soy sauce".to_string(),
                "Peanut butter".to_string(),
                "Brown sugar".to_string(),
                "Lime juice".to_string(),
                "Garlic".to_string(),
                "Shallots".to_string(),
                "Coriander".to_string(),
                "Cumin".to_string(),
                "Turmeric powder".to_string(),
                "Vegetable oil".to_string(),
                "Salt".to_string(),
                "Cucumber".to_string(),
                "Onion".to_string(),
            ],
            "Grilled chicken skewers served with peanut sauce, rice cakes, cucumber, and onion.",
        ).await,
    ]
}

// =============== ITALIAN SECTION ===============
async fn get_italian_meals() -> Vec<Meal> {
    vec![
        create_meal(
            "Spaghetti Carbonara",
            "Main Course",
            Some(550),
            vec![
                "Spaghetti pasta".to_string(),
                "Pancetta or guanciale".to_string(),
                "Eggs".to_string(),
                "Pecorino Romano cheese".to_string(),
                "Parmigiano Reggiano".to_string(),
                "Black pepper".to_string(),
                "Salt".to_string(),
                "Garlic".to_string(),
                "Extra virgin olive oil".to_string(),
            ],
            "Classic Roman pasta dish with eggs, cheese, pancetta, and black pepper.",
        ).await,
        
        create_meal(
            "Margherita Pizza",
            "Main Course",
            Some(850),
            vec![
                "Pizza dough".to_string(),
                "San Marzano tomatoes".to_string(),
                "Fresh mozzarella".to_string(),
                "Fresh basil".to_string(),
                "Extra virgin olive oil".to_string(),
                "Salt".to_string(),
                "Active dry yeast".to_string(),
                "Sugar".to_string(),
                "Water".to_string(),
                "Flour".to_string(),
            ],
            "Neapolitan pizza with tomato, mozzarella, fresh basil, and olive oil.",
        ).await,
        
        create_meal(
            "Tiramisu",
            "Dessert",
            Some(350),
            vec![
                "Ladyfinger cookies".to_string(),
                "Mascarpone cheese".to_string(),
                "Eggs".to_string(),
                "Sugar".to_string(),
                "Strong espresso coffee".to_string(),
                "Cocoa powder".to_string(),
                "Marsala wine".to_string(),
                "Vanilla extract".to_string(),
            ],
            "Classic Italian dessert made of ladyfingers dipped in coffee, layered with mascarpone cream.",
        ).await,
        
        create_meal(
            "Lasagna",
            "Main Course",
            Some(650),
            vec![
                "Lasagna noodles".to_string(),
                "Ground beef".to_string(),
                "Italian sausage".to_string(),
                "Marinara sauce".to_string(),
                "Ricotta cheese".to_string(),
                "Mozzarella cheese".to_string(),
                "Parmesan cheese".to_string(),
                "Eggs".to_string(),
                "Onion".to_string(),
                "Garlic".to_string(),
                "Italian seasoning".to_string(),
                "Salt and pepper".to_string(),
            ],
            "Layered pasta dish with meat sauce, cheese, and marinara.",
        ).await,
    ]
}

// =============== JAPANESE SECTION ===============
async fn get_japanese_meals() -> Vec<Meal> {
    vec![
        create_meal(
            "Sushi Platter",
            "Main Course",
            Some(480),
            vec![
                "Sushi rice".to_string(),
                "Nori seaweed".to_string(),
                "Fresh salmon".to_string(),
                "Tuna".to_string(),
                "Shrimp".to_string(),
                "Cucumber".to_string(),
                "Avocado".to_string(),
                "Soy sauce".to_string(),
                "Wasabi".to_string(),
                "Pickled ginger".to_string(),
                "Rice vinegar".to_string(),
                "Sugar".to_string(),
                "Salt".to_string(),
            ],
            "Assorted sushi including nigiri, maki rolls, and sashimi.",
        ).await,
        
        create_meal(
            "Ramen",
            "Soup",
            Some(520),
            vec![
                "Ramen noodles".to_string(),
                "Pork broth".to_string(),
                "Chashu pork".to_string(),
                "Soft-boiled eggs".to_string(),
                "Bamboo shoots".to_string(),
                "Nori seaweed".to_string(),
                "Green onions".to_string(),
                "Bean sprouts".to_string(),
                "Soy sauce".to_string(),
                "Mirin".to_string(),
                "Sake".to_string(),
                "Garlic".to_string(),
                "Ginger".to_string(),
            ],
            "Japanese noodle soup with rich broth, chashu pork, and soft-boiled eggs.",
        ).await,
        
        create_meal(
            "Tempura",
            "Appetizer",
            Some(320),
            vec![
                "Shrimp".to_string(),
                "Sweet potato".to_string(),
                "Zucchini".to_string(),
                "Eggplant".to_string(),
                "Flour".to_string(),
                "Cornstarch".to_string(),
                "Egg".to_string(),
                "Ice water".to_string(),
                "Baking powder".to_string(),
                "Vegetable oil".to_string(),
                "Tempura dipping sauce".to_string(),
                "Daikon radish".to_string(),
                "Ginger".to_string(),
            ],
            "Lightly battered and deep-fried seafood and vegetables.",
        ).await,
    ]
}

// =============== MEXICAN SECTION ===============
async fn get_mexican_meals() -> Vec<Meal> {
    vec![
        create_meal(
            "Tacos al Pastor",
            "Main Course",
            Some(350),
            vec![
                "Pork shoulder".to_string(),
                "Pineapple".to_string(),
                "Corn tortillas".to_string(),
                "Onion".to_string(),
                "Cilantro".to_string(),
                "Lime wedges".to_string(),
                "Guajillo chilies".to_string(),
                "Achiote paste".to_string(),
                "Garlic".to_string(),
                "Vinegar".to_string(),
                "Oregano".to_string(),
                "Cumin".to_string(),
                "Salt and pepper".to_string(),
            ],
            "Mexican tacos with marinated pork, pineapple, onions, and cilantro.",
        ).await,
        
        create_meal(
            "Guacamole",
            "Appetizer",
            Some(220),
            vec![
                "Avocados".to_string(),
                "Lime juice".to_string(),
                "Red onion".to_string(),
                "Tomato".to_string(),
                "Cilantro".to_string(),
                "Jalapeño pepper".to_string(),
                "Garlic".to_string(),
                "Salt".to_string(),
                "Tortilla chips".to_string(),
            ],
            "Fresh avocado dip with lime, onion, tomato, and cilantro.",
        ).await,
        
        create_meal(
            "Churros",
            "Dessert",
            Some(280),
            vec![
                "Flour".to_string(),
                "Water".to_string(),
                "Butter".to_string(),
                "Eggs".to_string(),
                "Sugar".to_string(),
                "Cinnamon".to_string(),
                "Salt".to_string(),
                "Vegetable oil".to_string(),
                "Chocolate sauce".to_string(),
            ],
            "Fried dough pastry dusted with cinnamon sugar, served with chocolate sauce.",
        ).await,
    ]
}

// =============== INTERNATIONAL DESSERTS ===============
async fn get_international_desserts() -> Vec<Meal> {
    vec![
        create_meal(
            "Chocolate Lava Cake",
            "Dessert",
            Some(420),
            vec![
                "Dark chocolate".to_string(),
                "Butter".to_string(),
                "Eggs".to_string(),
                "Sugar".to_string(),
                "Flour".to_string(),
                "Cocoa powder".to_string(),
                "Vanilla ice cream".to_string(),
                "Powdered sugar".to_string(),
                "Salt".to_string(),
            ],
            "Decadent chocolate cake with a molten chocolate center, served with vanilla ice cream.",
        ).await,
        
        create_meal(
            "Crème Brûlée",
            "Dessert",
            Some(320),
            vec![
                "Heavy cream".to_string(),
                "Egg yolks".to_string(),
                "Sugar".to_string(),
                "Vanilla bean".to_string(),
                "Salt".to_string(),
                "Berries for garnish".to_string(),
            ],
            "French dessert consisting of rich custard topped with caramelized sugar.",
        ).await,
        
        create_meal(
            "Cheesecake",
            "Dessert",
            Some(380),
            vec![
                "Cream cheese".to_string(),
                "Graham cracker crumbs".to_string(),
                "Sugar".to_string(),
                "Eggs".to_string(),
                "Sour cream".to_string(),
                "Vanilla extract".to_string(),
                "Butter".to_string(),
                "Lemon juice".to_string(),
                "Fresh berries".to_string(),
            ],
            "Creamy cheesecake with graham cracker crust, topped with fresh berries.",
        ).await,
        
        create_meal(
            "Macarons",
            "Dessert",
            Some(180),
            vec![
                "Almond flour".to_string(),
                "Powdered sugar".to_string(),
                "Egg whites".to_string(),
                "Granulated sugar".to_string(),
                "Food coloring".to_string(),
                "Buttercream filling".to_string(),
                "Ganache filling".to_string(),
                "Salt".to_string(),
            ],
            "French sandwich cookies with crispy shell and creamy filling.",
        ).await,
        
        create_meal(
            "Gelato",
            "Dessert",
            Some(200),
            vec![
                "Whole milk".to_string(),
                "Heavy cream".to_string(),
                "Sugar".to_string(),
                "Egg yolks".to_string(),
                "Vanilla bean".to_string(),
                "Fresh fruit puree".to_string(),
                "Chocolate".to_string(),
                "Pistachios".to_string(),
            ],
            "Italian-style ice cream with intense flavors and creamy texture.",
        ).await,
    ]
}

// =============== HEALTHY OPTIONS ===============
async fn get_healthy_meals() -> Vec<Meal> {
    vec![
        create_meal(
            "Greek Salad",
            "Salad",
            Some(280),
            vec![
                "Romaine lettuce".to_string(),
                "Cucumber".to_string(),
                "Tomatoes".to_string(),
                "Red onion".to_string(),
                "Kalamata olives".to_string(),
                "Feta cheese".to_string(),
                "Olive oil".to_string(),
                "Lemon juice".to_string(),
                "Oregano".to_string(),
                "Salt and pepper".to_string(),
            ],
            "Fresh Mediterranean salad with vegetables, olives, and feta cheese.",
        ).await,
        
        create_meal(
            "Quinoa Bowl",
            "Healthy",
            Some(350),
            vec![
                "Quinoa".to_string(),
                "Chickpeas".to_string(),
                "Avocado".to_string(),
                "Kale".to_string(),
                "Cherry tomatoes".to_string(),
                "Cucumber".to_string(),
                "Lemon tahini dressing".to_string(),
                "Pumpkin seeds".to_string(),
                "Salt and pepper".to_string(),
            ],
            "Nutrient-packed bowl with quinoa, vegetables, and tahini dressing.",
        ).await,
        
        create_meal(
            "Grilled Salmon",
            "Healthy",
            Some(380),
            vec![
                "Salmon fillet".to_string(),
                "Lemon".to_string(),
                "Dill".to_string(),
                "Garlic".to_string(),
                "Olive oil".to_string(),
                "Asparagus".to_string(),
                "Brown rice".to_string(),
                "Salt and pepper".to_string(),
                "Butter".to_string(),
            ],
            "Grilled salmon with lemon dill sauce, served with asparagus and brown rice.",
        ).await,
        
        create_meal(
            "Smoothie Bowl",
            "Breakfast",
            Some(320),
            vec![
                "Frozen bananas".to_string(),
                "Mixed berries".to_string(),
                "Greek yogurt".to_string(),
                "Almond milk".to_string(),
                "Chia seeds".to_string(),
                "Granola".to_string(),
                "Coconut flakes".to_string(),
                "Honey".to_string(),
                "Fresh fruit toppings".to_string(),
            ],
            "Thick smoothie bowl topped with granola, fresh fruit, and seeds.",
        ).await,
    ]
}

async fn create_meal(
    name: &str,
    category: &str,
    calories: Option<i32>,
    ingredients: Vec<String>,
    description: &str,
) -> Meal {
    let image_url = match get_meal_image(name).await {
        Ok(Some(url)) => Some(url),
        Ok(None) => {
            eprintln!("No image found for {}, using placeholder", name);
            Some(format!("https://via.placeholder.com/400x300/FF6B6B/FFFFFF?text={}", 
                name.replace(" ", "+")))
        }
        Err(e) => {
            eprintln!("Error fetching image for {}: {}", name, e);
            Some("https://via.placeholder.com/400x300/4ECDC4/000000?text=Food+Image".to_string())
        }
    };

    Meal {
        id: None,
        name: name.to_string(),
        ingredients,
        category: category.to_string(),
        calories,
        image_url,
        description: Some(description.to_string()),
    }
}