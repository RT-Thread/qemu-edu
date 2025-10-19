import os
import sys
from building import *

Import('rtconfig')

cwd = GetCurrentDir()

# Import helper utilities
sys.path.append(os.path.join(cwd, 'tools'))
from build_support import PrebuildRust
from build_support import ClearFeature
from build_support import SeleceFeature
from build_support import PrepareSetFeature
from build_support import verify_rust_toolchain


def _has(sym: str) -> bool:
    try:
        return bool(GetDepend([sym]))
    except Exception:
        return bool(GetDepend(sym))

# Not enabled? Return empty group early
if not _has('RT_USING_RUST'):
    group = []
    Return('group')

# Source files – MSH command glue
src = ['rt-rust/rust_cmd.c']
LIBS = []
LIBPATH = []

if GetOption('clean'):
    ClearFeature(cwd)
    os.system("cd %s; rm -rf rust_out" % cwd)
    os.system("cd %s; rm -rf rust_dummy" % cwd)
    os.system("cd %s; rm -rf build" % cwd)
    group = DefineGroup('rust', src, depend=[])
else:
    if verify_rust_toolchain():
        PrepareSetFeature(cwd)
        if GetDepend("RT_USING_SMP"):
            SeleceFeature("smp")
        ret = PrebuildRust(cwd, rtconfig, Rtt_Root, Rtt_Root+"/../machines/qemu-virt-riscv64/rust-std/rtt_rust_example_app")
        if ret == "OK":
            LINKFLAGS = " -L%s" % (cwd + "/rust_out/")
            LINKFLAGS += " -Wl,--whole-archive -lrust -Wl,--no-whole-archive"
            LINKFLAGS += " -Wl,--allow-multiple-definition"
        elif ret == "PASS":
            pass
        elif ret == "ERR":
            raise Exception("RUST BUILD FATAL ERROR!!!")
    else:
        print('Warning: Rust toolchain not found')
        print('Please install Rust from https://rustup.rs')

    group = DefineGroup('rust', src, depend=['RT_USING_RUST'], LINKFLAGS=LINKFLAGS, LIBS=LIBS, LIBPATH=LIBPATH)

Return('group')