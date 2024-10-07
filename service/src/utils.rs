pub fn every_condition() -> sea_orm::sea_query::SimpleExpr {
    sea_orm::sea_query::SimpleExpr::Custom("1 = 1".to_string())
}
