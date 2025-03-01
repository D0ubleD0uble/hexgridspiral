# Changelog
Best-Effort (Means: worst-effort, actually).

## v0.2.6

* Expose `HGSTile::spiral_index(&self)` to get the tile index.
* Expose `HGSTile::ring(&self)` to get the ring.
* Expose `Ring::ring_index(&self)` to get the ring's index.

## v0.2.5
* Upgrade dependencies, including breaking changes:
    ```
       Upgrading derive_more ^1 -> ^2
       Upgrading rand ^0.8.5 -> ^0.9.0
       Upgrading rand_chacha ^0.3.1 -> ^0.9.0
    ```

## v0.2.4
* Derive `Hash` for `TileIndex`

## v0.2.1
* Replace relative links to images with absolute urls, so they should now render in crates.io and docs.rs.

## v0.2.0
* `RingCornerIndex::all_from()`  now takes ownership of its argument. This is more flexible because the caller can choose whether they want to clone or not.
