pub fn get_api_structure () -> Vec<String> {
    return Vec::from([
        String::from("_src/controllers/"),
        String::from("_src/models/"),
        String::from("_src/utils/"),
            String::from("_src/routes/GET/"),
            String::from("_src/routes/POST/"),
            String::from("_src/routes/PUT/"),
            String::from("_src/routes/DELETE/"),
        String::from("_src/config/"),
        String::from("_src/middlewares/"),
        String::from("_src/index.js"),
        String::from("_src/database.js"),
        String::from("_src/router.js"),
    ])
}