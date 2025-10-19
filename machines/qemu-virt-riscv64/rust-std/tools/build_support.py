import toml
import os
import subprocess
from building import *

TEMPLATE = {
    "package": {"name": "rust_dummy", "version": "0.0.0", "edition": "2021"},
    "lib": {"name": "rust", "crate-type": ["staticlib"]},
    "dependencies": {},
}

CARGO_CMD = {
    "f1": "cargo rustc",
    "f2": "-Z build-std=core,alloc",
    "f6": "-Z build-std-features=compiler-builtins-mem",
    "f3": "--target",
    "target-arch": "%s",
    "f4": "--release",
    "out-path": "--target-dir=%s",
    "f5": "--",
    "f6": "-Z build-std-features=compiler-builtins-mem",
}


RUSTC_CORE_PATH = "lib/rustlib/src/rust/library/core"
RUSTC_ALLOC_PATH = "lib/rustlib/src/rust/library/alloc"

FEATURE_FILE_PATH = ""

def _has(sym: str) -> bool:
    try:
        return bool(GetDepend([sym]))
    except Exception:
        return bool(GetDepend(sym))

def _parse_cflags(cflags: str):
    info = {
        "march": None,
        "mabi": None,
        "rv_bits": None,  # 32 or 64
        "has_f": False,
        "has_d": False,
    }

    if not cflags:
        return info

    parts = cflags.split()
    for flag in parts:
        if flag.startswith("-march="):
            info["march"] = flag.split("=", 1)[1]
            if "rv32" in info["march"]:
                info["rv_bits"] = 32
            elif "rv64" in info["march"]:
                info["rv_bits"] = 64
            # crude feature detection
            m = info["march"]
            if m:
                info["has_f"] = ("f" in m)
                info["has_d"] = ("d" in m)
        elif flag.startswith("-mabi="):
            info["mabi"] = flag.split("=", 1)[1]
            if info["mabi"] in ("ilp32d", "ilp32f", "lp64d", "lp64f"):
                # floating-point ABI implies FPU availability
                info["has_f"] = True
                info["has_d"] = info["mabi"].endswith("d")

    return info

def verify_rust_toolchain():
    try:
        r1 = subprocess.run(["rustc", "--version"], capture_output=True, text=True)
        r2 = subprocess.run(["cargo", "--version"], capture_output=True, text=True)
        return r1.returncode == 0 and r2.returncode == 0
    except Exception:
        return False

def ensure_rust_target_installed(target: str):
    try:
        result = subprocess.run(["rustup", "target", "list", "--installed"], capture_output=True, text=True)
        if result.returncode == 0 and target in result.stdout:
            return True
        print(f"Rust target '{target}' is not installed.")
        print(f"Please install it with: rustup target add {target}")
    except Exception:
        print("Warning: Failed to check rustup target list (rustup missing?)")
    return False

def make_rustflags(rtconfig, target: str):
    rustflags = [
        "-C", "opt-level=z",
        "-C", "panic=abort",
        "-C", "relocation-model=static",
    ]

    if "riscv" in target:
        rustflags += [
            "-C", "code-model=medium",
            "-C", "link-dead-code",
        ]
        # propagate march/mabi for consistency (use link-arg for staticlib builds – harmless)
        cflags = getattr(rtconfig, "CFLAGS", "")
        for flag in cflags.split():
            if flag.startswith("-march=") or flag.startswith("-mabi="):
                rustflags += ["-C", f"link-arg={flag}"]

    if "thumb" in target or "aarch64" in target:
        rustflags += ["-C", "link-arg=-nostartfiles"]

    return " ".join(rustflags)

def detect_rust_target(has, rtconfig):
    """
    Decide the Rust target triple based on RT-Thread Kconfig and rtconfig.*.
    `has` is a callable: has("SYMBOL") -> bool
    """
        # ARM Cortex-M
    if has("ARCH_ARM"):
        # FPU hints from flags/macros
        cflags = getattr(rtconfig, "CFLAGS", "")
        hard_float = "-mfloat-abi=hard" in cflags or has("ARCH_ARM_FPU") or has("ARCH_FPU_VFP")

        if has("ARCH_ARM_CORTEX_M3"):
            return "thumbv7m-none-eabi"
        if has("ARCH_ARM_CORTEX_M4") or has("ARCH_ARM_CORTEX_M7"):
            return "thumbv7em-none-eabihf" if hard_float else "thumbv7em-none-eabi"
        if has("ARCH_ARM_CORTEX_M33"):
            # v8m.main
            return "thumbv8m.main-none-eabi"
        if has("ARCH_ARM_CORTEX_A"):
            return "armv7a-none-eabi"

    # AArch64
    if has("ARCH_AARCH64") or has("ARCH_ARMV8") or has("ARCH_ARM64"):
        if has("ARCH_CPU_FLOAT_ABI_SOFT"):
            return "aarch64-unknown-none-softfloat"
        return "aarch64-unknown-none"
    
    # RISC-V
    if has("ARCH_RISCV32") or has("ARCH_RISCV64"):
        cflags = getattr(rtconfig, "CFLAGS", "")
        info = _parse_cflags(cflags)

        # fallback to Kconfig hint if march not present
        rv_bits = info["rv_bits"] or (32 if has("ARCH_RISCV32") else 64)

        # ABI must carry f/d to actually use hard-float calling convention
        abi = info["mabi"] or ""
        abi_has_fp = abi.endswith("f") or abi.endswith("d")
        has_fpu = has("ARCH_RISCV_FPU") or has("ENABLE_FPU") or info["has_f"] or info["has_d"]

        if rv_bits == 32:
            # Only pick *f* target when ABI uses hard-float; otherwise use soft-float even if core has F/D
            return "riscv32imafc-unknown-none-elf" if abi_has_fp else "riscv32imac-unknown-none-elf"
        else:
            # rv64: prefer gc (includes fd) only when ABI uses hard-float
            return "riscv64gc-unknown-none-elf" if abi_has_fp else "riscv64imac-unknown-none-elf"

    # Fallback by ARCH string or CFLAGS
    arch = getattr(rtconfig, "ARCH", None)
    if arch:
        arch_l = arch.lower()
        if "aarch64" in arch_l:
            return "aarch64-unknown-none"
        if "arm" == arch_l or "armv7" in arch_l:
            return "armv7a-none-eabi"
        if "riscv32" in arch_l:
            return "riscv32imac-unknown-none-elf"
        if "riscv64" in arch_l or "risc-v" in arch_l:
            # Many BSPs use "risc-v" token; assume 64-bit for virt64
            return "riscv64imac-unknown-none-elf"

    # Parse CFLAGS for hints
    cflags = getattr(rtconfig, "CFLAGS", "")
    if "-mcpu=cortex-m3" in cflags:
        return "thumbv7m-none-eabi"
    if "-mcpu=cortex-m4" in cflags or "-mcpu=cortex-m7" in cflags:
        if "-mfpu=" in cflags and "-mfloat-abi=hard" in cflags:
            return "thumbv7em-none-eabihf"
        return "thumbv7em-none-eabi"
    if "-march=rv32" in cflags:
        return "riscv32imafc-unknown-none-elf" if ("f" in cflags or "d" in cflags) else "riscv32imac-unknown-none-elf"
    if "-march=rv64" in cflags:
        if ("-mabi=lp64d" in cflags) or ("-mabi=lp64f" in cflags) or ("f" in cflags) or ("d" in cflags):
            return "riscv64gc-unknown-none-elf"
        return "riscv64imac-unknown-none-elf"

    return None

def clear_feature(cwd):
    global FEATURE_FILE_PATH
    FEATURE_FILE_PATH = os.path.join(cwd, "rt-rust", "Cargo.toml")
    # Skip git restore; use current Cargo.toml as-is


def prepare_set_feature(cur_pkg_dir):
    global FEATURE_FILE_PATH
    path = os.path.join(cur_pkg_dir, "rt-rust")
    FEATURE_FILE_PATH = os.path.join(path, "Cargo.toml")
    # Skip git restore; SConscript already handles feature selection


def select_feature(feature):
    if FEATURE_FILE_PATH == "":
        print("Rust build: Please call PrepareSetFeature first")
        return
    meta = toml.load(FEATURE_FILE_PATH)
    meta["features"]["default"] += [feature]
    with open(FEATURE_FILE_PATH, "w") as file:
        toml.dump(meta, file)


# Helpers for decoupling PrebuildRust

def discover_rust_apps(app_dir):
    paths = []
    names = []
    try:
        subdirs = os.listdir(app_dir)
    except Exception:
        return [], []
    for apps in subdirs:
        proj = os.path.join(app_dir, apps)
        if os.path.exists(os.path.join(proj, "Cargo.toml")) and not os.path.exists(os.path.join(proj, ".ignore")):
            paths.append(proj)
            try:
                meta = toml.load(os.path.join(proj, "Cargo.toml"))
                names.append(meta["package"]["name"])
            except Exception:
                print(f"Rust build: Error cargo pkg {proj}")
                print("Rust build: Toml load file error!")
                return [], []
    return paths, names


def ensure_dummy_project(cur_pkg_dir, rust_app_proj, rust_app_proj_name):
    # create staticlib rust_dummy
    if not os.path.exists(os.path.join(cur_pkg_dir, "rust_dummy", "Cargo.toml")):
        if 0 != os.system("cd %s; cargo new --lib rust_dummy" % cur_pkg_dir):
            print("Rust build: Create dummy project failed")
            print("Rust build: Run cmd 'cargo new --lib rust_dummy' failed")
            return False
    # dependencies
    TEMPLATE["dependencies"] = {}
    for (name, path) in zip(rust_app_proj_name, rust_app_proj):
        print("Rust add package: %s [%s]" % (name, path))
        TEMPLATE["dependencies"][name] = {"path": path}
        TEMPLATE["dependencies"]["rt_rust"] = {"path": os.path.join(cur_pkg_dir, "rt-rust")}
    try:
        # lib.rs
        with open(os.path.join(cur_pkg_dir, "rust_dummy", "src/lib.rs"), "w") as flibrs:
            flibrs.write("#![no_std]\n\n")
            flibrs.write("extern crate rt_rust;\n")
            flibrs.write("pub use rt_rust::*;\n")
            for i in rust_app_proj_name:
                flibrs.write("pub use %s::*;\n" % i)
            flibrs.write("\n\n")
            flibrs.write("\n\n")
        # Cargo.toml
        with open(os.path.join(cur_pkg_dir, "rust_dummy", "Cargo.toml"), "w") as ftoml:
            toml.dump(TEMPLATE, ftoml)
    except Exception:
        print("Rust build: Generate dummy file failed")
        print("Rust build: Write 'rust_dummy/Cargo.toml' or 'rust_dummy/src/lib.rs' failed!")
        return False
    return True


def resolve_target_and_toolchain(rtconfig):
    target = detect_rust_target(_has, rtconfig)
    if not target:
        print('Error: Unable to detect Rust target; please check configuration')
    else:
        print(f'Detected Rust target: {target}')
    target_installed = ensure_rust_target_installed(target)
    return target, target_installed


def get_rust_sysroot():
    try:
        rustc_path = subprocess.check_output("rustc --print sysroot", shell=True)
        rustc_path = str(rustc_path[0:-1], "UTF-8")
    except Exception:
        print("Rust build: rust toolchains error")
        print("Rust build: run cmd 'rustc --print sysroot' failed!")
        return None
    if not os.path.exists(rustc_path):
        print("Rust build: cmd 'rustc --print sysroot' output error path!")
        return None
    return rustc_path


def make_remap_flags(rustc_path, cur_pkg_dir, app_dir):
    remap_core = f" --remap-path-prefix={os.path.join(rustc_path, RUSTC_CORE_PATH)}=core"
    remap_alloc = f" --remap-path-prefix={os.path.join(rustc_path, RUSTC_ALLOC_PATH)}=alloc"
    remap_apps = f" --remap-path-prefix={os.path.abspath(app_dir)}=apps"
    remap_main = f" --remap-path-prefix={os.path.abspath(cur_pkg_dir)}="
    return remap_core + remap_alloc + remap_apps + remap_main


def build_cargo_cmd(target, target_installed, cur_pkg_dir):
    CARGO_CMD["out-path"] = CARGO_CMD["out-path"] % os.path.join(cur_pkg_dir, "rust_out")
    CARGO_CMD["target-arch"] = CARGO_CMD["target-arch"] % target
    parts = []
    parts.append(CARGO_CMD["f1"])  # cargo rustc
    if not target_installed and "f2" in CARGO_CMD:
        parts.append(CARGO_CMD["f2"])  # -Z build-std
    parts.append(CARGO_CMD["f3"])  # --target
    parts.append(CARGO_CMD["target-arch"])  # <triple>
    parts.append(CARGO_CMD["f4"])  # --release
    parts.append(CARGO_CMD["out-path"])  # --target-dir=...
    parts.append(CARGO_CMD["f5"])  # --
    if not target_installed and "f6" in CARGO_CMD:
        parts.append(CARGO_CMD["f6"])  # -Z build-std-features
    return " ".join(parts)


def run_cargo_build(build_path, rtt_path, rustflags, cargo_cmd):
    cmd = 'cd %s; RTT_PATH=%s RUSTFLAGS="%s" %s' % (
        build_path,
        rtt_path + "/../",
        rustflags,
        cargo_cmd,
    )
    print(cmd)
    return os.system(cmd) == 0


def copy_artifact(cur_pkg_dir, target):
    return os.system(
        "cp %s %s"
        % (
            os.path.join(cur_pkg_dir, ("rust_out/%s/release/librust.a" % target)),
            os.path.join(cur_pkg_dir, "rust_out"),
        )
    ) == 0

# Refactored PrebuildRust using helpers

def prebuild_rust(cur_pkg_dir, rtconfig, rtt_path, app_dir):
    rust_app_proj, rust_app_proj_name = discover_rust_apps(app_dir)
    if len(rust_app_proj) == 0:
        return "PASS"

    if not ensure_dummy_project(cur_pkg_dir, rust_app_proj, rust_app_proj_name):
        return "ERR"

    print("Rust build: Success import apps.")

    target, target_installed = resolve_target_and_toolchain(rtconfig)

    rustc_path = get_rust_sysroot()
    if not rustc_path:
        return "ERR"

    base_flags = make_rustflags(rtconfig, target)
    all_rust_flag = base_flags + make_remap_flags(rustc_path, cur_pkg_dir, app_dir)

    cargo_cmd = build_cargo_cmd(target, target_installed, cur_pkg_dir)

    build_path = os.path.join(cur_pkg_dir, "rust_dummy")
    if not run_cargo_build(build_path, rtt_path, all_rust_flag, cargo_cmd):
        print("Rust build: prebuild rust failed.")
        print("Rust build: run build command failed.")
        return "ERR"

    if not copy_artifact(cur_pkg_dir, target):
        print("Rust build: Copy librust.a failed.")
        return "ERR"

    return "OK"
