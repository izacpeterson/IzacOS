build:
	cd init && cargo build --release && cp ./target/release/init ../rootfs/
	cd rootfs/bin && ln -sf busybox mount
	cd rootfs/bin && ln -sf busybox ls

pack: 
	cd rootfs && find . | cpio -o -H newc | gzip > ../initramfs.cpio.gz

boot: 
	qemu-system-x86_64 \
		-kernel /home/izac/Dev/linux-6.12/arch/x86/boot/bzImage \
		-initrd initramfs.cpio.gz \
		-append "console=ttyS0 quiet" \
		-nographic \
		-m 16G \
		-enable-kvm \
		-cpu host \
		-nic user,hostfwd=tcp::3000-:3000

run:
	make build
	make pack
	make boot