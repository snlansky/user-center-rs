table! {
    user (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
        password -> Varchar,
        age -> Nullable<Unsigned<Integer>>,
        skey -> Nullable<Varchar>,
    }
}
