
test:
	cargo +nightly test


miri:
	MIRIFLAGS="-Zmiri-disable-isolation -Zmiri-ignore-leaks" cargo +nightly miri test
