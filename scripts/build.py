import tomllib, subprocess, sys, shutil, pathlib, re

release = "--release" in sys.argv[1:]
all = "--all" in sys.argv[1:]

cmd_output = subprocess.run(
    ["rustc", "-vV"], check=True, shell=True, stdout=subprocess.PIPE
).stdout.decode("utf-8")
host_triple_match = re.search(r"(?<=host\: ).+", cmd_output)
if host_triple_match == None:
    raise Exception(f"Unable to obtain host-triple!\nOutput:\n```\n{cmd_output}\n```")
host_triple = host_triple_match.group(0).strip()

with open("rust-toolchain.toml", "rb") as file:
    data = tomllib.load(file)
    targets = data["toolchain"]["targets"]
    if not all and host_triple not in targets:
        raise Exception(
            f"Building to your system is not supported by OxidizeMC. You can only build to other system targets.\nYour host-triple: '{host_triple}'\nSupported targets: {targets}"
        )
    elif not all:
        targets = [host_triple]

    commands = []
    for target in targets:
        tmp = [
            "cross",
            "build",
            "--target",
            target,
            "--target-dir",
            f"./target/{"release" if release else "debug"}/{target}",
        ]
        if release:
            tmp.append("--release")
        commands.append(tmp)

    try:
        shutil.rmtree(f"./target/{"release" if release else "debug"}/output")
    except FileNotFoundError:
        pass
    pathlib.Path(f"./target/{"release" if release else "debug"}/output").mkdir(
        parents=True
    )

    for index, command in enumerate(commands):
        print(
            f"BUILDING '{targets[index]}' ({"RELEASE" if release else "DEBUG"})...\nMay take a bit to start due to recompiling cargo"
        )
        # print(command)
        subprocess.run(command, check=False, shell=True)

        if "x86_64" in targets[index]:
            arch = "x86_64"
        elif "aarch64" in targets[index]:
            arch = "aarch64"
        else:
            print(f"UNHANDLED TARGET ARCH ERROR: {targets[index]}")
            print("Skipping dylib copy!")
            continue

        if "windows-msvc" in targets[index]:
            os = "win"
            src_file = "oxidizemc.dll"
            dest_file_ext = "dll"
        elif "linux-gnu" in targets[index]:
            os = "linux"
            src_file = "liboxidizemc.so"
            dest_file_ext = "so"
        elif "apple-darwin" in targets[index]:
            os = "osx"
            src_file = "liboxidizemc.dylib"
            dest_file_ext = "dylib"
        else:
            print(f"UNHANDLED TARGET OS ERROR: {targets[index]}")
            print("Skipping dylib copy!")
            continue

        src = f"./target/{"release" if release else "debug"}/{targets[index]}/{targets[index]}/{"release" if release else "debug"}"
        dest = f"./target/{"release" if release else "debug"}/output"

        print(f"COPYING ARTIFACT AS 'oxidizemc-{os}-{arch}.{dest_file_ext}'")
        shutil.copyfile(
            f"{src}/{src_file}", f"{dest}/oxidizemc-{os}-{arch}.{dest_file_ext}"
        )
        print("DONE\n")
