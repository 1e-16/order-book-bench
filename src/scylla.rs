use std::borrow::Cow;
use std::fmt::{Display};
use std::io;
use async_once::AsyncOnce;
use lazy_static::lazy_static;
use ntex::web;
use ntex::web::types::Json;
use scylla::{Session, SessionBuilder};
use crate::model::{GetOrderByOIdReq, GetOrderRsp, Order};

lazy_static! {
    static ref SCYLLA_SESSION: AsyncOnce<Session> = AsyncOnce::new(async {
        let session = async {
            SessionBuilder::new()
                .known_node("127.0.0.1:9042")
                .build()
                .await
        }.await.unwrap();

        session
    });
}

pub async fn get_scylla_session() -> &'static Session {
    let sess = SCYLLA_SESSION.get().await;
    sess
}

// pub async fn query<T>(stmt: impl Into<String>) -> Result<impl Iterator<Item = T>, io::Error>
// {
//     let session = get_scylla_session().await;
//     let ret = session.query(stmt,()).await.unwrap();
//     let v = ret.rows_typed_or_empty::<T>();
//
//     Ok(v.map(|r| r.unwrap()))
// }

fn format_value<T: Display + Quoted>(v: T) -> String {
    if <T as Quoted>::is_quoted() {
        return format!("'{v}'");
    }

    v.to_string()
}

pub(crate) trait Quoted {
    fn is_quoted() -> bool;
}

impl<'a> Quoted for &'a i64 {
    fn is_quoted() -> bool {
        false
    }
}

impl Quoted for i64 {
    fn is_quoted() -> bool {
        false
    }
}

impl<'a> Quoted for &'a f64 {
    fn is_quoted() -> bool {
        false
    }
}

impl Quoted for f64 {
    fn is_quoted() -> bool {
        false
    }
}

impl<'a> Quoted for &'a str {
    fn is_quoted() -> bool {
        true
    }
}

impl<'a> Quoted for &'a &str {
    fn is_quoted() -> bool {
        true
    }
}

impl Quoted for String {
    fn is_quoted() -> bool {
        true
    }
}

impl Quoted for &String {
    fn is_quoted() -> bool {
        true
    }
}

pub enum Op<'a, T>
    where T: Display + Quoted
{
    Eq(&'a str, T),
    Gt(&'a str, T),
    Lt(&'a str, T),
    GtE(&'a str, T),
    LtE(&'a str, T),

    In(&'a str, &'a Vec<T>),

    Contains(&'a str, T),
    ContainsKey(&'a str, T),
}

impl<'a, T> From<Op<'a, T>> for String
    where T: Display + Quoted, for<'b> &'b T: Quoted
{
    fn from(op: Op<'a, T>) -> Self {
        match op {
            Op::Eq(field, value) => format!("{} = {}", field, format_value(value)),
            Op::Gt(field, value) => format!("{} > {}", field, format_value(value)),
            Op::Lt(field, value) => format!("{} < {}", field, format_value(value)),
            Op::GtE(field, value) => format!("{} >= {}", field, format_value(value)),
            Op::LtE(field, value) => format!("{} <= {}", field, format_value(value)),
            Op::Contains(field, value) => format!("{} CONTAINS {}", field, format_value(value)),
            Op::ContainsKey(field, value) => format!("{} CONTAINS KEY {}", field, format_value(value)),
            Op::In(field, values) => {
                let values_str = values
                    .iter()
                    .map(|value| format_value(value))
                    .collect::<Vec<String>>()
                    .join(", ");
                format!("{} IN ({})", field, values_str)
            }
        }
    }
}

pub enum Ord<T>
    where T: Display
{
    Asc(T),
    Desc(T),
}

impl<T> From<Ord<T>> for String
    where T: Display + Quoted, for<'b> &'b T: Quoted
{
    fn from(ord: Ord<T>) -> Self {
        match ord {
            Ord::Asc(l) => format!("{l} ASC"),
            Ord::Desc(l) => format!("{l} DESC"),
        }
    }
}

#[derive(Default)]
pub struct Query
{
    select_clause: Option<Vec<String>>,
    from_clause: String,
    where_clause: Option<Vec<String>>,
    group_by_clause: Option<Vec<String>>,
    order_by_clause: Option<Vec<String>>,
    per_partition_limit_clause: usize,
    limit_clause: usize,
    allow_filtering: bool,
    bypass_cache: bool,
    timeout: usize,
}

impl Query {
    pub fn new(keyspace: impl Into<String>, tbl: impl Into<String>) -> Self {
        Query {
            from_clause: format!("{}.{}", keyspace.into(), tbl.into()),
            ..Default::default()
        }
    }

    // pub fn select(&mut self, field: impl Into<String>) -> &mut Self
    // {
    //     self.select_clause.get_or_insert_with(|| vec![]).push(field.into());
    //     self
    // }

    pub fn selects<'a, T>(&mut self, fields: T) -> &mut Self
        where
            T: IntoIterator,
            T::Item: Into<Cow<'a, str>>,
    {
        let fields = fields.into_iter().map(Into::into).collect::<Vec<Cow<str>>>();
        self.select_clause.get_or_insert_with(Vec::new).extend(fields.into_iter().map(Cow::into_owned));
        self
    }

    // pub fn where_(&mut self, op: impl Into<String>) -> &mut Self
    // {
    //     self.where_clause.get_or_insert_with(|| vec![]).push(op.into());
    //     self
    // }

    pub fn wheres<'a, T>(&mut self, ops: T) -> &mut Self
        where
            T: IntoIterator,
            T::Item: Into<String>,
    {
        let fields: Vec<String> = ops.into_iter().map(Into::into).collect(); // 使用 Into::into
        self.where_clause
            .get_or_insert_with(Vec::new)
            .extend(fields);
        self
    }

    // pub fn group(&mut self, field: impl Into<String>) -> &mut Self
    // {
    //     self.group_by_clause.get_or_insert_with(|| vec![]).push(field.into());
    //     self
    // }

    pub fn groups<'a, T>(&mut self, fields: T) -> &mut Self
        where
            T: IntoIterator,
            T::Item: Into<Cow<'a, str>>,
    {
        let fields = fields.into_iter().map(Into::into).collect::<Vec<Cow<str>>>();
        self.group_by_clause.get_or_insert_with(Vec::new).extend(fields.into_iter().map(Cow::into_owned));
        self
    }

    // pub fn order(&mut self, ord: impl Into<String>) -> &mut Self
    // {
    //     self.order_by_clause.get_or_insert_with(|| vec![]).push(ord.into());
    //     self
    // }

    pub fn orders<'a, T>(&mut self, ops: T) -> &mut Self
        where
            T: IntoIterator,
            T::Item: Into<String>,
    {
        let fields: Vec<String> = ops.into_iter().map(Into::into).collect(); // 使用 Into::into
        self.order_by_clause
            .get_or_insert_with(Vec::new)
            .extend(fields);
        self
    }

    pub fn per_partition_limit(&mut self, u: usize) -> &mut Self
    {
        self.per_partition_limit_clause = u;
        self
    }

    pub fn limit(&mut self, u: usize) -> &mut Self
    {
        self.limit_clause = u;
        self
    }

    pub fn allow_filtering(&mut self) -> &mut Self
    {
        self.allow_filtering = true;
        self
    }

    pub fn bypass_cache(&mut self) -> &mut Self
    {
        self.bypass_cache = true;
        self
    }

    pub fn timeout(&mut self, t: usize) -> &mut Self
    {
        self.timeout = t;
        self
    }

    pub async fn finish<T: scylla::FromRow>(&self) -> Result<impl Iterator<Item = T>, io::Error>
    {
        let stmt: String = self.into();
        let session = get_scylla_session().await;
        let ret = session.query(stmt,()).await.unwrap();
        let v = ret.rows_typed_or_empty::<T>();
        Ok(v.map(|r| r.unwrap()))
    }
}

impl From<&Query> for String {
    fn from(q: &Query) -> Self {
        let mut clause = vec!["SELECT".into()];
        match &q.select_clause {
            None => {
                clause.push("*".into());
            }
            Some(v) => {
                if v.len() == 0 {
                    clause.push("*".into());
                } else {
                    clause.push(v.join(", "));
                }
            }
        }

        clause.push("FROM".into());
        clause.push(q.from_clause.as_str().into());

        if let Some(v) = &q.where_clause {
            clause.push("WHERE".into());
            clause.push(v.join(" AND "));
        }

        if let Some(v) = &q.group_by_clause {
            clause.push("GROUP BY".into());
            clause.push(v.join(", "));
        }

        if let Some(v) = &q.order_by_clause {
            clause.push("ORDER BY".into());
            clause.push(v.join(", "));
        }

        if q.per_partition_limit_clause > 0 {
            clause.push(format!("PER PARTITION LIMIT {}", q.per_partition_limit_clause))
        }

        if q.limit_clause > 0 {
            clause.push(format!("LIMIT {}", q.limit_clause))
        }

        if q.allow_filtering {
            clause.push("ALLOW FILTERING".into())
        }

        if q.bypass_cache {
            clause.push("BYPASS CACHE".into())
        }

        if q.timeout > 0 {
            clause.push(format!("USING TIMEOUT {}", q.timeout))
        }

        clause.join(" ")
    }
}

#[test]
fn test_query() {
    let mut q = Query::new("biz","orders");
    q.selects(["b", "c", "d"]);
    q.selects(vec!["b", "c", "d"]);
    q.wheres([Op::Lt("fff", 100)]);
    q.wheres([Op::Eq("a", "b")]);

    q.groups(["b", "c", "d"]);
    q.orders([Ord::Asc("e"), Ord::Desc("gbg")]);
    q.limit(10);
    q.per_partition_limit(5);
    q.allow_filtering();
    q.bypass_cache();
    q.timeout(01);

    let cql: String = (&q).into();
    println!("{}", cql)
}