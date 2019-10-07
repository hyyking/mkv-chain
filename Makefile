bump:
	sed -i 's/$(OLD)\"/$(NEW)\"/g' *.md *.toml **/*.rs
