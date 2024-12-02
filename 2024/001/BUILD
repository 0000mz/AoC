load("@rules_rust//rust:defs.bzl", "rust_binary")

rust_binary(
  name = "rust",
  srcs = ["rust/main.rs"],
  data = ["input.txt"],
)

cc_binary(
  name = "c",
  srcs = ["c/main.c"],
  data = ["input.txt"],
)