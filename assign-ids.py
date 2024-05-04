import sys
from pathlib import Path
from base64 import urlsafe_b64encode as b64encode, urlsafe_b64decode as b64decode
import struct
import string
import tomlkit
from datetime import datetime

md_dir = Path(sys.argv[1])
# Exclude markdown files directly in md_dir
md_files = list(md_dir.glob("*/*.md")) + list(md_dir.glob("*/*/*.md"))

# Current email address
email = "anna@annaaurora.eu"

frontmatters = []

for md_file in md_files:
	with open(md_file, "r+") as fp:
		contents = fp.readlines()
		for i, line in enumerate(contents):
			line_fm_end: int
			if i == 0:
				assert line == "```toml\n", f"{md_file} doesn't start with toml code block, 1th line is: {line}"
			if line == "```\n":
				line_fm_end = i
				break

		frontmatter_list = []
		for i, line in enumerate(contents):
			if i != 0 and i < line_fm_end:
				frontmatter_list.append(line)

		frontmatter = tomlkit.parse("".join(frontmatter_list))
		date2 = datetime.fromisoformat(frontmatter["date_published"])

		# Make sure that the atom_id_parts.object is not the same for any frontmatters with the same date
		obj2 = 0
		for fm in frontmatters:
			date = datetime.fromisoformat(fm["date_published"])
			obj = int("0x" + fm["atom_id_parts"]["object"], 16)
			# Check if date, not time is the same
			if date.year == date2.year and date.month == date2.month and date.day == date2.day:
				while obj == obj2:
					obj2 += 1

		if "atom_id_parts" not in frontmatter:
			print(f"{md_file} doesn't have atom_id_parts yet, assigning…")
			frontmatter["atom_id_parts"] = {
				"email": email,
				"object": hex(obj2).removeprefix("0x")
			}

		frontmatters.append(frontmatter)

		# dict to TOML string
		fm_str = tomlkit.dumps(frontmatter)
		# Set writehead to start
		fp.seek(0)
		# Write the frontmatter
		fp.write("```toml\n")
		fp.write(fm_str)
		fp.write("```\n")
		# Write rest of the file
		for i, line in enumerate(contents):
			if i > line_fm_end:
				fp.write(line)

