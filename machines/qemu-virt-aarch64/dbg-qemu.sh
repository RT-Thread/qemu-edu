if [ ! -f "sd.bin" ]; then
dd if=/dev/zero of=sd.bin bs=1024 count=65536
fi

qemu-system-aarch64 --version
qemu-system-aarch64 -M virt,gic-version=2 -cpu cortex-a53 -m 128M -kernel rtthread.bin -nographic \
-drive if=none,file=sd.bin,format=raw,id=blk0 -device virtio-blk-device,drive=blk0,bus=virtio-mmio-bus.0 \
-netdev user,id=net0 -device virtio-net-device,netdev=net0,bus=virtio-mmio-bus.1 \
-device virtio-serial-device -S -s
