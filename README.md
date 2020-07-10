# find-duplicate-subtrees
My take on https://leetcode.com/problems/find-duplicate-subtrees/ in Rust.

# Design decisions
* I modify the original struct to work with any type within a couple trait bounds
* The tree `val` type is assumed to be relatively cheap to clone
* Rust's `Hash` trait is not used in favor of in-order repr uniqueness property
  * This decision was made to enable writing a two-birds-one-stone
    procedure for building an in-order representation and looking for
    the duplicates
