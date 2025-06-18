
<details>

<summary>Windows</summary>

[Rust](https://www.rust-lang.org/tools/install) - desc.<br>
[Postgres](https://www.enterprisedb.com/downloads/postgres-postgresql-downloads) - desc.

</details>

```sql
create user admin with password 'password';
create database site with owner = admin;
grant all privileges on database site to admin;
```