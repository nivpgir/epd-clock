

# justfile_dir := `cygpath {{justfile_directory()}}`
run-pi:
	# target_dir={{justfile_directory()}} echo $target_dir
	ssh pi@raspberrypi.local "pkill epd-clock" || true
	cd ~/experiments/epd-clock/rpi && cargo build
	scp "{{justfile_directory()}}"/target/arm-unknown-linux-gnueabihf/debug/rpi pi@raspberrypi.local:/tmp/epd-clock
	ssh pi@raspberrypi.local "chmod +x /tmp/epd-clock && /tmp/epd-clock" &
	# sleep 5
	# ssh pi@raspberrypi.local "pkill epd-clock" || true
	# popd
