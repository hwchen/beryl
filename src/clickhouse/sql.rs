use itertools::join;

use crate::query_ir::{QueryIr, Constraint};

pub fn clickhouse_sql(query_ir: QueryIr) -> String {
    let project_cols_sql = join(query_ir.projection, ", ");

    let table = query_ir.table;

    let filter_sql = if !query_ir.filters.is_empty() {
        let filters = query_ir.filters.iter()
            .map(|f| {
                match f.constraint {
                    Constraint::Compare { ref comparison, n }=> {
                        let single_quote = if f.is_text {
                            "'".to_owned()
                        } else {
                            "".to_owned()
                        };
                        format!("{} {} {single_quote}{}{single_quote}",
                            f.column,
                            comparison.sql_string(),
                            n,
                            single_quote = single_quote,
                        )
                    },
                    Constraint::StringMatch { ref substring } => {
                        format!("{} LIKE '%{}%'",
                            f.column,
                            substring,
                        )
                    },
                    Constraint::InArray { ref in_members, ref not_in_members } => {
                        let single_quote = if f.is_text {
                            "'".to_owned()
                        } else {
                            "".to_owned()
                        };

                        let mut res = String::new();
                        if !in_members.is_empty() {
                            let ms = in_members
                                .iter()
                                .map(|m| {
                                    format!("{}{}{}",
                                        single_quote,
                                        m,
                                        single_quote,
                                    )
                                });

                            res.push_str(&format!("hasAll({}, [{}])",
                                f.column,
                                join(ms, ", "),
                            ));
                        };
                        if !not_in_members.is_empty() {
                            let ms = not_in_members
                                .iter()
                                .map(|m| {
                                    format!("{}{}{}",
                                        single_quote,
                                        m,
                                        single_quote,
                                    )
                                });

                            if !in_members.is_empty() {
                                res.push_str(" AND ");
                            }

                            res.push_str(&format!("NOT hasAny({}, [{}])",
                                f.column,
                                join(ms, ", "),
                            ));
                        };

                        res
                    },
                }
            });

        let filters_str = join(filters, " and ");

        format!("where {}", filters_str)
    } else {
        "".into()
    };

    let sort_sql = if let Some(srt) = query_ir.sort {
        format!("order by {} {}",
                srt.column,
                srt.direction.sql_string(),
        )
    } else {
        "".into()
    };

    let limit_sql = {
        if let Some(lmt) = query_ir.limit {
            if let Some(offset) = lmt.offset {
                format!("limit {}, {}", offset, lmt.n)
            } else {
                format!("limit {}", lmt.n)
            }
        } else {
            "".to_string()
        }
    };

    format!("select {} from {} {} {} {}",
        project_cols_sql,
        table,
        filter_sql,
        sort_sql,
        limit_sql,
    )
}
