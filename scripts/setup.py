import tomllib, subprocess, sys, os, pathlib
from typing import Literal


def runcmd(
    args: list[str],
    check: bool = True,
    text: Literal[False] | None = False,
    stdout=None,
    stderr=None,
    shush: bool = False,
):
    quiet = "-q" in sys.argv or "--quiet" in sys.argv or shush
    print(f"RUNNING COMMAND `{' '.join(args)}`{' (QUIET)' if quiet else ''}")
    if quiet:
        cmdoutput = subprocess.run(
            args,
            shell=True,
            check=check,
            text=text,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
        )
    else:
        cmdoutput = subprocess.run(
            args, shell=True, check=check, text=text, stdout=stdout, stderr=stderr
        ).stdout

    if quiet:
        return
    if stdout == subprocess.PIPE and (stderr == None or stderr == subprocess.STDOUT):
        return str(cmdoutput.stdout)
    elif stderr == subprocess.PIPE and stdout == None:
        return str(cmdoutput.stderr)
    elif stdout == subprocess.PIPE and stderr == subprocess.PIPE:
        return [str(cmdoutput.stdout), str(cmdoutput.stderr)]


channel = ""
targets = []
with open("rust-toolchain.toml", "rb") as file:
    data = tomllib.load(file)
    channel = data["toolchain"]["channel"]
    targets = data["toolchain"]["targets"]

print("Fixing toolchains...")
runcmd(["python", "./scripts/uninstall-targets.py"])
runcmd(["python", "./scripts/install-targets.py"])

print("\nInstalling `cross` + extra components...")
runcmd(["cargo", "install", "cross"])

print("\nCleaning project...")
if "y" in input("Delete all 'cross-rs/cross' Docker Images? [y/N] ").lower():
    runcmd(["cross-util", "clean", "-e", "-f", "-l"], shush=True)
else:
    print("Skipped cleaning Docker Images")
runcmd(["cross", "clean"], shush=True)

if not pathlib.Path("./cross").exists():
    print("\nCloning `cross-rs/cross`...")
    runcmd(["git", "clone", "https://github.com/cross-rs/cross"])
    os.chdir("cross")
    runcmd(["git", "submodule", "update", "--init", "--remote"])
else:
    os.chdir("cross")

print("\nBuilding `xtask`...")
runcmd(["cargo", "build", "-p", "xtask"])

print("\nBuilding local Docker Images...")
bin = "target\\debug\\xtask.exe" if os.name == "nt" else "./target/debug/xtask"
runcmd([f"{bin}", "configure-crosstool"])
runcmd(
    [f"{bin}", "build-docker-image", "x86_64-pc-windows-msvc-cross", "--tag", "local"]
)
runcmd(
    [f"{bin}", "build-docker-image", "aarch64-pc-windows-msvc-cross", "--tag", "local"]
)
runcmd(
    [
        f"{bin}",
        "build-docker-image",
        "x86_64-apple-darwin-cross",
        "--tag",
        "local",
        "--build-arg",
        "MACOS_SDK_URL=https://github.com/joseluisq/macosx-sdks/releases/download/12.3/MacOSX12.3.sdk.tar.xz",
    ]
)
runcmd(
    [
        f"{bin}",
        "build-docker-image",
        "aarch64-apple-darwin-cross",
        "--tag",
        "local",
        "--build-arg",
        "MACOS_SDK_URL=https://github.com/joseluisq/macosx-sdks/releases/download/12.3/MacOSX12.3.sdk.tar.xz",
    ]
)
