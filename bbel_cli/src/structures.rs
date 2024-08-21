pub fn get_structure(structure_type: &str, root: Option<&str>) -> Vec<String> {
    let root = root.unwrap_or("src");
    match structure_type {
        "api" => vec![
            // Src
               format!("{}/controllers/", &root),
               format!("{}/models/", &root),
                // Routes
                   format!("{}/routes/GET/", &root),
                   format!("{}/routes/POST/", &root),
                   format!("{}/routes/PUT/", &root),
                   format!("{}/routes/DELETE/", &root),
               format!("{}/utils/", &root),
               format!("{}/middlewares/", &root),
               format!("{}/config/", &root),
               format!("{}/index.js$", &root),
               format!("{}/router.js$", &root),
               format!("{}/database.js$", &root),
        ],
        "bot" => vec![
            //Src
               format!("{}/events/", &root),
                // Commands
                   format!("{}/commands/genereal/", &root),
                   format!("{}/commands/moderation/", &root),
                   format!("{}/commands/admin/", &root),
               format!("{}/utils/", &root),
               format!("{}/config/", &root),
               format!("{}/middlewares/", &root),
               format!("{}/bot.js$", &root),
               format!("{}/index.js$", &root),
            String::from(".env$"),
            String::from("README.md$"),
        ],
        "website" => vec![
            //Src
                format!("{}/controllers/", &root),
                format!("{}/models/", &root),
                // Views
                    format!("{}/views/pages/", &root),
                    format!("{}/views/partials/", &root),
                format!("{}/utils/", &root),
                format!("{}/middlewares/", &root),
                format!("{}/config/", &root),
                // Routes
                    format!("{}/routes/GET", &root),
                    format!("{}/routes/POST", &root),
                    format!("{}/routes/PUT", &root),
                    format!("{}/routes/DELETE", &root),
                // Public
                    format!("{}/public/css/", &root),
                    format!("{}/public/js/", &root),
                    format!("{}/public/img/", &root),
                format!("{}/index.js$", &root),
                format!("{}/router.js$", &root),
                format!("{}/app.js$", &root),
        ],
        _ => vec![],

    }
}