use model::user::User;
use model::article::Article;

#[derive(Deserialize,Serialize, Debug)]
pub struct SignupMsgs {
    pub status: i32,
    pub message : String,
}

#[derive(Deserialize,Serialize, Debug)]
pub struct SigninMsgs {
    pub status: i32,
    pub token: String,
    pub signin_user: User,
    pub message : String,
}

#[derive(Deserialize,Serialize, Debug)]
pub struct ArticleListMsgs {
    pub status: i32,
    pub message : String,
    pub article_list: Vec<Article>,
}
