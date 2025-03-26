```ps
New-Service -Name "DragonCompanion" -DisplayName "Dragon Companion" -Description "Dragon Companion WebServer" -StartupType Manual -BinaryPathName "C:\projects\companion\target\release\companion.exe --service"
```
