import tomllib, subprocess

with open("rust-toolchain.toml", "rb") as file:
    data = tomllib.load(file)
    channel = data["toolchain"]["channel"]
    targets = data["toolchain"]["targets"]

    commands = []
    for target in targets:
        commands.append(
            [
                "rustup",
                "+default",
                "toolchain",
                "install",
                f"{channel}-{target}",
                "--force-non-host",
                "--component",
                "rust-std",
                "--component",
                "llvm-tools"
            ]
        )

    for command in commands:
        print(f"INSTALLING '{command[4]}'...")
        # print(command)
        subprocess.run(command, check=True, shell=True)
        print()
