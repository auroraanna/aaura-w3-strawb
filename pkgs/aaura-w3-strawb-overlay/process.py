import sys
from pathlib import Path
from ffmpy import FFmpeg
from PIL import Image, features, ImageOps, ImageCms
import tempfile

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
		img = Image.open(fp=path, formats=("JPEG", "PNG", "WEBP"))
		imgFormat = img.format

		# Convert color profile to sRGB because web browsers are bad at any color profile but sRGB
		icc = img.info.get("icc_profile")
		if type(icc) is bytes:
			icc_path = tempfile.mkstemp(suffix=".icc")[1]
			with open(icc_path, "wb") as f:
				f.write(icc)
		else:
			# AdobeRGB photos taken on a dedicated camera don't include the color profile data.
			icc_path = "Compatible with Adobe RGB (1998).icc"
		
		srgb = ImageCms.createProfile("sRGB")
		img = ImageCms.profileToProfile(img, icc_path, srgb, ImageCms.Intent.RELATIVE_COLORIMETRIC)
		
		savePath = str(savePath).rsplit(".")[0] + "-lossier"
		# JPEG Photos with AdobeRGB taken on a dedicated camera are MP0 for some reason.
		if imgFormat == "JPEG" or imgFormat == "MPO":
			saveFormat = "JPEG"
			savePath = savePath + ".jpg"
		else:
			saveFormat = "WEBP"
			savePath = savePath + ".webp"

		ImageOps.exif_transpose(
			image=img,
			in_place=True
		)

		ratio = img.height // img.width
		width = img.width // 3
		height = img.height // 3
		if width < 800:
			width = 800
			height = width * ratio
		
		img = img.resize((
			width,
			height
		))

		img.save(
			fp=savePath,
			format=saveFormat,
			quality=60,
		)
		img.close()
