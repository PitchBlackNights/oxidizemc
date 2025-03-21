import tomllib, subprocess

with open("rust-toolchain.toml", "rb") as file:
    data = tomllib.load(file)
    channel = data["toolchain"]["channel"]
    targets = data["toolchain"]["targets"]

    commands = []
    for target in targets:
        commands.append(
            ["rustup", "+default", "toolchain", "remove", f"{channel}-{target}"]
        )

    for command in commands:
        print(f"UNINSTALLING '{command[4]}'...")
        # print(command)
        subprocess.run(command, check=True, shell=True, stdout=subprocess.DEVNULL)
        print()
