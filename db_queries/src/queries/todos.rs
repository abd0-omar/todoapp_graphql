// This file was generated with `clorinde`. Do not modify.

#[derive(Debug)]
pub struct CreateParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub title: T1,
    pub description: T2,
    pub is_completed: bool,
}
#[derive(Debug)]
pub struct UpdateParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub title: T1,
    pub description: T2,
    pub is_completed: bool,
    pub id: uuid::Uuid,
}
#[derive(Debug)]
pub struct UpdateTagsParams<T1: crate::StringSql, T2: crate::ArraySql<Item = T1>> {
    pub tags: T2,
    pub id: uuid::Uuid,
}
#[derive(Debug, Clone, PartialEq)]
pub struct LoadAll {
    pub id: uuid::Uuid,
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    pub is_completed: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
pub struct LoadAllBorrowed<'a> {
    pub id: uuid::Uuid,
    pub title: &'a str,
    pub description: &'a str,
    pub tags: crate::ArrayIterator<'a, &'a str>,
    pub is_completed: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
impl<'a> From<LoadAllBorrowed<'a>> for LoadAll {
    fn from(
        LoadAllBorrowed {
            id,
            title,
            description,
            tags,
            is_completed,
            created_at,
            updated_at,
        }: LoadAllBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            title: title.into(),
            description: description.into(),
            tags: tags.map(|v| v.into()).collect(),
            is_completed,
            created_at,
            updated_at,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Load {
    pub id: uuid::Uuid,
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    pub is_completed: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
pub struct LoadBorrowed<'a> {
    pub id: uuid::Uuid,
    pub title: &'a str,
    pub description: &'a str,
    pub tags: crate::ArrayIterator<'a, &'a str>,
    pub is_completed: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
impl<'a> From<LoadBorrowed<'a>> for Load {
    fn from(
        LoadBorrowed {
            id,
            title,
            description,
            tags,
            is_completed,
            created_at,
            updated_at,
        }: LoadBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            title: title.into(),
            description: description.into(),
            tags: tags.map(|v| v.into()).collect(),
            is_completed,
            created_at,
            updated_at,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Create {
    pub id: uuid::Uuid,
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    pub is_completed: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
pub struct CreateBorrowed<'a> {
    pub id: uuid::Uuid,
    pub title: &'a str,
    pub description: &'a str,
    pub tags: crate::ArrayIterator<'a, &'a str>,
    pub is_completed: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
impl<'a> From<CreateBorrowed<'a>> for Create {
    fn from(
        CreateBorrowed {
            id,
            title,
            description,
            tags,
            is_completed,
            created_at,
            updated_at,
        }: CreateBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            title: title.into(),
            description: description.into(),
            tags: tags.map(|v| v.into()).collect(),
            is_completed,
            created_at,
            updated_at,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Update {
    pub id: uuid::Uuid,
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    pub is_completed: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
pub struct UpdateBorrowed<'a> {
    pub id: uuid::Uuid,
    pub title: &'a str,
    pub description: &'a str,
    pub tags: crate::ArrayIterator<'a, &'a str>,
    pub is_completed: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
impl<'a> From<UpdateBorrowed<'a>> for Update {
    fn from(
        UpdateBorrowed {
            id,
            title,
            description,
            tags,
            is_completed,
            created_at,
            updated_at,
        }: UpdateBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            title: title.into(),
            description: description.into(),
            tags: tags.map(|v| v.into()).collect(),
            is_completed,
            created_at,
            updated_at,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct UpdateTags {
    pub id: uuid::Uuid,
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    pub is_completed: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
pub struct UpdateTagsBorrowed<'a> {
    pub id: uuid::Uuid,
    pub title: &'a str,
    pub description: &'a str,
    pub tags: crate::ArrayIterator<'a, &'a str>,
    pub is_completed: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
impl<'a> From<UpdateTagsBorrowed<'a>> for UpdateTags {
    fn from(
        UpdateTagsBorrowed {
            id,
            title,
            description,
            tags,
            is_completed,
            created_at,
            updated_at,
        }: UpdateTagsBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            title: title.into(),
            description: description.into(),
            tags: tags.map(|v| v.into()).collect(),
            is_completed,
            created_at,
            updated_at,
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct LoadAllQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<LoadAllBorrowed, tokio_postgres::Error>,
    mapper: fn(LoadAllBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> LoadAllQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(LoadAllBorrowed) -> R) -> LoadAllQuery<'c, 'a, 's, C, R, N> {
        LoadAllQuery {
            client: self.client,
            params: self.params,
            query: self.query,
            cached: self.cached,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let row =
            crate::client::async_::one(self.client, self.query, &self.params, self.cached).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let opt_row =
            crate::client::async_::opt(self.client, self.query, &self.params, self.cached).await?;
        Ok(opt_row
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stream = crate::client::async_::raw(
            self.client,
            self.query,
            crate::slice_iter(&self.params),
            self.cached,
        )
        .await?;
        let mapped = stream
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(mapped)
    }
}
pub struct LoadQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<LoadBorrowed, tokio_postgres::Error>,
    mapper: fn(LoadBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> LoadQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(LoadBorrowed) -> R) -> LoadQuery<'c, 'a, 's, C, R, N> {
        LoadQuery {
            client: self.client,
            params: self.params,
            query: self.query,
            cached: self.cached,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let row =
            crate::client::async_::one(self.client, self.query, &self.params, self.cached).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let opt_row =
            crate::client::async_::opt(self.client, self.query, &self.params, self.cached).await?;
        Ok(opt_row
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stream = crate::client::async_::raw(
            self.client,
            self.query,
            crate::slice_iter(&self.params),
            self.cached,
        )
        .await?;
        let mapped = stream
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(mapped)
    }
}
pub struct CreateQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<CreateBorrowed, tokio_postgres::Error>,
    mapper: fn(CreateBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> CreateQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(CreateBorrowed) -> R) -> CreateQuery<'c, 'a, 's, C, R, N> {
        CreateQuery {
            client: self.client,
            params: self.params,
            query: self.query,
            cached: self.cached,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let row =
            crate::client::async_::one(self.client, self.query, &self.params, self.cached).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let opt_row =
            crate::client::async_::opt(self.client, self.query, &self.params, self.cached).await?;
        Ok(opt_row
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stream = crate::client::async_::raw(
            self.client,
            self.query,
            crate::slice_iter(&self.params),
            self.cached,
        )
        .await?;
        let mapped = stream
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(mapped)
    }
}
pub struct UpdateQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<UpdateBorrowed, tokio_postgres::Error>,
    mapper: fn(UpdateBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> UpdateQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(UpdateBorrowed) -> R) -> UpdateQuery<'c, 'a, 's, C, R, N> {
        UpdateQuery {
            client: self.client,
            params: self.params,
            query: self.query,
            cached: self.cached,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let row =
            crate::client::async_::one(self.client, self.query, &self.params, self.cached).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let opt_row =
            crate::client::async_::opt(self.client, self.query, &self.params, self.cached).await?;
        Ok(opt_row
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stream = crate::client::async_::raw(
            self.client,
            self.query,
            crate::slice_iter(&self.params),
            self.cached,
        )
        .await?;
        let mapped = stream
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(mapped)
    }
}
pub struct UpdateTagsQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<UpdateTagsBorrowed, tokio_postgres::Error>,
    mapper: fn(UpdateTagsBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> UpdateTagsQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(UpdateTagsBorrowed) -> R,
    ) -> UpdateTagsQuery<'c, 'a, 's, C, R, N> {
        UpdateTagsQuery {
            client: self.client,
            params: self.params,
            query: self.query,
            cached: self.cached,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let row =
            crate::client::async_::one(self.client, self.query, &self.params, self.cached).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let opt_row =
            crate::client::async_::opt(self.client, self.query, &self.params, self.cached).await?;
        Ok(opt_row
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stream = crate::client::async_::raw(
            self.client,
            self.query,
            crate::slice_iter(&self.params),
            self.cached,
        )
        .await?;
        let mapped = stream
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(mapped)
    }
}
pub struct UuidUuidQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<uuid::Uuid, tokio_postgres::Error>,
    mapper: fn(uuid::Uuid) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> UuidUuidQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(uuid::Uuid) -> R) -> UuidUuidQuery<'c, 'a, 's, C, R, N> {
        UuidUuidQuery {
            client: self.client,
            params: self.params,
            query: self.query,
            cached: self.cached,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let row =
            crate::client::async_::one(self.client, self.query, &self.params, self.cached).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let opt_row =
            crate::client::async_::opt(self.client, self.query, &self.params, self.cached).await?;
        Ok(opt_row
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stream = crate::client::async_::raw(
            self.client,
            self.query,
            crate::slice_iter(&self.params),
            self.cached,
        )
        .await?;
        let mapped = stream
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(mapped)
    }
}
pub struct LoadAllStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn load_all() -> LoadAllStmt {
    LoadAllStmt(
        "SELECT id, title, description, COALESCE(tags, ARRAY[]::text[]) AS tags, is_completed, created_at, updated_at FROM todos",
        None,
    )
}
impl LoadAllStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s self,
        client: &'c C,
    ) -> LoadAllQuery<'c, 'a, 's, C, LoadAll, 0> {
        LoadAllQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<LoadAllBorrowed, tokio_postgres::Error> {
                    Ok(LoadAllBorrowed {
                        id: row.try_get(0)?,
                        title: row.try_get(1)?,
                        description: row.try_get(2)?,
                        tags: row.try_get(3)?,
                        is_completed: row.try_get(4)?,
                        created_at: row.try_get(5)?,
                        updated_at: row.try_get(6)?,
                    })
                },
            mapper: |it| LoadAll::from(it),
        }
    }
}
pub struct LoadStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn load() -> LoadStmt {
    LoadStmt(
        "SELECT id, title, description, COALESCE(tags, ARRAY[]::text[]) AS tags, is_completed, created_at, updated_at FROM todos WHERE id = $1",
        None,
    )
}
impl LoadStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s self,
        client: &'c C,
        id: &'a uuid::Uuid,
    ) -> LoadQuery<'c, 'a, 's, C, Load, 1> {
        LoadQuery {
            client,
            params: [id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row: &tokio_postgres::Row| -> Result<LoadBorrowed, tokio_postgres::Error> {
                Ok(LoadBorrowed {
                    id: row.try_get(0)?,
                    title: row.try_get(1)?,
                    description: row.try_get(2)?,
                    tags: row.try_get(3)?,
                    is_completed: row.try_get(4)?,
                    created_at: row.try_get(5)?,
                    updated_at: row.try_get(6)?,
                })
            },
            mapper: |it| Load::from(it),
        }
    }
}
pub struct CreateStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn create() -> CreateStmt {
    CreateStmt(
        "INSERT INTO todos ( title, description, is_completed ) VALUES ( $1, $2, $3 ) RETURNING id, title, description, COALESCE(tags, ARRAY[]::text[]) AS tags, is_completed, created_at, updated_at",
        None,
    )
}
impl CreateStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>(
        &'s self,
        client: &'c C,
        title: &'a T1,
        description: &'a T2,
        is_completed: &'a bool,
    ) -> CreateQuery<'c, 'a, 's, C, Create, 3> {
        CreateQuery {
            client,
            params: [title, description, is_completed],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<CreateBorrowed, tokio_postgres::Error> {
                    Ok(CreateBorrowed {
                        id: row.try_get(0)?,
                        title: row.try_get(1)?,
                        description: row.try_get(2)?,
                        tags: row.try_get(3)?,
                        is_completed: row.try_get(4)?,
                        created_at: row.try_get(5)?,
                        updated_at: row.try_get(6)?,
                    })
                },
            mapper: |it| Create::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        CreateParams<T1, T2>,
        CreateQuery<'c, 'a, 's, C, Create, 3>,
        C,
    > for CreateStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a CreateParams<T1, T2>,
    ) -> CreateQuery<'c, 'a, 's, C, Create, 3> {
        self.bind(
            client,
            &params.title,
            &params.description,
            &params.is_completed,
        )
    }
}
pub struct UpdateStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn update() -> UpdateStmt {
    UpdateStmt(
        "UPDATE todos SET title = $1, description = $2, is_completed = $3, updated_at = CURRENT_TIMESTAMP WHERE id = $4 RETURNING id, title, description, COALESCE(tags, ARRAY[]::text[]) AS tags, is_completed, created_at, updated_at",
        None,
    )
}
impl UpdateStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>(
        &'s self,
        client: &'c C,
        title: &'a T1,
        description: &'a T2,
        is_completed: &'a bool,
        id: &'a uuid::Uuid,
    ) -> UpdateQuery<'c, 'a, 's, C, Update, 4> {
        UpdateQuery {
            client,
            params: [title, description, is_completed, id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<UpdateBorrowed, tokio_postgres::Error> {
                    Ok(UpdateBorrowed {
                        id: row.try_get(0)?,
                        title: row.try_get(1)?,
                        description: row.try_get(2)?,
                        tags: row.try_get(3)?,
                        is_completed: row.try_get(4)?,
                        created_at: row.try_get(5)?,
                        updated_at: row.try_get(6)?,
                    })
                },
            mapper: |it| Update::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        UpdateParams<T1, T2>,
        UpdateQuery<'c, 'a, 's, C, Update, 4>,
        C,
    > for UpdateStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a UpdateParams<T1, T2>,
    ) -> UpdateQuery<'c, 'a, 's, C, Update, 4> {
        self.bind(
            client,
            &params.title,
            &params.description,
            &params.is_completed,
            &params.id,
        )
    }
}
pub struct UpdateTagsStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn update_tags() -> UpdateTagsStmt {
    UpdateTagsStmt(
        "UPDATE todos SET tags = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2 RETURNING id, title, description, COALESCE(tags, ARRAY[]::text[]) AS tags, is_completed, created_at, updated_at",
        None,
    )
}
impl UpdateTagsStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<
        'c,
        'a,
        's,
        C: GenericClient,
        T1: crate::StringSql,
        T2: crate::ArraySql<Item = T1>,
    >(
        &'s self,
        client: &'c C,
        tags: &'a T2,
        id: &'a uuid::Uuid,
    ) -> UpdateTagsQuery<'c, 'a, 's, C, UpdateTags, 2> {
        UpdateTagsQuery {
            client,
            params: [tags, id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<UpdateTagsBorrowed, tokio_postgres::Error> {
                    Ok(UpdateTagsBorrowed {
                        id: row.try_get(0)?,
                        title: row.try_get(1)?,
                        description: row.try_get(2)?,
                        tags: row.try_get(3)?,
                        is_completed: row.try_get(4)?,
                        created_at: row.try_get(5)?,
                        updated_at: row.try_get(6)?,
                    })
                },
            mapper: |it| UpdateTags::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::ArraySql<Item = T1>>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        UpdateTagsParams<T1, T2>,
        UpdateTagsQuery<'c, 'a, 's, C, UpdateTags, 2>,
        C,
    > for UpdateTagsStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a UpdateTagsParams<T1, T2>,
    ) -> UpdateTagsQuery<'c, 'a, 's, C, UpdateTags, 2> {
        self.bind(client, &params.tags, &params.id)
    }
}
pub struct DeleteStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn delete() -> DeleteStmt {
    DeleteStmt("DELETE FROM todos WHERE id = $1 RETURNING id", None)
}
impl DeleteStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s self,
        client: &'c C,
        id: &'a uuid::Uuid,
    ) -> UuidUuidQuery<'c, 'a, 's, C, uuid::Uuid, 1> {
        UuidUuidQuery {
            client,
            params: [id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row| Ok(row.try_get(0)?),
            mapper: |it| it,
        }
    }
}
