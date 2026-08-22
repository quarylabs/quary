"""Module extension for downloading platform-specific repository tools."""

_RATCHET_VERSION = "0.11.4"
_RATCHET_PLATFORMS = {
    "linux_amd64": {
        "url": "https://github.com/sethvargo/ratchet/releases/download/v{version}/ratchet_{version}_linux_amd64.tar.gz",
        "sha256": "7141236c5500dce440bb764a964c9d9d8130a3a421604c75b7f7fbaa55cf89f5",
    },
    "linux_arm64": {
        "url": "https://github.com/sethvargo/ratchet/releases/download/v{version}/ratchet_{version}_linux_arm64.tar.gz",
        "sha256": "11050a91f2531d65d76d463e710263270a33b5d8b4cc3ec258c58a835a2bb58c",
    },
    "darwin_amd64": {
        "url": "https://github.com/sethvargo/ratchet/releases/download/v{version}/ratchet_{version}_darwin_amd64.tar.gz",
        "sha256": "78756b000dee07e4d32e3c2bf518e81e971a6cef56627d9eafc25afef7644d57",
    },
    "darwin_arm64": {
        "url": "https://github.com/sethvargo/ratchet/releases/download/v{version}/ratchet_{version}_darwin_arm64.tar.gz",
        "sha256": "319f4c35b818f8d0f42467960e50fbd9d62032ae3bb170aa5aec00985e613336",
    },
}

def _platform(repository_ctx):
    os_name = repository_ctx.os.name.lower()
    arch = repository_ctx.os.arch

    if "mac" in os_name or "darwin" in os_name:
        os_key = "darwin"
    elif "linux" in os_name:
        os_key = "linux"
    else:
        fail("Unsupported OS: {}".format(os_name))

    if arch in ["aarch64", "arm64"]:
        arch_key = "arm64"
    elif arch in ["x86_64", "amd64"]:
        arch_key = "amd64"
    else:
        fail("Unsupported architecture: {}".format(arch))

    return "{}_{}".format(os_key, arch_key)

def _ratchet_repo_impl(repository_ctx):
    platform = _platform(repository_ctx)
    config = _RATCHET_PLATFORMS.get(platform)
    if not config:
        fail("Unsupported platform for ratchet: {}".format(platform))

    repository_ctx.download_and_extract(
        url = config["url"].format(version = _RATCHET_VERSION),
        sha256 = config["sha256"],
    )
    repository_ctx.file("BUILD.bazel", 'exports_files(["ratchet"])')

_ratchet_repo = repository_rule(implementation = _ratchet_repo_impl)

def _tools_impl(_module_ctx):
    _ratchet_repo(name = "ratchet")

tools = module_extension(implementation = _tools_impl)
