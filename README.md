# Example of using CockroachDB with diesel

This is almost verbatim the example from

https://github.com/diesel-rs/diesel/tree/3ae353c3aff8a7ea64ed6cb39b3a045ac86cd60e/examples/postgres

with minor adjustments:

1. after `diesel setup`, edit the created `up.sql` and `down.sql` to contain
   only a trivial statement (like `SELECT 1`). This is necessary because by
   default they contain a [random trigger] that CockroachDB can't handle.
2. `schema.rs` was manually spelled out using the `table!` macro. The tutorial
   uses `infer_schema!` for which we don't have all the [internals]. Note that
   we use `BigInt` to properly support CockroachDB's `SERIAL` type. In turn,
   `struct Post` has `id: i64` instead of `i32`.
3. Some adjustments in `show_posts` to list the ID which is otherwise impossible
   to guess.
4. Pinned the diesel dependency for no good reason (to hopefully have this go
   stale later).

There are likely more problems not discovered by this toy example. See the
[tracking issue].

[random trigger]: https://github.com/diesel-rs/diesel/blob/master/diesel_cli/src/setup_sql/postgres/initial_setup/up.sql
[internals]: https://github.com/cockroachdb/cockroach/issues/8675
[tracking issue]: https://github.com/cockroachdb/cockroach/issues/13787
