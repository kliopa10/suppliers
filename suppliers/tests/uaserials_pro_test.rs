use suppliers::{get_supplier, AllContentSuppliers, models::ContentSupplier};


#[tokio::test]
async fn should_load_channel() {
    let sup = get_supplier(suppliers::uaserials_pro::NAME).unwrap();
    let res = AllContentSuppliers::load_channel(&sup, "Серіали", 2).await.unwrap();
    println!("{res:#?}");
    assert_eq!(true, res.len() > 0)
}

#[tokio::test]
async fn should_search() {
    let sup = get_supplier(suppliers::uaserials_pro::NAME).unwrap();
    let res = AllContentSuppliers::search(&sup, "Термінатор", vec![]).await.unwrap();
    println!("{res:#?}");
    assert_eq!(true, res.len() > 0)
}

#[tokio::test]
async fn should_load_content_details() {
    let sup = get_supplier(suppliers::uaserials_pro::NAME).unwrap();
    let res = AllContentSuppliers::get_content_details(
        &sup, 
        "8831-gotel-kokayin"
    ).await.unwrap();
    println!("{res:#?}");
}

#[tokio::test]
async fn should_load_media_items() {
    let sup = get_supplier(suppliers::uaserials_pro::NAME).unwrap();
    let res = AllContentSuppliers::load_media_items(
        &sup, 
        "8831-gotel-kokayin",
        vec![String::from("https://hdvbua.pro/embed/8831")]
    ).await.unwrap();
    println!("{res:#?}");
}