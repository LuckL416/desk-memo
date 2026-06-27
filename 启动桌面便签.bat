@echo off
cd /d F:\sticky-notes
set PATH=%USERPROFILE%\.cargo\bin;%PATH%
start "" /B npm run tauri dev
