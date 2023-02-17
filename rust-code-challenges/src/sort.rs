fn sort_usernames<T: AsRef<str>>(users: &mut Vec<T>) {
    users.sort_by_cached_key(|user| user.as_ref().to_lowercase())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_users_1() {
        let mut users = vec!["Todd", "amy"];
        sort_usernames(&mut users);
        assert_eq!(users, vec!["amy", "Todd"]);
    }
}