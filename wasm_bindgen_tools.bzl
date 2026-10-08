"""Pinned wasm-bindgen CLI toolchains matching the Cargo crate version."""

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

_VERSION = "0.2.114"

# Checksums from the upstream release assets.
_PLATFORMS = {
    "darwin_arm64": (
        "aarch64-apple-darwin",
        "macos",
        "aarch64",
        "b0ef565865b3004bca5df72c83fef9256fa059e7aaa9075493f4e392b1d17350",
    ),
    "darwin_amd64": (
        "x86_64-apple-darwin",
        "macos",
        "x86_64",
        "7c7d4ee4d810cd59745848c9a5888046719e948b27a7a89d0256641e98e6eaca",
    ),
    "linux_arm64": (
        "aarch64-unknown-linux-musl",
        "linux",
        "aarch64",
        "39103878ad3c0016f2d04b7b4a6206c974240286852abac5736d017a43703958",
    ),
    "linux_amd64": (
        "x86_64-unknown-linux-musl",
        "linux",
        "x86_64",
        "ce21be002bdbc22ede2a5cf250cb801d2106956fc3c529c75d9452729db3490c",
    ),
    "windows_amd64": (
        "x86_64-pc-windows-msvc",
        "windows",
        "x86_64",
        "32139f3e36dcf3fcc909f7366a538fa1eec9ee87c92941af4a42041ba7826b6e",
    ),
}

_BUILD = """
load("@aspect_bazel_lib//lib:copy_file.bzl", "copy_file")
load("@rules_rust_wasm_bindgen//:defs.bzl", "rust_wasm_bindgen_toolchain")

copy_file(
    name = "cli",
    src = "wasm-bindgen{suffix}",
    out = "bin/wasm-bindgen{suffix}",
    is_executable = True,
)

copy_file(
    name = "test_runner",
    src = "wasm-bindgen-test-runner{suffix}",
    out = "bin/wasm-bindgen-test-runner{suffix}",
    is_executable = True,
)

rust_wasm_bindgen_toolchain(
    name = "toolchain_impl",
    wasm_bindgen_cli = ":cli",
    wasm_bindgen_test_runner = ":test_runner",
)

toolchain(
    name = "toolchain",
    toolchain = ":toolchain_impl",
    toolchain_type = "@rules_rust_wasm_bindgen//:toolchain_type",
    exec_compatible_with = ["@platforms//os:{os}", "@platforms//cpu:{cpu}"],
    visibility = ["//visibility:public"],
)
"""

def _wasm_bindgen_tools_impl(module_ctx):
    for platform, (triple, os, cpu, sha256) in _PLATFORMS.items():
        archive = "wasm-bindgen-{}-{}".format(_VERSION, triple)
        http_archive(
            name = "wasm_bindgen_cli_" + platform,
            urls = ["https://github.com/wasm-bindgen/wasm-bindgen/releases/download/{}/{}.tar.gz".format(_VERSION, archive)],
            sha256 = sha256,
            strip_prefix = archive,
            build_file_content = _BUILD.format(os = os, cpu = cpu, suffix = ".exe" if os == "windows" else ""),
        )
    return module_ctx.extension_metadata(reproducible = True)

wasm_bindgen_tools = module_extension(implementation = _wasm_bindgen_tools_impl)
