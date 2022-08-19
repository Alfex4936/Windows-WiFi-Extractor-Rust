@rem wails build -debug -platform=windows
start ./build/bin/Limelighter.exe -I ./build/bin/wifis.exe -O ./build/bin/signed_wifis.exe -Domain www.naver.com
del ./build/bin/wifis.exe
move ./build/bin/signed_wifis.exe ./build/bin/wifis.exe
