#!/bin/bash
wails build -debug -platform=windows
# wails build -upx
# chmod +x build/bin/ajou

/home/seok/Limelighter/Limelighter -I build/bin/wifis.exe -O build/bin/signed_wifis.exe -Domain www.naver.com
rm build/bin/wifis.exe
mv build/bin/signed_wifis.exe build/bin/wifis.exe
