use podchecker::Router;

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    let podcasts = read_podcasts_from_xml("https://workingdraft.de/feed/").await?;
    let app_state = Arc::new(podcasts);
    let router = Router::new()
        .route("/", get(podchecker::root()))
        .route("/:id", get(podchecker::podcast()))
        .with_state(app_state);

    Ok(router.into())
}
