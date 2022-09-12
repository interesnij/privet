use actix_web::{
    HttpRequest,
    HttpResponse,
    web,
    error::InternalError,
    http::StatusCode,
    Responder,
};
use crate::models::User;
use std::borrow::BorrowMut;
use crate::diesel::{
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
};
use crate::utils::{
    category_form,
    help_form,
    establish_connection,
    is_signed_in,
    get_request_user_data,
    get_first_load_page,
};
use crate::schema;
use crate::models::{
    HelpItemCategorie,
    NewHelpItemCategorie,
    HelpItem,
    NewHelpItem,
};
use actix_session::Session;
use actix_multipart::Multipart;
use sailfish::TemplateOnce;


pub fn help_routes(config: &mut web::ServiceConfig) {
    config.service(web::resource("/help/create_categories/")
        .route(web::get().to(create_categories_page))
        .route(web::post().to(create_categories))
    );
    config.service(web::resource("/help/edit_category/{id}/")
        .route(web::get().to(edit_category_page))
        .route(web::post().to(edit_category))
    );
    config.service(web::resource("/help/create_item/")
        .route(web::get().to(create_item_page))
        .route(web::post().to(create_item))
    );
    config.service(web::resource("/help/edit_item/{id}/")
        .route(web::get().to(edit_item_page))
        .route(web::post().to(edit_item))
    );
    config.route("/help/delete_item/{id}/", web::get().to(delete_item));
    config.route("/help/delete_category/{id}/", web::get().to(delete_category));
    config.service(web::resource("/help/{id}/")
        .route(web::get().to(category_page))
    );
}

pub async fn create_categories_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    use crate::utils::get_device_and_ajax;

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Создание категории помощи".to_string(),
            "вебсервисы.рф: Создание категории помощи".to_string(),
            "/help/create_categories/".to_string(),
            "/static/images/dark/store.jpg".to_string(),
        ).await
    }
    else if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm != 60 {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
        else {
            use schema::help_item_categories::dsl::help_item_categories;

            let _connection = establish_connection();
            let _categories = help_item_categories.load::<HelpItemCategorie>(&_connection).expect("E");

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/help/create_categories.stpl")]
                struct Template {
                    request_user: User,
                    help_cats:    Vec<HelpItemCategorie>,
                    is_ajax:      i32,
                }
                let body = Template {
                    request_user: _request_user,
                    help_cats:    _categories,
                    is_ajax:      is_ajax,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/help/create_categories.stpl")]
                struct Template {
                    help_cats:    Vec<HelpItemCategorie>,
                    is_ajax:      i32,
                }
                let body = Template {
                    help_cats:    _categories,
                    is_ajax:      is_ajax,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
    }
}

pub async fn create_item_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    use crate::utils::get_device_and_ajax;

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Создание объекта помощи".to_string(),
            "вебсервисы.рф: Создание объекта помощи".to_string(),
            "/help/create_item/".to_string(),
            "/static/images/dark/store.jpg".to_string(),
        ).await
    }
    else if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm != 60 {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
        else {
            use crate::schema::help_item_categories::dsl::help_item_categories;

            let _connection = establish_connection();
            let _help_categories = help_item_categories
                .load::<HelpItemCategorie>(&_connection)
                .expect("E");

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/help/create_item.stpl")]
                struct Template {
                    help_cats:    Vec<HelpItemCategorie>,
                    request_user: User,
                    is_ajax:      i32,
                }
                let body = Template {
                    help_cats:    _help_categories,
                    request_user: _request_user,
                    is_ajax:      is_ajax,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/help/create_item.stpl")]
                struct Template {
                    help_cats: Vec<HelpItemCategorie>,
                    is_ajax:   i32,
                }
                let body = Template {
                    help_cats: _help_categories,
                    is_ajax:   is_ajax,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
    }
}

pub async fn edit_category_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::utils::get_device_and_ajax;
    use crate::schema::help_item_categories::dsl::help_item_categories;

    let _cat_id: i32 = *_id;
    let _connection = establish_connection();
    let _categorys = help_item_categories.filter(schema::help_item_categories::id.eq(&_cat_id)).load::<HelpItemCategorie>(&_connection).expect("E");
    let _category = _categorys.into_iter().nth(0).unwrap();
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Изменение категории помощи ".to_string() + &_category.title,
            "вебсервисы.рф: Изменение категории помощи ".to_string() + &_category.title,
            "/help/edit_category/".to_string() + &_category.id.to_string() + &"/".to_string(),
            "".to_string(),
        ).await
    }
    else if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = get_request_user_data(&session);
        let _help_categories = help_item_categories.load::<HelpItemCategorie>(&_connection).expect("E");

        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/help/edit_category.stpl")]
            struct Template {
                request_user: User,
                help_cats:    Vec<HelpItemCategorie>,
                category:     HelpItemCategorie,
                is_ajax:      i32,
            }
            let body = Template {
                request_user: _request_user,
                help_cats:    _help_categories,
                category:     _category,
                is_ajax:      is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/help/edit_category.stpl")]
            struct Template {
                help_cats:    Vec<HelpItemCategorie>,
                category:     HelpItemCategorie,
                is_ajax:      i32,
            }
            let body = Template {
                help_cats:    _help_categories,
                category:     _category,
                is_ajax:      is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn edit_item_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::utils::get_device_and_ajax;
    use crate::schema::help_items::dsl::help_items;

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let _connection = establish_connection();
    let _item_id: i32 = *_id;
    let _items = help_items.filter(schema::help_items::id.eq(&_item_id)).load::<HelpItem>(&_connection).expect("E");
    let _item = _items.into_iter().nth(0).unwrap();

    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Изменение элемента помощи ".to_string() + &_item.title,
            "вебсервисы.рф: Изменение элемента помощи ".to_string() + &_item.title,
            "/help/edit_item/".to_string() + &_item.id.to_string() + &"/".to_string(),
            "".to_string(),
        ).await
    }
    else if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        use crate::schema::help_item_categories::dsl::help_item_categories;

        let _request_user = get_request_user_data(&session);
        let _help_cats = help_item_categories
            .load::<HelpItemCategorie>(&_connection)
            .expect("E");

        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/help/edit_item.stpl")]
            struct Template {
                request_user: User,
                help_cats:    Vec<HelpItemCategorie>,
                object:       HelpItem,
                is_ajax:      i32,
            }
            let body = Template {
                request_user: _request_user,
                help_cats:    _help_cats,
                object:       _item,
                is_ajax:      is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/help/edit_item.stpl")]
            struct Template {
                help_cats: Vec<HelpItemCategorie>,
                object:    HelpItem,
                is_ajax:   i32,
            }
            let body = Template {
                help_cats: _help_cats,
                object:    _item,
                is_ajax:   is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn create_categories(session: Session, mut payload: Multipart) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            let _connection = establish_connection();
            let form = category_form(payload.borrow_mut(), _request_user.id).await;
            let new_cat = NewHelpItemCategorie {
                title: form.name.clone(),
                view:  0,
                height:  0.0,
                seconds:  0,
                position: form.position as i32,
            };
            let _new_help = diesel::insert_into(schema::help_item_categories::table)
                .values(&new_cat)
                .get_result::<HelpItemCategorie>(&_connection)
                .expect("E.");
        }
    }
    return HttpResponse::Ok();
}

pub async fn edit_category(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    use crate::schema::help_item_categories::dsl::help_item_categories;

    let _connection = establish_connection();
    let _cat_id: i32 = *_id;
    let _categorys = help_item_categories.filter(schema::help_item_categories::id.eq(_cat_id)).load::<HelpItemCategorie>(&_connection).expect("E");
    let _category = _categorys.into_iter().nth(0).unwrap();

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            let form = category_form(payload.borrow_mut(), _request_user.id).await;
            let new_cat = NewHelpItemCategorie {
                title:    form.name.clone(),
                view:     _category.view,
                height:   _category.height,
                seconds:  _category.seconds,
                position: form.position as i32,
            };
            diesel::update(&_category)
                .set(new_cat)
                .get_result::<HelpItemCategorie>(&_connection)
                .expect("E");
        }
    }
    return HttpResponse::Ok();
}

pub async fn create_item(session: Session, mut payload: Multipart) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            let _connection = establish_connection();
            let form = help_form(payload.borrow_mut()).await;

            let _new_item = NewHelpItem {
                category_id: form.category_id,
                title:       form.title.clone(),
                content:     form.description.clone(),
                position:    form.position,
            };

            let _item = diesel::insert_into(schema::help_items::table)
                .values(&_new_item)
                .get_result::<HelpItem>(&_connection)
                .expect("E.");
        }
    }
    return HttpResponse::Ok();
}

pub async fn edit_item(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    use crate::schema::help_items::dsl::help_items;

    let _item_id: i32 = *_id;
    let _connection = establish_connection();
    let _items = help_items
        .filter(schema::help_items::id.eq(&_item_id))
        .load::<HelpItem>(&_connection)
        .expect("E");
    let _item = _items.into_iter().nth(0).unwrap();

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            let form = help_form(payload.borrow_mut()).await;
            let _new_item = NewHelpItem {
                category_id: form.category_id,
                title:       form.title.clone(),
                content:     form.description.clone(),
                position:    form.position,
            };

            diesel::update(&_item)
                .set(_new_item)
                .get_result::<HelpItem>(&_connection)
                .expect("E");
        }
    }
    return HttpResponse::Ok();
}


pub async fn delete_item(session: Session, _id: web::Path<i32>) -> impl Responder {
    use crate::schema::help_items::dsl::help_items;

    let _connection = establish_connection();
    let _item_id: i32 = *_id;
    let _items = help_items.filter(schema::help_items::id.eq(_item_id)).load::<HelpItem>(&_connection).expect("E");
    let _item = _items.into_iter().nth(0).unwrap();

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            diesel::delete(&_item).execute(&_connection).expect("E");
        }
    }
    HttpResponse::Ok()
}

pub async fn delete_category(session: Session, _id: web::Path<i32>) -> impl Responder {
    use crate::schema::help_item_categories::dsl::help_item_categories;

    let _connection = establish_connection();
    let _cat_id: i32 = *_id;
    let _categorys = help_item_categories.filter(schema::help_item_categories::id.eq(_cat_id)).load::<HelpItemCategorie>(&_connection).expect("E");
    let _category = _categorys.into_iter().nth(0).unwrap();

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            diesel::delete(help_item_categories.filter(schema::help_item_categories::id.eq(_cat_id))).execute(&_connection).expect("E");
        }
    }
    HttpResponse::Ok()
}

pub async fn category_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::schema::help_item_categories::dsl::help_item_categories;
    use crate::utils::get_device_and_ajax;

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let _cat_id: i32 = *_id;
    let _connection = establish_connection();

    let all_categories = help_item_categories
        .load::<HelpItemCategorie>(&_connection)
        .expect("E");

    let _categorys = help_item_categories
        .filter(schema::help_item_categories::id.eq(_cat_id))
        .load::<HelpItemCategorie>(&_connection)
        .expect("E");
    let _category = _categorys.into_iter().nth(0).unwrap();

    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Категория помощи ".to_string() + &_category.title,
            "вебсервисы.рф: Категория помощи ".to_string() + &_category.title,
            "/help/".to_string() + &_category.id.to_string() + &"/".to_string(),
            "/static/images/dark/store.jpg".to_string(),
        ).await
    }
    else {
        let object_list = _category.get_list();

        if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/help/category.stpl")]
                struct Template {
                    request_user:     User,
                    category:         HelpItemCategorie,
                    help_cats:        Vec<HelpItemCategorie>,
                    object_list:      Vec<HelpItem>,
                    is_ajax:          i32,
                }
                let body = Template {
                    request_user:     _request_user,
                    category:         _category,
                    help_cats:        all_categories,
                    object_list:      object_list,
                    is_ajax:          is_ajax,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/help/category.stpl")]
                struct Template {
                    category:         HelpItemCategorie,
                    help_cats:        Vec<HelpItemCategorie>,
                    object_list:      Vec<HelpItem>,
                    is_ajax:          i32,
                }
                let body = Template {
                    category:         _category,
                    help_cats:        all_categories,
                    object_list:      object_list,
                    is_ajax:          is_ajax,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
        else {
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/help/anon_category.stpl")]
                struct Template {
                    category:         HelpItemCategorie,
                    help_cats:        Vec<HelpItemCategorie>,
                    object_list:      Vec<HelpItem>,
                    is_ajax:          i32,
                }
                let body = Template {
                    category:         _category,
                    help_cats:        all_categories,
                    object_list:      object_list,
                    is_ajax:          is_ajax,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/help/anon_category.stpl")]
                struct Template {
                    category:         HelpItemCategorie,
                    help_cats:        Vec<HelpItemCategorie>,
                    object_list:      Vec<HelpItem>,
                    is_ajax:          i32,
                }
                let body = Template {
                    category:         _category,
                    help_cats:        all_categories,
                    object_list:      object_list,
                    is_ajax:          is_ajax,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
    }
}
