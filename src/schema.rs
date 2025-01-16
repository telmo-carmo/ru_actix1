// @generated automatically by Diesel CLI.

diesel::table! {
    bonus (ename) {
        ename -> Text,
        job -> Text,
        sal -> Integer,
        comm -> Integer,
    }
}

diesel::table! {
    dept (deptno) {
        deptno -> Integer,
        dname -> Text,
        loc -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(bonus, dept,);
