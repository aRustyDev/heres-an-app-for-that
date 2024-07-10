#!/bin/sh

set -eu

# create_iconset() {
# 	mkdir -p Ghidra.iconset
# 	cat "iconset.json" > Ghidra.iconset/Contents.json
# 	for size in 16 32 64 128 256 512 1024; do
# 		convert "$1" -resize "${size}x${size}" "Ghidra.iconset/icon_${size}x${size}.png"
# 	done
# }

# # Get the path to the Ghidra folder
# if [ $# -ne 1 ]; then
# 	echo "Usage: $0 [path to ghidra folder]" >&2
# 	exit 1
# fi

# # Create the app bundle
# mkdir -p Ghidra.app/Contents/MacOS
# cat "1.cs" | clang -x objective-c -fmodules -framework Foundation -o Ghidra.app/Contents/MacOS/Ghidra -

# # Copy the Ghidra files
# mkdir -p Ghidra.app/Contents/Resources/
# rm -rf Ghidra.app/Contents/Resources/ghidra
# # Copy Given PATHs contents to the app bundle
# cp -R "$(echo "$1" | sed s,/*$,,)" Ghidra.app/Contents/Resources/ghidra

# sed "s/bg Ghidra/fg Ghidra/" < "$1/ghidraRun" > Ghidra.app/Contents/Resources/ghidra/ghidraRun
# sed "s/apple.laf.useScreenMenuBar=false/apple.laf.useScreenMenuBar=true/" < "$1/support/launch.properties" > Ghidra.app/Contents/Resources/ghidra/support/launch.properties
# echo "APPL????" > Ghidra.app/Contents/PkgInfo

# jar -x -f Ghidra.app/Contents/Resources/ghidra/Ghidra/Framework/Gui/lib/Gui.jar images/GhidraIcon256.png
# if [ "$( (sw_vers -productVersion; echo "11.0") | sort -V | head -n 1)" = "11.0" ]; then
# 	convert \( -size 1024x1024 canvas:none -fill white -draw 'roundRectangle 100,100 924,924 180,180' \) \( +clone -background black -shadow 25x12+0+12 \) +swap -background none -layers flatten -crop 1024x1024+0+0 \( images/GhidraIcon256.png -resize 704x704 -gravity center \) -composite GhidraIcon.png
# else
# 	mv images/GhidraIcon256.png GhidraIcon.png
# fi

# create_iconset GhidraIcon.png
# for size in 16 24 32 40 48 64 128 256; do
# 	convert GhidraIcon.png -resize "${size}x${size}" "images/GhidraIcon${size}.png"
# 	jar -u -f Ghidra.app/Contents/Resources/ghidra/Ghidra/Framework/Generic/lib/Generic.jar "images/GhidraIcon${size}.png"
# done

# iconutil -c icns Ghidra.iconset
# cp Ghidra.icns Ghidra.app/Contents/Resources
SetFile -a B Ghidra.app
cat "Info.plist" > Ghidra.app/Contents/Info.plist


mkdir -p docking/widgets/filechooser/
cat "GhidraFileChooser.java" > docking/widgets/filechooser/GhidraFileChooser.java


javac -cp "$(find Ghidra.app -regex '.*\.jar' | tr '\n' ':')" docking/widgets/filechooser/GhidraFileChooser.java
cp -R docking Ghidra.app/Contents/Resources/Ghidra/ghidra/patch/

cat "OpenGhidra.java" > OpenGhidra.java

javac OpenGhidra.java
cp OpenGhidra.class Ghidra.app/Contents/Resources
cat "OpenGhidraAgent.java" > OpenGhidraAgent.java


javac -cp "$(find Ghidra.app -regex '.*\.jar' | tr '\n' ':')" OpenGhidraAgent.java
cat "manifest" > manifest

jar --create --file OpenGhidra.jar --manifest manifest OpenGhidraAgent*.class
cp OpenGhidra.jar Ghidra.app/Contents/Resources
