use crate::schema;
use crate::diesel::{
    Queryable,
    Insertable,
    BelongingToDsl,
    QueryDsl,
    RunQueryDsl,
    ExpressionMethods,
};
use serde::{Serialize, Deserialize,};
use crate::models::User;
use crate::schema::{
    store_categories,
    stores,
    store_category,
    store_images,
    store_videos,
};
use crate::utils::establish_connection;


#[derive(Debug, Serialize, Identifiable, Queryable, Associations)]
#[table_name="store_categories"]
pub struct StoreCategories {
    pub id:          i32,
    pub name:        String,
    pub description: Option<String>,
    pub position:    i32,
    pub image:       Option<String>,
    pub count:       i32,
}
impl StoreCategories {
    pub fn get_stores_list(&self, page: i32, limit: i32) -> (Vec<Store>, i32) {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: Vec<Store>;

        if page > 1 {
            have_next = page * limit + 1;
            object_list = self.get_stores(limit.into(), have_next.into());
        }
        else {
            have_next = limit + 1;
            object_list = self.get_stores(limit.into(), 0);
        }
        if self.get_stores(1, have_next.into()).len() > 0 {
            next_page_number = page + 1;
        }

        return (object_list, next_page_number);
    }
    pub fn get_stores(&self, limit: i64, offset: i64) -> Vec<Store> {
        use crate::schema::stores::dsl::stores;

        let _connection = establish_connection();
        let ids = StoreCategory::belonging_to(self)
            .select(schema::store_category::store_id);
        return stores
            .filter(schema::stores::id.eq_any(ids))
            .filter(schema::stores::is_active.eq(true))
            .order(schema::stores::created.desc())
            .limit(limit)
            .offset(offset)
            .load::<Store>(&_connection)
            .expect("E.");
    }
}

#[derive(Insertable)]
#[table_name="store_categories"]
pub struct NewStoreCategories {
    pub name:        String,
    pub description: Option<String>,
    pub position:    i32,
    pub image:       Option<String>,
    pub count:       i32,
}

#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="store_categories"]
pub struct EditStoreCategories {
    pub name:        String,
    pub description: Option<String>,
    pub position:    i32,
    pub image:       Option<String>,
    pub count:       i32,
}

#[derive(Debug, Serialize, Queryable, Identifiable, Associations)]
#[belongs_to(User)]
pub struct Store {
    pub id:           i32,
    pub title:        String,
    pub description:  Option<String>,
    pub content:      Option<String>,
    pub link:         Option<String>,
    pub image:        Option<String>,
    pub is_active:    bool,
    pub price:        Option<i32>,
    pub price_acc:    Option<i32>,
    pub social_price: Option<i32>,
    pub user_id:      i32,
    pub created:      chrono::NaiveDateTime,
}

#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="stores"]
pub struct EditStore {
    pub title:        String,
    pub description:  Option<String>,
    pub link:         Option<String>,
    pub image:        Option<String>,
    pub is_active:    bool,
    pub price:        Option<i32>,
    pub price_acc:    Option<i32>,
    pub social_price: Option<i32>,
}

#[derive(Serialize, Insertable)]
#[table_name="stores"]
pub struct NewStore {
    pub title:        String,
    pub description:  Option<String>,
    pub link:         Option<String>,
    pub image:        Option<String>,
    pub is_active:    bool,
    pub price:        Option<i32>,
    pub price_acc:    Option<i32>,
    pub social_price: Option<i32>,
    pub user_id:      i32,
    pub created:      chrono::NaiveDateTime,
}

impl NewStore {
    pub fn from_store_form (
        title: String,
        description: String,
        link: String,
        image: String,
        is_active: bool,
        price: Option<i32>,
        price_acc: Option<i32>,
        social_price: Option<i32>,
        user_id: i32
    ) -> Self {
        NewStore {
            title: title,
            description: Some(description),
            link: Some(link),
            image: Some(image),
            is_active: is_active,
            price: price,
            price_acc: price_acc,
            social_price: social_price,
            user_id: user_id,
            created: chrono::Local::now().naive_utc(),
        }
    }
}


#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(StoreCategories)]
#[belongs_to(Store)]
#[table_name="store_category"]
pub struct StoreCategory {
    pub id:                  i32,
    pub store_categories_id: i32,
    pub store_id:            i32,
}

#[derive(Insertable)]
#[table_name="store_category"]
pub struct NewStoreCategory {
    pub store_categories_id: i32,
    pub store_id:            i32,
}

#[derive(Debug, Serialize, Queryable, Identifiable, Associations)]
#[belongs_to(Store, foreign_key="store")]
pub struct StoreImage {
    pub id:    i32,
    pub store: i32,
    pub src:   String,
}

#[derive(Serialize, Insertable)]
#[table_name="store_images"]
pub struct NewStoreImage {
    pub store: i32,
    pub src:   String,
}

impl NewStoreImage {
    pub fn from_store_images_form(
        store_id: i32, src: String) -> Self {
        NewStoreImage {
            store: store_id,
            src:   src,
        }
    }
}

#[derive(Debug, Serialize, Queryable, Identifiable, Associations)]
#[belongs_to(Store, foreign_key="store")]
pub struct StoreVideo {
    pub id:    i32,
    pub store: i32,
    pub src:   String,
}

#[derive(Serialize, Insertable)]
#[table_name="store_videos"]
pub struct NewStoreVideo {
    pub store: i32,
    pub src:   String,
}

impl NewStoreVideo {
    pub fn from_store_videos_form (
        store_id: i32, src: String) -> Self {
        NewStoreVideo {
            store: store_id,
            src:   src,
        }
    }
}
