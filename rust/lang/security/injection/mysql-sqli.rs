fn mysql_query_first() {
    //ruleid: mysql-sqli
    conn.query_first(format!("SELECT {} from tmp", "x"));
}

fn mysql_query_first1() {
    //ruleid: mysql-sqli
    conn.query_first(x + y);
}
