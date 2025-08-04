# PostgreSQL Advise

These are **PostgreSQL** database recommendations to follow during product development.

## Indexes
Recommendations:
- Use EXPLAIN ANALYZE to check if an index is being used.
- Create indexes on sorted columns.
- Use `GROUP BY` instead of DISTINCT, if possible.

## Queries
Recommendations:
- Select only the specific columns, instead of `SELECT *`. 
- Join queries on indexed columns.
- Prefer a join or CTE (common table expression) over subqueries.
- Use **pg_stat_statements** to monitor slow queries.

## Maintenance
Over time tables and indexes get bloated, so regularly run:
- VACUUM FULL and REINDEX
- Use **pg_stat_user_tables**

## Data Types
Choose appropriate data types:
- Use INTEGER instead of TEXT for numerical values.
- Use UUID instead of large VARCHAR for IDs.
- Use TIMESTAMP instead of STRING for date-related values.

## Performance
For large workloads, adjust the parameters:
- shared_buffers, work_mem, effective_cache_size
- Use connection pooling, like **PgBouncer**
