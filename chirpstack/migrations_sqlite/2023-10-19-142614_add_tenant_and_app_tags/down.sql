-- The index needs to be dropped before its target column.
-- Because SQLite fails to drop the column if an index exists on it
drop index idx_application_tags;
alter table application drop column tags;

drop index idx_tenant_tags;
alter table tenant drop column tags;
