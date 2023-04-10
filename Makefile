kernel = target/x86_64-unknown-uefi/release/kernel.efi
image = image.bin

$(image): $(kernel)
	mkdir -p esp
	sudo mount efi esp
	sudo mkdir -p esp/EFI/BOOT
	sudo cp $(kernel) esp/EFI/BOOT/BOOTX64.EFI
	sudo umount efi
	cat gpt_table efi > $(image)

$(kernel): src
	cargo +nightly build --target=x86_64-unknown-uefi -Z build-std=core,alloc --release

flash: $(image)
	sudo dd if=$(image) of=/dev/sdb && sync

test: $(image)
	qemu-system-x86_64 -drive if=pflash,format=raw,readonly=on,file=/usr/share/OVMF/OVMF_CODE.fd -drive if=pflash,format=raw,readonly=on,file=/usr/share/OVMF/OVMF_VARS.fd $(image)

clean:
	rm -rf esp $(image)
	cargo clean
