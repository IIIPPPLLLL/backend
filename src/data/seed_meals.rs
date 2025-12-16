use crate::models::meals::Meal;

// data/seed_meals.rs
pub fn get_seed_meals() -> Vec<Meal> {
    vec![
        // 🍚 Indonesian Food
        Meal {
            id: None,
            name: "Nasi Goreng".to_string(),
            ingredients: vec![
                "nasi".to_string(),
                "telur".to_string(),
                "ayam".to_string(),
                "bawang merah".to_string(),
                "kecap manis".to_string(),
            ],
            category: "Indonesian".to_string(),
            calories: Some(450),
            image_url: Some(
                "https://images.unsplash.com/photo-1555939594-58d7cb561ad1?w=800&fit=crop"
                    .to_string(),
            ),
            description: Some("Indonesian fried rice with chicken and egg".to_string()),
        },
        Meal {
            id: None,
            name: "Rendang".to_string(),
            ingredients: vec![
                "daging sapi".to_string(),
                "santan".to_string(),
                "serai".to_string(),
                "lengkuas".to_string(),
                "daun jeruk".to_string(),
                "cabe merah".to_string(),
            ],
            category: "Indonesian".to_string(),
            calories: Some(520),
            image_url: Some(
                "https://images.unsplash.com/photo-1586190848861-99aa4a171e90?w=800&fit=crop"
                    .to_string(),
            ),
            description: Some("Spicy Indonesian beef stew slow-cooked in coconut milk".to_string()),
        },
        Meal {
            id: None,
            name: "Sate Ayam".to_string(),
            ingredients: vec![
                "daging ayam".to_string(),
                "kacang tanah".to_string(),
                "kecap manis".to_string(),
                "bawang merah".to_string(),
                "jeruk limau".to_string(),
            ],
            category: "Indonesian".to_string(),
            calories: Some(320),
            image_url: Some(
                "https://images.unsplash.com/photo-1550547660-d9450f859349?w=800&fit=crop"
                    .to_string(),
            ),
            description: Some("Indonesian chicken skewers with peanut sauce".to_string()),
        },
        Meal {
            id: None,
            name: "Gado-gado".to_string(),
            ingredients: vec![
                "sayuran".to_string(),
                "tahu".to_string(),
                "tempe".to_string(),
                "telur".to_string(),
                "saus kacang".to_string(),
                "kerupuk".to_string(),
            ],
            category: "Indonesian".to_string(),
            calories: Some(280),
            image_url: Some(
                "https://images.unsplash.com/photo-1546069901-ba9599a7e63c?w=800&fit=crop"
                    .to_string(),
            ),
            description: Some("Indonesian vegetable salad with peanut sauce".to_string()),
        },
        Meal {
            id: None,
            name: "Soto Ayam".to_string(),
            ingredients: vec![
                "ayam".to_string(),
                "soun".to_string(),
                "kol".to_string(),
                "seledri".to_string(),
                "bawang goreng".to_string(),
                "kunyit".to_string(),
            ],
            category: "Indonesian".to_string(),
            calories: Some(300),
            image_url: Some(
                "https://images.unsplash.com/photo-1546833999-b9f581a1996d?w=800&fit=crop"
                    .to_string(),
            ),
            description: Some("Indonesian chicken soup with turmeric".to_string()),
        },
        // 🍜 Asian Cuisine
        Meal {
            id: None,
            name: "Pad Thai".to_string(),
            ingredients: vec![
                "rice noodles".to_string(),
                "shrimp".to_string(),
                "tofu".to_string(),
                "bean sprouts".to_string(),
                "peanuts".to_string(),
                "tamarind sauce".to_string(),
            ],
            category: "Thai".to_string(),
            calories: Some(400),
            image_url: Some(
                "https://images.unsplash.com/photo-1559314809-2b99056a8c4a?w=800&fit=crop"
                    .to_string(),
            ),
            description: Some("Stir-fried rice noodles with shrimp and peanuts".to_string()),
        },
        Meal {
            id: None,
            name: "Korean Bibimbap".to_string(),
            ingredients: vec![
                "rice".to_string(),
                "beef".to_string(),
                "spinach".to_string(),
                "carrots".to_string(),
                "mushrooms".to_string(),
                "fried egg".to_string(),
                "gochujang".to_string(),
            ],
            category: "Korean".to_string(),
            calories: Some(480),
            image_url: Some(
                "https://images.unsplash.com/photo-1447279506476-3faec8071eee?w=800&fit=crop"
                    .to_string(),
            ),
            description: Some("Korean mixed rice with vegetables and beef".to_string()),
        },
        Meal {
            id: None,
            name: "Japanese Ramen".to_string(),
            ingredients: vec![
                "ramen noodles".to_string(),
                "pork belly".to_string(),
                "boiled egg".to_string(),
                "nori".to_string(),
                "bamboo shoots".to_string(),
                "spring onions".to_string(),
            ],
            category: "Japanese".to_string(),
            calories: Some(520),
            image_url: Some(
                "https://images.unsplash.com/photo-1569718212165-3a8278d5f624?w=800&fit=crop"
                    .to_string(),
            ),
            description: Some("Japanese noodle soup with pork and egg".to_string()),
        },
        Meal {
            id: None,
            name: "Vietnamese Pho".to_string(),
            ingredients: vec![
                "rice noodles".to_string(),
                "beef broth".to_string(),
                "beef slices".to_string(),
                "bean sprouts".to_string(),
                "basil".to_string(),
                "lime".to_string(),
            ],
            category: "Vietnamese".to_string(),
            calories: Some(350),
            image_url: Some(
                "https://images.unsplash.com/photo-1563245372-f21724e3856d?w=800&fit=crop"
                    .to_string(),
            ),
            description: Some("Vietnamese noodle soup with beef and herbs".to_string()),
        },
        Meal {
            id: None,
            name: "Chinese Dim Sum".to_string(),
            ingredients: vec![
                "shrimp".to_string(),
                "pork".to_string(),
                "mushrooms".to_string(),
                "bamboo".to_string(),
                "wonton wrappers".to_string(),
            ],
            category: "Chinese".to_string(),
            calories: Some(280),
            image_url: Some(
                "https://images.unsplash.com/photo-1563245372-f21724e3856d?w=800&fit=crop"
                    .to_string(),
            ),
            description: Some("Assorted Chinese dumplings and small dishes".to_string()),
        },
        // 🥗 Healthy & Salads
        Meal {
            id: None,
            name: "Grilled Chicken Salad".to_string(),
            ingredients: vec![
                "chicken breast".to_string(),
                "lettuce".to_string(),
                "tomato".to_string(),
                "cucumber".to_string(),
                "olive oil".to_string(),
                "lemon".to_string(),
            ],
            category: "Salad".to_string(),
            calories: Some(350),
            image_url: Some(
                "https://images.unsplash.com/photo-1546069901-d5bfd2cbfb1f?w=800&fit=crop"
                    .to_string(),
            ),
            description: Some("Fresh salad with grilled chicken breast".to_string()),
        },
        Meal {
            id: None,
            name: "Quinoa Bowl".to_string(),
            ingredients: vec![
                "quinoa".to_string(),
                "avocado".to_string(),
                "chickpeas".to_string(),
                "spinach".to_string(),
                "cherry tomatoes".to_string(),
                "lemon dressing".to_string(),
            ],
            category: "Healthy".to_string(),
            calories: Some(380),
            image_url: Some(
                "https://images.unsplash.com/photo-1512621776951-a57141f2eefd?w=800&fit=crop"
                    .to_string(),
            ),
            description: Some("Nutritious quinoa bowl with fresh vegetables".to_string()),
        },
        Meal {
            id: None,
            name: "Salmon with Vegetables".to_string(),
            ingredients: vec![
                "salmon fillet".to_string(),
                "broccoli".to_string(),
                "carrots".to_string(),
                "asparagus".to_string(),
                "olive oil".to_string(),
                "lemon".to_string(),
            ],
            category: "Seafood".to_string(),
            calories: Some(420),
            image_url: Some(
                "https://images.unsplash.com/photo-1467003909585-2f8a72700288?w=800&fit=crop"
                    .to_string(),
            ),
            description: Some("Baked salmon with roasted vegetables".to_string()),
        },
        // 🍕 Western Food
        Meal {
            id: None,
            name: "Pepperoni Pizza".to_string(),
            ingredients: vec![
                "pizza dough".to_string(),
                "tomato sauce".to_string(),
                "mozzarella".to_string(),
                "pepperoni".to_string(),
                "oregano".to_string(),
            ],
            category: "Italian".to_string(),
            calories: Some(680),
            image_url: Some(
                "https://images.unsplash.com/photo-1565299624946-b28f40a0ae38?w=800&fit=crop"
                    .to_string(),
            ),
            description: Some("Classic pepperoni pizza with mozzarella".to_string()),
        },
        Meal {
            id: None,
            name: "Beef Burger".to_string(),
            ingredients: vec![
                "beef patty".to_string(),
                "burger bun".to_string(),
                "cheese".to_string(),
                "lettuce".to_string(),
                "tomato".to_string(),
                "onion".to_string(),
                "special sauce".to_string(),
            ],
            category: "American".to_string(),
            calories: Some(650),
            image_url: Some(
                "https://images.unsplash.com/photo-1568901346375-23c9450c58cd?w=800&fit=crop"
                    .to_string(),
            ),
            description: Some("Juicy beef burger with cheese and vegetables".to_string()),
        },
        Meal {
            id: None,
            name: "Spaghetti Carbonara".to_string(),
            ingredients: vec![
                "spaghetti".to_string(),
                "eggs".to_string(),
                "parmesan".to_string(),
                "pancetta".to_string(),
                "black pepper".to_string(),
                "garlic".to_string(),
            ],
            category: "Italian".to_string(),
            calories: Some(550),
            image_url: Some(
                "https://images.unsplash.com/photo-1600803907087-f56d462fd26b?w=800&fit=crop"
                    .to_string(),
            ),
            description: Some("Creamy Italian pasta with eggs and pancetta".to_string()),
        },
        // 🌱 Vegan & Vegetarian
        Meal {
            id: None,
            name: "Tofu Stir Fry".to_string(),
            ingredients: vec![
                "tofu".to_string(),
                "bell peppers".to_string(),
                "broccoli".to_string(),
                "carrots".to_string(),
                "soy sauce".to_string(),
                "ginger".to_string(),
            ],
            category: "Vegan".to_string(),
            calories: Some(320),
            image_url: Some(
                "https://images.unsplash.com/photo-1546069901-ba9599a7e63c?w=800&fit=crop"
                    .to_string(),
            ),
            description: Some("Stir-fried tofu with mixed vegetables".to_string()),
        },
        Meal {
            id: None,
            name: "Vegetable Lasagna".to_string(),
            ingredients: vec![
                "lasagna noodles".to_string(),
                "spinach".to_string(),
                "ricotta".to_string(),
                "tomato sauce".to_string(),
                "zucchini".to_string(),
                "eggplant".to_string(),
            ],
            category: "Vegetarian".to_string(),
            calories: Some(380),
            image_url: Some(
                "https://images.unsplash.com/photo-1574894709920-11b28e7367e3?w=800&fit=crop"
                    .to_string(),
            ),
            description: Some("Layered pasta with vegetables and cheese".to_string()),
        },
        // 🍣 Seafood
        Meal {
            id: None,
            name: "Shrimp Scampi".to_string(),
            ingredients: vec![
                "shrimp".to_string(),
                "linguine".to_string(),
                "garlic".to_string(),
                "butter".to_string(),
                "white wine".to_string(),
                "parsley".to_string(),
            ],
            category: "Seafood".to_string(),
            calories: Some(480),
            image_url: Some(
                "https://images.unsplash.com/photo-1563379926898-05f4575a45d8?w=800&fit=crop"
                    .to_string(),
            ),
            description: Some("Garlic butter shrimp with pasta".to_string()),
        },
        Meal {
            id: None,
            name: "Fish Tacos".to_string(),
            ingredients: vec![
                "white fish".to_string(),
                "corn tortillas".to_string(),
                "cabbage slaw".to_string(),
                "lime crema".to_string(),
                "avocado".to_string(),
                "cilantro".to_string(),
            ],
            category: "Mexican".to_string(),
            calories: Some(350),
            image_url: Some(
                "https://images.unsplash.com/photo-1565299585323-38d6b0865b47?w=800&fit=crop"
                    .to_string(),
            ),
            description: Some("Crispy fish tacos with fresh toppings".to_string()),
        },
        // 🥘 Soups & Stews
        Meal {
            id: None,
            name: "Tomato Basil Soup".to_string(),
            ingredients: vec![
                "tomatoes".to_string(),
                "basil".to_string(),
                "onion".to_string(),
                "garlic".to_string(),
                "cream".to_string(),
            ],
            category: "Soup".to_string(),
            calories: Some(180),
            image_url: Some(
                "https://images.unsplash.com/photo-1547592166-23ac45744acd?w=800&fit=crop"
                    .to_string(),
            ),
            description: Some("Creamy tomato soup with fresh basil".to_string()),
        },
        Meal {
            id: None,
            name: "Beef Stew".to_string(),
            ingredients: vec![
                "beef chuck".to_string(),
                "potatoes".to_string(),
                "carrots".to_string(),
                "onion".to_string(),
                "red wine".to_string(),
                "beef broth".to_string(),
            ],
            category: "Stew".to_string(),
            calories: Some(320),
            image_url: Some(
                "https://images.unsplash.com/photo-1505252585461-04db1eb84625?w=800&fit=crop"
                    .to_string(),
            ),
            description: Some("Hearty beef stew with vegetables".to_string()),
        },
        // 🥪 Sandwiches
        Meal {
            id: None,
            name: "Club Sandwich".to_string(),
            ingredients: vec![
                "turkey".to_string(),
                "bacon".to_string(),
                "lettuce".to_string(),
                "tomato".to_string(),
                "mayonnaise".to_string(),
                "bread".to_string(),
            ],
            category: "Sandwich".to_string(),
            calories: Some(420),
            image_url: Some(
                "https://images.unsplash.com/photo-1481070555726-e2fe8357725c?w=800&fit=crop"
                    .to_string(),
            ),
            description: Some("Triple-decker turkey and bacon sandwich".to_string()),
        },
        Meal {
            id: None,
            name: "Chicken Caesar Wrap".to_string(),
            ingredients: vec![
                "chicken".to_string(),
                "romaine lettuce".to_string(),
                "parmesan".to_string(),
                "caesar dressing".to_string(),
                "croutons".to_string(),
                "tortilla".to_string(),
            ],
            category: "Wrap".to_string(),
            calories: Some(380),
            image_url: Some(
                "https://images.unsplash.com/photo-1563379926898-05f4575a45d8?w=800&fit=crop"
                    .to_string(),
            ),
            description: Some("Caesar salad wrap with grilled chicken".to_string()),
        },
        // 🥞 Breakfast
        Meal {
            id: None,
            name: "Pancakes".to_string(),
            ingredients: vec![
                "flour".to_string(),
                "eggs".to_string(),
                "milk".to_string(),
                "butter".to_string(),
                "maple syrup".to_string(),
            ],
            category: "Breakfast".to_string(),
            calories: Some(380),
            image_url: Some(
                "https://images.unsplash.com/photo-1567620905732-2d1ec7ab7445?w=800&fit=crop"
                    .to_string(),
            ),
            description: Some("Fluffy pancakes with maple syrup".to_string()),
        },
        Meal {
            id: None,
            name: "Avocado Toast".to_string(),
            ingredients: vec![
                "sourdough bread".to_string(),
                "avocado".to_string(),
                "eggs".to_string(),
                "everything seasoning".to_string(),
                "lemon".to_string(),
            ],
            category: "Breakfast".to_string(),
            calories: Some(320),
            image_url: Some(
                "https://images.unsplash.com/photo-1525351484163-7529414344d8?w=800&fit=crop"
                    .to_string(),
            ),
            description: Some("Avocado toast with poached eggs".to_string()),
        },
        // 🍛 Rice Dishes
        Meal {
            id: None,
            name: "Spanish Paella".to_string(),
            ingredients: vec![
                "rice".to_string(),
                "shrimp".to_string(),
                "mussels".to_string(),
                "clams".to_string(),
                "chorizo".to_string(),
                "saffron".to_string(),
            ],
            category: "Spanish".to_string(),
            calories: Some(520),
            image_url: Some(
                "https://images.unsplash.com/photo-1565299624946-b28f40a0ae38?w=800&fit=crop"
                    .to_string(),
            ),
            description: Some("Spanish seafood rice dish with saffron".to_string()),
        },
        Meal {
            id: None,
            name: "Indian Biryani".to_string(),
            ingredients: vec![
                "basmati rice".to_string(),
                "chicken".to_string(),
                "yogurt".to_string(),
                "saffron".to_string(),
                "onions".to_string(),
                "spices".to_string(),
            ],
            category: "Indian".to_string(),
            calories: Some(460),
            image_url: Some(
                "https://images.unsplash.com/photo-1565557623262-b51c2513a641?w=800&fit=crop"
                    .to_string(),
            ),
            description: Some("Fragrant Indian rice dish with chicken".to_string()),
        },
        // Add 20+ more meals here...
        Meal {
            id: None,
            name: "Mie Goreng".to_string(),
            ingredients: vec!["mie".to_string(), "ayam".to_string(), "sayur".to_string()],
            category: "Indonesian".to_string(),
            calories: Some(380),
            image_url: Some(
                "https://images.unsplash.com/photo-1563245372-f21724e3856d?w=800&fit=crop"
                    .to_string(),
            ),
            description: Some("Indonesian fried noodles".to_string()),
        },
        Meal {
            id: None,
            name: "Bakso".to_string(),
            ingredients: vec![
                "daging sapi".to_string(),
                "tahu".to_string(),
                "mie".to_string(),
            ],
            category: "Indonesian".to_string(),
            calories: Some(280),
            image_url: Some(
                "https://images.unsplash.com/photo-1547592166-23ac45744acd?w=800&fit=crop"
                    .to_string(),
            ),
            description: Some("Indonesian meatball soup".to_string()),
        },
        Meal {
            id: None,
            name: "Martabak".to_string(),
            ingredients: vec![
                "telur".to_string(),
                "daging".to_string(),
                "bawang".to_string(),
            ],
            category: "Indonesian".to_string(),
            calories: Some(420),
            image_url: Some(
                "https://images.unsplash.com/photo-1484980972926-edee96e0960d?w=800&fit=crop"
                    .to_string(),
            ),
            description: Some("Indonesian stuffed pancake".to_string()),
        },
    ]
}
