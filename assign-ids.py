import sys
from pathlib import Path
import secrets
import string

md_dir = Path(sys.argv[1])
# Exclude markdown files directly in md_dir
md_files = list(md_dir.glob("*/*.md")) + list(md_dir.glob("*/*/*.md"))

# Current email address
email = "anna@annaaurora.eu"

for md_file in md_files:
	with open(md_file, "r+") as fp:
		contents = fp.readlines()
		for i, line in enumerate(contents):
			if i == 0:
				assert line == "```toml\n", f"{md_file} doesn't start with toml code block, 1th line is: {line}"
			elif i == 1:
				if line.startswith("id = \"") == False:
					print(f"{md_file} is missing an id, assigning one…")
					random = secrets.token_hex(1)
					# 1 Byte is enough because it is only used to uniquely identify posts per day
					contents.insert(1, f"atom_id_parts = {{ email = \"{email}\", object = \"{random}\" }}\n")
				else:
					print(f"{md_file} already has an id")

		fp.seek(0)
		fp.writelines(contents)

