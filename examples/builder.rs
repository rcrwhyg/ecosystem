use anyhow::Result;
use chrono::{DateTime, Datelike, Utc};
use derive_builder::Builder;

#[allow(unused)]
#[derive(Debug, Builder)]
#[builder(build_fn(name = "_priv_build"))]
struct User {
    #[builder(setter(into))]
    name: String,
    #[builder(default, setter(into, strip_option))]
    email: Option<String>,
    #[builder(setter(custom))]
    dob: DateTime<Utc>,
    #[builder(setter(skip))]
    age: u32,
    #[builder(default = "vec![]", setter(each(name = "skill", into)))]
    skills: Vec<String>,
}

fn main() -> Result<()> {
    let user = User::builder()
        .name("Alice")
        .email("lyn@awesome.com")
        .dob("1990-01-01T00:00:00Z")
        .skill("programing")
        .skill("debugging")
        .build()?;

    println!("{:?}", user);

    Ok(())
}

impl User {
    pub fn builder() -> UserBuilder {
        UserBuilder::default()
    }
}

impl UserBuilder {
    pub fn build(&self) -> Result<User> {
        let mut user = self._priv_build()?;
        user.age = (Utc::now().year() - user.dob.year()) as _;
        Ok(user)
    }

    pub fn dob(&mut self, value: &str) -> &mut Self {
        self.dob = Some(DateTime::from(DateTime::parse_from_rfc3339(value).unwrap()));
        self
    }
}
