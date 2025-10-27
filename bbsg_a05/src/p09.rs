use axum::routing::get;
use std::error::Error;

pub async fn web1() -> Result<(), Box<dyn Error>> {
    println!("web1");
    //let x: axum::routing::MethodRouter = get(crate::web::sbb01::sbb01);
    let app = axum::Router::new()
        // field
        .route("/sbb01", get(crate::web::sbb01::page))
        .route("/sbb02", get(crate::web::sbb02::page))
        .route("/sbb03", get(crate::web::sbb03::page))
        .route("/sbb04", get(crate::web::sbb04::page))
        .route("/sbb05", get(crate::web::sbb05::page))
        .route("/sbb06", get(crate::web::sbb06::page))
        .route("/sbb07", get(crate::web::sbb07::page))
        .route("/sbb08", get(crate::web::sbb08::page))
        .route("/sbb09", get(crate::web::sbb09::page))
        .route("/sbb10", get(crate::web::sbb10::page))
        .route("/sbb11", get(crate::web::sbb11::page))
        .route("/sbb12", get(crate::web::sbb12::page))
        .route("/sbb13", get(crate::web::sbb13::page))
        // sub
        .route("/sba01", get(crate::sba01::sba01))
        .route("/sba02", get(crate::sba02::sba02))
        .route("/sba03", get(crate::sba03::sba03))
        // sub
        .route("/sb01", get(crate::sb01::sb01))
        .route("/sb02", get(crate::sb02::sb02))
        .route("/sb03", get(crate::sb03::sb03))
        .route("/sb04", get(crate::sb04::sb04))
        .route("/sb05", get(crate::sb05::sb05))
        // trans
        .route("/tr01", get(crate::tr01::tr01))
        .route("/tr02", get(crate::tr02::tr02))
        .route("/tr03", get(crate::tr03::tr03))
        .route("/tr04", get(crate::tr04::tr04))
        .route("/tr05", get(crate::tr05::tr05))
        .route("/tr06", get(crate::tr06::tr06))
        // ___
        .route("/a01", get(crate::a01::a01))
        .route("/a02", get(crate::a02::a02))
        .route("/a03", get(crate::a03::a03))
        .route("/q02", get(crate::web::q02::q02))
        .route("/p02", get(crate::web::p02::p02))
        .route("/p03", get(crate::web::p03::p03))
        .route("/p04", get(crate::web::p04::p04))
        .route("/p05", get(crate::web::p05::p05))
        .route("/p06", get(crate::web::p06::p06))
        .route("/p07", get(crate::web::p07::p07))
        .route("/p08", get(crate::web::p08::p08))
        .route("/m01", get(crate::m01::m01))
        .route("/m02", get(crate::m02::m02))
        .route("/", get(crate::sba01::sba01));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
