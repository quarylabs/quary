"""A hermetic-ish test rule for repository-wide command-line checks.

Each check runs in a private copy of its declared workspace inputs. This keeps
formatters, generators, Cargo, and pnpm from writing into the source checkout.
"""

def _ci_test_impl(ctx):
    script = ctx.actions.declare_file(ctx.label.name + ".sh")
    command = ctx.attr.command
    ctx.actions.write(
        output = script,
        is_executable = True,
        content = """#!/usr/bin/env bash
set -euo pipefail
work="$TEST_TMPDIR/workspace"
mkdir -p "$work"
cp -RL "$RUNFILES_DIR/$TEST_WORKSPACE/." "$work/"
cd "$work"
{command}
""".format(command = command),
    )
    runfiles = ctx.runfiles(files = ctx.files.workspace)
    return [DefaultInfo(executable = script, runfiles = runfiles)]

ci_test = rule(
    implementation = _ci_test_impl,
    test = True,
    attrs = {
        "command": attr.string(mandatory = True),
        "workspace": attr.label_list(allow_files = True),
    },
)

_WORKSPACE = [
    "//:ci_sources",
    "//:node_modules",
    "//js/packages/eslint-config-shared:ci_sources",
    "//js/packages/eslint-config-shared:node_modules",
    "//js/packages/proto:ci_sources",
    "//js/packages/proto:node_modules",
    "//js/packages/quary-extension-bus:ci_sources",
    "//js/packages/quary-extension-bus:node_modules",
    "//js/packages/quary-extension-ui:ci_sources",
    "//js/packages/quary-extension-ui:node_modules",
    "//js/packages/quary-extension:ci_sources",
    "//js/packages/quary-extension:node_modules",
]

def check(name, command, size = "medium", tags = []):
    ci_test(
        name = name,
        command = command,
        size = size,
        tags = tags + ["no-sandbox"],
        workspace = _WORKSPACE,
    )

def js_check(name, command, size = "large"):
    check(
        name = name,
        command = command,
        size = size,
    )
