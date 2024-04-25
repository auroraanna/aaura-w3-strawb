import sys
from pathlib import Path
from ffmpy import FFmpeg
from PIL import Image, features

out = Path(sys.argv[2])

paths = list(Path(sys.argv[1]).rglob("*"))
reducedPaths = paths.copy()

# Sort out non-image paths
for path in paths:
	print(path)

	if (path.suffix == ".md") or (path.suffix == ".mmpz") or path.is_dir():
		reducedPaths.remove(path)

for path in reducedPaths:
	parts = list(path.parts)
	for i in range(4):
		parts.pop(0)

	savePath = out
	for part in parts:
		savePath = savePath / part

	dirParts = list(savePath.parts)
	dirParts.pop()
	dir = Path(dirParts[0])
	dirParts.pop(0)
	for part in dirParts:
		dir = dir / part
	dir.mkdir(
		parents=True,
		exist_ok=True
	)

	# Transcode lossless audio to lossy
	if path.suffix == ".flac":
		savePath = str(savePath).removesuffix(".flac") + "-lossier" + ".opus"

		ff = FFmpeg(
			inputs={path: None},
			outputs={savePath: "-b:a 160k"}
		)
		print(ff.cmd)
		ff.run()
	# Resize and save image with reduced quality
	else:
		img = Image.open(fp=path, formats=("JPEG", "PNG"))
		
		savePath = str(savePath).rsplit(".")[0] + "-lossier"
		if img.format == "JPEG":
			saveFormat = "JPEG"
			savePath = savePath + ".jpg"
		else:
			saveFormat = "WEBP"
			savePath = savePath + ".webp"

		img = img.resize((
			img.width // 3,
			img.height // 3
		))

		img.save(
			fp=savePath,
			format=saveFormat,
			quality=50
		)
