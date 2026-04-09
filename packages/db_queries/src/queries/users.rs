// This file was generated with `clorinde`. Do not modify.

#[derive(Debug)]
pub struct CreateParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub email: T1,
    pub password_hash: T2,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Create {
    pub id: uuid::Uuid,
    pub email: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
pub struct CreateBorrowed<'a> {
    pub id: uuid::Uuid,
    pub email: &'a str,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
impl<'a> From<CreateBorrowed<'a>> for Create {
    fn from(
        CreateBorrowed {
            id,
            email,
            created_at,
        }: CreateBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            email: email.into(),
            created_at,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct LoadByEmail {
    pub id: uuid::Uuid,
    pub email: String,
    pub password_hash: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
pub struct LoadByEmailBorrowed<'a> {
    pub id: uuid::Uuid,
    pub email: &'a str,
    pub password_hash: &'a str,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
impl<'a> From<LoadByEmailBorrowed<'a>> for LoadByEmail {
    fn from(
        LoadByEmailBorrowed {
            id,
            email,
            password_hash,
            created_at,
        }: LoadByEmailBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            email: email.into(),
            password_hash: password_hash.into(),
            created_at,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct LoadById {
    pub id: uuid::Uuid,
    pub email: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
pub struct LoadByIdBorrowed<'a> {
    pub id: uuid::Uuid,
    pub email: &'a str,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
impl<'a> From<LoadByIdBorrowed<'a>> for LoadById {
    fn from(
        LoadByIdBorrowed {
            id,
            email,
            created_at,
        }: LoadByIdBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            email: email.into(),
            created_at,
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
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
pub struct LoadByEmailQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<LoadByEmailBorrowed, tokio_postgres::Error>,
    mapper: fn(LoadByEmailBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> LoadByEmailQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(LoadByEmailBorrowed) -> R,
    ) -> LoadByEmailQuery<'c, 'a, 's, C, R, N> {
        LoadByEmailQuery {
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
pub struct LoadByIdQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<LoadByIdBorrowed, tokio_postgres::Error>,
    mapper: fn(LoadByIdBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> LoadByIdQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(LoadByIdBorrowed) -> R) -> LoadByIdQuery<'c, 'a, 's, C, R, N> {
        LoadByIdQuery {
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
pub struct CreateStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn create() -> CreateStmt {
    CreateStmt(
        "INSERT INTO users (email, password_hash) VALUES ($1, $2) RETURNING id, email, created_at",
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
        email: &'a T1,
        password_hash: &'a T2,
    ) -> CreateQuery<'c, 'a, 's, C, Create, 2> {
        CreateQuery {
            client,
            params: [email, password_hash],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<CreateBorrowed, tokio_postgres::Error> {
                    Ok(CreateBorrowed {
                        id: row.try_get(0)?,
                        email: row.try_get(1)?,
                        created_at: row.try_get(2)?,
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
        CreateQuery<'c, 'a, 's, C, Create, 2>,
        C,
    > for CreateStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a CreateParams<T1, T2>,
    ) -> CreateQuery<'c, 'a, 's, C, Create, 2> {
        self.bind(client, &params.email, &params.password_hash)
    }
}
pub struct LoadByEmailStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn load_by_email() -> LoadByEmailStmt {
    LoadByEmailStmt(
        "SELECT id, email, password_hash, created_at FROM users WHERE email = $1",
        None,
    )
}
impl LoadByEmailStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s self,
        client: &'c C,
        email: &'a T1,
    ) -> LoadByEmailQuery<'c, 'a, 's, C, LoadByEmail, 1> {
        LoadByEmailQuery {
            client,
            params: [email],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<LoadByEmailBorrowed, tokio_postgres::Error> {
                    Ok(LoadByEmailBorrowed {
                        id: row.try_get(0)?,
                        email: row.try_get(1)?,
                        password_hash: row.try_get(2)?,
                        created_at: row.try_get(3)?,
                    })
                },
            mapper: |it| LoadByEmail::from(it),
        }
    }
}
pub struct LoadByIdStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn load_by_id() -> LoadByIdStmt {
    LoadByIdStmt(
        "SELECT id, email, created_at FROM users WHERE id = $1",
        None,
    )
}
impl LoadByIdStmt {
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
    ) -> LoadByIdQuery<'c, 'a, 's, C, LoadById, 1> {
        LoadByIdQuery {
            client,
            params: [id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<LoadByIdBorrowed, tokio_postgres::Error> {
                    Ok(LoadByIdBorrowed {
                        id: row.try_get(0)?,
                        email: row.try_get(1)?,
                        created_at: row.try_get(2)?,
                    })
                },
            mapper: |it| LoadById::from(it),
        }
    }
}
