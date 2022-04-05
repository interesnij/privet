use actix_web::{
    HttpRequest,
    Responder,
    HttpResponse,
    web
};
use serde::Deserialize;
use tera::Context;
use crate::utils::{get_template_2, establish_connection, TEMPLATES};
use crate::NewUser;
use crate::diesel::RunQueryDsl;


#[derive(Debug, Deserialize)]
pub struct SParams {
    pub q: String,
}
pub async fn index(req: HttpRequest) -> impl Responder {
    use crate::diesel::QueryDsl;
    use crate::diesel::ExpressionMethods;
    use crate::schema::works::dsl::*;
    use crate::schema::services::dsl::*;
    use crate::schema::blogs::dsl::*;
    use crate::schema::stores::dsl::*;
    use crate::schema::wikis::dsl::*;

    use crate::models::{Work,Service,Wiki,Blog,Store};

    let _connection = establish_connection();
    let _last_works :Vec<Work> = works.filter(is_work_active.eq(true)).order(work_created.desc()).limit(3).load(&_connection).expect(".");
    let _last_services :Vec<Service> = services.filter(is_service_active.eq(true)).order(service_created.desc()).limit(3).load(&_connection).expect(".");
    let _last_wikis :Vec<Wiki> = wikis.filter(is_wiki_active.eq(true)).order(wiki_created.desc()).limit(3).load(&_connection).expect(".");
    let _last_blogs :Vec<Blog> = blogs.filter(is_blog_active.eq(true)).order(blog_created.desc()).order(blog_created.desc()).limit(3).load(&_connection).expect(".");
    let _last_stores :Vec<Store> = stores.filter(is_store_active.eq(true)).order(store_created.desc()).limit(3).load(&_connection).expect(".");

    let mut data = Context::new();

    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("last_works", &_last_works);
    data.insert("last_services", &_last_services);
    data.insert("last_wikis", &_last_wikis);
    data.insert("last_blogs", &_last_blogs);
    data.insert("last_stores", &_last_stores);
    data.insert("is_admin", &_is_admin);

    let _template = _type + &"main/mainpage.html".to_string();
    let _rendered = TEMPLATES.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}
pub async fn about(req: HttpRequest) -> impl Responder {
    let mut data = Context::new();
    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("is_admin", &_is_admin);

    let _template = _type + &"about.html".to_string();
    let _rendered = TEMPLATES.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}
pub async fn signup(req: HttpRequest) -> impl Responder {
    let mut data = Context::new();
    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("is_admin", &_is_admin);

    let _template = _type + &"signup.html".to_string();
    let rendered = TEMPLATES.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(rendered)
}
pub async fn process_signup(data: web::Form<NewUser>) -> impl Responder {
    use crate::schema::users;
    use crate::models::User;

    let connection = establish_connection();

    diesel::insert_into(users::table)
        .values(&*data)
        .get_result::<User>(&connection)
        .expect("Error registering user.");

    println!("{:?}", data);
    HttpResponse::Ok().body(format!("Successfully saved user: {}", data.username))
}

//use actix_multipart::Multipart;
pub async fn create_feedback(mut payload: actix_multipart::Multipart) -> impl Responder {
    use crate::schema::feedbacks;
    use std::borrow::BorrowMut;
    use crate::models::{Feedback,NewFeedback};
    use crate::utils::feedback_form;

    let _connection = establish_connection();
    let form = feedback_form(payload.borrow_mut()).await;
    let new_feedback = NewFeedback {
        username: form.username.clone(),
        email: form.email.clone(),
        message: form.message.clone()
    };
    let _new_feedback = diesel::insert_into(feedbacks::table)
        .values(&new_feedback)
        .get_result::<Feedback>(&_connection)
        .expect("E.");
    return HttpResponse::Ok();
}

pub async fn feedback_list_page(req: HttpRequest) -> impl Responder {
    use crate::schema::feedbacks::dsl::feedbacks;
    use crate::models::Feedback;

    let _connection = establish_connection();
    let _feedbacks = feedbacks.load::<Feedback>(&_connection).expect("E");

    let mut data = Context::new();
    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("is_admin", &_is_admin);
    data.insert("feedback_list", &_feedbacks);

    let _template = _type + &"main/feedback_list.html".to_string();
    let _rendered = TEMPLATES.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

pub async fn serve_list_page(req: HttpRequest) -> impl Responder {
    use diesel::prelude::*;
    use crate::models::{Serve, TechCategories, ServeCategories};
    use crate::schema;
    use crate::schema::{
        serve::dsl::serve,
        serve_categories::dsl::serve_categories,
        tech_categories::dsl::tech_categories,
    };

    let _connection = establish_connection();
    let mut data = Context::new();

    let all_tech_categories :Vec<TechCategories> = tech_categories
        .order(schema::tech_categories::tech_position.asc())
        .load(&_connection)
        .expect("E.");
    let mut _count: i32 = 0;
    for _cat in all_tech_categories.iter() {
        _count += 1;
        let mut _let_int : String = _count.to_string().parse().unwrap();
        let _let_serve_categories: String = "serve_categories".to_string() + &_let_int;
        let __serve_categories :Vec<ServeCategories> = serve_categories
            .filter(schema::serve_categories::tech_categories.eq(_cat.id))
            .order(schema::serve_categories::serve_position.asc())
            .load(&_connection)
            .expect("E.");
        data.insert(&_let_serve_categories, &__serve_categories);

        let mut _serve_count: i32 = 0;
        for __cat in __serve_categories.iter() {
            _serve_count += 1;
            let mut _serve_int : String = _serve_count.to_string().parse().unwrap();
            let _serve_int_dooble = "_".to_string() + &_let_int;
            let _let_serves: String = _serve_int_dooble.to_owned() + &"serves".to_string() + &_serve_int;
            let __serves :Vec<Serve> = serve.filter(schema::serve::serve_categories.eq(__cat.id)).load(&_connection).expect("E.");
            data.insert(&_let_serves, &__serves);
        }
    };
    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("tech_categories", &all_tech_categories);
    data.insert("is_admin", &_is_admin);

    let _template = _type + &"main/serve_list.html".to_string();
    let _rendered = TEMPLATES.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

#[derive(Debug, Deserialize)]
pub struct LoadParams {
    pub _object_type: String,
    pub _owner_type: String,
    pub _object_pk: i32,
    pub _owner_pk: i32,
}
pub async fn get_load_page(req: HttpRequest) -> impl Responder {
    use crate::schema;
    use diesel::prelude::*;

    let mut _object_type : String = "".to_string();
    let mut _owner_type : String = "".to_string();
    let mut _object_pk : i32 = 0;
    let mut _owner_pk : i32 = 0;

    let _connection = establish_connection();
    let params = web::Query::<LoadParams>::from_query(&req.query_string());
    if params.is_ok() {
        let wrap = params.unwrap();
        if wrap._object_type != "".to_string() {
            _object_type = wrap._object_type.clone();
        }
        if wrap._owner_type != "".to_string() {
            _owner_type = wrap._owner_type.clone();
        }
        if wrap._object_pk != 0 {
            _object_pk = wrap._object_pk.clone();
        }
        if wrap._owner_pk != 0 {
            _owner_pk = wrap._owner_pk.clone();
        }
    }

    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    let mut data = Context::new();
    let mut _template = "".to_string();

    if _object_type == "serve_category".to_string() {
        // тип запрашиваемого объекта "serve_category".
        // получаем объект и записываем в контекст, получаем строку шаблона
        use crate::models::ServeCategories;
        use crate::schema::serve_categories::dsl::*;

        let _serve_category = serve_categories
            .filter(schema::serve_categories::id.eq(&_object_pk))
            .load::<ServeCategories>(&_connection)
            .expect("E");
        data.insert("object", &_serve_category[0]);
        data.insert("object_type", &"serve_category".to_string());
        _template = _type + &"load/serve_category.html".to_string();
    } else if _object_type == "serve".to_string() {
        // тип запрашиваемого объекта - опция.
        // получаем объект и записываем в контекст, получаем строку шаблона
        use crate::models::Serve;
        use crate::schema::serve::dsl::serve;

        let _serve = serve
            .filter(schema::serve::id.eq(&_object_pk))
            .load::<Serve>(&_connection)
            .expect("E");
        data.insert("object", &_serve[0]);
        data.insert("object_type", &"serve".to_string());
        if _owner_type == "service".to_string() {
            // тип объекта-владельца - услуга.
            // получаем объект и записываем в контекст, получаем строку шаблона
            use crate::models::{Service, ServeItems};
            use crate::schema::services::dsl::services;
            let _service_id : i32 = _owner_pk;
            let _service = services
                .filter(schema::services::id.eq(&_service_id))
                .load::<Service>(&_connection)
                .expect("E");
            data.insert("service", &_service[0]);
            data.insert("owner_type", &"service".to_string());
        }
        _template = _type + &"load/serve.html".to_string();
    }
    data.insert("is_admin", &_is_admin);
    let _rendered = TEMPLATES.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}
