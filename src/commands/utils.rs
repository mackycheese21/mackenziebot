use serenity::model::user::User;

pub fn is_owner(user: &User) -> bool {
    user.id.0 == 523919572273856523
}