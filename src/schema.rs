table! {
    user (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
        passsword -> Varchar,
        age -> Nullable<Unsigned<Integer>>,
        skey -> Nullable<Varchar>,
    }
}
